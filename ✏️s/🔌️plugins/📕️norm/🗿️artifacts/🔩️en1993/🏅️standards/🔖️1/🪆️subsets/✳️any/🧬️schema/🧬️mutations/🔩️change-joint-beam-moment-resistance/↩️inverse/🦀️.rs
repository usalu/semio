//! ↩️ `change-joint-beam-moment-resistance` inverse: restores the previous `beam_moment_resistance`.

use crate::change_joint_beam_moment_resistance::ChangeJointBeamMomentResistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamMomentResistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_moment_resistance::ChangeJointBeamMomentResistance> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBeamMomentResistance { id: payload.id.clone(), new_beam_moment_resistance: row.beam_moment_resistance })],
        None => Vec::new(),
    })
}
