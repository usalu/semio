//! ↩️ `change-joint-beam-moment-res` inverse: restores the previous `beam_moment_resistance`.

use crate::change_joint_beam_moment_res::ChangeJointBeamMomentRes;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamMomentRes, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_moment_res::ChangeJointBeamMomentRes> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBeamMomentRes { id: payload.id.clone(), new_beam_moment_resistance: row.beam_moment_resistance })],
        None => Vec::new(),
    })
}
