//! 🗂️ Lowpoly play app — the always-visible mesh-domain granularity/selection-mode window-chrome group
//! (mirrors puzzle 3d's select measures group). Shared verbatim by both windows.
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches the framework-injected
//! `setInteractionGranularity`/`setSelectionMode` actions (mesh domain) instead of the deleted
//! `toggleSelectionKind`/`setSelectionMethod`/`setSelectionModeDefault`. `ArtifactApp::window_measures`
//! is not threaded an `InteractionView` this wave (only `handle`/`copy_fragment`/`cut_operations` are),
//! so every toggle's `pressed` is `false` here — a real known gap, not an oversight; the shell surfaces
//! the live granularity/mode generically off the same domain.

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::lowpoly_action;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::MESH_INTERACTION_DOMAIN;
use semio_framework_plugin::{LabelText, WindowMeasure};
use serde_json::json;

/// 🎯️ One mesh-domain granularity toggle — dispatches `setInteractionGranularity`.
fn granularity_toggle(id: &str, icon: &str, label: LabelText, granularity_id: &str) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: format!("lowpoly-select-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed: false,
        text: None,
        on_change: lowpoly_action("setInteractionGranularity", Some(json!({ "domainId": MESH_INTERACTION_DOMAIN, "granularityId": granularity_id }))),
    }
}

/// 🎯️ One mesh-domain selection-mode toggle — dispatches `setSelectionMode`.
fn selection_mode_toggle(id: &str, icon: &str, label: LabelText, mode: &str) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: format!("lowpoly-select-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed: false,
        text: None,
        on_change: lowpoly_action("setSelectionMode", Some(json!({ "domainId": MESH_INTERACTION_DOMAIN, "mode": mode }))),
    }
}

/// 🎛️ The live chrome measure for this option.
pub fn measure(_config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
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
            selection_mode_toggle("mode-single", "mouse-pointer", labels.selective, "single"),
            selection_mode_toggle("mode-multiple", "plus", labels.additive, "multiple"),
            granularity_toggle("mesh", "box", labels.mesh, "object"),
            granularity_toggle("vertex", "circle", labels.vertex, "vertex"),
            granularity_toggle("edge", "minus", labels.edge, "edge"),
            granularity_toggle("face", "square", labels.face, "face"),
        ],
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn select_window_options_expose_mesh_domain_granularity_and_mode_toggles() {
        let measure = measure(&LowpolyConfig::default(), semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>("en-US"));
        let (active_utility_id, children) = match measure {
            WindowMeasure::Group { active_utility_id, children, .. } => (active_utility_id, children),
            other => panic!("expected Group, got {other:?}"),
        };
        assert_eq!(active_utility_id, None, "Select options must always surface in window options");
        let toggles: Vec<(&str, &semio_framework_plugin::ActionDescriptor)> = children
            .iter()
            .filter_map(|measure| match measure {
                WindowMeasure::Toggle { id, on_change, .. } => Some((id.as_str(), on_change)),
                _ => None,
            })
            .collect();
        let ids: Vec<&str> = toggles.iter().map(|(id, _)| *id).collect();
        assert_eq!(ids, vec!["lowpoly-select-mode-single", "lowpoly-select-mode-multiple", "lowpoly-select-mesh", "lowpoly-select-vertex", "lowpoly-select-edge", "lowpoly-select-face"]);
        // 🕹️ Every toggle dispatches a framework-injected mesh-domain interaction verb, never a
        // deleted app command.
        for (id, action) in &toggles {
            assert!(action.action == "setInteractionGranularity" || action.action == "setSelectionMode", "{id} must dispatch a framework interaction verb, got {}", action.action);
        }
    }
}
//#endregion 🧪️Tests
