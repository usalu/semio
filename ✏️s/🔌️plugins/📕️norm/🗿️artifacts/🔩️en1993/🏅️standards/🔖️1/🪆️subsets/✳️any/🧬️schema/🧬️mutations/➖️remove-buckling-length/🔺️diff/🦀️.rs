//! ➖️ `remove-buckling-length` diff — removes the row at the index.

use crate::remove_buckling_length::RemoveBucklingLength;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BucklingLengthDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveBucklingLength, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.buckling_lengths.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("buckling-length index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { buckling_lengths: En1993BucklingLengthDelta::removal(&base.buckling_lengths, payload.index), ..Default::default() })
}
