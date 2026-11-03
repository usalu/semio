//! ↩️ `change-joint-column-fy` inverse: restores the previous `column_fy`.

use crate::change_joint_column_fy::ChangeJointColumnFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_fy::ChangeJointColumnFy> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnFy { id: payload.id.clone(), new_column_fy: row.column_fy })],
        None => Vec::new(),
    })
}
