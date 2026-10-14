//! Inverse for `insert-foundation`.
use crate::insert_foundation::InsertFoundation;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_foundation;

pub fn inverse<Op>(payload: &InsertFoundation, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_foundation::RemoveFoundation> {
    Ok((|| {
    vec![Op::from(remove_foundation::RemoveFoundation { index: payload.index.unwrap_or(usize::MAX).min(base.foundations.len()) })]

    })())
}
