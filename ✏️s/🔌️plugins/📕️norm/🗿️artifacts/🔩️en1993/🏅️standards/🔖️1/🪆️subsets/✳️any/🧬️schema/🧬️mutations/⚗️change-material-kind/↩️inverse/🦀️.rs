//! ↩️ `change-material-kind` inverse: restores the previous `kind`.

use crate::change_material_kind::ChangeMaterialKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_kind::ChangeMaterialKind> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
