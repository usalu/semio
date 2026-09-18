//! ↩️ Inverse for `ChangeTileWeight` — restores the base weight (empty when the tile is gone).

use crate::mutations::{change_tile_weight, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::ChangeTileWeight, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    match base.tiles.iter().find(|tile| tile.id == payload.id) {
        Some(tile) => vec![change_tile_weight(payload.id.clone(), tile.weight)],
        None => Vec::new(),
    }
}
