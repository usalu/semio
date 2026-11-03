//! 🔺️ `change-silo-liquid-density` diff: patches `liquid_density` of the addressed silo shell row.

use crate::change_silo_liquid_density::ChangeSiloLiquidDensity;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SiloShellDelta, En1993SiloShellPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSiloLiquidDensity, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_liquid_density.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.silo_shells.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "silo", Vec::<String>::new());
    };
    if row.liquid_density == payload.new_liquid_density {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo shell already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: En1993SiloShellDelta::modification(payload.id.clone(), En1993SiloShellPatch { liquid_density: Some(payload.new_liquid_density), ..Default::default() }), ..Default::default() })
}
