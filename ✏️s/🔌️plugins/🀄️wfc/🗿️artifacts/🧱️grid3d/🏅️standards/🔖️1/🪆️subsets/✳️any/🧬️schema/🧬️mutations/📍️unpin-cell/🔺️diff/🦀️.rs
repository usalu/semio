//! 🔺️ Sparse diff builder for `UnpinCell` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dRows};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::UnpinCell, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let key = cell_key(payload.x, payload.y, payload.z);
    if pinned_index(base, payload.x, payload.y, payload.z).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Cell {key} carries no pin."), [key]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { pinned: Grid3dRows { removed: vec![key], ..Default::default() }, ..Default::default() })
}
