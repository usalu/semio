//! 🏗 `change-fatigue-model-lane` diff: patches `lane` of the addressed fatigue model row.

use crate::change_fatigue_model_lane::ChangeFatigueModelLane;
use semio_s_artifact_norm_en1991_diff::{En1991FatigueModelDelta, En1991FatigueModelPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFatigueModelLane, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.fatigue_models.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue model {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.lane == payload.new_lane {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue model already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { fatigue_models: En1991FatigueModelDelta::modification(&payload.id, En1991FatigueModelPatch { lane: Some(payload.new_lane), ..Default::default() }), ..Default::default() })
}
