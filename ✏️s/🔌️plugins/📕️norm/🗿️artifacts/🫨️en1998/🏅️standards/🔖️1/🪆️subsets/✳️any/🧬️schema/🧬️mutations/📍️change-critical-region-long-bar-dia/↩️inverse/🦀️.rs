//! ↩️ `change-critical-region-long-bar-dia` inverse: restores the previous `longitudinal_bar_diameter_m`.

use crate::change_critical_region_long_bar_dia::ChangeCriticalRegionLongitudinalBarDiameterM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionLongitudinalBarDiameterM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_long_bar_dia::ChangeCriticalRegionLongitudinalBarDiameterM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionLongitudinalBarDiameterM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionLongitudinalBarDiameterM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionLongitudinalBarDiameterM { id: payload.id.clone(), new_longitudinal_bar_diameter_m: r0.longitudinal_bar_diameter_m })
}
