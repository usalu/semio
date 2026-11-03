//! ↩️ `change-pile-section-modulus` inverse: restores the previous `section_modulus`.

use crate::change_pile_section_modulus::ChangePileSectionModulus;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileSectionModulus, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_section_modulus::ChangePileSectionModulus> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileSectionModulus { id: payload.id.clone(), new_section_modulus: row.section_modulus })],
        None => Vec::new(),
    })
}
