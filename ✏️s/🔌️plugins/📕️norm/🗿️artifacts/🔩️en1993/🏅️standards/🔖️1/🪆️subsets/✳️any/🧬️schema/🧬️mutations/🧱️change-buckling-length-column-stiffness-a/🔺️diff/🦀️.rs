//! 🔺️ `change-buckling-length-column-stiffness-a` diff: patches `column_stiffness_a` of the addressed buckling_length block row.

use crate::change_buckling_length_column_stiffness_a::ChangeBucklingLengthColumnStiffnessA;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BucklingLengthDelta, En1993BucklingLengthPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBucklingLengthColumnStiffnessA, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_column_stiffness_a.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.buckling_lengths.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "buckling-length", Vec::<String>::new());
    };
    if row.column_stiffness_a == payload.new_column_stiffness_a {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The buckling_length block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { buckling_lengths: En1993BucklingLengthDelta::modification(payload.id.clone(), En1993BucklingLengthPatch { column_stiffness_a: Some(payload.new_column_stiffness_a), ..Default::default() }), ..Default::default() })
}
