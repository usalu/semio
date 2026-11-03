//! 🔺️ `change-member-section-id` diff: patches `section_id` of the addressed member row.

use crate::change_member_section_id::ChangeMemberSectionId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersPatch, En1999MembersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeMemberSectionId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if r0.section_id == payload.new_section_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(&payload.member_id, En1999MembersPatch { section_id: Some(payload.new_section_id.clone()), ..Default::default() })), ..Default::default() })
}
