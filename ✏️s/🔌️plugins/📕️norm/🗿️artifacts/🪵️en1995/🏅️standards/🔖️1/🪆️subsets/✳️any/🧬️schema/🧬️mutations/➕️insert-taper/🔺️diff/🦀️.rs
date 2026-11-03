use crate::insert_taper::InsertTaper;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995TaperDelta;
pub fn diff(payload: &InsertTaper, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.tapers.iter().any(|existing| existing.id == payload.taper.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Taper id {} already exists.", payload.taper.id), [payload.taper.id.clone()]);
    }
    let at = payload.index.min(base.tapers.len());
    protocol::MutationOutcome::new(En1995Diff { tapers: En1995TaperDelta::insertion(at, payload.taper.clone()), ..Default::default() })
}
