//! ↩️ `change-joint-plate-plate-width` inverse: restores the previous `plate_width`.

use crate::change_joint_plate_plate_width::ChangeJointPlatePlateWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlatePlateWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_plate_width::ChangeJointPlatePlateWidth> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlatePlateWidth { id: payload.id.clone(), new_plate_width: row.plate_width })],
        None => Vec::new(),
    })
}
