//! ↩️ `change-joint-plate-pitch` inverse: restores the previous `pitch`.

use crate::change_joint_plate_pitch::ChangeJointPlatePitch;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlatePitch, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_pitch::ChangeJointPlatePitch> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlatePitch { id: payload.id.clone(), new_pitch: row.pitch })],
        None => Vec::new(),
    })
}
