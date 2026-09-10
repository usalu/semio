use super::*;
use crate::editor::generation3d::testkit::{app_with_registry, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app_with_registry().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await.contains("node-graph"));
}

/// 🛍️ The scene names its operators by KIND ID (inside `fixtureJson`) and carries no operator records
/// of its own: the registered catalogue is app-static and ~100 KB with the real `brep`/`math` sets
/// installed, three times the fixed 32 KiB per-surface admission this very render is checked against
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
#[semio_framework_async_macros::async_test]
async fn main_graph_scene_exports_flow_backed_node_graph_fields() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app_with_registry().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::testkit::decode_fixture_scene::<NodeGraphScene>(&json).expect("node-graph scene decodes off the rendered surface");
    assert!(scene.fixture_json.as_deref().is_some_and(|fixture| fixture.contains("flow.fixture")));
    let capabilities = scene.capabilities_json.clone().unwrap_or_default();
    assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
    assert!(scene.operators.is_empty(), "a flow-backed scene must carry no operator records, carries {}", scene.operators.len());
}

/// 🛍️ …and the catalogue the scene no longer carries is exactly what this app publishes on the
/// reserved `framework.section.catalogue` retained surface, once per app instance.
#[semio_framework_async_macros::async_test]
async fn the_app_catalogue_section_carries_the_registered_operators() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let catalogue: serde_json::Value = serde_json::from_str(&semio_framework_os_flow::flow_app_catalogue_json()).expect("app catalogue json");
    let operators = catalogue.get("operators").and_then(|value| value.as_array()).expect("operators array");
    assert!(operators.iter().any(|operator| operator.get("id").and_then(|value| value.as_str()).is_some_and(|id| id.contains("math.add") || id.contains("brep."))));
    assert!(catalogue.get("sections").and_then(|value| value.as_array()).is_some_and(|sections| !sections.is_empty()));
}
