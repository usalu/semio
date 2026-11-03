//! 🔺️ `change-fastened-joint-count` diff: patches `count` of the addressed fastened_joint block row.

use crate::change_fastened_joint_count::ChangeFastenedJointCount;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FastenedJointDelta, En1993FastenedJointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFastenedJointCount, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fastened_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fastened-joint", Vec::<String>::new());
    };
    if row.count == payload.new_count {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fastened_joint block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fastened_joints: En1993FastenedJointDelta::modification(payload.id.clone(), En1993FastenedJointPatch { count: Some(payload.new_count), ..Default::default() }), ..Default::default() })
}
