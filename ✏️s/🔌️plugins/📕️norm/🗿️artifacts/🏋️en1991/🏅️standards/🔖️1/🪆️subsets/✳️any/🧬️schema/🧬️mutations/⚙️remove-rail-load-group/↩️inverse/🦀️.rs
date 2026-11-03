//! ↩️ `remove-rail-load-group` inverse: inserts the removed row back at its position.

use crate::remove_rail_load_group::RemoveRailLoadGroup;
use crate::insert_rail_load_group::InsertRailLoadGroup;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveRailLoadGroup, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_rail_load_group::InsertRailLoadGroup> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveRailLoadGroup, base: &En1991Snapshot) -> Option<InsertRailLoadGroup> {
    let (index, row) = base.rail_load_groups.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertRailLoadGroup { index, item: row.clone() })
}
