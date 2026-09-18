//! ↩️ Inverse for `ChangeTileMedia` — restores the tile's base media (empty when the tile is gone).

use crate::mutations::{change_tile_media, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::ChangeTileMedia, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    match base.tiles.iter().find(|tile| tile.id == payload.id) {
        Some(tile) => vec![change_tile_media(payload.id.clone(), tile.media.clone())],
        None => Vec::new(),
    }
}
