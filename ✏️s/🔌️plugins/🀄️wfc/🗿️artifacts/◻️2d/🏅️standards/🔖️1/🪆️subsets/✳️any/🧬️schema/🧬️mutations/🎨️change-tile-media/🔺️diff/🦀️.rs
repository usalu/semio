//! 🔺️ Sparse diff builder for `ChangeTileMedia` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::ChangeTileMedia, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.tile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    };
    let tile = &base.tiles[index];
    if tile.media == payload.media {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" already carries that media.", payload.tile_id));
    }
    let repainted = crate::schema::snapshot::Wfc2dTile { media: payload.media.clone(), ..tile.clone() };
    protocol::MutationOutcome::new(Wfc2dDiff { tiles_upserted: vec![(index, repainted)], ..Default::default() })
}
