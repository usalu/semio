//! 🗂️ Lowpoly play app — the always-visible mesh-domain granularity/selection-mode window-chrome group
//! (mirrors puzzle 3d's select measures group). Shared verbatim by both windows.
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches the framework-injected
//! `setInteractionGranularity`/`setSelectionMode` actions (mesh domain) instead of the deleted
//! `toggleSelectionKind`/`setSelectionMethod`/`setSelectionModeDefault`. Since 2026-09-18 the app's
//! `window_measures_with_request_context` threads the live `InteractionView`, so every toggle's `pressed`
//! IS the mesh domain's current granularity / selection mode (`SelectState`).

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::lowpoly_window_action;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{LabelText, WindowMeasure};

/// 🎯️ What the mesh domain currently selects with: the armed granularity and the selection mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectState {
    pub granularity: String,
    pub mode: String,
}

impl Default for SelectState {
    /// 🎯️ What the framework resolves before any dispatch: the domain's first declared granularity and
    /// its first declared mode (`modes: [Multiple, Single]` in the app manifest, so `multiple`).
    fn default() -> Self {
        Self { granularity: MESH_GRANULARITY_OBJECT.into(), mode: "multiple".into() }
    }
}

impl SelectState {
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let granularity = interaction.active_granularity(MESH_INTERACTION_DOMAIN).filter(|granularity| !granularity.is_empty()).unwrap_or(MESH_GRANULARITY_OBJECT).to_string();
        let mode = match interaction.active_mode(MESH_INTERACTION_DOMAIN) {
            Some(protocol::SelectionMode::Single) => "single",
            _ => "multiple",
        }
        .to_string();
        Self { granularity, mode }
    }
}

/// 🎯️ One mesh-domain granularity toggle — dispatches `setInteractionGranularity`.
fn granularity_toggle(id: &str, icon: &str, label: LabelText, granularity_id: &str, pressed: bool) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: format!("lowpoly-select-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: lowpoly_window_action(
            "setInteractionGranularity",
            Some((&dsl::DslValue::object([("domainId".to_string(), dsl::DslValue::String(MESH_INTERACTION_DOMAIN.to_string())), ("granularityId".to_string(), dsl::DslValue::String(granularity_id.to_string()))])).into()),
        ),
    }
}

/// 🎯️ One mesh-domain selection-mode toggle — dispatches `setSelectionMode`.
fn selection_mode_toggle(id: &str, icon: &str, label: LabelText, mode: &str, pressed: bool) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: format!("lowpoly-select-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: lowpoly_window_action(
            "setSelectionMode",
            Some((&dsl::DslValue::object([("domainId".to_string(), dsl::DslValue::String(MESH_INTERACTION_DOMAIN.to_string())), ("mode".to_string(), dsl::DslValue::String(mode.to_string()))])).into()),
        ),
    }
}

/// 🎛️ The live chrome measure for this option.
pub fn measure(_config: &LowpolyConfig, labels: &LowpolyLabels, state: &SelectState) -> WindowMeasure {
    WindowMeasure::Group {
        id: "lowpoly-select".into(),
        label: labels.select.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            selection_mode_toggle("mode-single", "mouse-pointer", labels.selective, "single", state.mode == "single"),
            selection_mode_toggle("mode-multiple", "plus", labels.additive, "multiple", state.mode == "multiple"),
            granularity_toggle("mesh", "box", labels.mesh, MESH_GRANULARITY_OBJECT, state.granularity == MESH_GRANULARITY_OBJECT),
            granularity_toggle("vertex", "circle", labels.vertex, "vertex", state.granularity == "vertex"),
            granularity_toggle("edge", "minus", labels.edge, "edge", state.granularity == "edge"),
            granularity_toggle("face", "square", labels.face, "face", state.granularity == "face"),
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
