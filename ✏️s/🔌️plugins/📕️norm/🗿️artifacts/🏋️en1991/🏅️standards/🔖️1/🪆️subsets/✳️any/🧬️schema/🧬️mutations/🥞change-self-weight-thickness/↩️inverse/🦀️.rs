//! ↩️ `change-self-weight-thickness` inverse: restores the previous `thickness`.

use crate::change_self_weight_thickness::ChangeSelfWeightThickness;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSelfWeightThickness, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_self_weight_thickness::ChangeSelfWeightThickness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSelfWeightThickness, base: &En1991Snapshot) -> Option<ChangeSelfWeightThickness> {
    let r0 = base.self_weight_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSelfWeightThickness { id: payload.id.clone(), new_thickness: r0.thickness })
}
