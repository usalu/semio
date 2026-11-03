//! ↩️ `change-joint-plate-plate-fy` inverse: restores the previous `plate_fy`.

use crate::change_joint_plate_plate_fy::ChangeJointPlatePlateFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlatePlateFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_plate_fy::ChangeJointPlatePlateFy> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlatePlateFy { id: payload.id.clone(), new_plate_fy: row.plate_fy })],
        None => Vec::new(),
    })
}
