//! ↩️ Inverse for `remove-foundation`.
use crate::remove_foundation::RemoveFoundation;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_foundation;

pub fn inverse<Op>(payload: &RemoveFoundation, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_foundation::InsertFoundation> {
    Ok((|| {
    match base.foundations.get(payload.index) {
        Some(item) => vec![Op::from(insert_foundation::InsertFoundation { index: Some(payload.index), foundation: item.clone() })],
        None => Vec::new(),
    }

    })())
}
