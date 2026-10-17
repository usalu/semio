//! ↩️ Inverse for `UnpinCell` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::UnpinCell, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    Ok((|| {
    match pinned_index(base, payload.x, payload.y, payload.z) {
        Some(index) => vec![pin_cell(base.pinned[index].clone())],
        None => Vec::new(),
    }

    })())
}
