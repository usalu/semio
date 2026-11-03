//! ↩️ Inverse for `change-slab-action-kind`.
use crate::change_slab_action_kind::ChangeSlabActionKind;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabActionKind, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_action_kind::ChangeSlabActionKind> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.slab_id) else {
        return Ok(Vec::new());
    };
    let Some(action_index) = base.slabs[index].actions.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    let Some(action) = slab.actions.get(action_index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabActionKind { slab_id: payload.slab_id.clone(), id: payload.id.clone(), new_kind: action.kind })])
}
