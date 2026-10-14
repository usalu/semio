use crate::insert_connection::InsertConnection;
use crate::remove_connection;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertConnection, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_connection::RemoveConnection> {
    Ok((|| {
    let at = payload.index.min(base.connections.len());
    vec![Op::from(remove_connection::RemoveConnection { index: at })]

    })())
}
