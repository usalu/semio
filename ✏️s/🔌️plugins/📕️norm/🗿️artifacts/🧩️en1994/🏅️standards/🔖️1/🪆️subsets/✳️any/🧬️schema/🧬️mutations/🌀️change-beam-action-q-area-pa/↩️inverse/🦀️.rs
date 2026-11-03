//! ↩️ Inverse for `change-beam-action-q-area-pa`.
use crate::change_beam_action_q_area_pa::ChangeBeamActionQAreaPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamActionQAreaPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_action_q_area_pa::ChangeBeamActionQAreaPa> {
    Ok((|| {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.beam_id) else {
        return Vec::new();
    };
    let Some(action_index) = base.beams[index].actions.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(beam) = base.beams.get(index) else { return Vec::new(); };
    let Some(action) = beam.actions.get(action_index) else { return Vec::new(); };
    vec![Op::from(ChangeBeamActionQAreaPa { beam_id: payload.beam_id.clone(), id: payload.id.clone(), new_q_area_pa: action.q_area_pa })]

    })())
}
