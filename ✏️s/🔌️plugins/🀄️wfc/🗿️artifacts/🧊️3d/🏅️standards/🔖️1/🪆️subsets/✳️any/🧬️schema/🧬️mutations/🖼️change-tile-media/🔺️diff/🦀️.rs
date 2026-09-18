//! 🔺️ Sparse diff builder for `ChangeTileMedia` — one id-keyed replacement at the tile's OWN index.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ChangeTileMedia, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("wfc3d.tile.missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let tile = &base.tiles[index];
    if tile.media == payload.media {
        return protocol::MutationOutcome::empty().warn("wfc3d.tile.media-unchanged", format!("Tile \"{}\" already carries that media.", payload.id));
    }
    let mut redressed = tile.clone();
    redressed.media = payload.media.clone();
    protocol::MutationOutcome::new(Wfc3dDiff { tiles_upserted: vec![(index, redressed)], ..Default::default() })
}
