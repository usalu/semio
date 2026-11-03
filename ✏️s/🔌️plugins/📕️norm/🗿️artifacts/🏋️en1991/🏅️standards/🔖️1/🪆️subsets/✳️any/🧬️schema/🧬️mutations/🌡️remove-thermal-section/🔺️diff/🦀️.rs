//! 🌡 `remove-thermal-section` diff: removes the addressed thermal section row.

use crate::remove_thermal_section::RemoveThermalSection;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ThermalSectionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveThermalSection, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.thermal_sections.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal section {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { thermal_sections: En1991ThermalSectionDelta::removal(&base.thermal_sections, index), ..Default::default() })
}
