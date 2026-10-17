//! ↩️ Inverse for `UnpinCell` — re-pins the exact tile from a real BASE lookup.

use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::UnpinCell, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    Ok((|| {
    match base.pinned.iter().find(|cell| cell.x == payload.x && cell.y == payload.y) {
        Some(cell) => vec![pin_cell(cell.x, cell.y, cell.tile_id.clone())],
        None => Vec::new(),
    }

    })())
}
