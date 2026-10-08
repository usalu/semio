//! 🔺️ Sparse diff builder for `ChangeTileWeight` — one id-keyed replacement at the tile's OWN index.

use crate::diff::{Wfc3dDiff, Wfc3dTilePatch, Wfc3dTilesDelta, Wfc3dTilesModification};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ChangeTileWeight, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.weight.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" must carry a positive selection weight.", payload.id), [payload.id.clone()]);
    }
    let tile = &base.tiles[index];
    if tile.weight == payload.weight {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" already weighs {}.", payload.id, payload.weight));
    }
    protocol::MutationOutcome::new(Wfc3dDiff { tiles: Wfc3dTilesDelta { modified: vec![Wfc3dTilesModification { id: tile.id.clone(), patch: Wfc3dTilePatch { weight: Some(payload.weight), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
