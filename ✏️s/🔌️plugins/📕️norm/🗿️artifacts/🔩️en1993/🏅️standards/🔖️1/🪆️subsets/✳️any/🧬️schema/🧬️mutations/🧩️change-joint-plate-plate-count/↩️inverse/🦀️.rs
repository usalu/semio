//! ↩️ `change-joint-plate-plate-count` inverse: restores the previous `plate_count`.

use crate::change_joint_plate_plate_count::ChangeJointPlatePlateCount;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlatePlateCount, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_plate_count::ChangeJointPlatePlateCount> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlatePlateCount { id: payload.id.clone(), new_plate_count: row.plate_count })],
        None => Vec::new(),
    })
}
