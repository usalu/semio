//! ↩️ `remove-geometry-connection` — undo restores the BASE connection via `add` at its original position; missing
//! geometry/connection ⇒ `Vec::new()`.

use super::RemoveGeometryConnection;
use crate::mutations::add_geometry_connection;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveGeometryConnection, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let existing = base.geometry.get(&payload.id).and_then(|geometry| geometry.connections.iter().enumerate().find(|(_, c)| c.id == payload.connection_id));
    match existing {
        Some((position, old)) => vec![Vdi3805Mutation::AddGeometryConnection(add_geometry_connection::AddGeometryConnection { id: payload.id.clone(), connection: old.clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
