//! ↩️ Inverse for `change-beam-stud-spacing-m`.
use crate::change_beam_stud_spacing_m::ChangeBeamStudSpacingM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamStudSpacingM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_stud_spacing_m::ChangeBeamStudSpacingM> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamStudSpacingM { index: payload.index, new_spacing_m: beam.studs.spacing_m })]

    })())
}
