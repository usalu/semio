//! 🏛️ `change-member-punching-moment-c2` diff: patches `punching.moment_c2` of the addressed member row.

use crate::change_member_punching_moment_c2::ChangeMemberPunchingMomentC2;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberPunchingMomentC2, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_punching_moment_c2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(object) = r0.punching.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The optional punching is absent.", Vec::<String>::new());
    };
    if object.moment_c2 == payload.new_punching_moment_c2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { punching_moment_c2: Some(payload.new_punching_moment_c2), ..Default::default() })), ..Default::default() })
}
