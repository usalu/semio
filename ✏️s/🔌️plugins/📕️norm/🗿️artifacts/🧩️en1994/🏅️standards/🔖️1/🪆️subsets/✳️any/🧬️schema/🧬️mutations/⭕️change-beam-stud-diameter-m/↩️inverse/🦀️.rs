//! Inverse for `change-beam-stud-diameter-m`.
use crate::change_beam_stud_diameter_m::ChangeBeamStudDiameterM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamStudDiameterM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_stud_diameter_m::ChangeBeamStudDiameterM> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamStudDiameterM { index: payload.index, new_diameter_m: beam.studs.diameter_m })]

    })())
}
