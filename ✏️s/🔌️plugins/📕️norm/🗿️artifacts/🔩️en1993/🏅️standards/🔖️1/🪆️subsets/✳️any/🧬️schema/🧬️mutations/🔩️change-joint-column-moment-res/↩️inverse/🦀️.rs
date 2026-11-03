//! ↩️ `change-joint-column-moment-res` inverse: restores the previous `column_moment_resistance`.

use crate::change_joint_column_moment_res::ChangeJointColumnMomentRes;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnMomentRes, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_moment_res::ChangeJointColumnMomentRes> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnMomentRes { id: payload.id.clone(), new_column_moment_resistance: row.column_moment_resistance })],
        None => Vec::new(),
    })
}
