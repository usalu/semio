//! ↩️ `remove-accelerogram` inverse: inserts the removed row back at its position.

use crate::remove_accelerogram::RemoveAccelerogram;
use crate::insert_accelerogram::InsertAccelerogram;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveAccelerogram, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_accelerogram::InsertAccelerogram> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveAccelerogram, base: &En1998Snapshot) -> Option<InsertAccelerogram> {
    let (index, row) = base.accelerograms.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertAccelerogram { index: Some(index), accelerogram: row.clone() })
}
