//! ↩️ `add-geometry-connection` — undo restores the BASE connection, or `remove`s it if it was
//! previously absent (this mutation upserts, so a fresh connection's undo is `remove`).

use super::mutation::AddGeometryConnection;
use crate::artifacts::vdi3805::mutations::remove_geometry_connection;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &AddGeometryConnection, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let existing = base.geometry.get(&payload.id).and_then(|geometry| geometry.connections.iter().find(|c| c.id == payload.connection.id));
    match existing {
        Some(old) => vec![Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection { id: payload.id.clone(), connection: old.clone() })],
        None => vec![Vdi3805Mutation::RemoveGeometryConnection(remove_geometry_connection::mutation::RemoveGeometryConnection { id: payload.id.clone(), connection_id: payload.connection.id.clone() })],
    }
}
//#endregion 🔖️Inverse
