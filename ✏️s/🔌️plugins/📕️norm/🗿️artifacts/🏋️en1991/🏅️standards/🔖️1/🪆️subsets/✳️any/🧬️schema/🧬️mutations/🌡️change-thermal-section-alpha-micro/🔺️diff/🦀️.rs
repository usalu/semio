//! 🌡 `change-thermal-section-alpha-micro` diff: patches `alpha_micro` of the addressed thermal section row.

use crate::change_thermal_section_alpha_micro::ChangeThermalSectionAlphaMicro;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ThermalSectionDelta, En1991ThermalSectionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeThermalSectionAlphaMicro, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_alpha_micro.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.thermal_sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal section {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.alpha_micro == payload.new_alpha_micro {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The thermal section already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_sections: En1991ThermalSectionDelta::modification(&payload.id, En1991ThermalSectionPatch { alpha_micro: Some(payload.new_alpha_micro), ..Default::default() }), ..Default::default() })
}
