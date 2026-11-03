//! ↩️ `insert-transverse-stiffener` inverse.

use crate::insert_transverse_stiffener::InsertTransverseStiffener;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_transverse_stiffener;

pub fn inverse<Op>(payload: &InsertTransverseStiffener, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_transverse_stiffener::RemoveTransverseStiffener> {
    Ok((|| {
    vec![Op::from(remove_transverse_stiffener::RemoveTransverseStiffener { id: payload.transverse_stiffener.id.clone() })]

    })())
}
