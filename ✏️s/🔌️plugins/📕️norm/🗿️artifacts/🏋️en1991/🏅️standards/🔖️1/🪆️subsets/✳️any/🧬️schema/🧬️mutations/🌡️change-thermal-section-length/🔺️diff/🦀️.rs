//! 🌡 `change-thermal-section-length` diff: patches `length` of the addressed thermal section row.

use crate::change_thermal_section_length::ChangeThermalSectionLength;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ThermalSectionDelta, En1991ThermalSectionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeThermalSectionLength, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.thermal_sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal section {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.length == payload.new_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The thermal section already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_sections: En1991ThermalSectionDelta::modification(&payload.id, En1991ThermalSectionPatch { length: Some(payload.new_length), ..Default::default() }), ..Default::default() })
}
