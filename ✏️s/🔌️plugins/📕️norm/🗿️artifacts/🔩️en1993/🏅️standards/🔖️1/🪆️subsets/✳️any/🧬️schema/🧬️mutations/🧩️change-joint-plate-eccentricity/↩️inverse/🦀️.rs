//! ↩️ `change-joint-plate-eccentricity` inverse: restores the previous `eccentricity`.

use crate::change_joint_plate_eccentricity::ChangeJointPlateEccentricity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateEccentricity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_eccentricity::ChangeJointPlateEccentricity> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateEccentricity { id: payload.id.clone(), new_eccentricity: row.eccentricity })],
        None => Vec::new(),
    })
}
