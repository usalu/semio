//! 🌡 `insert-thermal-section` diff: inserts the thermal section row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_thermal_section::InsertThermalSection;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ThermalSectionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertThermalSection, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.thermal_sections.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Thermal section {key} already exists."), [key]);
    }
    if payload.index > base.thermal_sections.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.thermal_sections.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { thermal_sections: En1991ThermalSectionDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
