//! Inverse for `change-slab-action-q-area-pa`.
use crate::change_slab_action_q_area_pa::ChangeSlabActionQAreaPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabActionQAreaPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_action_q_area_pa::ChangeSlabActionQAreaPa> {
    Ok((|| {
    let Some(slab) = base.slabs.get(payload.index) else { return Vec::new(); };
    let Some(action) = slab.actions.get(payload.action_index) else { return Vec::new(); };
    vec![Op::from(ChangeSlabActionQAreaPa { index: payload.index, action_index: payload.action_index, new_q_area_pa: action.q_area_pa })]

    })())
}
