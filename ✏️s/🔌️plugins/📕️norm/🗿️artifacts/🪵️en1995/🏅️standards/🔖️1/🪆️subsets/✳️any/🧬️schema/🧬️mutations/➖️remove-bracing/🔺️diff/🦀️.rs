use crate::remove_bracing::RemoveBracing;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995BracingDelta;
pub fn diff(payload: &RemoveBracing, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.bracings.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bracing index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { bracings: En1995BracingDelta::removal(&base.bracings, payload.index), ..Default::default() })
}
