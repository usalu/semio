//! 🏗 `change-construction-load-magnitude` diff: patches `magnitude` of the addressed construction load row.

use crate::change_construction_load_magnitude::ChangeConstructionLoadMagnitude;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionLoadDelta, En1991ConstructionLoadPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeConstructionLoadMagnitude, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_magnitude.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.construction_loads.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction load {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.magnitude == payload.new_magnitude {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The construction load already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_loads: En1991ConstructionLoadDelta::modification(&payload.id, En1991ConstructionLoadPatch { magnitude: Some(payload.new_magnitude), ..Default::default() }), ..Default::default() })
}
