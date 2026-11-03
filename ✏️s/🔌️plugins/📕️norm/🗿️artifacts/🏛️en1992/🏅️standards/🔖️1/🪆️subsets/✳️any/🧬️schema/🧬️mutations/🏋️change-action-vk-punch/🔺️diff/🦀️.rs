//! 🏋️ `change-action-vk-punch` diff: patches `v_k_punch` of the addressed member action row.

use crate::change_action_v_k_punch::ChangeActionVKPunch;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersActionsPatch, En1992MembersActionsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeActionVKPunch, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_v_k_punch.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Action {} does not exist.", payload.action_id), [payload.action_id.clone()]);
    };
    if r1.v_k_punch == payload.new_v_k_punch {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member action already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { actions: Some(En1992MembersActionsRows::modification(&payload.action_id, En1992MembersActionsPatch { v_k_punch: Some(payload.new_v_k_punch), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
