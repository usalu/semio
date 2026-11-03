//! ↩️ `change-material-ductility-elongation` inverse: restores the previous `elongation`.

use crate::change_material_ductility_elongation::ChangeMaterialDuctilityElongation;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialDuctilityElongation, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_ductility_elongation::ChangeMaterialDuctilityElongation> {
    Ok(match base.material_ductility.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialDuctilityElongation { id: payload.id.clone(), new_elongation: row.elongation })],
        None => Vec::new(),
    })
}
