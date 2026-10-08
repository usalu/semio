//! 🛢️ `update-silo-shell-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateSiloShellInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993SiloShellEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.silo_shells.iter().position(|row| row.id == payload.silo_shell.id) {
        Some(index) if base.silo_shells[index] == payload.silo_shell => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993SiloShellEdit::replace(index, payload.silo_shell.id.clone(), payload.silo_shell.clone()),
        None => En1993SiloShellEdit::insert(base.silo_shells.len(), payload.silo_shell.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { silo_shells: vec![edit], ..Default::default() })
}
