//! ↩️ Inverse for `insert-slab`.
use crate::insert_slab::InsertSlab;
use crate::remove_slab::RemoveSlab;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &InsertSlab, _base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_slab::RemoveSlab> {
    Ok((|| {
    vec![Op::from(RemoveSlab { index: payload.index })]

    })())
}
