//! ↩️ `insert-foundation` inverse: removes the inserted row again.

use crate::insert_foundation::InsertFoundation;
use crate::remove_foundation::RemoveFoundation;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertFoundation, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_foundation::RemoveFoundation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertFoundation, base: &En1998Snapshot) -> Option<RemoveFoundation> {
    Some(RemoveFoundation { id: payload.foundation.id.clone() })
}
