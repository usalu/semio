//! 🚁 `change-helicopter-height` diff: patches `height` of the addressed helicopter landing row.

use crate::change_helicopter_height::ChangeHelicopterHeight;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991HelicopterLandingDelta, En1991HelicopterLandingPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeHelicopterHeight, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_height.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.helicopter_landings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Helicopter landing {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.height == payload.new_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The helicopter landing already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { helicopter_landings: En1991HelicopterLandingDelta::modification(&payload.id, En1991HelicopterLandingPatch { height: Some(payload.new_height), ..Default::default() }), ..Default::default() })
}
