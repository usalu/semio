//! ↩️ `change-thermal-section-depth` inverse: restores the previous `depth`.

use crate::change_thermal_section_depth::ChangeThermalSectionDepth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalSectionDepth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_section_depth::ChangeThermalSectionDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeThermalSectionDepth, base: &En1991Snapshot) -> Option<ChangeThermalSectionDepth> {
    let r0 = base.thermal_sections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeThermalSectionDepth { id: payload.id.clone(), new_depth: r0.depth })
}
