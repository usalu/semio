//! 🔺️ `remove-connection-action` diff.

use crate::remove_connection_action::RemoveConnectionAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ConnectionsPatch, En1999ConnectionsRows, En1999ConnectionsActionsRows};

pub fn diff(payload: &RemoveConnectionAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.connections.iter().find(|row| row.id == payload.connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown connection id {}", payload.connection_id), vec![payload.connection_id.clone()]);
    };
    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown connection action id {}", payload.id), Vec::<String>::new());
    };
    let nested = En1999ConnectionsActionsRows::removal(&parent.actions, index);
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(payload.connection_id.clone(), En1999ConnectionsPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
