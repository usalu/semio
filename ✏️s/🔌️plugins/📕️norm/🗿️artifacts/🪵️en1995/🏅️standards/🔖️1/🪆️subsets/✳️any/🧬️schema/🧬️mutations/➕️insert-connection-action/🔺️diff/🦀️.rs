use super::InsertConnectionAction;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995ConnectionActionDelta, En1995ConnectionDelta, En1995ConnectionPatch};
pub fn diff(payload: &InsertConnectionAction, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.connections.iter().position(|item| item.id == payload.connection_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown connection id.", vec![payload.connection_id.clone()]);
    };
    let connection = &base.connections[idx];
    let at = payload.index.min(base.connections[idx].actions.len());
    if connection.actions.iter().any(|existing| existing.id == payload.action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.action.id), [payload.action.id.clone()]);
    }
    protocol::MutationOutcome::new(En1995Diff { connections: En1995ConnectionDelta::modification(&connection.id, En1995ConnectionPatch { actions: En1995ConnectionActionDelta::insertion(at, payload.action.clone()), ..Default::default() }), ..Default::default() })
}
