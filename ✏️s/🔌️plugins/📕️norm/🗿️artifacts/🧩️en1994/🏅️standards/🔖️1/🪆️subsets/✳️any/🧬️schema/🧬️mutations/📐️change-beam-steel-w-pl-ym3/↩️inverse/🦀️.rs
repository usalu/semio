//! ↩️ Inverse for `change-beam-steel-w-pl-ym3`.
use crate::change_beam_steel_w_pl_y_m3::ChangeBeamSteelWPlYM3;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelWPlYM3, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_w_pl_y_m3::ChangeBeamSteelWPlYM3> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelWPlYM3 { id: payload.id.clone(), new_w_pl_y_m3: beam.steel.w_pl_y_m3 })])
}
