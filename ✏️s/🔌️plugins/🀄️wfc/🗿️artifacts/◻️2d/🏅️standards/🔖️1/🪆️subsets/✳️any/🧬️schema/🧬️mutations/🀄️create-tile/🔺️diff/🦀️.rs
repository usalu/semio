//! 🔺️ Sparse diff builder for `CreateTile` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::CreateTile, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \"{}\" already exists.", payload.tile.id), [payload.tile.id.clone()]);
    }
    if !payload.tile.weight.is_finite() || payload.tile.weight < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" needs a finite non-negative weight.", payload.tile.id), [payload.tile.id.clone()]);
    }
    let at = crate::mutations::ordered_index(&base.tiles, &payload.tile.id, |tile| tile.id.as_str());
    protocol::MutationOutcome::new(Wfc2dDiff { tiles_upserted: vec![(at, payload.tile.clone())], ..Default::default() })
}
