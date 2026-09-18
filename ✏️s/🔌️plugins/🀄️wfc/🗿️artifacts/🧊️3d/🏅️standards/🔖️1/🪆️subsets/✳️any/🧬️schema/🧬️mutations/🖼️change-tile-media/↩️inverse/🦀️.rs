//! ↩️ Inverse for `ChangeTileMedia` — restores the tile's BASE media (missing id ⇒ empty).

use crate::mutations::{change_tile_media, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::ChangeTileMedia, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![change_tile_media(tile.id.clone(), tile.media.clone())]
}
