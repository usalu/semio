//! 🔺️ `change-action-torsion-k` diff: patches `torsion_k` of the addressed member action row.

use crate::change_action_torsion_k::ChangeActionTorsionK;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersActionsPatch, En1999MembersActionsRows, En1999MembersPatch, En1999MembersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeActionTorsionK, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_torsion_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.torsion_k == payload.new_torsion_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(&payload.member_id, En1999MembersPatch { actions: Some(En1999MembersActionsRows::modification(&payload.action_id, En1999MembersActionsPatch { torsion_k: Some(payload.new_torsion_k), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
