//! ⚙ `remove-rail-load-group` diff: removes the addressed rail load group row.

use crate::remove_rail_load_group::RemoveRailLoadGroup;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailLoadGroupDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveRailLoadGroup, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.rail_load_groups.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail load group {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { rail_load_groups: En1991RailLoadGroupDelta::removal(&base.rail_load_groups, index), ..Default::default() })
}
