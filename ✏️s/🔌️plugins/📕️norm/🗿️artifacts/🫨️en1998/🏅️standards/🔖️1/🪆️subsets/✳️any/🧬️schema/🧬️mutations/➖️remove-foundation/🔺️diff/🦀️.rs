//! ➖️ `remove-foundation` diff — removes the row at the index, guarded by the row's own id; an index past the collection's end is a `mutation.target-missing`.

use crate::remove_foundation::RemoveFoundation;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if payload.index >= base.foundations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation #{}", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::removal(&base.foundations, payload.index), ..Default::default() })
}
