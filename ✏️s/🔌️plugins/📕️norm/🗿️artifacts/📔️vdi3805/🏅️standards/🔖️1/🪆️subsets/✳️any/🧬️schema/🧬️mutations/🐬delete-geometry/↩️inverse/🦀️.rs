//! ↩️ `delete-geometry` — undo re-`create`s the geometry from BASE state; missing id ⇒
//! `Vec::new()`.

use super::DeleteGeometry;
use crate::artifacts::vdi3805::mutations::create_geometry;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteGeometry, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    match base.geometry.get(&payload.id) {
        Some(geometry) => vec![Vdi3805Mutation::CreateGeometry(create_geometry::CreateGeometry { geometry: geometry.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
