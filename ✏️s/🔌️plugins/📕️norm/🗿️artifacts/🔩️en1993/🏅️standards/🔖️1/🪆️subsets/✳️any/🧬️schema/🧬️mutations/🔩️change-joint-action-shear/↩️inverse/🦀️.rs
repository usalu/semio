//! ↩️ `change-joint-action-shear` inverse: restores the previous `shear`.

use crate::change_joint_action_shear::ChangeJointActionShear;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointActionShear, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_action_shear::ChangeJointActionShear> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeJointActionShear { id: payload.id.clone(), element_id: payload.element_id.clone(), new_shear: row.shear })],
        None => Vec::new(),
    })
}
