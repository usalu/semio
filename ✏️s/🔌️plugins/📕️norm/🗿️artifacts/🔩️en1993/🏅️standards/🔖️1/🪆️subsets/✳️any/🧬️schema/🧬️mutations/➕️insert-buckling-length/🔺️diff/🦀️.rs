//! ➕️ `insert-buckling-length` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_buckling_length::InsertBucklingLength;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BucklingLengthDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertBucklingLength, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.buckling_lengths.iter().any(|existing| existing.id == payload.buckling_length.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Buckling length id {} already exists.", payload.buckling_length.id), [payload.buckling_length.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.buckling_lengths.len());
    protocol::MutationOutcome::new(En1993Diff { buckling_lengths: En1993BucklingLengthDelta::insertion(index, payload.buckling_length.clone()), ..Default::default() })
}
