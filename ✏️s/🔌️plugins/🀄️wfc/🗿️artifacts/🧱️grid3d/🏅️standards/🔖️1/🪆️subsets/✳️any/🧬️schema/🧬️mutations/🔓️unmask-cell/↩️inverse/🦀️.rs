//! ↩️ Inverse for `UnmaskCell` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::mask_cell::mask_cell;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::UnmaskCell, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::mask_cell::MaskCell>,
{
    Ok((|| {
    match masked_index(base, payload.x, payload.y, payload.z) {
        Some(index) => vec![mask_cell(base.masked[index])],
        None => Vec::new(),
    }

    })())
}
