//! ↩️ `remove-placement` inverse: inserts the removed row back at its position.

use crate::remove_placement::RemovePlacement;
use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemovePlacement, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_placement::InsertPlacement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePlacement, base: &En1996Snapshot) -> Option<InsertPlacement> {
    let (index, row) = base.placements.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertPlacement { index, item: row.clone() })
}
