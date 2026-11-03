//! ↩️ `insert-imperfection` inverse: removes the inserted row again.

use crate::insert_imperfection::InsertImperfection;
use crate::remove_imperfection::RemoveImperfection;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertImperfection, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_imperfection::RemoveImperfection> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertImperfection, base: &En1991Snapshot) -> Option<RemoveImperfection> {
    Some(RemoveImperfection { id: payload.item.id.clone() })
}
