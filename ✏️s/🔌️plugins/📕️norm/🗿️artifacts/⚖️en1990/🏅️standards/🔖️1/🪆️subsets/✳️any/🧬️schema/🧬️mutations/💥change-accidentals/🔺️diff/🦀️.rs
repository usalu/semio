//! 💥 `change-accidentals` diff — replaces the whole collection: every base row is removed back to front, then every new row is inserted in order.

use super::ChangeAccidentals;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990AccidentalEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAccidentals, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.accidentals == mutation.new_accidentals {
        return MutationOutcome::empty().warning("mutation.no-op", "accidentals already has this value.");
    }
    let removed = (0..base.accidentals.len()).rev().map(|index| En1990AccidentalEdit::remove(index, base.accidentals[index].id.clone()));
    let inserted = mutation.new_accidentals.iter().cloned().enumerate().map(|(index, row)| En1990AccidentalEdit::insert(index, row));
    MutationOutcome::new(En1990Diff { accidentals: removed.chain(inserted).collect(), ..En1990Diff::default() })
}
