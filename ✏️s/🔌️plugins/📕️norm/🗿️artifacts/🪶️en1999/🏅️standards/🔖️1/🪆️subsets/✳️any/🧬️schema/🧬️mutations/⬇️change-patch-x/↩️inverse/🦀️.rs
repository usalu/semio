//! ↩️ `change-patch-x` inverse: restores the previous `x`.

use crate::change_patch_x::ChangePatchX;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchX, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_x::ChangePatchX> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchX { patch_load_id: payload.patch_load_id.clone(), new_x: row.x })],
        None => Vec::new(),
    })
}
