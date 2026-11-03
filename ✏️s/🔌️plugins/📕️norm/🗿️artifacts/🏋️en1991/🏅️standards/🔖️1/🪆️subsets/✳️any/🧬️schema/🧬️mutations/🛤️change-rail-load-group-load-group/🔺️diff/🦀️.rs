//! 🛤 `change-rail-load-group-load-group` diff: patches `load_group` of the addressed fatigue load_group row.

use crate::change_rail_load_group_load_group::ChangeRailLoadGroupLoadGroup;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailLoadGroupDelta, En1991RailLoadGroupPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRailLoadGroupLoadGroup, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.rail_load_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue load_group {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.load_group == payload.new_load_group {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue load_group already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_load_groups: En1991RailLoadGroupDelta::modification(&payload.id, En1991RailLoadGroupPatch { load_group: Some(payload.new_load_group.clone()), ..Default::default() }), ..Default::default() })
}
