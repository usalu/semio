//! 💥 `change-accidentals` diff — replaces the whole collection: every base row leaves from its base index, every new row enters at its index.

use super::ChangeAccidentals;
use crate::diff::{En1990AccidentalDelta, En1990AccidentalInsertion, En1990AccidentalRemoval, En1990Diff};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAccidentals, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.accidentals == mutation.new_accidentals {
        return MutationOutcome::empty().warning("mutation.no-op", "accidentals already has this value.");
    }
    let removed = base.accidentals.iter().enumerate().map(|(index, row)| En1990AccidentalRemoval { id: row.id.clone(), index }).collect();
    let inserted = mutation.new_accidentals.iter().enumerate().map(|(index, row)| En1990AccidentalInsertion { index, row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { accidentals: En1990AccidentalDelta { removed, inserted, ..En1990AccidentalDelta::default() }, ..En1990Diff::default() })
}
