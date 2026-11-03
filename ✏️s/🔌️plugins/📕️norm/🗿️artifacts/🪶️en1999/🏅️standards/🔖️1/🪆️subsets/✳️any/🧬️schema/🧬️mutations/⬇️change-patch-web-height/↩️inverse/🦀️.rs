//! ↩️ `change-patch-web-height` inverse: restores the previous `web_height`.

use crate::change_patch_web_height::ChangePatchWebHeight;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchWebHeight, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_web_height::ChangePatchWebHeight> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchWebHeight { patch_load_id: payload.patch_load_id.clone(), new_web_height: row.web_height })],
        None => Vec::new(),
    })
}
