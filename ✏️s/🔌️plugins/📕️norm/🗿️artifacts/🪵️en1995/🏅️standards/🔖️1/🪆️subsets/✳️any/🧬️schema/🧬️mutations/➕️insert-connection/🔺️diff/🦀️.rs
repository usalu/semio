use super::InsertConnection;
use crate::diff::En1995ConnectionList;
use crate::{En1995Diff, En1995Snapshot};
pub fn diff(payload: &InsertConnection, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.connections.iter().any(|existing| existing.id == payload.connection.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Connection id {} already exists.", payload.connection.id), [payload.connection.id.clone()]);
    }
    let mut connections = base.connections.clone();
    let at = payload.index.min(connections.len());
    connections.insert(at, payload.connection.clone());
    protocol::MutationOutcome::new(En1995Diff { connections: Some(En1995ConnectionList { values: connections }), ..Default::default() })
}
