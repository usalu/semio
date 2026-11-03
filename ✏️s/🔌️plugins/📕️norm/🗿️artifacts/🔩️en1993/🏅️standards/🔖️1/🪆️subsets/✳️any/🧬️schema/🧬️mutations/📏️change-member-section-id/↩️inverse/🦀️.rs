//! ↩️ `change-member-section-id` inverse: restores the previous `section_id`.

use crate::change_member_section_id::ChangeMemberSectionId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSectionId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_section_id::ChangeMemberSectionId> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberSectionId { id: payload.id.clone(), new_section_id: row.section_id.clone() })],
        None => Vec::new(),
    })
}
