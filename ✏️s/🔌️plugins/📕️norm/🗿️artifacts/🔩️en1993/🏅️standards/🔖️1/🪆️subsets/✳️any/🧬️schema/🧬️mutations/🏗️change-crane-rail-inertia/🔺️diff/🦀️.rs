//! 🔺️ `change-crane-rail-inertia` diff: patches `rail_inertia` of the addressed crane runway row.

use crate::change_crane_rail_inertia::ChangeCraneRailInertia;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993CraneRunwayDelta, En1993CraneRunwayPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeCraneRailInertia, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_rail_inertia.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.crane_runways.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "crane", Vec::<String>::new());
    };
    if row.rail_inertia == payload.new_rail_inertia {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane runway already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::modification(payload.id.clone(), En1993CraneRunwayPatch { rail_inertia: Some(payload.new_rail_inertia), ..Default::default() }), ..Default::default() })
}
