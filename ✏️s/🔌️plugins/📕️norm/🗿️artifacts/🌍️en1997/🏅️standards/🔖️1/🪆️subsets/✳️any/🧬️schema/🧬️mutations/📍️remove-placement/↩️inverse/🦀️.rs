//! ↩️ `remove-placement` inverse: inserts the removed row back at its position.

use crate::remove_placement::RemovePlacement;
use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &RemovePlacement, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_placement::InsertPlacement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePlacement, base: &En1997Snapshot) -> Option<InsertPlacement> {
    let index = payload.index;
    let row = base.placements.get(index)?;
    Some(InsertPlacement { index, item: row.clone() })
}
