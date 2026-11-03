//! 🔺️ `change-joint-base-moment` diff: patches `base_moment` of the addressed joint row.

use crate::change_joint_base_moment::ChangeJointBaseMoment;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointBaseMoment, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_base_moment.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.base_moment == payload.new_base_moment {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { base_moment: Some(payload.new_base_moment), ..Default::default() }), ..Default::default() })
}
