//! ↩️ Inverse for `ChangeTileWeight` — restores the tile's BASE weight (missing id ⇒ empty).

use crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::ChangeTileWeight, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::ChangeTileWeight>,
{
    Ok((|| {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![change_tile_weight(tile.id.clone(), tile.weight)]

    })())
}
