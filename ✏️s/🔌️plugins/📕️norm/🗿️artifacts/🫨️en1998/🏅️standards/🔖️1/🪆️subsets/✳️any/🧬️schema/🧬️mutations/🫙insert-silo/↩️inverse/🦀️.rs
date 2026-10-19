//! ↩️ Inverse for `insert-silo`.
use crate::insert_silo::InsertSilo;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_silo;

pub fn inverse<Op>(payload: &InsertSilo, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_silo::RemoveSilo> {
    Ok((|| {
    vec![Op::from(remove_silo::RemoveSilo { index: payload.index.unwrap_or(usize::MAX).min(base.silos.len()) })]

    })())
}
