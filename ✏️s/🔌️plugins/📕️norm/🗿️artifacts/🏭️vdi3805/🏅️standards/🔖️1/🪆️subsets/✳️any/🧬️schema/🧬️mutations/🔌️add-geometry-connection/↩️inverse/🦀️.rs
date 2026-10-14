//! ↩️ `add-geometry-connection` — undo restores the BASE connection at its original position, or `remove`s it if it was
//! previously absent (this mutation upserts, so a fresh connection's undo is `remove`).

use crate::add_geometry_connection::AddGeometryConnection;
use crate::remove_geometry_connection;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &AddGeometryConnection, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_geometry_connection::AddGeometryConnection> + From<crate::remove_geometry_connection::RemoveGeometryConnection> {
    Ok((|| {
    let existing = base.geometry.get(&payload.id).and_then(|geometry| geometry.connections.iter().enumerate().find(|(_, c)| c.id == payload.connection.id));
    match existing {
        Some((position, old)) => vec![Op::from(AddGeometryConnection { id: payload.id.clone(), connection: old.clone(), index: Some(position) })],
        None => vec![Op::from(remove_geometry_connection::RemoveGeometryConnection { id: payload.id.clone(), connection_id: payload.connection.id.clone() })],
    }

    })())
}
//#endregion 🔖️Inverse
