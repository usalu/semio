use crate::insert_connection::InsertConnection;
use crate::remove_connection;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertConnection, base: &En1995Snapshot) -> Option<crate::remove_connection::RemoveConnection> {
    let at = payload.index.min(base.connections.len());
    Some(remove_connection::RemoveConnection { index: at })
}
