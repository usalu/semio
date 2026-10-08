//! 🔺️ Sparse diff builder for `ChangeTileMedia` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::{Grid3dDiff, Grid3dRowPatch, Grid3dRows, Grid3dTilePatch};
use crate::schema::snapshot::*;

pub fn diff(payload: &super::ChangeTileMedia, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    let Some(index) = tile_index(base, &payload.tile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No tile with id \"{}\" exists.", payload.tile_id), [payload.tile_id.clone()]);
    };
    if base.tiles[index].media == payload.media {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" already carries that media.", payload.tile_id));
    }
    protocol::MutationOutcome::new(Grid3dDiff { tiles: Grid3dRows { patched: vec![Grid3dRowPatch { id: payload.tile_id.clone(), patch: Grid3dTilePatch { media: Some(payload.media.clone()), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
