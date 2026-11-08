//! ↩️ `change-critical-region-section-width-m` inverse: restores the previous `section_width_m`.

use crate::change_critical_region_section_width_m::ChangeCriticalRegionSectionWidthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeCriticalRegionSectionWidthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_critical_region_section_width_m::ChangeCriticalRegionSectionWidthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCriticalRegionSectionWidthM, base: &En1998Snapshot) -> Option<ChangeCriticalRegionSectionWidthM> {
    let r0 = base.critical_regions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCriticalRegionSectionWidthM { id: payload.id.clone(), new_section_width_m: r0.section_width_m })
}
