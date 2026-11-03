//! ↩️ `change-joint-column-web-stress` inverse: restores the previous `column_web_stress`.

use crate::change_joint_column_web_stress::ChangeJointColumnWebStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnWebStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_web_stress::ChangeJointColumnWebStress> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnWebStress { id: payload.id.clone(), new_column_web_stress: row.column_web_stress })],
        None => Vec::new(),
    })
}
