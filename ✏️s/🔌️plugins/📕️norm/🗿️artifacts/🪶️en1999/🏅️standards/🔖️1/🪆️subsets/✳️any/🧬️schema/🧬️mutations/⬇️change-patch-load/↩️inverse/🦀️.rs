//! ↩️ `change-patch-load` inverse: restores the previous `load`.

use crate::change_patch_load::ChangePatchLoad;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchLoad, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_load::ChangePatchLoad> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchLoad { patch_load_id: payload.patch_load_id.clone(), new_load: row.load })],
        None => Vec::new(),
    })
}
