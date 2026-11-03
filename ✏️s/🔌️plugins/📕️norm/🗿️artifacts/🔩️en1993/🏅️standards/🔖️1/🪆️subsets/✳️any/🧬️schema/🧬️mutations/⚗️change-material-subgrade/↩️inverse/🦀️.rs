//! ↩️ `change-material-subgrade` inverse: restores the previous `subgrade`.

use crate::change_material_subgrade::ChangeMaterialSubgrade;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialSubgrade, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_subgrade::ChangeMaterialSubgrade> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialSubgrade { id: payload.id.clone(), new_subgrade: row.subgrade.clone() })],
        None => Vec::new(),
    })
}
