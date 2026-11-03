//! 🔺️ `change-buckling-length-restraint-b` diff: patches `restraint_b` of the addressed buckling_length row.

use crate::change_buckling_length_restraint_b::ChangeBucklingLengthRestraintB;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BucklingLengthDelta, En1993BucklingLengthPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBucklingLengthRestraintB, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.buckling_lengths.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "buckling_length", Vec::<String>::new());
    };
    if row.restraint_b == payload.new_restraint_b {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The buckling_length already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { buckling_lengths: En1993BucklingLengthDelta::modification(payload.id.clone(), En1993BucklingLengthPatch { restraint_b: Some(payload.new_restraint_b), ..Default::default() }), ..Default::default() })
}
