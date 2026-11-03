//! ↩️ `change-joint-column-web-thickness` inverse: restores the previous `column_web_thickness`.

use crate::change_joint_column_web_thickness::ChangeJointColumnWebThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnWebThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_web_thickness::ChangeJointColumnWebThickness> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnWebThickness { id: payload.id.clone(), new_column_web_thickness: row.column_web_thickness })],
        None => Vec::new(),
    })
}
