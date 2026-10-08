//! ➕️ `insert-silo-shell` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertSiloShell;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993SiloShellEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertSiloShell, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.silo_shells.iter().any(|existing| existing.id == payload.silo_shell.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Silo shell id {} already exists.", payload.silo_shell.id), [payload.silo_shell.id.clone()]);
    }
    let index = payload.index.min(base.silo_shells.len());
    protocol::MutationOutcome::new(En1993Diff { silo_shells: vec![En1993SiloShellEdit::insert(index, payload.silo_shell.clone())], ..Default::default() })
}
