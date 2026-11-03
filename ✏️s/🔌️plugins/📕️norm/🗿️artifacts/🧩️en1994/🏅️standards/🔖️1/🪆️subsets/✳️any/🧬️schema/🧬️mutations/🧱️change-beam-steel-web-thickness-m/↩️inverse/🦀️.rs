//! ↩️ Inverse for `change-beam-steel-web-thickness-m`.
use crate::change_beam_steel_web_thickness_m::ChangeBeamSteelWebThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelWebThicknessM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_web_thickness_m::ChangeBeamSteelWebThicknessM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelWebThicknessM { id: payload.id.clone(), new_tw_m: beam.steel.tw_m })])
}
