//! 🚦 `insert-traffic-group` diff: inserts the traffic group row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_traffic_group::InsertTrafficGroup;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TrafficGroupDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &InsertTrafficGroup, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.traffic_groups.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Traffic group {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.traffic_groups.len());
    let outcome = protocol::MutationOutcome::new(En1990Diff { traffic_groups: En1990TrafficGroupDelta::insertion(index, payload.item.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the traffic group list; inserted at {index}.", payload.index.unwrap_or(index)))
}
