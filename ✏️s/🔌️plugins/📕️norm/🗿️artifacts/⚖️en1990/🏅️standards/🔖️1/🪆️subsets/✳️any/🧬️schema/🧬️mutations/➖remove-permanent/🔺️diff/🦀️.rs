//! ➖ `remove-permanent` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemovePermanent;
use crate::diff::{En1990Diff, En1990PermanentDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemovePermanent, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.permanents.len() {
        return MutationOutcome::error("mutation.target-missing", "permanents index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { permanents: En1990PermanentDelta::removal(&base.permanents[payload.index].id), ..En1990Diff::default() })
}
