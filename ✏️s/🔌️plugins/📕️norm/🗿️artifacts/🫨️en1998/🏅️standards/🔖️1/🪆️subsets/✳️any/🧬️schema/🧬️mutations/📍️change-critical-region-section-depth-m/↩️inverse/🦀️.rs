//! ↩️ `change-critical-region-section-depth-m` inverse: restores the previous `section_depth_m`.

use crate::change_critical_region_section_depth_m::ChangeCriticalRegionSectionDepthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionSectionDepthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_section_depth_m::ChangeCriticalRegionSectionDepthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionSectionDepthM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionSectionDepthM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionSectionDepthM { id: payload.id.clone(), new_section_depth_m: r0.section_depth_m })
}
