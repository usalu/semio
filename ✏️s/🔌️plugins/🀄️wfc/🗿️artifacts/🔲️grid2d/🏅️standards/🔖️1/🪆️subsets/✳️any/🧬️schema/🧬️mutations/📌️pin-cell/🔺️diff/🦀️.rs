//! 🔺️ Sparse diff builder for `PinCell` — a row-major upsert into `pinned`, refusing an
//! out-of-bounds cell, an unknown tile, or a cell the mask has already cut out of the problem.

use crate::diff::Grid2dDiff;
use crate::mutations::ordered_cell_index;
use crate::schema::snapshot::{in_bounds, Grid2dSnapshot, WfcPinnedCell2d};

pub fn diff(payload: &super::PinCell, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    let target = format!("{},{}", payload.x, payload.y);
    if !in_bounds(base, payload.x, payload.y) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell ({}, {}) is outside the {}×{} grid.", payload.x, payload.y, base.width, base.height), [target]);
    }
    if !base.tiles.iter().any(|tile| tile.id == payload.tile_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell ({}, {}) pins unknown tile \"{}\".", payload.x, payload.y, payload.tile_id), [payload.tile_id.clone()]);
    }
    if base.masked.iter().any(|cell| cell.x == payload.x && cell.y == payload.y) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell ({}, {}) is masked and cannot be pinned.", payload.x, payload.y), [target]);
    }
    let cell = WfcPinnedCell2d { x: payload.x, y: payload.y, tile_id: payload.tile_id.clone() };
    match base.pinned.iter().position(|existing| existing.x == payload.x && existing.y == payload.y) {
        Some(index) if base.pinned[index] == cell => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cell ({}, {}) is already pinned to \"{}\".", payload.x, payload.y, payload.tile_id)),
        Some(index) => protocol::MutationOutcome::new(Grid2dDiff { pinned_upserted: vec![(index, cell)], ..Default::default() }),
        None => {
            let at = ordered_cell_index(&base.pinned, payload.x, payload.y, |existing| (existing.y, existing.x));
            protocol::MutationOutcome::new(Grid2dDiff { pinned_upserted: vec![(at, cell)], ..Default::default() })
        }
    }
}
