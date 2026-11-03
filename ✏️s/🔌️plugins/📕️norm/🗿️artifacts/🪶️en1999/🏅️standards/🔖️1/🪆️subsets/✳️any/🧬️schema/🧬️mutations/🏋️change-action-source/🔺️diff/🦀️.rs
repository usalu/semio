//! 🔺️ `change-action-source` diff: patches `source` of the addressed member action row.

use crate::change_action_source::ChangeActionSource;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersActionsPatch, En1999MembersActionsRows, En1999MembersPatch, En1999MembersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeActionSource, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.source == payload.new_source {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(&payload.member_id, En1999MembersPatch { actions: Some(En1999MembersActionsRows::modification(&payload.action_id, En1999MembersActionsPatch { source: Some(payload.new_source), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
