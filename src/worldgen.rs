use bevy_ecs_tilemap::{
    prelude::TilemapSize,
    tiles::{TilePos, TileTextureIndex},
};
use rand::prelude::*;

pub(crate) fn new_world(size: TilemapSize) -> Vec<(TilePos, TileTextureIndex)> {
    const WATER: u32 = 1;
    const GRASS: u32 = 0;
    const GROW_PERCENT: f32 = 45.0;

    //generate some random locations to start building continents from
    let mut growing_locs = Vec::new();
    let mut grid = vec![vec![WATER; size.x as usize]; size.y as usize];
    let mut rng = rand::thread_rng();
    for _ in 0..rng.gen_range(2..10) {
        let x = rng.gen_range(0..size.x as usize);
        let y = rng.gen_range(0..size.y as usize);
        growing_locs.push((x, y));
        grid[y][x] = GRASS;
    }

    while let Some((x, y)) = growing_locs.pop() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if rng.gen_range(0.0..=100.0) < GROW_PERCENT {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                if let Some(v @ &mut WATER) = grid.get_mut(ny).and_then(|v| v.get_mut(nx)) {
                    *v = GRASS;
                    growing_locs.push((nx, ny));
                }
            }
        }
    }
    //convert to required format and return
    let mut world = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &id) in row.iter().enumerate() {
            world.push((
                TilePos {
                    x: i as u32,
                    y: j as u32,
                },
                TileTextureIndex(id),
            ));
        }
    }
    world
}
