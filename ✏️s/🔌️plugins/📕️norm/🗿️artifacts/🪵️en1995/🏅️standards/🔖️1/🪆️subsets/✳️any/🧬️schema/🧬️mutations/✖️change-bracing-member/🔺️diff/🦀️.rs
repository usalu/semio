use crate::change_bracing_member::ChangeBracingMember;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995BracingDelta, En1995BracingPatch};
pub fn diff(payload: &ChangeBracingMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.bracings.iter().find(|item| item.id == payload.bracing_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown bracing id.", vec![payload.bracing_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { bracings: En1995BracingDelta::modification(&row.id, En1995BracingPatch { member_id: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
}
