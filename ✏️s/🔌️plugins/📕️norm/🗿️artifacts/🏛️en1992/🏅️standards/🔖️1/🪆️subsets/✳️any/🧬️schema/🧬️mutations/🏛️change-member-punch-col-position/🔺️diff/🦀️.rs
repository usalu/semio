//! 🏛️ `change-member-punch-col-position` diff: patches `punching.column_position` of the addressed member row.

use crate::change_member_punch_col_position::ChangeMemberPunchColPosition;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberPunchColPosition, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(object) = r0.punching.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The optional punching is absent.", Vec::<String>::new());
    };
    if object.column_position == payload.new_punching_column_position {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { punching_column_position: Some(payload.new_punching_column_position), ..Default::default() })), ..Default::default() })
}
