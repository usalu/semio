//! 🔺️ `change-shell-length` diff: patches `length` of the addressed shell row.

use crate::change_shell_length::ChangeShellLength;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsPatch, En1999ShellsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeShellLength, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.shells.iter().find(|row| row.id == payload.shell_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "shell", Vec::<String>::new());
    };
    if r0.length == payload.new_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The shell already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::modification(&payload.shell_id, En1999ShellsPatch { length: Some(payload.new_length), ..Default::default() })), ..Default::default() })
}
