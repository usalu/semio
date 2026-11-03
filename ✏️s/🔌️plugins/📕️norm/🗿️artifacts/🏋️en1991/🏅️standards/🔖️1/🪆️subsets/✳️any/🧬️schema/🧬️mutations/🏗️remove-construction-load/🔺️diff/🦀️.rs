//! 🏗 `remove-construction-load` diff: removes the addressed construction load row.

use crate::remove_construction_load::RemoveConstructionLoad;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionLoadDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveConstructionLoad, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.construction_loads.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction load {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { construction_loads: En1991ConstructionLoadDelta::removal(&base.construction_loads, index), ..Default::default() })
}
