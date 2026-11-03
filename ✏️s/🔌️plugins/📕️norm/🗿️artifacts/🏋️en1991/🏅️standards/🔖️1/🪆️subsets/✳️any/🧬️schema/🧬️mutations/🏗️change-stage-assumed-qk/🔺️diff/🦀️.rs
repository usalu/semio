//! 🏗️ `change-stage-assumed-qk` diff: patches `assumed_qk` of the addressed construction stage row.

use crate::change_stage_assumed_qk::ChangeStageAssumedQk;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionStageDelta, En1991ConstructionStagePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeStageAssumedQk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_qk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.construction_stages.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("construction stage {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_qk == payload.new_assumed_qk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The construction stage already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_stages: En1991ConstructionStageDelta::modification(&payload.id, En1991ConstructionStagePatch { assumed_qk: Some(payload.new_assumed_qk), ..Default::default() }), ..Default::default() })
}
