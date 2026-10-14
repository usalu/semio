//! Inverse for `change-beam-stud-fu-pa`.
use crate::change_beam_stud_fu_pa::ChangeBeamStudFUPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamStudFUPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_stud_fu_pa::ChangeBeamStudFUPa> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamStudFUPa { index: payload.index, new_f_u_pa: beam.studs.f_u_pa })]

    })())
}
