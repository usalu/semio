//! 🔺️ `change-shells` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_shells::ChangeShells;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeShells, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.shells == &payload.shells {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { shells: Some(payload.shells.clone()), ..Default::default() })
}
