//! 🚦 `remove-traffic-group` diff: removes the addressed traffic group row.

use crate::remove_traffic_group::RemoveTrafficGroup;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TrafficGroupDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &RemoveTrafficGroup, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let index = payload.index;
    if index >= base.traffic_groups.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "traffic_groups index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1990Diff { traffic_groups: En1990TrafficGroupDelta::removal(&base.traffic_groups, index), ..Default::default() })
}
