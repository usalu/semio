//! ↩️ `change-patch-corner-radius` inverse: restores the previous `corner_radius`.

use crate::change_patch_corner_radius::ChangePatchCornerRadius;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchCornerRadius, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_corner_radius::ChangePatchCornerRadius> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchCornerRadius { patch_load_id: payload.patch_load_id.clone(), new_corner_radius: row.corner_radius })],
        None => Vec::new(),
    })
}
