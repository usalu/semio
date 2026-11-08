//! ↩️ `change-critical-region-hoop-spacing-m` inverse: restores the previous `hoop_spacing_m`.

use crate::change_critical_region_hoop_spacing_m::ChangeCriticalRegionHoopSpacingM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionHoopSpacingM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_hoop_spacing_m::ChangeCriticalRegionHoopSpacingM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionHoopSpacingM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionHoopSpacingM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionHoopSpacingM { id: payload.id.clone(), new_hoop_spacing_m: r0.hoop_spacing_m })
}
