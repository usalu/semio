//! 🏗️ `change-stage-return-years` diff: patches `return_years` of the addressed construction stage row.

use crate::change_stage_return_years::ChangeStageReturnYears;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionStageDelta, En1991ConstructionStagePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeStageReturnYears, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_return_years.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.construction_stages.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("construction stage {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.return_years == payload.new_return_years {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The construction stage already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_stages: En1991ConstructionStageDelta::modification(&payload.id, En1991ConstructionStagePatch { return_years: Some(payload.new_return_years), ..Default::default() }), ..Default::default() })
}
