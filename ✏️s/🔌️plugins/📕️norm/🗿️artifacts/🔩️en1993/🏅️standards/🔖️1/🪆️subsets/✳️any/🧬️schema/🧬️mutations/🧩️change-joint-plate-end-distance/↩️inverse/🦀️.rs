//! ↩️ `change-joint-plate-end-distance` inverse: restores the previous `end_distance`.

use crate::change_joint_plate_end_distance::ChangeJointPlateEndDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateEndDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_end_distance::ChangeJointPlateEndDistance> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateEndDistance { id: payload.id.clone(), new_end_distance: row.end_distance })],
        None => Vec::new(),
    })
}
