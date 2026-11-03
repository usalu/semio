//! 🔺️ `change-joint-joint-coefficient` diff: patches `joint_coefficient` of the addressed joint row.

use crate::change_joint_joint_coefficient::ChangeJointJointCoefficient;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointJointCoefficient, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_joint_coefficient.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.joint_coefficient == payload.new_joint_coefficient {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { joint_coefficient: Some(payload.new_joint_coefficient), ..Default::default() }), ..Default::default() })
}
