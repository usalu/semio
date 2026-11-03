//! ↩️ `change-buckling-length-beam-stiffness-b` inverse: restores the previous `beam_stiffness_b`.

use crate::change_buckling_length_beam_stiffness_b::ChangeBucklingLengthBeamStiffnessB;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthBeamStiffnessB, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_beam_stiffness_b::ChangeBucklingLengthBeamStiffnessB> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthBeamStiffnessB { id: payload.id.clone(), new_beam_stiffness_b: row.beam_stiffness_b })],
        None => Vec::new(),
    })
}
