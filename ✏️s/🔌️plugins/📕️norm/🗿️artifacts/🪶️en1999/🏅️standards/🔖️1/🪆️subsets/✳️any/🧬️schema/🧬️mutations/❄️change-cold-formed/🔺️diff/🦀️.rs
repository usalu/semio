//! 🔺️ `change-cold-formed` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_cold_formed::ChangeColdFormed;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeColdFormed, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.cold_formed == &payload.cold_formed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(payload.cold_formed.clone()), ..Default::default() })
}
