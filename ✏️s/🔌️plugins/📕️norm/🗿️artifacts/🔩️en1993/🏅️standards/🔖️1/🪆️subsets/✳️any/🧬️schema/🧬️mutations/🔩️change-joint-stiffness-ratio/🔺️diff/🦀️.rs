//! 🔺️ `change-joint-stiffness-ratio` diff: patches `stiffness_ratio` of the addressed joint row.

use crate::change_joint_stiffness_ratio::ChangeJointStiffnessRatio;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointStiffnessRatio, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_stiffness_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.stiffness_ratio == payload.new_stiffness_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { stiffness_ratio: Some(payload.new_stiffness_ratio), ..Default::default() }), ..Default::default() })
}
