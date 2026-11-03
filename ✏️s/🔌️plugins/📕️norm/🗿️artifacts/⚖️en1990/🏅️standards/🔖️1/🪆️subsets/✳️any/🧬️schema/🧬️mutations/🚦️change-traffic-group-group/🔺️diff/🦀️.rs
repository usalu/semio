//! 🚦 `change-traffic-group-group` diff: patches `group` of the addressed traffic group row.

use crate::change_traffic_group_group::ChangeTrafficGroupGroup;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TrafficGroupDelta, En1990TrafficGroupPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeTrafficGroupGroup, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.traffic_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Traffic group {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.group == payload.new_group {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The traffic group already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { traffic_groups: En1990TrafficGroupDelta::modification(&payload.id, En1990TrafficGroupPatch { group: Some(payload.new_group), ..Default::default() }), ..Default::default() })
}
