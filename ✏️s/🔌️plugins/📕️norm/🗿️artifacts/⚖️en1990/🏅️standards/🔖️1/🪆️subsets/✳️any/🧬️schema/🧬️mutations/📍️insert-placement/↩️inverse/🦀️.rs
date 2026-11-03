//! ↩️ `insert-placement` inverse: removes the inserted row again.

use crate::insert_placement::InsertPlacement;
use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertPlacement, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_placement::RemovePlacement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertPlacement, base: &En1990Snapshot) -> Option<RemovePlacement> {
    let index = payload.index.unwrap_or(usize::MAX).min(base.placements.len());
    Some(RemovePlacement { index })
}
