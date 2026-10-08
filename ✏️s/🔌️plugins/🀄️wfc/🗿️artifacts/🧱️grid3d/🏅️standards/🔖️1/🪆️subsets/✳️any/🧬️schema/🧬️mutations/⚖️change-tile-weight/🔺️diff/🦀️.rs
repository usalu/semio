//! 🔺️ Sparse diff builder for `ChangeTileWeight` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dTilePatch, Grid3dTilesDelta, Grid3dTilesModification};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::ChangeTileWeight, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let Some(index) = tile_index(base, &payload.tile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No tile with id \"{}\" exists.", payload.tile_id), [payload.tile_id.clone()]);
    };
    if !payload.weight.is_finite() || payload.weight <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" needs a finite positive weight.", payload.tile_id), [payload.tile_id.clone()]);
    }
    if base.tiles[index].weight == payload.weight {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" already carries that weight.", payload.tile_id));
    }
    protocol::MutationOutcome::new(Grid3dDiff { tiles: Grid3dTilesDelta { modified: vec![Grid3dTilesModification { id: payload.tile_id.clone(), patch: Grid3dTilePatch { weight: Some(payload.weight), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
