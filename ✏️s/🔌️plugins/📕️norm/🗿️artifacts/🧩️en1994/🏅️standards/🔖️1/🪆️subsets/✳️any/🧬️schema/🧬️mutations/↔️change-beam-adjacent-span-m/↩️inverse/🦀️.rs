//! ↩️ Inverse for `change-beam-adjacent-span-m`.
use crate::change_beam_adjacent_span_m::ChangeBeamAdjacentSpanM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamAdjacentSpanM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_adjacent_span_m::ChangeBeamAdjacentSpanM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamAdjacentSpanM { id: payload.id.clone(), new_adjacent_span_m: beam.adjacent_span_m })])
}
