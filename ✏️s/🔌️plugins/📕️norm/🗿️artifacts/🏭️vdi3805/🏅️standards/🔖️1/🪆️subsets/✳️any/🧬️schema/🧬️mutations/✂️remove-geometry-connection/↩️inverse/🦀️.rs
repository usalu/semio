//! ↩️ `remove-geometry-connection` — undo restores the BASE connection via `add` at its original position; missing
//! geometry/connection ⇒ `Vec::new()`.

use crate::remove_geometry_connection::RemoveGeometryConnection;
use crate::add_geometry_connection;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveGeometryConnection, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_geometry_connection::AddGeometryConnection> {
    Ok((|| {
    let existing = base.geometry.get(&payload.id).and_then(|geometry| geometry.connections.iter().enumerate().find(|(_, c)| c.id == payload.connection_id));
    match existing {
        Some((position, old)) => vec![Op::from(add_geometry_connection::AddGeometryConnection { id: payload.id.clone(), connection: old.clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
