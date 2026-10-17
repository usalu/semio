//! ↩️ Inverse for `ChangeTileWeight` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::ChangeTileWeight, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::ChangeTileWeight>,
{
    Ok((|| {
    let Some(index) = tile_index(base, &payload.tile_id) else { return Vec::new() };
    vec![change_tile_weight(payload.tile_id.clone(), base.tiles[index].weight)]

    })())
}
