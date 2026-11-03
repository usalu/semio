//! ↩️ `change-joint-beam-web-thickness` inverse: restores the previous `beam_web_thickness`.

use crate::change_joint_beam_web_thickness::ChangeJointBeamWebThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamWebThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_web_thickness::ChangeJointBeamWebThickness> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBeamWebThickness { id: payload.id.clone(), new_beam_web_thickness: row.beam_web_thickness })],
        None => Vec::new(),
    })
}
