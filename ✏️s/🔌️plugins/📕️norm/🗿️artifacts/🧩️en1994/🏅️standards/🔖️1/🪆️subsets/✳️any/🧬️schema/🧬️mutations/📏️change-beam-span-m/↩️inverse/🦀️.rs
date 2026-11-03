//! ↩️ Inverse for `change-beam-span-m`.
use crate::change_beam_span_m::ChangeBeamSpanM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSpanM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_span_m::ChangeBeamSpanM> {
    Ok((|| {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(beam) = base.beams.get(index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamSpanM { id: payload.id.clone(), new_span_m: beam.span_m })]

    })())
}
