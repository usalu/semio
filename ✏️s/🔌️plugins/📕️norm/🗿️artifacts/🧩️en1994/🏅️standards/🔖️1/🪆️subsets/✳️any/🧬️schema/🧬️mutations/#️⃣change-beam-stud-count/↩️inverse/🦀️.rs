//! Inverse for `change-beam-stud-count`.
use crate::change_beam_stud_count::ChangeBeamStudCount;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamStudCount, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_stud_count::ChangeBeamStudCount> {
    Ok((|| {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamStudCount { index: payload.index, new_total_count: beam.studs.total_count })]

    })())
}
