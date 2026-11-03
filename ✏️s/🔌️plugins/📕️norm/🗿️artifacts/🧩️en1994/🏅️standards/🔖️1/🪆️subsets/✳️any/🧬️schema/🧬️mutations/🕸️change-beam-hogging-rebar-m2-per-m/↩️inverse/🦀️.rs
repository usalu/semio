//! ↩️ Inverse for `change-beam-hogging-rebar-m2-per-m`.
use crate::change_beam_hogging_rebar_m2_per_m::ChangeBeamHoggingRebarM2PerM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamHoggingRebarM2PerM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_hogging_rebar_m2_per_m::ChangeBeamHoggingRebarM2PerM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamHoggingRebarM2PerM { id: payload.id.clone(), new_as_hogging_m2_per_m: beam.as_hogging_m2_per_m })])
}
