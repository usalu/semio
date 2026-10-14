use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use crate::change_member_fire_rating::ChangeMemberFireRating;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberFireRating, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(f) = m.fire.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "Member has no fire spec.", Vec::<String>::new());
    };
    if f.rating == payload.new_rating {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { fire_rating: Some(payload.new_rating), ..Default::default() })),
        ..Default::default()
    })
}
