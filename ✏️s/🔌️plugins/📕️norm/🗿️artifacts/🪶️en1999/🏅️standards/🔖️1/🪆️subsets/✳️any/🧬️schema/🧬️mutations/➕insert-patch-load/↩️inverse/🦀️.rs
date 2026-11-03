//! ↩️ `insert-patch-load` inverse.

use crate::insert_patch_load::InsertPatchLoad;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_patch_load;

pub fn inverse<Op>(payload: &InsertPatchLoad, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_patch_load::RemovePatchLoad> {
    Ok((|| {
    vec![Op::from(remove_patch_load::RemovePatchLoad { id: payload.patch_load.id.clone() })]

    })())
}
