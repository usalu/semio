//! 🔺️ `change-member-toughness-stress-ratio` diff: patches `toughness_stress_ratio` of the addressed member row.

use crate::change_member_toughness_stress_ratio::ChangeMemberToughnessStressRatio;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta, En1993MemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMemberToughnessStressRatio, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_toughness_stress_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if row.toughness_stress_ratio == payload.new_toughness_stress_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::modification(payload.id.clone(), En1993MemberPatch { toughness_stress_ratio: Some(payload.new_toughness_stress_ratio), ..Default::default() }), ..Default::default() })
}
