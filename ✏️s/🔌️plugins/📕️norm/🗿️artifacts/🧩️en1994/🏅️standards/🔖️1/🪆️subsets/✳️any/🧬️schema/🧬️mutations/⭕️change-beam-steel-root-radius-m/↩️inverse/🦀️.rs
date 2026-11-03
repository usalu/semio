//! ↩️ Inverse for `change-beam-steel-root-radius-m`.
use crate::change_beam_steel_root_radius_m::ChangeBeamSteelRootRadiusM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelRootRadiusM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_root_radius_m::ChangeBeamSteelRootRadiusM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelRootRadiusM { id: payload.id.clone(), new_r_m: beam.steel.r_m })])
}
