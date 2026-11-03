//! ↩️ Inverse for `change-beam-steel-tf-m`.
use crate::change_beam_steel_tf_m::ChangeBeamSteelTfM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamSteelTfM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_steel_tf_m::ChangeBeamSteelTfM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamSteelTfM { id: payload.id.clone(), new_tf_m: beam.steel.tf_m })])
}
