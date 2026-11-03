//! 🏛️ `change-member-stirrups-legs` diff: patches `stirrups.legs` of the addressed member row.

use crate::change_member_stirrups_legs::ChangeMemberStirrupsLegs;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberStirrupsLegs, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(object) = r0.stirrups.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The optional stirrups is absent.", Vec::<String>::new());
    };
    if object.legs == payload.new_stirrups_legs {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { stirrups_legs: Some(payload.new_stirrups_legs), ..Default::default() })), ..Default::default() })
}
