//! ↩️ `change-row-joint-id` inverse: restores the previous `joint_id`.

use crate::change_row_joint_id::ChangeRowJointId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowJointId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_joint_id::ChangeRowJointId> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowJointId { id: payload.id.clone(), new_joint_id: row.joint_id.clone() })],
        None => Vec::new(),
    })
}
