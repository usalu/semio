//! 🏗️ `change-stage-activity` diff: patches `activity` of the addressed construction stage row.

use crate::change_stage_activity::ChangeStageActivity;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionStageDelta, En1991ConstructionStagePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeStageActivity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.construction_stages.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("construction stage {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.activity == payload.new_activity {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The construction stage already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_stages: En1991ConstructionStageDelta::modification(&payload.id, En1991ConstructionStagePatch { activity: Some(payload.new_activity.clone()), ..Default::default() }), ..Default::default() })
}
