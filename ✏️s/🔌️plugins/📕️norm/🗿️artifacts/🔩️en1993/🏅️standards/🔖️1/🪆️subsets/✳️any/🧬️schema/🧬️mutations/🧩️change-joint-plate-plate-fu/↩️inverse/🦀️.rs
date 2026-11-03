//! ↩️ `change-joint-plate-plate-fu` inverse: restores the previous `plate_fu`.

use crate::change_joint_plate_plate_fu::ChangeJointPlatePlateFu;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlatePlateFu, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_plate_fu::ChangeJointPlatePlateFu> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlatePlateFu { id: payload.id.clone(), new_plate_fu: row.plate_fu })],
        None => Vec::new(),
    })
}
