//! ↩️ `change-joint-plate-plate-height` inverse: restores the previous `plate_height`.

use crate::change_joint_plate_plate_height::ChangeJointPlatePlateHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlatePlateHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_plate_height::ChangeJointPlatePlateHeight> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlatePlateHeight { id: payload.id.clone(), new_plate_height: row.plate_height })],
        None => Vec::new(),
    })
}
