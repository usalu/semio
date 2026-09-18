use super::*;

#[semio_framework_async_macros::async_test]
async fn select_window_options_expose_mesh_domain_granularity_and_mode_toggles() {
    let state = SelectState { granularity: "face".into(), mode: "multiple".into() };
    let measure = measure(&LowpolyConfig::default(), semio_framework_plugin::resolve_labels::<LowpolyLabels>(&semio_framework_plugin::ViewModel::default()), &state);
    let (active_utility_id, children) = match measure {
        WindowMeasure::Group { active_utility_id, children, .. } => (active_utility_id, children),
        other => panic!("expected Group, got {other:?}"),
    };
    assert_eq!(active_utility_id, None, "Select options must always surface in window options");
    let toggles: Vec<(&str, &semio_framework_plugin::ActionDescriptor, bool)> = children
        .iter()
        .filter_map(|measure| match measure {
            WindowMeasure::Toggle { id, on_change, pressed, .. } => Some((id.as_str(), on_change, *pressed)),
            _ => None,
        })
        .collect();
    let ids: Vec<&str> = toggles.iter().map(|(id, _, _)| *id).collect();
    // 🎯️ The pressed toggles ARE the live state: multiple mode and the face granularity here.
    let pressed: Vec<&str> = toggles.iter().filter(|(_, _, pressed)| *pressed).map(|(id, _, _)| *id).collect();
    assert_eq!(pressed, vec!["lowpoly-select-mode-multiple", "lowpoly-select-face"]);
    assert_eq!(ids, vec!["lowpoly-select-mode-single", "lowpoly-select-mode-multiple", "lowpoly-select-mesh", "lowpoly-select-vertex", "lowpoly-select-edge", "lowpoly-select-face"]);
    // 🕹️ Every toggle dispatches a framework-injected mesh-domain interaction verb, never a
    // deleted app command.
    for (id, action, _) in &toggles {
        assert!(action.action == "setInteractionGranularity" || action.action == "setSelectionMode", "{id} must dispatch a framework interaction verb, got {}", action.action);
    }
}
