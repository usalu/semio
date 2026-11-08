//! ↩️ `change-critical-region-long-steel-area` inverse: restores the previous `longitudinal_steel_area_m2`.

use crate::change_critical_region_long_steel_area::ChangeCriticalRegionLongitudinalSteelAreaM2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionLongitudinalSteelAreaM2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_long_steel_area::ChangeCriticalRegionLongitudinalSteelAreaM2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionLongitudinalSteelAreaM2, base: &En1998Snapshot) -> Option<ChangeCriticalRegionLongitudinalSteelAreaM2> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionLongitudinalSteelAreaM2 { id: payload.id.clone(), new_longitudinal_steel_area_m2: r0.longitudinal_steel_area_m2 })
}
