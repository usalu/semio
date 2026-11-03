//! 🔺️ `change-shell-action-nk` diff: patches `n_k` of the addressed shell action row.

use crate::change_shell_action_n_k::ChangeShellActionNK;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsActionsPatch, En1999ShellsActionsRows, En1999ShellsPatch, En1999ShellsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeShellActionNK, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_n_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.shells.iter().find(|row| row.id == payload.shell_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "shell", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.n_k == payload.new_n_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The shell action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::modification(&payload.shell_id, En1999ShellsPatch { actions: Some(En1999ShellsActionsRows::modification(&payload.action_id, En1999ShellsActionsPatch { n_k: Some(payload.new_n_k), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
