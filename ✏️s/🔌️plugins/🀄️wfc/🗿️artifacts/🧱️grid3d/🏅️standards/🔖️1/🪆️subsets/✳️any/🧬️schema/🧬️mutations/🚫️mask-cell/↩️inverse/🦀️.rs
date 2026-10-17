//! ↩️ Inverse for `MaskCell` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::unmask_cell::unmask_cell;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::MaskCell, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::unmask_cell::UnmaskCell>,
{
    Ok((|| {
    let _ = base;
    vec![unmask_cell(payload.cell.x, payload.cell.y, payload.cell.z)]

    })())
}
