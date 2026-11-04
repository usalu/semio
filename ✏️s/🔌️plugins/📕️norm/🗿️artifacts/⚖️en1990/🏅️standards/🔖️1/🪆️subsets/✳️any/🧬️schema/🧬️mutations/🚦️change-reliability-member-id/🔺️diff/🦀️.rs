//! 🚦 `change-reliability-member-id` diff: patches `member_id` of the addressed reliability analysis row.

use crate::change_reliability_member_id::ChangeReliabilityMemberId;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990ReliabilityAnalysisDelta, En1990ReliabilityAnalysisPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeReliabilityMemberId, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.reliability_analyses.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reliability analysis {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The reliability analysis already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { reliability_analyses: En1990ReliabilityAnalysisDelta::modification(&payload.id, En1990ReliabilityAnalysisPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
