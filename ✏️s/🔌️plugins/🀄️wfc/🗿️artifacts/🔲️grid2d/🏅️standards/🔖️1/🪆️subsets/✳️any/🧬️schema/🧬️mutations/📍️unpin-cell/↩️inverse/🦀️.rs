//! ↩️ Inverse for `UnpinCell` — re-pins the exact tile from a real BASE lookup.

use crate::mutations::{pin_cell, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::UnpinCell, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    match base.pinned.iter().find(|cell| cell.x == payload.x && cell.y == payload.y) {
        Some(cell) => vec![pin_cell(cell.x, cell.y, cell.tile_id.clone())],
        None => Vec::new(),
    }
}
