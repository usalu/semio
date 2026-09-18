//! ↩️ Inverse for `ChangeTileMedia` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;

pub fn inverse(payload: &super::ChangeTileMedia, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
    let Some(index) = tile_index(base, &payload.tile_id) else { return Vec::new() };
    vec![crate::mutations::change_tile_media(payload.tile_id.clone(), base.tiles[index].media.clone())]
}
