//! 🧯 `remove-accidental` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveAccidental;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990AccidentalEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveAccidental, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.accidentals.len() {
        return MutationOutcome::error("mutation.target-missing", "accidentals index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { accidentals: vec![En1990AccidentalEdit::remove(payload.index, base.accidentals[payload.index].id.clone())], ..En1990Diff::default() })
}
