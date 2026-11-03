//! 🔺️ `remove-shell-action` diff.

use crate::remove_shell_action::RemoveShellAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsPatch, En1999ShellsRows, En1999ShellsActionsRows};

pub fn diff(payload: &RemoveShellAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.shells.iter().find(|row| row.id == payload.shell_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown shell id {}", payload.shell_id), vec![payload.shell_id.clone()]);
    };
    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown shell action id {}", payload.id), Vec::<String>::new());
    };
    let nested = En1999ShellsActionsRows::removal(&parent.actions, index);
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::modification(payload.shell_id.clone(), En1999ShellsPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
