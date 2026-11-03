//! 🔺️ `insert-shell` diff.

use crate::insert_shell::InsertShell;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsRows};

pub fn diff(payload: &InsertShell, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.shells.iter().any(|existing| existing.id == payload.shell.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Shell id {} already exists.", payload.shell.id), [payload.shell.id.clone()]);
    }
    if (payload.index as usize) > base.shells.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.shells.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::insertion(payload.index as usize, payload.shell.clone())), ..Default::default() })
}
