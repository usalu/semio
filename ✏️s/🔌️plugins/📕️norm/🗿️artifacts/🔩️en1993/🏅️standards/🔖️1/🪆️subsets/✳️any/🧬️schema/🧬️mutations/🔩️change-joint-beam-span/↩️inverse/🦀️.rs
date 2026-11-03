//! ↩️ `change-joint-beam-span` inverse: restores the previous `beam_span`.

use crate::change_joint_beam_span::ChangeJointBeamSpan;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamSpan, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_span::ChangeJointBeamSpan> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBeamSpan { id: payload.id.clone(), new_beam_span: row.beam_span })],
        None => Vec::new(),
    })
}
