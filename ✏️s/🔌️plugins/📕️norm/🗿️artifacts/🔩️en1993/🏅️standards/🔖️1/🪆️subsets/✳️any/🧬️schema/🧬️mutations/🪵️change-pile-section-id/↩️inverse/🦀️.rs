//! ↩️ `change-pile-section-id` inverse: restores the previous `section_id`.

use crate::change_pile_section_id::ChangePileSectionId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePileSectionId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_section_id::ChangePileSectionId> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileSectionId { id: payload.id.clone(), new_section_id: row.section_id.clone() })],
        None => Vec::new(),
    })
}
