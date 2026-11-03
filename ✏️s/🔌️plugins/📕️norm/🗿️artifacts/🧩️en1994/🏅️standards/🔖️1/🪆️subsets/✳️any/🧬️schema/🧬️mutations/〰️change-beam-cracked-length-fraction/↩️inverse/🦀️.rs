//! ↩️ Inverse for `change-beam-cracked-length-fraction`.
use crate::change_beam_cracked_length_fraction::ChangeBeamCrackedLengthFraction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamCrackedLengthFraction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_cracked_length_fraction::ChangeBeamCrackedLengthFraction> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamCrackedLengthFraction { id: payload.id.clone(), new_cracked_length_fraction: beam.cracked_length_fraction })])
}
