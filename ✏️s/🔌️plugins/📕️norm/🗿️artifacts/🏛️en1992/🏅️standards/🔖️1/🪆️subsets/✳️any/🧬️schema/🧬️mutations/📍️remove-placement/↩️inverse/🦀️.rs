//! ↩️ `remove-placement` inverse: inserts the removed row back at its position.

use crate::remove_placement::RemovePlacement;
use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemovePlacement, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_placement::InsertPlacement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePlacement, base: &En1992Snapshot) -> Option<InsertPlacement> {
    let index = payload.index;
    let row = base.placements.get(index)?;
    Some(InsertPlacement { index, placement: row.clone() })
}
