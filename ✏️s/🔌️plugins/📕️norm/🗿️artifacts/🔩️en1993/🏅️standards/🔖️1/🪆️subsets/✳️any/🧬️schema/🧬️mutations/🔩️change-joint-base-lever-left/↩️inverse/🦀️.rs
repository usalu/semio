//! ↩️ `change-joint-base-lever-left` inverse: restores the previous `base_lever_left`.

use crate::change_joint_base_lever_left::ChangeJointBaseLeverLeft;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBaseLeverLeft, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_base_lever_left::ChangeJointBaseLeverLeft> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBaseLeverLeft { id: payload.id.clone(), new_base_lever_left: row.base_lever_left })],
        None => Vec::new(),
    })
}
