//! ↩️ `remove-traffic-group` inverse: inserts the removed row back at its position.

use crate::remove_traffic_group::RemoveTrafficGroup;
use crate::insert_traffic_group::InsertTrafficGroup;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveTrafficGroup, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_traffic_group::InsertTrafficGroup> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTrafficGroup, base: &En1990Snapshot) -> Option<InsertTrafficGroup> {
    let index = payload.index;
    let row = base.traffic_groups.get(index)?;
    Some(InsertTrafficGroup { index: Some(index), item: row.clone() })
}
