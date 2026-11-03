//! ↩️ `change-joint-plate-joint-id` inverse: restores the previous `joint_id`.

use crate::change_joint_plate_joint_id::ChangeJointPlateJointId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateJointId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_joint_id::ChangeJointPlateJointId> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateJointId { id: payload.id.clone(), new_joint_id: row.joint_id.clone() })],
        None => Vec::new(),
    })
}
