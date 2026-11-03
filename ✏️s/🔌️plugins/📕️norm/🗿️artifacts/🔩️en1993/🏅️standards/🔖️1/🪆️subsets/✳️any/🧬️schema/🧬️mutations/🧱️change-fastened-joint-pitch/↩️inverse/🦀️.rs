//! ↩️ `change-fastened-joint-pitch` inverse: restores the previous `pitch`.

use crate::change_fastened_joint_pitch::ChangeFastenedJointPitch;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointPitch, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_pitch::ChangeFastenedJointPitch> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointPitch { id: payload.id.clone(), new_pitch: row.pitch })],
        None => Vec::new(),
    })
}
