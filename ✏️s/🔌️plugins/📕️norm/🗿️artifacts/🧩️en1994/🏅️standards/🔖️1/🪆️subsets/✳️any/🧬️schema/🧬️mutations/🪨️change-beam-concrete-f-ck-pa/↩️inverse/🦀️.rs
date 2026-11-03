//! ↩️ Inverse for `change-beam-concrete-f-ck-pa`.
use crate::change_beam_concrete_f_ck_pa::ChangeBeamConcreteFCkPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamConcreteFCkPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_concrete_f_ck_pa::ChangeBeamConcreteFCkPa> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamConcreteFCkPa { id: payload.id.clone(), new_concrete_f_ck_pa: beam.concrete_f_ck_pa })])
}
