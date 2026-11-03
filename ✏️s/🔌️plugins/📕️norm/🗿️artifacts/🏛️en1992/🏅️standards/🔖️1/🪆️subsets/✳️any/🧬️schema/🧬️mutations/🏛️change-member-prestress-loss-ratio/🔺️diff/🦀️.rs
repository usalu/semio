//! 🏛️ `change-member-prestress-loss-ratio` diff: patches `prestress.loss_ratio` of the addressed member row.

use crate::change_member_prestress_loss_ratio::ChangeMemberPrestressLossRatio;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberPrestressLossRatio, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_prestress_loss_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(object) = r0.prestress.as_ref() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The optional prestress is absent.", Vec::<String>::new());
    };
    if object.loss_ratio == payload.new_prestress_loss_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { prestress_loss_ratio: Some(payload.new_prestress_loss_ratio), ..Default::default() })), ..Default::default() })
}
