//! ↩️ `change-self-weight-material` inverse: restores the previous `material`.

use crate::change_self_weight_material::ChangeSelfWeightMaterial;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSelfWeightMaterial, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_self_weight_material::ChangeSelfWeightMaterial> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSelfWeightMaterial, base: &En1991Snapshot) -> Option<ChangeSelfWeightMaterial> {
    let r0 = base.self_weight_elements.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSelfWeightMaterial { id: payload.id.clone(), new_material: r0.material.clone() })
}
