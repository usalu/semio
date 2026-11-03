use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995PlacementDelta;
pub fn diff(payload: &InsertPlacement, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.placement.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement id {} already exists.", payload.placement.id), [payload.placement.id.clone()]);
    }
    let at = payload.index.min(base.placements.len());
    protocol::MutationOutcome::new(En1995Diff { placements: En1995PlacementDelta::insertion(at, payload.placement.clone()), ..Default::default() })
}
