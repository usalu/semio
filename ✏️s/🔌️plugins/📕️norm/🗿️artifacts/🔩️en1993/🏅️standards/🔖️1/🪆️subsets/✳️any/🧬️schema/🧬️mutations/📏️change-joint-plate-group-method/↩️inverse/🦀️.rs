//! ↩️ `change-joint-plate-group-method` inverse: restores the previous `group_method`.

use crate::change_joint_plate_group_method::ChangeJointPlateGroupMethod;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateGroupMethod, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_group_method::ChangeJointPlateGroupMethod> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateGroupMethod { id: payload.id.clone(), new_group_method: row.group_method })],
        None => Vec::new(),
    })
}
