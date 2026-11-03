//! ↩️ `change-joint-beam-flange-thickness` inverse: restores the previous `beam_flange_thickness`.

use crate::change_joint_beam_flange_thickness::ChangeJointBeamFlangeThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamFlangeThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_flange_thickness::ChangeJointBeamFlangeThickness> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBeamFlangeThickness { id: payload.id.clone(), new_beam_flange_thickness: row.beam_flange_thickness })],
        None => Vec::new(),
    })
}
