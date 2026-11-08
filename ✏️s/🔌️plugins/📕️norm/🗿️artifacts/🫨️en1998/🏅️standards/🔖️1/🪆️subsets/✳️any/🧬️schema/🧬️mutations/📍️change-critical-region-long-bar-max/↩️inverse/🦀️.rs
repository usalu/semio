//! ↩️ `change-critical-region-long-bar-max` inverse: restores the previous `longitudinal_bar_max_diameter_m`.

use crate::change_critical_region_long_bar_max::ChangeCriticalRegionLongitudinalBarMaxDiameterM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionLongitudinalBarMaxDiameterM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_long_bar_max::ChangeCriticalRegionLongitudinalBarMaxDiameterM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionLongitudinalBarMaxDiameterM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionLongitudinalBarMaxDiameterM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionLongitudinalBarMaxDiameterM { id: payload.id.clone(), new_longitudinal_bar_max_diameter_m: r0.longitudinal_bar_max_diameter_m })
}
