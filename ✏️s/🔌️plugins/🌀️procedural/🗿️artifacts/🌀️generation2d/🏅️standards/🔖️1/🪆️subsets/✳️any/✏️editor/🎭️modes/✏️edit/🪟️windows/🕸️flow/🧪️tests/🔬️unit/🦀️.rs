
use super::*;
use crate::editor::generation2d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_main_graph_scene() {
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION2D_PLAY_BODY_MAIN).await.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn main_graph_scene_exports_flow_backed_node_graph_fields() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_MAIN).await;
    let value: serde_json::Value = serde_json::from_str(&json).expect("ui node json");
    let graph = value.get("nodeGraph").expect("nodeGraph");
    assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
    assert!(graph.get("operators").and_then(|v| v.as_array()).is_some_and(|items| !items.is_empty()));
    assert!(graph.get("capabilitiesJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow")));
}

#[test]
fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, GENERATION2D_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}
