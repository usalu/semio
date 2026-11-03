//! ↩️ `insert-connection` inverse.

use crate::insert_connection::InsertConnection;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_connection;

pub fn inverse<Op>(payload: &InsertConnection, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_connection::RemoveConnection> {
    Ok((|| {
    vec![Op::from(remove_connection::RemoveConnection { id: payload.connection.id.clone() })]

    })())
}
