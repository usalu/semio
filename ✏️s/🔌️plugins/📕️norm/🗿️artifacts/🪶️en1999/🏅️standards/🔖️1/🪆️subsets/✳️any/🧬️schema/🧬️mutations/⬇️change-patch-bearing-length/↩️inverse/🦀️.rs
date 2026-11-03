//! ↩️ `change-patch-bearing-length` inverse: restores the previous `bearing_length`.

use crate::change_patch_bearing_length::ChangePatchBearingLength;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePatchBearingLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_patch_bearing_length::ChangePatchBearingLength> {
    Ok(match base.patch_loads.iter().find(|row| row.id == payload.patch_load_id) {
        Some(row) => vec![Op::from(ChangePatchBearingLength { patch_load_id: payload.patch_load_id.clone(), new_bearing_length: row.bearing_length })],
        None => Vec::new(),
    })
}
