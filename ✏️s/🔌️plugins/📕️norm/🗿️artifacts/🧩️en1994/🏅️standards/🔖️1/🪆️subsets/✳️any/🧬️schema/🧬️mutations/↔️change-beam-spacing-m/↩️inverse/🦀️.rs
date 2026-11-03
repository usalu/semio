//! ↩️ Inverse for `change-beam-spacing-m`.
use crate::change_beam_spacing_m::ChangeBeamSpacingM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSpacingM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_spacing_m::ChangeBeamSpacingM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSpacingM { id: payload.id.clone(), new_spacing_m: beam.spacing_m })])
}
