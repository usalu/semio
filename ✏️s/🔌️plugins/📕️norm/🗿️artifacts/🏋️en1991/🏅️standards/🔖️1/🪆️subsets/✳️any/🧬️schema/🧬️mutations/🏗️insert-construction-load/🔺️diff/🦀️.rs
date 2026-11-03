//! 🏗 `insert-construction-load` diff: inserts the construction load row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_construction_load::InsertConstructionLoad;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionLoadDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertConstructionLoad, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.construction_loads.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Construction load {key} already exists."), [key]);
    }
    if payload.index > base.construction_loads.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.construction_loads.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { construction_loads: En1991ConstructionLoadDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
