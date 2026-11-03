//! 🏗️ `change-member-rd-equ-destab` diff: patches `rd_equ_destab` of the addressed member row.

use crate::change_member_rd_equ_destab::ChangeMemberRdEquDestab;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990MemberDelta, En1990MemberPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeMemberRdEquDestab, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_rd_equ_destab.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.rd_equ_destab == payload.new_rd_equ_destab {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { members: En1990MemberDelta::modification(&payload.id, En1990MemberPatch { rd_equ_destab: Some(payload.new_rd_equ_destab), ..Default::default() }), ..Default::default() })
}
