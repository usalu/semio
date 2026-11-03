//! ⚙ `change-local-fire-horizontal-distance` diff: patches `horizontal_distance` of the addressed localised fire row.

use crate::change_local_fire_horizontal_distance::ChangeLocalFireHorizontalDistance;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LocalFireDelta, En1991LocalFirePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLocalFireHorizontalDistance, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_horizontal_distance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.local_fires.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Localised fire {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.horizontal_distance == payload.new_horizontal_distance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The localised fire already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { local_fires: En1991LocalFireDelta::modification(&payload.id, En1991LocalFirePatch { horizontal_distance: Some(payload.new_horizontal_distance), ..Default::default() }), ..Default::default() })
}
