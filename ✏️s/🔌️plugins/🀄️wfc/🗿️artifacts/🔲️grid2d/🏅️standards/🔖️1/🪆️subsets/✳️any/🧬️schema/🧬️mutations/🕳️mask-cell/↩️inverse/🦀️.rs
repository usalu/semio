//! ↩️ Inverse for `MaskCell` — unmasks the cell and restores the pin the mask cascaded away.

use crate::mutations::{pin_cell, unmask_cell, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::MaskCell, base: &Grid2dSnapshot) -> Result<Vec<Grid2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.masked.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return Vec::new();
    }
    let mut restore = vec![unmask_cell(payload.x, payload.y)];
    if let Some(cell) = base.pinned.iter().find(|cell| cell.x == payload.x && cell.y == payload.y) {
        restore.push(pin_cell(cell.x, cell.y, cell.tile_id.clone()));
    }
    restore.reverse();
    restore

    })())
}
