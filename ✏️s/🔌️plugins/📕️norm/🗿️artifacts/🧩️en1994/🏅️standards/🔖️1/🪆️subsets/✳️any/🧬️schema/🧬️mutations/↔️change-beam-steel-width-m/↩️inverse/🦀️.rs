//! ↩️ Inverse for `change-beam-steel-width-m`.
use crate::change_beam_steel_width_m::ChangeBeamSteelWidthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelWidthM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_width_m::ChangeBeamSteelWidthM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelWidthM { id: payload.id.clone(), new_width_m: beam.steel.width_m })])
}
