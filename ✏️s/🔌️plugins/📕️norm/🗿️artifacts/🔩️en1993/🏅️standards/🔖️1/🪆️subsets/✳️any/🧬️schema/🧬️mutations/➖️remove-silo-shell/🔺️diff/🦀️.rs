//! ➖️ `remove-silo-shell` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveSiloShell;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993SiloShellEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveSiloShell, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.silo_shells.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("silo-shell index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { silo_shells: vec![En1993SiloShellEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
