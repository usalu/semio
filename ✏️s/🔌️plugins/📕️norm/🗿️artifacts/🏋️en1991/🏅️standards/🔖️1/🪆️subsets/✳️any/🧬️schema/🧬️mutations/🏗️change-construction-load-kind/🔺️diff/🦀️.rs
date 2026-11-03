//! 🏗 `change-construction-load-kind` diff: patches `kind` of the addressed construction load row.

use crate::change_construction_load_kind::ChangeConstructionLoadKind;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionLoadDelta, En1991ConstructionLoadPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeConstructionLoadKind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.construction_loads.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction load {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The construction load already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_loads: En1991ConstructionLoadDelta::modification(&payload.id, En1991ConstructionLoadPatch { kind: Some(payload.new_kind), ..Default::default() }), ..Default::default() })
}
