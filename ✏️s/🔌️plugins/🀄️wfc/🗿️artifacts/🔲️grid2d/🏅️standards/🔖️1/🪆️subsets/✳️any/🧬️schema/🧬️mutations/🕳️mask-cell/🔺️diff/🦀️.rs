//! 🔺️ Sparse diff builder for `MaskCell` — a row-major upsert into `masked` that also cascades the
//! cell's pin away: a cell outside the problem can carry no pre-assignment.

use crate::diff::{cell_id, Grid2dDiff, Grid2dRow, Grid2dMaskedDelta, Grid2dPinnedDelta};
use crate::schema::snapshot::{in_bounds, Grid2dSnapshot, WfcCell2d};

pub fn diff(payload: &super::MaskCell, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    let target = cell_id(payload.x, payload.y);
    if !in_bounds(base, payload.x, payload.y) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell ({}, {}) is outside the {}×{} grid.", payload.x, payload.y, base.width, base.height), [target]);
    }
    if base.masked.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Cell ({}, {}) is already masked.", payload.x, payload.y));
    }
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|cell| cell.x == payload.x && cell.y == payload.y).map(|cell| cell_id(cell.x, cell.y)).collect();
    let cascaded = !pinned_removed.is_empty();
    let outcome = protocol::MutationOutcome::new(Grid2dDiff { masked: Grid2dMaskedDelta::insertion(Grid2dRow::insert_at(&base.masked, &WfcCell2d { x: payload.x, y: payload.y }), WfcCell2d { x: payload.x, y: payload.y }), pinned: Grid2dPinnedDelta::removals(&base.pinned, &base.pinned.iter().enumerate().filter(|(_, row)| pinned_removed.contains(&protocol::list_delta::Keyed::key(*row))).map(|(index, _)| index).collect::<Vec<_>>()), ..Default::default() });
    if cascaded {
        outcome.info("mutation.cascade", format!("Masking cell ({}, {}) also dropped its pin.", payload.x, payload.y))
    } else {
        outcome
    }
}
