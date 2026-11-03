//! ↩️ `remove-rail-dynamic` inverse: inserts the removed row back at its position.

use crate::remove_rail_dynamic::RemoveRailDynamic;
use crate::insert_rail_dynamic::InsertRailDynamic;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveRailDynamic, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_rail_dynamic::InsertRailDynamic> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveRailDynamic, base: &En1991Snapshot) -> Option<InsertRailDynamic> {
    let (index, row) = base.rail_dynamics.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertRailDynamic { index, item: row.clone() })
}
