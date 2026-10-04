use super::RemoveConnection;
use crate::mutations::{insert_connection, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &RemoveConnection, base: &En1995Snapshot) -> Result<Vec<En1995Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.connections.len() { return Vec::new(); }
    vec![En1995Mutation::InsertConnection(insert_connection::InsertConnection { index: payload.index, connection: base.connections[payload.index].clone() })]

    })())
}
