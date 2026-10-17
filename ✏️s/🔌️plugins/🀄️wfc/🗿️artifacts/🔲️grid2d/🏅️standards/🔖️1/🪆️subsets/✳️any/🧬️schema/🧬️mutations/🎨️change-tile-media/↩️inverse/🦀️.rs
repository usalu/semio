//! ↩️ Inverse for `ChangeTileMedia` — restores the tile's base media (empty when the tile is gone).

use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::ChangeTileMedia, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia>,
{
    Ok((|| {
    match base.tiles.iter().find(|tile| tile.id == payload.id) {
        Some(tile) => vec![change_tile_media(payload.id.clone(), tile.media.clone())],
        None => Vec::new(),
    }

    })())
}
