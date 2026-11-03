//! 🔺️ `change-fastened-joint-member-id` diff: patches `member_id` of the addressed fastened_joints block row.

use crate::change_fastened_joint_member_id::ChangeFastenedJointMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FastenedJointsDelta, En1993FastenedJointsPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFastenedJointMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fastened_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fastened-joints", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fastened_joints block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fastened_joints: En1993FastenedJointsDelta::modification(payload.id.clone(), En1993FastenedJointsPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
