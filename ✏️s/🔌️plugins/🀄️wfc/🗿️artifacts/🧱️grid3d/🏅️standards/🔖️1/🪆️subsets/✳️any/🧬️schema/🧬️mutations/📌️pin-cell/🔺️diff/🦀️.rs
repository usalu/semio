//! 🔺️ Sparse diff builder for `PinCell` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dPinnedPatch, Grid3dRow, Grid3dPinnedDelta, Grid3dPinnedModification};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::PinCell, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let key = cell_key(payload.pinned.x, payload.pinned.y, payload.pinned.z);
    if !cell_in_grid(base, payload.pinned.x, payload.pinned.y, payload.pinned.z) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is outside the grid."), [key]);
    }
    if !base.tiles.iter().any(|tile| tile.id == payload.pinned.tile_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} pins unknown tile \"{}\".", payload.pinned.tile_id), [payload.pinned.tile_id.clone()]);
    }
    if masked_index(base, payload.pinned.x, payload.pinned.y, payload.pinned.z).is_some() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cell {key} is masked out and cannot be pinned."), [key]);
    }
    if base.pinned.iter().any(|cell| cell == &payload.pinned) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Cell {key} already carries that pin."));
    }
    match pinned_index(base, payload.pinned.x, payload.pinned.y, payload.pinned.z) {
        Some(_) => protocol::MutationOutcome::new(Grid3dDiff { pinned: Grid3dPinnedDelta { modified: vec![Grid3dPinnedModification { id: key, patch: Grid3dPinnedPatch { tile_id: Some(payload.pinned.tile_id.clone()) } }], ..Default::default() }, ..Default::default() }),
        None => protocol::MutationOutcome::new(Grid3dDiff { pinned: Grid3dPinnedDelta::insertion(Grid3dRow::insert_at(&base.pinned, &payload.pinned), payload.pinned.clone()), ..Default::default() }),
    }
}
