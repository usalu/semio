//! ↩️ `remove-connection` inverse.

use crate::remove_connection::RemoveConnection;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_connection;

pub fn inverse<Op>(payload: &RemoveConnection, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_connection::InsertConnection> {
    Ok((|| {
    if let Some((index, row)) = base.connections.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_connection::InsertConnection { index: index as u32, connection: row.clone() })]
    } else { Vec::new() }

    })())
}
