//! ↩️ `insert-rail-dynamic` inverse: removes the inserted row again.

use crate::insert_rail_dynamic::InsertRailDynamic;
use crate::remove_rail_dynamic::RemoveRailDynamic;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertRailDynamic, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_rail_dynamic::RemoveRailDynamic> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertRailDynamic, base: &En1991Snapshot) -> Option<RemoveRailDynamic> {
    Some(RemoveRailDynamic { id: payload.item.id.clone() })
}
