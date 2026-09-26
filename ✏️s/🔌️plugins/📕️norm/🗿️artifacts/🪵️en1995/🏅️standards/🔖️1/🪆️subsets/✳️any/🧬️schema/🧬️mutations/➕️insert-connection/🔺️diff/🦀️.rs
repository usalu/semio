use super::InsertConnection;
use crate::diff::En1995ConnectionList;
use crate::{En1995Diff, En1995Snapshot};
pub fn diff(payload: &InsertConnection, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let mut connections = base.connections.clone();
    let at = payload.index.min(connections.len());
    connections.insert(at, payload.connection.clone());
    protocol::MutationOutcome::new(En1995Diff { connections: Some(En1995ConnectionList { values: connections }), ..Default::default() })
}
