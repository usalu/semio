//! 🔺️ `remove-shell` diff.

use crate::remove_shell::RemoveShell;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsRows};

pub fn diff(payload: &RemoveShell, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.shells.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown shell id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::removal(&base.shells, index)), ..Default::default() })
}
