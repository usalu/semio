//! 🛤 `change-rail-dynamic-maintenance` diff: patches `maintenance` of the addressed rail span row.

use crate::change_rail_dynamic_maintenance::ChangeRailDynamicMaintenance;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RailDynamicDelta, En1991RailDynamicPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRailDynamicMaintenance, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.rail_dynamics.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rail span {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.maintenance == payload.new_maintenance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The rail span already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rail_dynamics: En1991RailDynamicDelta::modification(&payload.id, En1991RailDynamicPatch { maintenance: Some(payload.new_maintenance.clone()), ..Default::default() }), ..Default::default() })
}
