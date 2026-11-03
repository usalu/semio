//! 🔺️ `change-buckling-length-member-id` diff: patches `member_id` of the addressed buckling_length block row.

use crate::change_buckling_length_member_id::ChangeBucklingLengthMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993BucklingLengthDelta, En1993BucklingLengthPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeBucklingLengthMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.buckling_lengths.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "buckling-length", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The buckling_length block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { buckling_lengths: En1993BucklingLengthDelta::modification(payload.id.clone(), En1993BucklingLengthPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
