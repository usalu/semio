//! 🔺️ `change-buckling-length-beam-stiffness-b` diff: patches `beam_stiffness_b` of the addressed buckling_length block row.

use crate::change_buckling_length_beam_stiffness_b::ChangeBucklingLengthBeamStiffnessB;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BucklingLengthDelta, En1993BucklingLengthPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBucklingLengthBeamStiffnessB, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_beam_stiffness_b.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.buckling_lengths.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "buckling-length", Vec::<String>::new());
    };
    if row.beam_stiffness_b == payload.new_beam_stiffness_b {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The buckling_length block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { buckling_lengths: En1993BucklingLengthDelta::modification(payload.id.clone(), En1993BucklingLengthPatch { beam_stiffness_b: Some(payload.new_beam_stiffness_b), ..Default::default() }), ..Default::default() })
}
