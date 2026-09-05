//! ↩️ `remove-geometry-connection` — undo restores the BASE connection via `add`; missing
//! geometry/connection ⇒ `Vec::new()`.

use super::RemoveGeometryConnection;
use crate::artifacts::vdi3805::mutations::add_geometry_connection;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveGeometryConnection, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let existing = base.geometry.get(&payload.id).and_then(|geometry| geometry.connections.iter().find(|c| c.id == payload.connection_id));
    match existing {
        Some(old) => vec![Vdi3805Mutation::AddGeometryConnection(add_geometry_connection::AddGeometryConnection { id: payload.id.clone(), connection: old.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
