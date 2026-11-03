//! ⚙ `change-rail-load-group-assumed-longitudinal` diff: patches `assumed_longitudinal` of the addressed rail load group row.

use crate::change_rail_load_group_assumed_longitudinal::ChangeRailLoadGroupAssumedLongitudinal;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailLoadGroupDelta, En1991RailLoadGroupPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRailLoadGroupAssumedLongitudinal, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_longitudinal.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.rail_load_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail load group {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_longitudinal == payload.new_assumed_longitudinal {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The rail load group already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_load_groups: En1991RailLoadGroupDelta::modification(&payload.id, En1991RailLoadGroupPatch { assumed_longitudinal: Some(payload.new_assumed_longitudinal), ..Default::default() }), ..Default::default() })
}
