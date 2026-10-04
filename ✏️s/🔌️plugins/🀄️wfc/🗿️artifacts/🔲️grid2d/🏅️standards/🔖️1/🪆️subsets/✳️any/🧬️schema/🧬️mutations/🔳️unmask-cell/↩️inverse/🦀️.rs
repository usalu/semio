//! ↩️ Inverse for `UnmaskCell` — masks the cell again.

use crate::mutations::{mask_cell, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::UnmaskCell, base: &Grid2dSnapshot) -> Result<Vec<Grid2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if !base.masked.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return Vec::new();
    }
    vec![mask_cell(payload.x, payload.y)]

    })())
}
