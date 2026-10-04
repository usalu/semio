//! ↩️ Inverse for `ChangeTileWeight` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::mutations::{change_tile_weight, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse(payload: &super::ChangeTileWeight, base: &Wfc2dSnapshot) -> Result<Vec<Wfc2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.tile_id) else {
        return Vec::new();
    };
    vec![change_tile_weight(payload.tile_id.clone(), tile.weight)]

    })())
}
