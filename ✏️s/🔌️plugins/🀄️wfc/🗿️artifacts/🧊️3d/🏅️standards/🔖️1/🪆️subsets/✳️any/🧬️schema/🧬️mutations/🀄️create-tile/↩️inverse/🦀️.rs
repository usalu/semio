//! ↩️ Inverse for `CreateTile` — the `delete-tile` of the id it created.

use crate::mutations::{delete_tile, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse(payload: &super::CreateTile, _base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![delete_tile(payload.tile.id.clone())]

    })())
}
