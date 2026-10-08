//! ➖️ `remove-silo-shell` diff — removes the row at the index.

use super::RemoveSiloShell;
use crate::diff::{En1993Diff, En1993SiloShellDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveSiloShell, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.silo_shells.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("silo-shell index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: En1993SiloShellDelta::removal(&base.silo_shells, payload.index), ..Default::default() })
}
