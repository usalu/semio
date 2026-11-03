//! ↩️ `change-joint-plate-thickness` inverse: restores the previous `plate_thickness`.

use crate::change_joint_plate_thickness::ChangeJointPlateThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_thickness::ChangeJointPlateThickness> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateThickness { id: payload.id.clone(), new_plate_thickness: row.plate_thickness })],
        None => Vec::new(),
    })
}
