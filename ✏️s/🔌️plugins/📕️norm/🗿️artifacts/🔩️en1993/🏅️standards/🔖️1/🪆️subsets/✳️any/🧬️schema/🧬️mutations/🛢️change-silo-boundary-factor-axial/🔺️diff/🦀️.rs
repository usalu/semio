//! 🔺️ `change-silo-boundary-factor-axial` diff: patches `boundary_factor_axial` of the addressed silo shell row.

use crate::change_silo_boundary_factor_axial::ChangeSiloBoundaryFactorAxial;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SiloShellDelta, En1993SiloShellPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSiloBoundaryFactorAxial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_boundary_factor_axial.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.silo_shells.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "silo", Vec::<String>::new());
    };
    if row.boundary_factor_axial == payload.new_boundary_factor_axial {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo shell already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: En1993SiloShellDelta::modification(payload.id.clone(), En1993SiloShellPatch { boundary_factor_axial: Some(payload.new_boundary_factor_axial), ..Default::default() }), ..Default::default() })
}
