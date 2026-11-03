//! ↩️ Inverse for `change-slab-action-delta-tau-pa`.
use crate::change_slab_action_delta_tau_pa::ChangeSlabActionDeltaTauPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabActionDeltaTauPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_action_delta_tau_pa::ChangeSlabActionDeltaTauPa> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.slab_id) else {
        return Ok(Vec::new());
    };
    let Some(action_index) = base.slabs[index].actions.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    let Some(action) = slab.actions.get(action_index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabActionDeltaTauPa { slab_id: payload.slab_id.clone(), id: payload.id.clone(), new_delta_tau_k_pa: action.delta_tau_k_pa })])
}
