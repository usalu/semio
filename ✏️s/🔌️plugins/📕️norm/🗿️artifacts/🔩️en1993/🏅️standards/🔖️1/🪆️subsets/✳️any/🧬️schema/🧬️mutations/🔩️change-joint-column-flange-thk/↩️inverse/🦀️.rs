//! ↩️ `change-joint-column-flange-thk` inverse: restores the previous `column_flange_thickness`.

use crate::change_joint_column_flange_thk::ChangeJointColumnFlangeThk;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnFlangeThk, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_flange_thk::ChangeJointColumnFlangeThk> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnFlangeThk { id: payload.id.clone(), new_column_flange_thickness: row.column_flange_thickness })],
        None => Vec::new(),
    })
}
