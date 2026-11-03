use crate::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995PlacementDelta, En1995PlacementPatch};
pub fn diff(payload: &ChangePlacementX, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(placement) = base.placements.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown placement id.", vec![payload.id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { placements: En1995PlacementDelta::modification(&placement.id, En1995PlacementPatch { x: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
