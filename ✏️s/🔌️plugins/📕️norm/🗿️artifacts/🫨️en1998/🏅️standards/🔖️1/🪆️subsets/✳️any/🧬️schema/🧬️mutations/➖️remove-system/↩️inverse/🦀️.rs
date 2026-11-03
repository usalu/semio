//! ↩️ `remove-system` inverse: inserts the removed row back at its position.

use crate::remove_system::RemoveSystem;
use crate::insert_system::InsertSystem;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveSystem, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_system::InsertSystem> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSystem, base: &En1998Snapshot) -> Option<InsertSystem> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let (index, row) = r0.systems.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSystem { building_id: payload.building_id.clone(), index: Some(index), system: row.clone() })
}
