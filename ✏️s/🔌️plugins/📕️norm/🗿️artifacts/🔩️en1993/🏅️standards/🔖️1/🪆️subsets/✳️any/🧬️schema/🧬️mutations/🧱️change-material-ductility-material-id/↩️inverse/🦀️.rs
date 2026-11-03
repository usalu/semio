//! ↩️ `change-material-ductility-material-id` inverse: restores the previous `material_id`.

use crate::change_material_ductility_material_id::ChangeMaterialDuctilityMaterialId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialDuctilityMaterialId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_ductility_material_id::ChangeMaterialDuctilityMaterialId> {
    Ok(match base.material_ductility.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialDuctilityMaterialId { id: payload.id.clone(), new_material_id: row.material_id.clone() })],
        None => Vec::new(),
    })
}
