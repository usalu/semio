//! ↩️ Inverse for `change-slab-action-delta-sigma-pa`.
use crate::change_slab_action_delta_sigma_pa::ChangeSlabActionDeltaSigmaPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabActionDeltaSigmaPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_action_delta_sigma_pa::ChangeSlabActionDeltaSigmaPa> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.slab_id) else {
        return Ok(Vec::new());
    };
    let Some(action_index) = base.slabs[index].actions.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    let Some(action) = slab.actions.get(action_index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabActionDeltaSigmaPa { slab_id: payload.slab_id.clone(), id: payload.id.clone(), new_delta_sigma_k_pa: action.delta_sigma_k_pa })])
}
