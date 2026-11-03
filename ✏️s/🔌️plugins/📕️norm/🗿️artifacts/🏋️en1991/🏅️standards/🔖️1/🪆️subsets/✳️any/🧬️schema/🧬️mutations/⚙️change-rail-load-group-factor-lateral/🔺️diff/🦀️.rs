//! ⚙ `change-rail-load-group-factor-lateral` diff: patches `factor_lateral` of the addressed rail load group row.

use crate::change_rail_load_group_factor_lateral::ChangeRailLoadGroupFactorLateral;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailLoadGroupDelta, En1991RailLoadGroupPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRailLoadGroupFactorLateral, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_factor_lateral.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.rail_load_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail load group {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.factor_lateral == payload.new_factor_lateral {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The rail load group already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_load_groups: En1991RailLoadGroupDelta::modification(&payload.id, En1991RailLoadGroupPatch { factor_lateral: Some(payload.new_factor_lateral), ..Default::default() }), ..Default::default() })
}
