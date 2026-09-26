//! 🔺️ `change-accidentals` sparse diff.

use super::ChangeAccidentals;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAccidentals, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.accidentals == &mutation.new_accidentals {
        return MutationOutcome::empty().warn("mutation.no-op", "accidentals already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        accidentals: Some(mutation.new_accidentals.clone()),
        ..En1990Diff::default()
    })
}
