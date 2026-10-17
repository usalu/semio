//! ↩️ Inverse for `UnmaskCell` — masks the cell again.

use crate::standards::v1::subsets::any::schema::mutations::mask_cell::mask_cell;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::UnmaskCell, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell>,
{
    Ok((|| {
    if !base.masked.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return Vec::new();
    }
    vec![mask_cell(payload.x, payload.y)]

    })())
}
