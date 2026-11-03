//! ↩️ `change-joint-weld-grade` inverse: restores the previous `weld_grade`.

use crate::change_joint_weld_grade::ChangeJointWeldGrade;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointWeldGrade, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_weld_grade::ChangeJointWeldGrade> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointWeldGrade { id: payload.id.clone(), new_weld_grade: row.weld_grade.clone() })],
        None => Vec::new(),
    })
}
