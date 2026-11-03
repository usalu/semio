//! ↩️ `insert-system` inverse: removes the inserted row again.

use crate::insert_system::InsertSystem;
use crate::remove_system::RemoveSystem;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertSystem, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_system::RemoveSystem> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSystem, base: &En1998Snapshot) -> Option<RemoveSystem> {
    let _r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    Some(RemoveSystem { building_id: payload.building_id.clone(), id: payload.system.id.clone() })
}
