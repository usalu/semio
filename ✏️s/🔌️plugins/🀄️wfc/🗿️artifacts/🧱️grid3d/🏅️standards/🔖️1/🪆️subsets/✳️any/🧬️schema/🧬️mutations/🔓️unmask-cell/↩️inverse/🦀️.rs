//! ↩️ Inverse for `UnmaskCell` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::UnmaskCell, base: &Grid3dSnapshot) -> Result<Vec<Grid3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match masked_index(base, payload.x, payload.y, payload.z) {
        Some(index) => vec![crate::mutations::mask_cell(base.masked[index])],
        None => Vec::new(),
    }

    })())
}
