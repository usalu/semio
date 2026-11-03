//! ↩️ `change-joint-plate-bolt-diameter` inverse: restores the previous `bolt_diameter`.

use crate::change_joint_plate_bolt_diameter::ChangeJointPlateBoltDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateBoltDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_bolt_diameter::ChangeJointPlateBoltDiameter> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateBoltDiameter { id: payload.id.clone(), new_bolt_diameter: row.bolt_diameter })],
        None => Vec::new(),
    })
}
