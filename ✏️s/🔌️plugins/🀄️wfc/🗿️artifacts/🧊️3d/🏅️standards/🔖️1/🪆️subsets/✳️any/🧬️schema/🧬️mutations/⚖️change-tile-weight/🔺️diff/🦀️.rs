//! 🔺️ Sparse diff builder for `ChangeTileWeight` — one id-keyed replacement at the tile's OWN index.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ChangeTileWeight, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("wfc3d.tile.missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.weight.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return protocol::MutationOutcome::fatal("wfc3d.tile.non-positive-weight", format!("Tile \"{}\" must carry a positive selection weight.", payload.id), [payload.id.clone()]);
    }
    let tile = &base.tiles[index];
    if tile.weight == payload.weight {
        return protocol::MutationOutcome::empty().warn("wfc3d.tile.weight-unchanged", format!("Tile \"{}\" already weighs {}.", payload.id, payload.weight));
    }
    let mut reweighted = tile.clone();
    reweighted.weight = payload.weight;
    protocol::MutationOutcome::new(Wfc3dDiff { tiles_upserted: vec![(index, reweighted)], ..Default::default() })
}
