use crate::insert_diaphragm::InsertDiaphragm;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995DiaphragmDelta;
pub fn diff(payload: &InsertDiaphragm, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.diaphragms.iter().any(|existing| existing.id == payload.diaphragm.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Diaphragm id {} already exists.", payload.diaphragm.id), [payload.diaphragm.id.clone()]);
    }
    let at = payload.index.min(base.diaphragms.len());
    protocol::MutationOutcome::new(En1995Diff { diaphragms: En1995DiaphragmDelta::insertion(at, payload.diaphragm.clone()), ..Default::default() })
}
