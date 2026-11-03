use crate::insert_truss::InsertTruss;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995TrussDelta;
pub fn diff(payload: &InsertTruss, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.trusses.iter().any(|existing| existing.id == payload.truss.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Truss id {} already exists.", payload.truss.id), [payload.truss.id.clone()]);
    }
    let at = payload.index.min(base.trusses.len());
    protocol::MutationOutcome::new(En1995Diff { trusses: En1995TrussDelta::insertion(at, payload.truss.clone()), ..Default::default() })
}
