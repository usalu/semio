//! 🔺️ Sparse diff builder for `ChangeTileMedia` — one id-keyed replacement at the tile's OWN index.

use crate::diff::{Wfc3dDiff, Wfc3dRowPatch, Wfc3dRows, Wfc3dTilePatch};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ChangeTileMedia, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let tile = &base.tiles[index];
    if tile.media == payload.media {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" already carries that media.", payload.id));
    }
    protocol::MutationOutcome::new(Wfc3dDiff { tiles: Wfc3dRows { patched: vec![Wfc3dRowPatch { id: tile.id.clone(), patch: Wfc3dTilePatch { media: Some(payload.media.clone()), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
