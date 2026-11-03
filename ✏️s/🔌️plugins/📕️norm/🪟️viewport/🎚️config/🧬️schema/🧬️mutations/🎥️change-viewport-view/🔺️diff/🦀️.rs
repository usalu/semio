use crate::mutations::change_viewport_view::ChangeViewportView;
use crate::diff::{NormViewportWindowConfigDiff, ViewState};
use crate::schema::NormViewportWindowConfig;

pub fn diff(payload: &ChangeViewportView, base: &NormViewportWindowConfig) -> protocol::MutationOutcome<NormViewportWindowConfigDiff> {
    let view = ViewState { camera_preset: payload.camera_preset.clone(), hidden_layers: payload.hidden_layers.clone(), colour_map: payload.colour_map.clone(), unit_system: payload.unit_system.clone() };
    if view == ViewState::of(base) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The viewport view is already this value.");
    }
    protocol::MutationOutcome::new(NormViewportWindowConfigDiff { view: Some(view) })
}
