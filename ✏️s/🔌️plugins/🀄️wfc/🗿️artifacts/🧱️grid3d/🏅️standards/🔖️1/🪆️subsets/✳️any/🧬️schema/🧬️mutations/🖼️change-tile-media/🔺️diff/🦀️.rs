//! 🔺️ Sparse diff builder for `ChangeTileMedia` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::ChangeTileMedia, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let Some(index) = tile_index(base, &payload.tile_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("No tile with id \"{}\" exists.", payload.tile_id), [payload.tile_id.clone()]);
    };
    if base.tiles[index].media == payload.media {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" already carries that media.", payload.tile_id));
    }
    let mut tile = base.tiles[index].clone();
    tile.media = payload.media.clone();
    protocol::MutationOutcome::new(Grid3dDiff { tiles_upserted: vec![(index, tile)], ..Default::default() })
}
