//! ↩️ Inverse for `change-beam-concrete-e-cm-pa`.
use crate::change_beam_concrete_e_cm_pa::ChangeBeamConcreteECmPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamConcreteECmPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_concrete_e_cm_pa::ChangeBeamConcreteECmPa> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamConcreteECmPa { id: payload.id.clone(), new_concrete_e_cm_pa: beam.concrete_e_cm_pa })])
}
