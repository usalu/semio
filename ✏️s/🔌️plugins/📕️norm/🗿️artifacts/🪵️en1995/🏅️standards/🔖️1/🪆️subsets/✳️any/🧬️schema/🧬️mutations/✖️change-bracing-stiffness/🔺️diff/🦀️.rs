use crate::change_bracing_stiffness::ChangeBracingStiffness;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995BracingDelta, En1995BracingPatch};
pub fn diff(payload: &ChangeBracingStiffness, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.bracings.iter().find(|item| item.id == payload.bracing_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown bracing id.", vec![payload.bracing_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { bracings: En1995BracingDelta::modification(&row.id, En1995BracingPatch { stiffness_n_per_m: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
