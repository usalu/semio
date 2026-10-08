//! 🔺️ `change-shells` diff.

use crate::mutations::change_shells::ChangeShells;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999ShellsRows};

pub fn diff(payload: &ChangeShells, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.shells == &payload.shells {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.shells.iter().enumerate().find(|(at, row)| payload.shells[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Shell id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::setting(&base.shells, &payload.shells)), ..Default::default() })
}
