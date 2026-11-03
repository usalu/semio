//! ⚙ `change-fatigue-model-assumed-axle-load` diff: patches `assumed_axle_load` of the addressed fatigue model row.

use crate::change_fatigue_model_assumed_axle_load::ChangeFatigueModelAssumedAxleLoad;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FatigueModelDelta, En1991FatigueModelPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFatigueModelAssumedAxleLoad, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_axle_load.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fatigue_models.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue model {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_axle_load == payload.new_assumed_axle_load {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue model already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { fatigue_models: En1991FatigueModelDelta::modification(&payload.id, En1991FatigueModelPatch { assumed_axle_load: Some(payload.new_assumed_axle_load), ..Default::default() }), ..Default::default() })
}
