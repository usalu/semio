//! ↩️ `change-member-section-id` inverse: restores the previous `section_id`.

use crate::change_member_section_id::ChangeMemberSectionId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSectionId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_section_id::ChangeMemberSectionId> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberSectionId { member_id: payload.member_id.clone(), new_section_id: row.section_id.clone() })],
        None => Vec::new(),
    })
}
