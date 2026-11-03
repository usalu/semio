//! 🛤 `remove-rail-dynamic` diff: removes the addressed rail span row.

use crate::remove_rail_dynamic::RemoveRailDynamic;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDynamicDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveRailDynamic, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.rail_dynamics.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail span {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { rail_dynamics: En1991RailDynamicDelta::removal(&base.rail_dynamics, index), ..Default::default() })
}
