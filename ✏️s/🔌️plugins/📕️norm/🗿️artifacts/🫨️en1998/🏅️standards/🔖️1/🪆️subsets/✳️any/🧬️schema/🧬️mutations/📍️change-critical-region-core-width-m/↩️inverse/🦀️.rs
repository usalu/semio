//! ↩️ `change-critical-region-core-width-m` inverse: restores the previous `core_width_m`.

use crate::change_critical_region_core_width_m::ChangeCriticalRegionCoreWidthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionCoreWidthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_core_width_m::ChangeCriticalRegionCoreWidthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionCoreWidthM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionCoreWidthM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionCoreWidthM { id: payload.id.clone(), new_core_width_m: r0.core_width_m })
}
