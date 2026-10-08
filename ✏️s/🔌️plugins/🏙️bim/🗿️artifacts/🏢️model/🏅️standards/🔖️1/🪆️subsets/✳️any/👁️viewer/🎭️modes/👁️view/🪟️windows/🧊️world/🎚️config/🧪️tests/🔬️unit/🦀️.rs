use super::*;
use crate::render::window_config::assert_window_config_laws;

fn arguments(pairs: &[(&str, semio_framework_value::DslValue)]) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::DslValue::object(pairs.iter().map(|(name, value)| (name.to_string(), value.clone()))))
}

fn text(value: &str) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::String(value.to_string())
}

#[test]
fn a_fresh_world_window_is_unframed_with_every_storey_visible() {
    let config = BimViewerWorldWindowConfig::default();
    assert!(!config.framed);
    assert!(config.shows("any-storey"));
    assert_eq!(config.projection, WorldProjectionConfig::default());
}

#[test]
fn the_first_orbit_frames_the_window() {
    let orbit = store::Viewport3dOrbit { position: [3.0, -4.0, 5.0], target: [1.0, 1.0, 0.0], zoom: 2.0, up: None };
    let next = BimViewerWorldWindowConfig::default().with_orbit(orbit);
    assert!(next.framed);
    assert_eq!(next.orbit, orbit);
}

#[test]
fn storey_visibility_keeps_a_sorted_list_without_duplicates() {
    let hidden = BimViewerWorldWindowConfig::default().with_storey_visible("b", false).with_storey_visible("a", false).with_storey_visible("b", false);
    assert_eq!(hidden.hidden_storeys, vec!["a".to_string(), "b".to_string()]);
    assert!(!hidden.shows("a"));
    let shown = hidden.with_storey_visible("a", true);
    assert_eq!(shown.hidden_storeys, vec!["b".to_string()]);
    assert!(shown.shows("a"));
}

#[test]
fn a_projection_view_change_poses_the_orbit_around_its_target_at_the_same_distance() {
    let base = BimViewerWorldWindowConfig { orbit: store::Viewport3dOrbit { position: [0.0, -10.0, 0.0], target: [0.0; 3], zoom: 1.0, up: None }, ..BimViewerWorldWindowConfig::default() };
    let next = base.with_projection_action("setProjection", &arguments(&[("field", text("orthographicView")), ("value", text("top"))])).expect("a known projection field");
    assert_eq!(next.projection.kind, "orthographic");
    assert_eq!(next.projection.orthographic_view, "top");
    assert_eq!(next.orbit.position, [0.0, 0.0, 10.0]);
    assert_eq!(next.orbit.up, Some([0.0, 1.0, 0.0]));
    assert!(next.framed);
}

#[test]
fn a_projection_parameter_keeps_the_pose() {
    let base = BimViewerWorldWindowConfig::default();
    let next = base.with_projection_action("setProjectionParam", &arguments(&[("param", text("fov")), ("value", semio_framework_value::DslValue::float(70.0))])).expect("a known parameter");
    assert_eq!(next.projection.fov, 70.0);
    assert_eq!(next.orbit, base.orbit);
}

#[test]
fn an_unknown_projection_field_or_parameter_is_refused() {
    let base = BimViewerWorldWindowConfig::default();
    assert!(base.with_projection_action("setProjection", &arguments(&[("field", text("nope")), ("value", text("x"))])).is_none());
    assert!(base.with_projection_action("setProjectionParam", &arguments(&[("param", text("nope")), ("value", semio_framework_value::DslValue::float(1.0))])).is_none());
}

#[semio_framework_async_macros::async_test]
async fn the_world_configuration_obeys_the_window_configuration_laws() {
    let base = BimViewerWorldWindowConfig::default();
    let next = base.with_orbit(store::Viewport3dOrbit { position: [9.0, -9.0, 6.0], target: [2.0, 2.0, 1.0], zoom: 1.5, up: Some([0.0, 0.0, 1.0]) }).with_storey_visible("level-1", false);
    assert_window_config_laws(&base, &BimViewerWorldWindowConfigMutation::Replace { config: next }).await;
}
