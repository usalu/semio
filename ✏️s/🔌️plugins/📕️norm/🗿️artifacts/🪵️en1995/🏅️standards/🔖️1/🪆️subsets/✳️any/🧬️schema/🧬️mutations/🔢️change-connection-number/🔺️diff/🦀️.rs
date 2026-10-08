use super::ChangeConnectionNumber;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995ConnectionDelta, En1995ConnectionPatch};
pub fn diff(payload: &ChangeConnectionNumber, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.connections.iter().position(|item| item.id == payload.connection_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown connection id.", vec![payload.connection_id.clone()]);
    };
    let connection = &base.connections[idx];
    protocol::MutationOutcome::new(En1995Diff { connections: En1995ConnectionDelta::modification(&connection.id, En1995ConnectionPatch { number: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
