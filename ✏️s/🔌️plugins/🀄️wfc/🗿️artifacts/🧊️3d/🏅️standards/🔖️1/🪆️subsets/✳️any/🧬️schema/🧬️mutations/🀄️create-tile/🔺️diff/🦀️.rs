//! 🔺️ Sparse diff builder for `CreateTile` — a real id-keyed upsert into `tiles`.

use crate::diff::{Wfc3dDiff, Wfc3dRows};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::CreateTile, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \"{}\" already exists.", payload.tile.id), [payload.tile.id.clone()]);
    }
    if payload.tile.weight.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" must carry a positive selection weight.", payload.tile.id), [payload.tile.id.clone()]);
    }
    let canonical = crate::schema::snapshot::canonical_tile_index(base, &payload.tile.id);
    if payload.index != canonical {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" must be inserted at its canonical position {canonical}, not {}.", payload.tile.id, payload.index), [payload.tile.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { tiles: Wfc3dRows { added: vec![payload.tile.clone()], ..Default::default() }, ..Default::default() })
}
