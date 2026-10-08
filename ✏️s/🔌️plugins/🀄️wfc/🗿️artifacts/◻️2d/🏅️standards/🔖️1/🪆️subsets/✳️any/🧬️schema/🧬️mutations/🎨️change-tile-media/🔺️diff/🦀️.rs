//! 🔺️ Sparse diff builder for `ChangeTileMedia` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::{Wfc2dDiff, Wfc2dTilePatch, Wfc2dTilesDelta, Wfc2dTilesModification};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::ChangeTileMedia, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.tile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.tile_id), [payload.tile_id.clone()]);
    };
    let tile = &base.tiles[index];
    if tile.media == payload.media {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" already carries that media.", payload.tile_id));
    }
    protocol::MutationOutcome::new(Wfc2dDiff { tiles: Wfc2dTilesDelta { modified: vec![Wfc2dTilesModification { id: tile.id.clone(), patch: Wfc2dTilePatch { media: Some(payload.media.clone()), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
