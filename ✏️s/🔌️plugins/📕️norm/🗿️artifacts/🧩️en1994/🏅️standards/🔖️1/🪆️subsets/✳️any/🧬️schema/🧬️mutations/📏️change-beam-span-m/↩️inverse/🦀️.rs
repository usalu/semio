//! ↩️ Inverse for `change-beam-span-m`.
use crate::change_beam_span_m::ChangeBeamSpanM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSpanM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_span_m::ChangeBeamSpanM> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamSpanM { index: payload.index, new_span_m: beam.span_m })]

    })())
}
