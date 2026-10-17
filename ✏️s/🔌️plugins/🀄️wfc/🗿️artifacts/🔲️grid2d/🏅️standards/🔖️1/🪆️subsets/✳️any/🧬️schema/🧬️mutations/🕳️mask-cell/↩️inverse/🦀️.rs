//! ↩️ Inverse for `MaskCell` — unmasks the cell and restores the pin the mask cascaded away.

use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::standards::v1::subsets::any::schema::mutations::unmask_cell::unmask_cell;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::MaskCell, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell>,
{
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
