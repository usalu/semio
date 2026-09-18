//! ↩️ Inverse for `ChangeTileWeight` — restores the tile's BASE weight (missing id ⇒ empty).

use crate::mutations::{change_tile_weight, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::ChangeTileWeight, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![change_tile_weight(tile.id.clone(), tile.weight)]
}
