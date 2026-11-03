//! 🏗️ `change-stage-duration-days` diff: patches `duration_days` of the addressed construction stage row.

use crate::change_stage_duration_days::ChangeStageDurationDays;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionStageDelta, En1991ConstructionStagePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeStageDurationDays, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_duration_days.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.construction_stages.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("construction stage {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.duration_days == payload.new_duration_days {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The construction stage already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_stages: En1991ConstructionStageDelta::modification(&payload.id, En1991ConstructionStagePatch { duration_days: Some(payload.new_duration_days), ..Default::default() }), ..Default::default() })
}
