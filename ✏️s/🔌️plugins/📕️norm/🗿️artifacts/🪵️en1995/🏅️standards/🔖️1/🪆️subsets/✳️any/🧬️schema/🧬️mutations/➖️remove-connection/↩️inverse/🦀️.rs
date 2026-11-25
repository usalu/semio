use crate::remove_connection::RemoveConnection;
use crate::insert_connection;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveConnection, base: &En1995Snapshot) -> Option<crate::insert_connection::InsertConnection> {
    if payload.index >= base.connections.len() { return None; }
    Some(insert_connection::InsertConnection { index: payload.index, connection: base.connections[payload.index].clone() })
}
