//! ↩️ `change-critical-region-building-id` inverse: restores the previous `building_id`.

use crate::change_critical_region_building_id::ChangeCriticalRegionBuildingId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionBuildingId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_building_id::ChangeCriticalRegionBuildingId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionBuildingId, base: &En1998Snapshot) -> Option<ChangeCriticalRegionBuildingId> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionBuildingId { id: payload.id.clone(), new_building_id: r0.building_id.clone() })
}
