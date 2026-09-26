use super::RemoveConnectionAction;
use crate::diff::En1995ConnectionList;
use crate::{En1995Diff, En1995Snapshot};
pub fn diff(payload: &RemoveConnectionAction, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.connections.iter().position(|item| item.id == payload.connection_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown connection id.", vec![payload.connection_id.clone()]);
    };
    if payload.index >= base.connections[idx].actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Connection action index {} out of range.", payload.index), vec![payload.connection_id.clone()]);
    }
    let mut connections = base.connections.clone();
    connections[idx].actions.remove(payload.index);
    protocol::MutationOutcome::new(En1995Diff { connections: Some(En1995ConnectionList { values: connections }), ..Default::default() })
}
