//! ↩️ `change-critical-region-hoop-diameter-m` inverse: restores the previous `hoop_diameter_m`.

use crate::change_critical_region_hoop_diameter_m::ChangeCriticalRegionHoopDiameterM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionHoopDiameterM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_hoop_diameter_m::ChangeCriticalRegionHoopDiameterM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionHoopDiameterM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionHoopDiameterM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionHoopDiameterM { id: payload.id.clone(), new_hoop_diameter_m: r0.hoop_diameter_m })
}
