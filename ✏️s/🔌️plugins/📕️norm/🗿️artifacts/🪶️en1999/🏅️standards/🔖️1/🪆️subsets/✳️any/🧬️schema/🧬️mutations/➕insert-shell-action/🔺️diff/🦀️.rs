//! 🔺️ `insert-shell-action` diff.

use crate::insert_shell_action::InsertShellAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsPatch, En1999ShellsRows, En1999ShellsActionsRows};

pub fn diff(payload: &InsertShellAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.shells.iter().find(|row| row.id == payload.shell_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown shell id {}", payload.shell_id), vec![payload.shell_id.clone()]);
    };
    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Shell action id {} already exists.", payload.action.id), [payload.action.id.clone()]);
    }
    if (payload.index as usize) > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    let nested = En1999ShellsActionsRows::insertion(payload.index as usize, payload.action.clone());
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::modification(payload.shell_id.clone(), En1999ShellsPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
