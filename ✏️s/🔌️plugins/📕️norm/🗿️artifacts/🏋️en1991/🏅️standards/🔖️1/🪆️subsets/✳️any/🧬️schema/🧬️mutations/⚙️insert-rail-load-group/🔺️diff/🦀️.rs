//! ⚙ `insert-rail-load-group` diff: inserts the rail load group row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_rail_load_group::InsertRailLoadGroup;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailLoadGroupDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertRailLoadGroup, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.rail_load_groups.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Rail load group {key} already exists."), [key]);
    }
    if payload.index > base.rail_load_groups.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.rail_load_groups.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { rail_load_groups: En1991RailLoadGroupDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
