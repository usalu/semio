//! ↩️ `change-thermal-section-alpha-micro` inverse: restores the previous `alpha_micro`.

use crate::change_thermal_section_alpha_micro::ChangeThermalSectionAlphaMicro;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalSectionAlphaMicro, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_section_alpha_micro::ChangeThermalSectionAlphaMicro> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeThermalSectionAlphaMicro, base: &En1991Snapshot) -> Option<ChangeThermalSectionAlphaMicro> {
    let r0 = base.thermal_sections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeThermalSectionAlphaMicro { id: payload.id.clone(), new_alpha_micro: r0.alpha_micro })
}
