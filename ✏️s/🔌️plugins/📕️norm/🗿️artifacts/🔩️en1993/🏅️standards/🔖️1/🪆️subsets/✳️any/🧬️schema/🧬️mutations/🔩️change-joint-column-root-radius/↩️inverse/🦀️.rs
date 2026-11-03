//! ↩️ `change-joint-column-root-radius` inverse: restores the previous `column_root_radius`.

use crate::change_joint_column_root_radius::ChangeJointColumnRootRadius;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnRootRadius, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_root_radius::ChangeJointColumnRootRadius> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnRootRadius { id: payload.id.clone(), new_column_root_radius: row.column_root_radius })],
        None => Vec::new(),
    })
}
