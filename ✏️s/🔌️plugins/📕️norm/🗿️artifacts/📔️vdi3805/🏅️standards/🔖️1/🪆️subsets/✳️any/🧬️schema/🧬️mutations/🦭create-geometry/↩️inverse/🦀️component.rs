//! ↩️ `create-geometry` — undo is `delete-geometry`, unless `base` already had this id (then
//! `create` was a no-op).

use super::mutation::CreateGeometry;
use crate::artifacts::vdi3805::mutations::delete_geometry;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &CreateGeometry, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    if base.geometry.contains_key(&payload.geometry.id) {
        return Vec::new();
    }
    vec![Vdi3805Mutation::DeleteGeometry(delete_geometry::mutation::DeleteGeometry { id: payload.geometry.id.clone() })]
}
//#endregion 🔖️Inverse
