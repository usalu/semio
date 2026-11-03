//! ↩️ `change-patch-web-angle` inverse: restores the previous `web_angle`.

use crate::change_patch_web_angle::ChangePatchWebAngle;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchWebAngle, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_web_angle::ChangePatchWebAngle> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchWebAngle { patch_load_id: payload.patch_load_id.clone(), new_web_angle: row.web_angle })],
        None => Vec::new(),
    })
}
