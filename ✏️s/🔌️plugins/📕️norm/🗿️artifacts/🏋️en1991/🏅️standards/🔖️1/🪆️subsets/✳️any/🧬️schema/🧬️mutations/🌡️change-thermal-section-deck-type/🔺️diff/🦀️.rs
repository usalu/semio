//! 🌡 `change-thermal-section-deck-type` diff: patches `deck_type` of the addressed thermal section row.

use crate::change_thermal_section_deck_type::ChangeThermalSectionDeckType;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ThermalSectionDelta, En1991ThermalSectionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeThermalSectionDeckType, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.thermal_sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal section {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.deck_type == payload.new_deck_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The thermal section already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_sections: En1991ThermalSectionDelta::modification(&payload.id, En1991ThermalSectionPatch { deck_type: Some(payload.new_deck_type), ..Default::default() }), ..Default::default() })
}
