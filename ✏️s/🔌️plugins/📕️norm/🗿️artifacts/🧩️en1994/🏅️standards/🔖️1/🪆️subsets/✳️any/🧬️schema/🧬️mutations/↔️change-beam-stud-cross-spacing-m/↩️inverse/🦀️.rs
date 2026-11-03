//! ↩️ Inverse for `change-beam-stud-cross-spacing-m`.
use crate::change_beam_stud_cross_spacing_m::ChangeBeamStudCrossSpacingM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamStudCrossSpacingM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_stud_cross_spacing_m::ChangeBeamStudCrossSpacingM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamStudCrossSpacingM { id: payload.id.clone(), new_stud_transverse_spacing_m: beam.stud_transverse_spacing_m })])
}
