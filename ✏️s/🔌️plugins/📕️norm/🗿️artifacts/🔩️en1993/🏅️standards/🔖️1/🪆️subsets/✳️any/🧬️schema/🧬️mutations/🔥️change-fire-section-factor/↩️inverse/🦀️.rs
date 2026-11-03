//! ↩️ `change-fire-section-factor` inverse: restores the previous `section_factor`.

use crate::change_fire_section_factor::ChangeFireSectionFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireSectionFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_section_factor::ChangeFireSectionFactor> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireSectionFactor { id: payload.id.clone(), new_section_factor: row.section_factor })],
        None => Vec::new(),
    })
}
