//! ↩️ Inverse for `ChangeTileWeight` — restores the base weight (empty when the tile is gone).

use crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::ChangeTileWeight, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::ChangeTileWeight>,
{
    Ok((|| {
    match base.tiles.iter().find(|tile| tile.id == payload.id) {
        Some(tile) => vec![change_tile_weight(payload.id.clone(), tile.weight)],
        None => Vec::new(),
    }

    })())
}
