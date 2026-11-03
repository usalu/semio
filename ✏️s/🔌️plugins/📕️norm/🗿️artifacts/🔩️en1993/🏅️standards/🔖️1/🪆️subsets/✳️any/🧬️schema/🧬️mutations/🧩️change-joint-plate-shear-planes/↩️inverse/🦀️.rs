//! ↩️ `change-joint-plate-shear-planes` inverse: restores the previous `shear_planes`.

use crate::change_joint_plate_shear_planes::ChangeJointPlateShearPlanes;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateShearPlanes, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_shear_planes::ChangeJointPlateShearPlanes> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateShearPlanes { id: payload.id.clone(), new_shear_planes: row.shear_planes })],
        None => Vec::new(),
    })
}
