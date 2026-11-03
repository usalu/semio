//! ↩️ `change-material-fy` inverse: restores the previous `fy`.

use crate::change_material_fy::ChangeMaterialFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_fy::ChangeMaterialFy> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialFy { id: payload.id.clone(), new_fy: row.fy })],
        None => Vec::new(),
    })
}
