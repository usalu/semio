//! 🔺️ Sparse diff builder for `ChangeTileWeight` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::ChangeTileWeight, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.tile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    };
    if !payload.weight.is_finite() || payload.weight < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" needs a finite non-negative weight.", payload.tile_id), [payload.tile_id.clone()]);
    }
    let tile = &base.tiles[index];
    if tile.weight == payload.weight {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" already has that weight.", payload.tile_id));
    }
    let reweighted = crate::schema::snapshot::Wfc2dTile { weight: payload.weight, ..tile.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { tiles_upserted: vec![(index, reweighted)], ..Default::default() })
}
