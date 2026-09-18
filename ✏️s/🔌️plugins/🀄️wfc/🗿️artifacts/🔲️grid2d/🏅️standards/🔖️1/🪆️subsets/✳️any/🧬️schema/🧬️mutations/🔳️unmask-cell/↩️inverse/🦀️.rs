//! ↩️ Inverse for `UnmaskCell` — masks the cell again.

use crate::mutations::{mask_cell, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::UnmaskCell, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    if !base.masked.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return Vec::new();
    }
    vec![mask_cell(payload.x, payload.y)]
}
