use super::RemoveConnection;
use crate::diff::En1995ConnectionList;
use crate::{En1995Diff, En1995Snapshot};
pub fn diff(payload: &RemoveConnection, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.connections.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Connection index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut connections = base.connections.clone();
    connections.remove(payload.index);
    protocol::MutationOutcome::new(En1995Diff { connections: Some(En1995ConnectionList { values: connections }), ..Default::default() })
}
