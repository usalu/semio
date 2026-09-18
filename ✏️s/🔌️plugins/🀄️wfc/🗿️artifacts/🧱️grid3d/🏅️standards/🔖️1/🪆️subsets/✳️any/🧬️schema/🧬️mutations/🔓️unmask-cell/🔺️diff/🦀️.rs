//! 🔺️ Sparse diff builder for `UnmaskCell` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::UnmaskCell, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let key = cell_key(payload.x, payload.y, payload.z);
    if masked_index(base, payload.x, payload.y, payload.z).is_none() {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("Cell {key} is not masked."), [key]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { masked_removed: vec![key], ..Default::default() })
}
