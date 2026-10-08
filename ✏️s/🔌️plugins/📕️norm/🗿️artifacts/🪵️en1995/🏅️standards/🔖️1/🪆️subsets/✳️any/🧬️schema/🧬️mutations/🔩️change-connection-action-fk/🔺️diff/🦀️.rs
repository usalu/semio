use super::ChangeConnectionActionFK;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995ConnectionActionDelta, En1995ConnectionActionPatch, En1995ConnectionDelta, En1995ConnectionPatch};
pub fn diff(payload: &ChangeConnectionActionFK, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.connections.iter().position(|item| item.id == payload.connection_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown connection id.", vec![payload.connection_id.clone()]);
    };
    let Some(action_idx) = base.connections[idx].actions.iter().position(|action| action.id == payload.action_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown connection action id.", vec![payload.connection_id.clone(), payload.action_id.clone()]);
    };
    let connection = &base.connections[idx];
    let action = &connection.actions[action_idx];
    protocol::MutationOutcome::new(En1995Diff { connections: En1995ConnectionDelta::modification(&connection.id, En1995ConnectionPatch { actions: En1995ConnectionActionDelta::modification(&action.id, En1995ConnectionActionPatch { f_k_n: Some(payload.new_value), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
