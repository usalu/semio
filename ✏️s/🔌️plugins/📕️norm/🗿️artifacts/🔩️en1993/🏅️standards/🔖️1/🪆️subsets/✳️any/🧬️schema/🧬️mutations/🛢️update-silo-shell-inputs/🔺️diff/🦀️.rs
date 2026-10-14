//! 🛢️ `update-silo-shell-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use crate::update_silo_shell_inputs::UpdateSiloShellInputs;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SiloShellDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.silo_shells.iter().position(|row| row.id == payload.silo_shell.id) {
        Some(index) if base.silo_shells[index] == payload.silo_shell => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993SiloShellDelta::removal(&base.silo_shells, index);
            replacement.absorb(En1993SiloShellDelta::insertion(index, payload.silo_shell.clone()));
            replacement
        }
        None => En1993SiloShellDelta::insertion(base.silo_shells.len(), payload.silo_shell.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { silo_shells: delta, ..Default::default() })
}
