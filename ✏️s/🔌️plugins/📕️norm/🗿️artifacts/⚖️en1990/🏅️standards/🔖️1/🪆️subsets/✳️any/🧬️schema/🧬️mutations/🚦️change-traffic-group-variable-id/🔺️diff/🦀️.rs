//! 🚦 `change-traffic-group-variable-id` diff: patches `variable_id` of the addressed traffic group row.

use crate::change_traffic_group_variable_id::ChangeTrafficGroupVariableId;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TrafficGroupDelta, En1990TrafficGroupPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeTrafficGroupVariableId, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.traffic_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Traffic group {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.variable_id == payload.new_variable_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The traffic group already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { traffic_groups: En1990TrafficGroupDelta::modification(&payload.id, En1990TrafficGroupPatch { variable_id: Some(payload.new_variable_id.clone()), ..Default::default() }), ..Default::default() })
}
