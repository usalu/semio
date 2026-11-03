//! ↩️ `remove-patch-load` inverse.

use crate::remove_patch_load::RemovePatchLoad;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_patch_load;

pub fn inverse<Op>(payload: &RemovePatchLoad, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_patch_load::InsertPatchLoad> {
    Ok((|| {
    if let Some((index, row)) = base.patch_loads.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_patch_load::InsertPatchLoad { index: index as u32, patch_load: row.clone() })]
    } else { Vec::new() }

    })())
}
