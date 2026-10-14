//! 🧯 `remove-accidental` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_accidental::RemoveAccidental;
use semio_s_artifact_norm_en1990_diff::{En1990AccidentalDelta, En1990Diff};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveAccidental, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.accidentals.len() {
        return MutationOutcome::error("mutation.target-missing", "accidentals index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { accidentals: En1990AccidentalDelta::removal(&base.accidentals, payload.index), ..En1990Diff::default() })
}
