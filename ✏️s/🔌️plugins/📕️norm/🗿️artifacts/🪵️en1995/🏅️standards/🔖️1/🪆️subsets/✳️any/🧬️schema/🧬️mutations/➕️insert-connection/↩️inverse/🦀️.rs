use super::InsertConnection;
use crate::mutations::{remove_connection, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &InsertConnection, base: &En1995Snapshot) -> Result<Vec<En1995Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.min(base.connections.len());
    vec![En1995Mutation::RemoveConnection(remove_connection::RemoveConnection { index: at })]

    })())
}
