//! 🔺️ Sparse diff builder for `MaskCell` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::MaskCell, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let key = cell_key(payload.cell.x, payload.cell.y, payload.cell.z);
    if !cell_in_grid(base, payload.cell.x, payload.cell.y, payload.cell.z) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is outside the grid."), [key]);
    }
    if pinned_index(base, payload.cell.x, payload.cell.y, payload.cell.z).is_some() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is pinned; unpin it before masking."), [key]);
    }
    if masked_index(base, payload.cell.x, payload.cell.y, payload.cell.z).is_some() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cell {key} is already masked."));
    }
    let at = crate::mutations::ordered_index(&base.masked, &key, |cell| cell_key(cell.x, cell.y, cell.z));
    protocol::MutationOutcome::new(Grid3dDiff { masked_upserted: vec![(at, payload.cell)], ..Default::default() })
}
