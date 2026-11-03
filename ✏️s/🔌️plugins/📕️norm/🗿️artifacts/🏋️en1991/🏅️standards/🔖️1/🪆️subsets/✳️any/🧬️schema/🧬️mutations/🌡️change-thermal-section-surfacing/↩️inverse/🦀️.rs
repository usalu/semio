//! ↩️ `change-thermal-section-surfacing` inverse: restores the previous `surfacing`.

use crate::change_thermal_section_surfacing::ChangeThermalSectionSurfacing;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalSectionSurfacing, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_section_surfacing::ChangeThermalSectionSurfacing> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeThermalSectionSurfacing, base: &En1991Snapshot) -> Option<ChangeThermalSectionSurfacing> {
    let r0 = base.thermal_sections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeThermalSectionSurfacing { id: payload.id.clone(), new_surfacing: r0.surfacing })
}
