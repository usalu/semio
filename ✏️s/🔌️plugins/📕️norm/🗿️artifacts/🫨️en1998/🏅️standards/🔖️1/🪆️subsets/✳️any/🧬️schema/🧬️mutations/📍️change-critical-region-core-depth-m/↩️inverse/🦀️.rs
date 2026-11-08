//! ↩️ `change-critical-region-core-depth-m` inverse: restores the previous `core_depth_m`.

use crate::change_critical_region_core_depth_m::ChangeCriticalRegionCoreDepthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionCoreDepthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_core_depth_m::ChangeCriticalRegionCoreDepthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionCoreDepthM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionCoreDepthM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionCoreDepthM { id: payload.id.clone(), new_core_depth_m: r0.core_depth_m })
}
