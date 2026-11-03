//! ↩️ `change-joint-column-flange-thickness` inverse: restores the previous `column_flange_thickness`.

use crate::change_joint_column_flange_thickness::ChangeJointColumnFlangeThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnFlangeThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_flange_thickness::ChangeJointColumnFlangeThickness> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnFlangeThickness { id: payload.id.clone(), new_column_flange_thickness: row.column_flange_thickness })],
        None => Vec::new(),
    })
}
