//! 🏗 `change-construction-load-stage` diff: patches `stage` of the addressed construction load row.

use crate::change_construction_load_stage::ChangeConstructionLoadStage;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionLoadDelta, En1991ConstructionLoadPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeConstructionLoadStage, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.construction_loads.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction load {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.stage == payload.new_stage {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The construction load already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_loads: En1991ConstructionLoadDelta::modification(&payload.id, En1991ConstructionLoadPatch { stage: Some(payload.new_stage.clone()), ..Default::default() }), ..Default::default() })
}
