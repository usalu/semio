//! 🛢️ `update-silo-shell-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateSiloShellInputs;
use crate::diff::{En1993Diff, En1993SiloShellDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.silo_shells.iter().position(|row| row.id == payload.silo_shell.id) {
        Some(index) if base.silo_shells[index] == payload.silo_shell => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993SiloShellDelta::removal(&payload.silo_shell.id);
            replacement.absorb(En1993SiloShellDelta::insertion(&base.silo_shells, index, payload.silo_shell.clone()));
            replacement
        }
        None => En1993SiloShellDelta::insertion(&base.silo_shells, base.silo_shells.len(), payload.silo_shell.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { silo_shells: delta, ..Default::default() })
}
