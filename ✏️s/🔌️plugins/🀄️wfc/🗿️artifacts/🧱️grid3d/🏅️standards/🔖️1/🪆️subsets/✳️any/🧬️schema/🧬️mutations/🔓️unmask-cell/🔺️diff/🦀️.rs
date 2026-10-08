//! 🔺️ Sparse diff builder for `UnmaskCell` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dMaskedDelta};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::UnmaskCell, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let key = cell_key(payload.x, payload.y, payload.z);
    if masked_index(base, payload.x, payload.y, payload.z).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Cell {key} is not masked."), [key]);
    }
    protocol::MutationOutcome::new(Grid3dDiff { masked: Grid3dMaskedDelta::removal(&base.masked, base.masked.iter().position(|row| protocol::list_delta::Keyed::key(row) == key).unwrap_or(usize::MAX)), ..Default::default() })
}
