//! ↩️ Inverse for `ChangeTileMedia` — restores the tile's BASE media (missing id ⇒ empty).

use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::ChangeTileMedia, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia>,
{
    Ok((|| {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![change_tile_media(tile.id.clone(), tile.media.clone())]

    })())
}
