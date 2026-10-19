//! ↩️ Inverse for `remove-slab`.
use crate::remove_slab::RemoveSlab;
use crate::insert_slab::InsertSlab;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &RemoveSlab, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_slab::InsertSlab> {
    Ok((|| {
    let Some(slab) = base.slabs.get(payload.index).cloned() else { return Vec::new(); };
    vec![Op::from(InsertSlab { index: payload.index, slab })]

    })())
}
