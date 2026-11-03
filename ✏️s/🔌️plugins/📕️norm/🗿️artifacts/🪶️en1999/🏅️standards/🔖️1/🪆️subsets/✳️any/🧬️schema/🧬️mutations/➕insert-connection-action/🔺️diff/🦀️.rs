//! 🔺️ `insert-connection-action` diff.

use crate::insert_connection_action::InsertConnectionAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ConnectionsPatch, En1999ConnectionsRows, En1999ConnectionsActionsRows};

pub fn diff(payload: &InsertConnectionAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.connections.iter().find(|row| row.id == payload.connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown connection id {}", payload.connection_id), vec![payload.connection_id.clone()]);
    };
    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Connection action id {} already exists.", payload.action.id), [payload.action.id.clone()]);
    }
    if (payload.index as usize) > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    let nested = En1999ConnectionsActionsRows::insertion(payload.index as usize, payload.action.clone());
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(payload.connection_id.clone(), En1999ConnectionsPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
