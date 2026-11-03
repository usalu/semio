//! ↩️ `insert-placement` inverse: removes the inserted row again.

use crate::insert_placement::InsertPlacement;
use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &InsertPlacement, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_placement::RemovePlacement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertPlacement, _base: &En1994Snapshot) -> Option<RemovePlacement> {
    Some(RemovePlacement { id: payload.item.id.clone() })
}
