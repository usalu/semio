//! ↩️ `remove-placement` inverse: inserts the removed row back at its position.

use crate::remove_placement::RemovePlacement;
use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &RemovePlacement, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_placement::InsertPlacement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePlacement, base: &En1994Snapshot) -> Option<InsertPlacement> {
    let index = base.placements.iter().position(|row| row.id == payload.id)?;
    let row = base.placements.get(index)?;
    Some(InsertPlacement { index, item: row.clone() })
}
