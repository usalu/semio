
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
