//! ↩️ Inverse for `PinCell` — re-pins the tile the cell carried before, or unpins it when the cell
//! was free.

use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::standards::v1::subsets::any::schema::mutations::unpin_cell::unpin_cell;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::PinCell, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_cell::UnpinCell>,
{
    Ok((|| {
    match base.pinned.iter().find(|cell| cell.x == payload.x && cell.y == payload.y) {
        Some(cell) if cell.tile_id == payload.tile_id => Vec::new(),
        Some(cell) => vec![pin_cell(cell.x, cell.y, cell.tile_id.clone())],
        None => vec![unpin_cell(payload.x, payload.y)],
    }

    })())
}
