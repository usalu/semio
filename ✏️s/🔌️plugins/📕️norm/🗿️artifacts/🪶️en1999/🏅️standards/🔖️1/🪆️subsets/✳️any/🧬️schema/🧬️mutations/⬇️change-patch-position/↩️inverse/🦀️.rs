//! ↩️ `change-patch-position` inverse: restores the previous `position`.

use crate::change_patch_position::ChangePatchPosition;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchPosition, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_position::ChangePatchPosition> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchPosition { patch_load_id: payload.patch_load_id.clone(), new_position: row.position })],
        None => Vec::new(),
    })
}
