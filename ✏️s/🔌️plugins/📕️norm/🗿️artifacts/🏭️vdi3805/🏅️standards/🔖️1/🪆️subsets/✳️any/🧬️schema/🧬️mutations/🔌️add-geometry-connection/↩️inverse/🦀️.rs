//! ↩️ `add-geometry-connection` — undo restores the BASE connection at its original position, or `remove`s it if it was
//! previously absent (this mutation upserts, so a fresh connection's undo is `remove`).

use super::AddGeometryConnection;
use crate::mutations::remove_geometry_connection;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &AddGeometryConnection, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let existing = base.geometry.get(&payload.id).and_then(|geometry| geometry.connections.iter().enumerate().find(|(_, c)| c.id == payload.connection.id));
    match existing {
        Some((position, old)) => vec![Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection { id: payload.id.clone(), connection: old.clone(), index: Some(position) })],
        None => vec![Vdi3805Mutation::RemoveGeometryConnection(remove_geometry_connection::RemoveGeometryConnection { id: payload.id.clone(), connection_id: payload.connection.id.clone() })],
    }

    })())
}
//#endregion 🔖️Inverse
