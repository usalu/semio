//! 🔺️ `change-joint-action-tension` diff: patches `actions[].tension` of the addressed row's list.

use crate::change_joint_action_tension::ChangeJointActionTension;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointActionTension, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_tension.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-action", Vec::<String>::new());
    };
    let Some(at) = row.actions.iter().position(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-action", Vec::<String>::new());
    };
    if row.actions[at].tension == payload.new_tension {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The element already has this value.");
    }
    let mut value = row.actions.clone();
    value[at].tension = payload.new_tension;
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
