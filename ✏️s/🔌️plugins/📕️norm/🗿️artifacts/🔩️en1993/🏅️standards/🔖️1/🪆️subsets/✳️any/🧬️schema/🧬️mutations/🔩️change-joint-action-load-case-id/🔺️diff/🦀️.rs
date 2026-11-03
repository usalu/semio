//! 🔺️ `change-joint-action-load-case-id` diff: patches `actions[].load_case_id` of the addressed row's list.

use crate::change_joint_action_load_case_id::ChangeJointActionLoadCaseId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointActionLoadCaseId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-action", Vec::<String>::new());
    };
    let Some(at) = row.actions.iter().position(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-action", Vec::<String>::new());
    };
    if row.actions[at].load_case_id == payload.new_load_case_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The element already has this value.");
    }
    let mut value = row.actions.clone();
    value[at].load_case_id = payload.new_load_case_id.clone();
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
