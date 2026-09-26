//! ↩️ `add-geometry` — undo is `remove-geometry`, unless `base` already had this id (then
//! `create` was a no-op).

use super::AddGeometry;
use crate::mutations::remove_geometry;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &AddGeometry, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    if base.geometry.contains_key(&payload.geometry.id) {
        return Vec::new();
    }
    vec![Vdi3805Mutation::RemoveGeometry(remove_geometry::RemoveGeometry { id: payload.geometry.id.clone() })]
}
//#endregion 🔖️Inverse
