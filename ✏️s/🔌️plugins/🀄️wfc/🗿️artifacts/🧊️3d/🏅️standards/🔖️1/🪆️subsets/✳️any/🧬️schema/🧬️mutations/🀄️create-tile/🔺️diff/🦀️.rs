//! 🔺️ Sparse diff builder for `CreateTile` — a real id-keyed upsert into `tiles`.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::CreateTile, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("wfc3d.tile.duplicate-id", format!("A tile with id \"{}\" already exists.", payload.tile.id), [payload.tile.id.clone()]);
    }
    if payload.tile.weight.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return protocol::MutationOutcome::fatal("wfc3d.tile.non-positive-weight", format!("Tile \"{}\" must carry a positive selection weight.", payload.tile.id), [payload.tile.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { tiles_upserted: vec![(payload.index, payload.tile.clone())], ..Default::default() })
}
