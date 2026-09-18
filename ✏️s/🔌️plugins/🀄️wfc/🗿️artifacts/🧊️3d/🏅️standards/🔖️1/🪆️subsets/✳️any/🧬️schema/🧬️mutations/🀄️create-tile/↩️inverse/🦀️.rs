//! ↩️ Inverse for `CreateTile` — the `delete-tile` of the id it created.

use crate::mutations::{delete_tile, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::CreateTile, _base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
    vec![delete_tile(payload.tile.id.clone())]
}
