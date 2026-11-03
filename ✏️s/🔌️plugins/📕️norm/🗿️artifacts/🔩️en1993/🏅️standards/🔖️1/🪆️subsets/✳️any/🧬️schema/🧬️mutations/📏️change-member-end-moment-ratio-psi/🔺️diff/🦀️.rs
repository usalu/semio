//! 🔺️ `change-member-end-moment-ratio-psi` diff: patches `end_moment_ratio_psi` of the addressed member row.

use crate::change_member_end_moment_ratio_psi::ChangeMemberEndMomentRatioPsi;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta, En1993MemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMemberEndMomentRatioPsi, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_end_moment_ratio_psi.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if row.end_moment_ratio_psi == payload.new_end_moment_ratio_psi {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::modification(payload.id.clone(), En1993MemberPatch { end_moment_ratio_psi: Some(payload.new_end_moment_ratio_psi), ..Default::default() }), ..Default::default() })
}
