//! ↩️ Inverse for `change-beam-crack-width-limit-m`.
use crate::change_beam_crack_width_limit_m::ChangeBeamCrackWidthLimitM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamCrackWidthLimitM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_crack_width_limit_m::ChangeBeamCrackWidthLimitM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamCrackWidthLimitM { id: payload.id.clone(), new_wk_limit_m: beam.wk_limit_m })])
}
