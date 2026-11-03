//! ↩️ `change-joint-plate-hole-diameter` inverse: restores the previous `hole_diameter`.

use crate::change_joint_plate_hole_diameter::ChangeJointPlateHoleDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateHoleDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_hole_diameter::ChangeJointPlateHoleDiameter> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateHoleDiameter { id: payload.id.clone(), new_hole_diameter: row.hole_diameter })],
        None => Vec::new(),
    })
}
