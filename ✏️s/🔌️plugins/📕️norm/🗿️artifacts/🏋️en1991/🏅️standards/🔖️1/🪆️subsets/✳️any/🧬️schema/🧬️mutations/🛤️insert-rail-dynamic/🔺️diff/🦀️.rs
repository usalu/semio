//! 🛤 `insert-rail-dynamic` diff: inserts the rail span row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_rail_dynamic::InsertRailDynamic;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDynamicDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertRailDynamic, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.rail_dynamics.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Rail span {key} already exists."), [key]);
    }
    if payload.index > base.rail_dynamics.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.rail_dynamics.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { rail_dynamics: En1991RailDynamicDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
