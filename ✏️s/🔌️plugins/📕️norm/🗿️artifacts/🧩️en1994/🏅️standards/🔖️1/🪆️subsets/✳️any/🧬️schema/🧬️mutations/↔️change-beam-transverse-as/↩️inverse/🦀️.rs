//! ↩️ Inverse for `change-beam-transverse-as`.
use crate::change_beam_transverse_as::ChangeBeamTransverseAs;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamTransverseAs, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_transverse_as::ChangeBeamTransverseAs> {
    Ok((|| {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(beam) = base.beams.get(index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamTransverseAs { id: payload.id.clone(), new_transverse_as_m2_per_m: beam.transverse_as_m2_per_m })]

    })())
}
