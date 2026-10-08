//! 💥 `change-accidentals` diff — replaces the whole collection: every base row leaves, every new row enters after the new row before it.

use super::ChangeAccidentals;
use crate::diff::{En1990AccidentalAddition, En1990AccidentalDelta, En1990Diff};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAccidentals, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.accidentals == mutation.new_accidentals {
        return MutationOutcome::empty().warning("mutation.no-op", "accidentals already has this value.");
    }
    let removed = base.accidentals.iter().map(|row| row.id.clone()).collect();
    let added = mutation.new_accidentals.iter().enumerate().map(|(index, row)| En1990AccidentalAddition { after: index.checked_sub(1).map(|previous| mutation.new_accidentals[previous].id.clone()), row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { accidentals: En1990AccidentalDelta { removed, added, ..En1990AccidentalDelta::default() }, ..En1990Diff::default() })
}
