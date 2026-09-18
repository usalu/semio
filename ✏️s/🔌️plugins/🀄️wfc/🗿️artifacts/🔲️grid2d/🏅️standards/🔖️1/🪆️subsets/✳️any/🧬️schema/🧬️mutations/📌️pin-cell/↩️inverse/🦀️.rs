//! ↩️ Inverse for `PinCell` — re-pins the tile the cell carried before, or unpins it when the cell
//! was free.

use crate::mutations::{pin_cell, unpin_cell, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::PinCell, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    match base.pinned.iter().find(|cell| cell.x == payload.x && cell.y == payload.y) {
        Some(cell) if cell.tile_id == payload.tile_id => Vec::new(),
        Some(cell) => vec![pin_cell(cell.x, cell.y, cell.tile_id.clone())],
        None => vec![unpin_cell(payload.x, payload.y)],
    }
}
