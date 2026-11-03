//! ⚙ `insert-rail-derailment` diff: inserts the rail derailment row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_rail_derailment::InsertRailDerailment;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDerailmentDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertRailDerailment, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.rail_derailments.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Rail derailment {key} already exists."), [key]);
    }
    if payload.index > base.rail_derailments.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.rail_derailments.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { rail_derailments: En1991RailDerailmentDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
