//! ↩️ `change-diaphragm-material` inverse: restores the previous `material`.

use crate::change_diaphragm_material::ChangeDiaphragmMaterial;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmMaterial, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_material::ChangeDiaphragmMaterial> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmMaterial, base: &En1998Snapshot) -> Option<ChangeDiaphragmMaterial> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmMaterial { id: payload.id.clone(), new_material: r0.material })
}
