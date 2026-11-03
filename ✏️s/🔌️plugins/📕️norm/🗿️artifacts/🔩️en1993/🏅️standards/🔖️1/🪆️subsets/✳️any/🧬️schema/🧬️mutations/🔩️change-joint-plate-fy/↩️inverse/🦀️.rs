//! ↩️ `change-joint-plate-fy` inverse: restores the previous `plate_fy`.

use crate::change_joint_plate_fy::ChangeJointPlateFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_fy::ChangeJointPlateFy> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateFy { id: payload.id.clone(), new_plate_fy: row.plate_fy })],
        None => Vec::new(),
    })
}
