//! 🔺️ `change-crane-flange-eff-width` diff: patches `flange_effective_width` of the addressed crane runway row.

use crate::change_crane_flange_eff_width::ChangeCraneFlangeEffWidth;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993CraneRunwayDelta, En1993CraneRunwayPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeCraneFlangeEffWidth, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_flange_effective_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.crane_runways.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "crane", Vec::<String>::new());
    };
    if row.flange_effective_width == payload.new_flange_effective_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane runway already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::modification(payload.id.clone(), En1993CraneRunwayPatch { flange_effective_width: Some(payload.new_flange_effective_width), ..Default::default() }), ..Default::default() })
}
