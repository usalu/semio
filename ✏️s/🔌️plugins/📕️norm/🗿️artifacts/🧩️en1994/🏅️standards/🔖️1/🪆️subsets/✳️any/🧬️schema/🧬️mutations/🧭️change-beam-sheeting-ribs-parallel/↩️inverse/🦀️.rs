//! ↩️ Inverse for `change-beam-sheeting-ribs-parallel`.
use crate::change_beam_sheeting_ribs_parallel::ChangeBeamSheetingRibsParallel;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSheetingRibsParallel, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_sheeting_ribs_parallel::ChangeBeamSheetingRibsParallel> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSheetingRibsParallel { id: payload.id.clone(), new_ribs_parallel_to_beam: beam.sheeting.ribs_parallel_to_beam })])
}
