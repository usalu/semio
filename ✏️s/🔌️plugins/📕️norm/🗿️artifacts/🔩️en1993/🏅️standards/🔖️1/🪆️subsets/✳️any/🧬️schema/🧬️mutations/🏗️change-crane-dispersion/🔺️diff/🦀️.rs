//! 🔺️ `change-crane-dispersion` diff: patches `dispersion` of the addressed crane runway row.

use crate::change_crane_dispersion::ChangeCraneDispersion;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993CraneRunwayDelta, En1993CraneRunwayPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeCraneDispersion, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_dispersion.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.crane_runways.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "crane", Vec::<String>::new());
    };
    if row.dispersion == payload.new_dispersion {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane runway already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::modification(payload.id.clone(), En1993CraneRunwayPatch { dispersion: Some(payload.new_dispersion), ..Default::default() }), ..Default::default() })
}
