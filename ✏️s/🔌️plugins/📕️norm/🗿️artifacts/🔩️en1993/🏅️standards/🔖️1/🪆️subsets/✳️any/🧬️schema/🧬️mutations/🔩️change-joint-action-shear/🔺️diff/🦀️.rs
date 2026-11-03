//! 🔺️ `change-joint-action-shear` diff: patches `actions[].shear` of the addressed row's list.

use crate::change_joint_action_shear::ChangeJointActionShear;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointActionShear, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_shear.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-action", Vec::<String>::new());
    };
    let Some(at) = row.actions.iter().position(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-action", Vec::<String>::new());
    };
    if row.actions[at].shear == payload.new_shear {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The element already has this value.");
    }
    let mut value = row.actions.clone();
    value[at].shear = payload.new_shear;
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
