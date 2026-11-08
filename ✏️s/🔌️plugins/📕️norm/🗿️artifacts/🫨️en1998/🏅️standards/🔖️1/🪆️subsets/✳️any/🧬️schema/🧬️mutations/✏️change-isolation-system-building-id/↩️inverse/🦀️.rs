//! ↩️ `change-isolation-system-building-id` inverse: restores the previous `building_id`.

use crate::change_isolation_system_building_id::ChangeIsolationSystemBuildingId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeIsolationSystemBuildingId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_isolation_system_building_id::ChangeIsolationSystemBuildingId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsolationSystemBuildingId, base: &En1998Snapshot) -> Option<ChangeIsolationSystemBuildingId> {
    let r0 = base.isolation_systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsolationSystemBuildingId { id: payload.id.clone(), new_building_id: r0.building_id.clone() })
}
