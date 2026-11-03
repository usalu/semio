//! ↩️ `change-joint-action-load-case-id` inverse: restores the previous `load_case_id`.

use crate::change_joint_action_load_case_id::ChangeJointActionLoadCaseId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointActionLoadCaseId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_action_load_case_id::ChangeJointActionLoadCaseId> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeJointActionLoadCaseId { id: payload.id.clone(), element_id: payload.element_id.clone(), new_load_case_id: row.load_case_id.clone() })],
        None => Vec::new(),
    })
}
