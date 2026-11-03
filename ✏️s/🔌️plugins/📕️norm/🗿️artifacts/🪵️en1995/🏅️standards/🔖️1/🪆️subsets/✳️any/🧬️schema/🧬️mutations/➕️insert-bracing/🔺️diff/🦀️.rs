use crate::insert_bracing::InsertBracing;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995BracingDelta;
pub fn diff(payload: &InsertBracing, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.bracings.iter().any(|existing| existing.id == payload.bracing.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bracing id {} already exists.", payload.bracing.id), [payload.bracing.id.clone()]);
    }
    let at = payload.index.min(base.bracings.len());
    protocol::MutationOutcome::new(En1995Diff { bracings: En1995BracingDelta::insertion(at, payload.bracing.clone()), ..Default::default() })
}
