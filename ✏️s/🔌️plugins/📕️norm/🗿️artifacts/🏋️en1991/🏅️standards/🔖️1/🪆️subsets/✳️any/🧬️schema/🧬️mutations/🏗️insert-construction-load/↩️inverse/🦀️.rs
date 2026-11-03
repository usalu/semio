//! ↩️ `insert-construction-load` inverse: removes the inserted row again.

use crate::insert_construction_load::InsertConstructionLoad;
use crate::remove_construction_load::RemoveConstructionLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertConstructionLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_construction_load::RemoveConstructionLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertConstructionLoad, base: &En1991Snapshot) -> Option<RemoveConstructionLoad> {
    Some(RemoveConstructionLoad { id: payload.item.id.clone() })
}
