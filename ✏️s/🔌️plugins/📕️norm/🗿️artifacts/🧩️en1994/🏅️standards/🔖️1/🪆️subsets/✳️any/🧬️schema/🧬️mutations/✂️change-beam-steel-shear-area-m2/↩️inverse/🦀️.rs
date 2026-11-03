//! ↩️ Inverse for `change-beam-steel-shear-area-m2`.
use crate::change_beam_steel_shear_area_m2::ChangeBeamSteelShearAreaM2;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelShearAreaM2, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_shear_area_m2::ChangeBeamSteelShearAreaM2> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelShearAreaM2 { id: payload.id.clone(), new_a_v_m2: beam.steel.a_v_m2 })])
}
