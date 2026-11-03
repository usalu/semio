//! ↩️ Inverse for `change-beam-action-point-force-n`.
use crate::change_beam_action_point_force_n::ChangeBeamActionPointForceN;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamActionPointForceN, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_action_point_force_n::ChangeBeamActionPointForceN> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.beam_id) else {
        return Ok(Vec::new());
    };
    let Some(action_index) = base.beams[index].actions.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    let Some(action) = beam.actions.get(action_index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamActionPointForceN { beam_id: payload.beam_id.clone(), id: payload.id.clone(), new_f_k_n: action.f_k_n })])
}
