//! 🏛️ `change-member-label-de` diff: patches `label_de` of the addressed member row.

use crate::change_member_label_de::ChangeMemberLabelDe;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberLabelDe, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    if r0.label_de == payload.new_label_de {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { label_de: Some(payload.new_label_de.clone()), ..Default::default() })), ..Default::default() })
}
