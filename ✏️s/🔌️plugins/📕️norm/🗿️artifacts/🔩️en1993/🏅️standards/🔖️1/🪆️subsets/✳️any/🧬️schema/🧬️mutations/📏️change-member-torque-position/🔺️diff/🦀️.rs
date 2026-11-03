//! 🔺️ `change-member-torque-position` diff: patches `torque_position` of the addressed member row.

use crate::change_member_torque_position::ChangeMemberTorquePosition;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta, En1993MemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMemberTorquePosition, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_torque_position.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if row.torque_position == payload.new_torque_position {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::modification(payload.id.clone(), En1993MemberPatch { torque_position: Some(payload.new_torque_position), ..Default::default() }), ..Default::default() })
}
