//! 🔺️ `change-seismics` sparse diff.

use super::ChangeSeismics;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeSeismics, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.seismics == &mutation.new_seismics {
        return MutationOutcome::empty().warn("mutation.no-op", "seismics already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        seismics: Some(mutation.new_seismics.clone()),
        ..En1990Diff::default()
    })
}
