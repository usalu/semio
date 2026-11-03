//! ↩️ `change-joint-base-lever-right` inverse: restores the previous `base_lever_right`.

use crate::change_joint_base_lever_right::ChangeJointBaseLeverRight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBaseLeverRight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_base_lever_right::ChangeJointBaseLeverRight> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBaseLeverRight { id: payload.id.clone(), new_base_lever_right: row.base_lever_right })],
        None => Vec::new(),
    })
}
