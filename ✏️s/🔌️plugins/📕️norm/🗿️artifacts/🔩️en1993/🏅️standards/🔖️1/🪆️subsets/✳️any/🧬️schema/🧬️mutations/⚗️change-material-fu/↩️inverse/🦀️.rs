//! ↩️ `change-material-fu` inverse: restores the previous `fu`.

use crate::change_material_fu::ChangeMaterialFu;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialFu, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_fu::ChangeMaterialFu> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialFu { id: payload.id.clone(), new_fu: row.fu })],
        None => Vec::new(),
    })
}
