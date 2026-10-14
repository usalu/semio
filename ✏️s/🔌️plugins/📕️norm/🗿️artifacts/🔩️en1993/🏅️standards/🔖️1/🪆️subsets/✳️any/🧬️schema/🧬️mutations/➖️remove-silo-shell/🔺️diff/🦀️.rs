//! ➖️ `remove-silo-shell` diff — removes the row at the index.

use crate::remove_silo_shell::RemoveSiloShell;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SiloShellDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveSiloShell, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.silo_shells.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("silo-shell index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: En1993SiloShellDelta::removal(&base.silo_shells, payload.index), ..Default::default() })
}
