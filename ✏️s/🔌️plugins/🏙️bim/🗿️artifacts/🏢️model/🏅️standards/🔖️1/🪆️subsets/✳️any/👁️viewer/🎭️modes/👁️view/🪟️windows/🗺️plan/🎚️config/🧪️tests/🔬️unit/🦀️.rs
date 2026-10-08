use super::*;
use crate::render::window_config::assert_window_config_laws;

#[test]
fn a_fresh_plan_window_shows_the_lowest_storey_and_asks_for_a_fit() {
    let config = BimViewerPlanWindowConfig::default();
    assert_eq!(config.storey, "");
    assert!(!config.framed);
}

#[test]
fn the_first_viewport_frames_the_window() {
    let next = BimViewerPlanWindowConfig::default().with_viewport(store::Viewport2d { x: 3.0, y: -2.0, zoom: 40.0 });
    assert!(next.framed);
    assert_eq!(next.viewport, store::Viewport2d { x: 3.0, y: -2.0, zoom: 40.0 });
}

#[test]
fn another_storey_asks_for_a_fresh_fit() {
    let framed = BimViewerPlanWindowConfig::default().with_viewport(store::Viewport2d { x: 3.0, y: -2.0, zoom: 40.0 });
    let next = framed.with_storey("level-2");
    assert_eq!(next.storey, "level-2");
    assert!(!next.framed);
    assert_eq!(next.viewport, framed.viewport);
}

#[semio_framework_async_macros::async_test]
async fn the_plan_configuration_obeys_the_window_configuration_laws() {
    let base = BimViewerPlanWindowConfig::default();
    let next = base.with_storey("level-2").with_viewport(store::Viewport2d { x: 18.0, y: -9.0, zoom: 12.5 });
    assert_window_config_laws(&base, &BimViewerPlanWindowConfigMutation::Snapshot { config: next }).await;
}
