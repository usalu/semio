//! ↩️ `change-buckling-length-beam-stiffness-a` inverse: restores the previous `beam_stiffness_a`.

use crate::change_buckling_length_beam_stiffness_a::ChangeBucklingLengthBeamStiffnessA;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthBeamStiffnessA, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_beam_stiffness_a::ChangeBucklingLengthBeamStiffnessA> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthBeamStiffnessA { id: payload.id.clone(), new_beam_stiffness_a: row.beam_stiffness_a })],
        None => Vec::new(),
    })
}
