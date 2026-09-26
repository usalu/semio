//! ↩️ `remove-geometry` — undo re-`create`s the geometry from BASE state; missing id ⇒
//! `Vec::new()`.

use super::RemoveGeometry;
use crate::mutations::add_geometry;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveGeometry, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    match base.geometry.get(&payload.id) {
        Some(geometry) => vec![Vdi3805Mutation::AddGeometry(add_geometry::AddGeometry { geometry: geometry.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
