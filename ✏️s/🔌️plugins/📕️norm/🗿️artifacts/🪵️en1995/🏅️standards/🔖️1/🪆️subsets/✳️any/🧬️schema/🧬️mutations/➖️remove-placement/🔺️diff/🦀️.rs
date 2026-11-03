use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995PlacementDelta;
pub fn diff(payload: &RemovePlacement, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.placements.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Placement index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { placements: En1995PlacementDelta::removal(&base.placements, payload.index), ..Default::default() })
}
