//! ↩️ `change-joint-beam-web-fy` inverse: restores the previous `beam_web_fy`.

use crate::change_joint_beam_web_fy::ChangeJointBeamWebFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamWebFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_web_fy::ChangeJointBeamWebFy> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBeamWebFy { id: payload.id.clone(), new_beam_web_fy: row.beam_web_fy })],
        None => Vec::new(),
    })
}
