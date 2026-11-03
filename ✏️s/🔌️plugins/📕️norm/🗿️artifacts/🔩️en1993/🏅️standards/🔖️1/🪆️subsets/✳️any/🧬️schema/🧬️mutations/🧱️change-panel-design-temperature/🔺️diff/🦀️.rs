//! 🔺️ `change-panel-design-temperature` diff: patches `design_temperature` of the addressed panel panel row.

use crate::change_panel_design_temperature::ChangePanelDesignTemperature;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PanelPanelDelta, En1993PanelPanelPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePanelDesignTemperature, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_design_temperature.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.plated_panels.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "panel", Vec::<String>::new());
    };
    if row.design_temperature == payload.new_design_temperature {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The panel panel already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PanelPanelDelta::modification(payload.id.clone(), En1993PanelPanelPatch { design_temperature: Some(payload.new_design_temperature), ..Default::default() }), ..Default::default() })
}
