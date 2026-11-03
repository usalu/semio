//! 🌡 `change-thermal-section-surfacing` diff: patches `surfacing` of the addressed thermal section row.

use crate::change_thermal_section_surfacing::ChangeThermalSectionSurfacing;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ThermalSectionDelta, En1991ThermalSectionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeThermalSectionSurfacing, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.thermal_sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal section {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.surfacing == payload.new_surfacing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The thermal section already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_sections: En1991ThermalSectionDelta::modification(&payload.id, En1991ThermalSectionPatch { surfacing: Some(payload.new_surfacing), ..Default::default() }), ..Default::default() })
}
