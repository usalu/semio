//! ↩️ `change-thermal-section-length` inverse: restores the previous `length`.

use crate::change_thermal_section_length::ChangeThermalSectionLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalSectionLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_section_length::ChangeThermalSectionLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeThermalSectionLength, base: &En1991Snapshot) -> Option<ChangeThermalSectionLength> {
    let r0 = base.thermal_sections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeThermalSectionLength { id: payload.id.clone(), new_length: r0.length })
}
