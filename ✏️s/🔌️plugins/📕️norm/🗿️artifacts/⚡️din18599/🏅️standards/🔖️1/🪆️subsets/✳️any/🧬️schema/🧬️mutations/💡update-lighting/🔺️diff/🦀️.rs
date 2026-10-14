//! 🔺️ `update-lighting` sparse diff.

use crate::update_lighting::UpdateLighting;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599LightingPatch};

pub fn diff(payload: &UpdateLighting, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.lighting == payload.new_lighting {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "lighting already has this value.");
    }
    let (old, new) = (&base.lighting, &payload.new_lighting);
    protocol::MutationOutcome::new(Din18599Diff {
        lighting: Some(Din18599LightingPatch {
            control_factor: (old.control_factor != new.control_factor).then(|| new.control_factor.clone()),
        }),
        ..Default::default()
    })
}
