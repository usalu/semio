use crate::remove_connection::RemoveConnection;
use crate::insert_connection;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveConnection, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_connection::InsertConnection> {
    Ok((|| {
    if payload.index >= base.connections.len() { return Vec::new(); }
    vec![Op::from(insert_connection::InsertConnection { index: payload.index, connection: base.connections[payload.index].clone() })]

    })())
}
