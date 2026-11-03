//! 🔺️ `change-member-analysis` diff: patches `analysis` of the addressed member row.

use crate::change_member_analysis::ChangeMemberAnalysis;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta, En1993MemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMemberAnalysis, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if row.analysis == payload.new_analysis {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::modification(payload.id.clone(), En1993MemberPatch { analysis: Some(payload.new_analysis), ..Default::default() }), ..Default::default() })
}
