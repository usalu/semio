//! ↩️ `insert-accelerogram` inverse: removes the inserted row again.

use crate::insert_accelerogram::InsertAccelerogram;
use crate::remove_accelerogram::RemoveAccelerogram;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertAccelerogram, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_accelerogram::RemoveAccelerogram> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertAccelerogram, base: &En1998Snapshot) -> Option<RemoveAccelerogram> {
    Some(RemoveAccelerogram { id: payload.accelerogram.id.clone() })
}
