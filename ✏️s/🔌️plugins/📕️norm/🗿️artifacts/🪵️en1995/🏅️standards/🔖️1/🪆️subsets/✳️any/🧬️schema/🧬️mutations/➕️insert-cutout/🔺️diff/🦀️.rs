use crate::insert_cutout::InsertCutout;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995CutoutDelta;
pub fn diff(payload: &InsertCutout, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.cutouts.iter().any(|existing| existing.id == payload.cutout.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Cutout id {} already exists.", payload.cutout.id), [payload.cutout.id.clone()]);
    }
    let at = payload.index.min(base.cutouts.len());
    protocol::MutationOutcome::new(En1995Diff { cutouts: En1995CutoutDelta::insertion(at, payload.cutout.clone()), ..Default::default() })
}
