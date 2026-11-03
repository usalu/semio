//! ↩️ `change-joint-pitch` inverse: restores the previous `pitch`.

use crate::change_joint_pitch::ChangeJointPitch;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPitch, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_pitch::ChangeJointPitch> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPitch { id: payload.id.clone(), new_pitch: row.pitch })],
        None => Vec::new(),
    })
}
