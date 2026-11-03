use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use crate::clear_member_tightness::ClearMemberTightness;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ClearMemberTightness, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    if m.tightness.is_none() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member has no tightness class.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { clear_tightness: Some(true), ..Default::default() })),
        ..Default::default()
    })
}
