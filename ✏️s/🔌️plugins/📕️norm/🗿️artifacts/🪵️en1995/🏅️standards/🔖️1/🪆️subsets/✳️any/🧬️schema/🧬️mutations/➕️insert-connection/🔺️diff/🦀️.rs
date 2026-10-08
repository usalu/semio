use super::InsertConnection;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995ConnectionDelta};
pub fn diff(payload: &InsertConnection, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.connections.iter().any(|existing| existing.id == payload.connection.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Connection id {} already exists.", payload.connection.id), [payload.connection.id.clone()]);
    }
    let at = payload.index.min(base.connections.len());
    protocol::MutationOutcome::new(En1995Diff { connections: En1995ConnectionDelta::insertion(&base.connections, at, payload.connection.clone()), ..Default::default() })
}
