//! ↩️ Inverse for `CreateTile` — deletes the id it created. A duplicate create was itself a no-op,
//! so it inverts to nothing.

use crate::mutations::{delete_tile, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::CreateTile, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return Vec::new();
    }
    vec![delete_tile(payload.tile.id.clone())]
}
