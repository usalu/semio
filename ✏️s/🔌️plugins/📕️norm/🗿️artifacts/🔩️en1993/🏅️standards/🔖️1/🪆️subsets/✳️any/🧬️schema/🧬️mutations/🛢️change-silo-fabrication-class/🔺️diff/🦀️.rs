//! 🔺️ `change-silo-fabrication-class` diff: patches `fabrication_class` of the addressed silo shell row.

use crate::change_silo_fabrication_class::ChangeSiloFabricationClass;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SiloShellDelta, En1993SiloShellPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSiloFabricationClass, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.silo_shells.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "silo", Vec::<String>::new());
    };
    if row.fabrication_class == payload.new_fabrication_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The silo shell already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: En1993SiloShellDelta::modification(payload.id.clone(), En1993SiloShellPatch { fabrication_class: Some(payload.new_fabrication_class), ..Default::default() }), ..Default::default() })
}
