//! 🔺️ Sparse diff builder for `ChangeTileWeight` — an in-place upsert at the tile's EXISTING index
//! (a weight change never moves a row).

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::ChangeTileWeight, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !(payload.weight.is_finite() && payload.weight > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" must carry a finite positive weight, got {}.", payload.id, payload.weight), [payload.id.clone()]);
    }
    if base.tiles[index].weight == payload.weight {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" already weighs {}.", payload.id, payload.weight));
    }
    let mut tile = base.tiles[index].clone();
    tile.weight = payload.weight;
    protocol::MutationOutcome::new(Grid2dDiff { tiles_upserted: vec![(index, tile)], ..Default::default() })
}
