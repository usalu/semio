//! ↩️ `change-joint-plate-role` inverse: restores the previous `role`.

use crate::change_joint_plate_role::ChangeJointPlateRole;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateRole, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_role::ChangeJointPlateRole> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateRole { id: payload.id.clone(), new_role: row.role.clone() })],
        None => Vec::new(),
    })
}
