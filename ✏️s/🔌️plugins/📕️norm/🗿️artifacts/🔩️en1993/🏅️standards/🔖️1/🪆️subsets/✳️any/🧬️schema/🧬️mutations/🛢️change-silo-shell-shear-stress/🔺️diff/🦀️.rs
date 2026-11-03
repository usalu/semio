//! 🔺️ `change-silo-shell-shear-stress` diff: patches `shell_shear_stress` of the addressed silo shell row.

use crate::change_silo_shell_shear_stress::ChangeSiloShellShearStress;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SiloShellDelta, En1993SiloShellPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSiloShellShearStress, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_shell_shear_stress.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.silo_shells.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "silo", Vec::<String>::new());
    };
    if row.shell_shear_stress == payload.new_shell_shear_stress {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo shell already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: En1993SiloShellDelta::modification(payload.id.clone(), En1993SiloShellPatch { shell_shear_stress: Some(payload.new_shell_shear_stress), ..Default::default() }), ..Default::default() })
}
