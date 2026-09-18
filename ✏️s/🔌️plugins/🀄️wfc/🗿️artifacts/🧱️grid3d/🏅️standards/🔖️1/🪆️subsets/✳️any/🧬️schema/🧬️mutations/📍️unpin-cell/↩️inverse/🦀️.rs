//! ↩️ Inverse for `UnpinCell` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::UnpinCell, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
    match pinned_index(base, payload.x, payload.y, payload.z) {
        Some(index) => vec![crate::mutations::pin_cell(base.pinned[index].clone())],
        None => Vec::new(),
    }
}
