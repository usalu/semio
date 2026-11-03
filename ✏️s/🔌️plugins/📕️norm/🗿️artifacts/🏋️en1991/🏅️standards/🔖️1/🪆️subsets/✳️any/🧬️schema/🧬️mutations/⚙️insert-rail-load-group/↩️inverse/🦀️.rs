//! ↩️ `insert-rail-load-group` inverse: removes the inserted row again.

use crate::insert_rail_load_group::InsertRailLoadGroup;
use crate::remove_rail_load_group::RemoveRailLoadGroup;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertRailLoadGroup, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_rail_load_group::RemoveRailLoadGroup> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertRailLoadGroup, base: &En1991Snapshot) -> Option<RemoveRailLoadGroup> {
    Some(RemoveRailLoadGroup { id: payload.item.id.clone() })
}
