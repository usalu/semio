//! ➖ `remove-permanent` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_permanent::RemovePermanent;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990PermanentDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemovePermanent, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.permanents.len() {
        return MutationOutcome::error("mutation.target-missing", "permanents index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { permanents: En1990PermanentDelta::removal(&base.permanents, payload.index), ..En1990Diff::default() })
}
