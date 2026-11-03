//! ↩️ `change-thermal-section-deck-type` inverse: restores the previous `deck_type`.

use crate::change_thermal_section_deck_type::ChangeThermalSectionDeckType;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeThermalSectionDeckType, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_thermal_section_deck_type::ChangeThermalSectionDeckType> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeThermalSectionDeckType, base: &En1991Snapshot) -> Option<ChangeThermalSectionDeckType> {
    let r0 = base.thermal_sections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeThermalSectionDeckType { id: payload.id.clone(), new_deck_type: r0.deck_type })
}
