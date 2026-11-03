//! ↩️ `change-joint-beam-inertia` inverse: restores the previous `beam_inertia`.

use crate::change_joint_beam_inertia::ChangeJointBeamInertia;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamInertia, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_inertia::ChangeJointBeamInertia> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBeamInertia { id: payload.id.clone(), new_beam_inertia: row.beam_inertia })],
        None => Vec::new(),
    })
}
