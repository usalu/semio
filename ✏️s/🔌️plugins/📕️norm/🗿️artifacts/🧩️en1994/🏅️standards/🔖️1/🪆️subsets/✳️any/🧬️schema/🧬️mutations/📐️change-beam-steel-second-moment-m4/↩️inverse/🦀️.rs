//! ↩️ Inverse for `change-beam-steel-second-moment-m4`.
use crate::change_beam_steel_second_moment_m4::ChangeBeamSteelSecondMomentM4;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelSecondMomentM4, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_second_moment_m4::ChangeBeamSteelSecondMomentM4> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelSecondMomentM4 { id: payload.id.clone(), new_i_y_m4: beam.steel.i_y_m4 })])
}
