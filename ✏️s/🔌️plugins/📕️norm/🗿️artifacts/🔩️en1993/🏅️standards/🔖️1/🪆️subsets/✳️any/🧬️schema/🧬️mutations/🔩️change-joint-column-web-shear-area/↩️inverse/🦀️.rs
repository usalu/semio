//! ↩️ `change-joint-column-web-shear-area` inverse: restores the previous `column_web_shear_area`.

use crate::change_joint_column_web_shear_area::ChangeJointColumnWebShearArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnWebShearArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_web_shear_area::ChangeJointColumnWebShearArea> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnWebShearArea { id: payload.id.clone(), new_column_web_shear_area: row.column_web_shear_area })],
        None => Vec::new(),
    })
}
