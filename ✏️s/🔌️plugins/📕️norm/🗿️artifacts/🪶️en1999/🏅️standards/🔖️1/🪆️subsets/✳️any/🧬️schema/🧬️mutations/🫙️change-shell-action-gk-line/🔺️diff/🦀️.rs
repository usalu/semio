//! 🔺️ `change-shell-action-gk-line` diff: patches `g_k_line` of the addressed shell action row.

use crate::change_shell_action_g_k_line::ChangeShellActionGKLine;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsActionsPatch, En1999ShellsActionsRows, En1999ShellsPatch, En1999ShellsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeShellActionGKLine, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_g_k_line.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.shells.iter().find(|row| row.id == payload.shell_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "shell", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.g_k_line == payload.new_g_k_line {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The shell action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::modification(&payload.shell_id, En1999ShellsPatch { actions: Some(En1999ShellsActionsRows::modification(&payload.action_id, En1999ShellsActionsPatch { g_k_line: Some(payload.new_g_k_line), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
