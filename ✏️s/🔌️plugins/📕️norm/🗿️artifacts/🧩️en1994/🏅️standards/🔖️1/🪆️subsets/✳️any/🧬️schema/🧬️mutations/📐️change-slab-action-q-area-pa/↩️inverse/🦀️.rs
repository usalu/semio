//! ↩️ Inverse for `change-slab-action-q-area-pa`.
use crate::change_slab_action_q_area_pa::ChangeSlabActionQAreaPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabActionQAreaPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_action_q_area_pa::ChangeSlabActionQAreaPa> {
    Ok((|| {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.slab_id) else {
        return Vec::new();
    };
    let Some(action_index) = base.slabs[index].actions.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(slab) = base.slabs.get(index) else { return Vec::new(); };
    let Some(action) = slab.actions.get(action_index) else { return Vec::new(); };
    vec![Op::from(ChangeSlabActionQAreaPa { slab_id: payload.slab_id.clone(), id: payload.id.clone(), new_q_area_pa: action.q_area_pa })]

    })())
}
