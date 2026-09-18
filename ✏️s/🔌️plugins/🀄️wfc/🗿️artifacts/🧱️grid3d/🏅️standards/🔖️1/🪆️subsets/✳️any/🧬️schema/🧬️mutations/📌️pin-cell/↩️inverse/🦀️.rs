//! ↩️ Inverse for `PinCell` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::PinCell, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
    match pinned_index(base, payload.pinned.x, payload.pinned.y, payload.pinned.z) {
        Some(index) => vec![crate::mutations::pin_cell(base.pinned[index].clone())],
        None => vec![crate::mutations::unpin_cell(payload.pinned.x, payload.pinned.y, payload.pinned.z)],
    }
}
