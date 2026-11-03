//! ↩️ Inverse for `change-beam-steel-height-m`.
use crate::change_beam_steel_height_m::ChangeBeamSteelHeightM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelHeightM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_height_m::ChangeBeamSteelHeightM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelHeightM { id: payload.id.clone(), new_height_m: beam.steel.height_m })])
}
