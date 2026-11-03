//! 🔺️ `change-buckling-length-sway` diff: patches `sway` of the addressed buckling_length row.

use crate::change_buckling_length_sway::ChangeBucklingLengthSway;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BucklingLengthDelta, En1993BucklingLengthPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBucklingLengthSway, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.buckling_lengths.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "buckling_length", Vec::<String>::new());
    };
    if row.sway == payload.new_sway {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The buckling_length already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { buckling_lengths: En1993BucklingLengthDelta::modification(payload.id.clone(), En1993BucklingLengthPatch { sway: Some(payload.new_sway), ..Default::default() }), ..Default::default() })
}
