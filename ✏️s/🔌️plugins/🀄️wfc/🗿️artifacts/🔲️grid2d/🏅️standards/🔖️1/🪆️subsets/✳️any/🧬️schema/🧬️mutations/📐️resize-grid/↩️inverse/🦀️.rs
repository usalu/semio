//! ↩️ Inverse for `ResizeGrid` — restores the base extent first, then every pinned/masked cell the
//! shrink cascaded away, each landing back at its canonical row-major position.

use crate::mutations::{mask_cell, pin_cell, resize_grid, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::ResizeGrid, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    if base.width == payload.width && base.height == payload.height {
        return Vec::new();
    }
    let outside = |x: u32, y: u32| x >= payload.width || y >= payload.height;
    let mut restore = vec![resize_grid(base.width, base.height)];
    restore.extend(base.masked.iter().filter(|cell| outside(cell.x, cell.y)).map(|cell| mask_cell(cell.x, cell.y)));
    restore.extend(base.pinned.iter().filter(|cell| outside(cell.x, cell.y)).map(|cell| pin_cell(cell.x, cell.y, cell.tile_id.clone())));
    restore
}
