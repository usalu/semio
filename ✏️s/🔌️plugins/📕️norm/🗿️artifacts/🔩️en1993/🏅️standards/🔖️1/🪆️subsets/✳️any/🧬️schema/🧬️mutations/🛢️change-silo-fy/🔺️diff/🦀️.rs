//! 🔺️ `change-silo-fy` diff: patches `fy` of the addressed silo shell row.

use crate::change_silo_fy::ChangeSiloFy;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SiloShellDelta, En1993SiloShellPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSiloFy, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_fy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.silo_shells.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "silo", Vec::<String>::new());
    };
    if row.fy == payload.new_fy {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo shell already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: En1993SiloShellDelta::modification(payload.id.clone(), En1993SiloShellPatch { fy: Some(payload.new_fy), ..Default::default() }), ..Default::default() })
}
