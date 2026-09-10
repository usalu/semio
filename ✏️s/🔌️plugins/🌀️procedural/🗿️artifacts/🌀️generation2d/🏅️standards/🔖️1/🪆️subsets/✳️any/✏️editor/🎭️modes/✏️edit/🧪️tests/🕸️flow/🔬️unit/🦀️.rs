use super::*;
use crate::editor::generation2d::testkit::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_main_graph_scene() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_MAIN).await;
    close(app);
    assert!(rendered.contains("node-graph"), "{rendered}");
}

/// 🛍️ The generation2d twin of the generation3d law: the registered operator catalogue is app-static
/// and rides the reserved `framework.section.catalogue` surface, never this scene's fixed-capacity
/// payload (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
#[semio_framework_async_macros::async_test]
async fn main_graph_scene_exports_flow_backed_node_graph_fields() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_MAIN).await;
    close(app);
    let scene = semio_framework_plugin::testkit::decode_fixture_scene::<NodeGraphScene>(&json).expect("node-graph scene decodes off the rendered surface");
    assert!(scene.fixture_json.as_deref().is_some_and(|fixture| fixture.contains("flow.fixture")));
    assert!(scene.capabilities_json.as_deref().is_some_and(|capabilities| capabilities.contains("flow")));
    assert!(scene.operators.is_empty(), "a flow-backed scene must carry no operator records, carries {}", scene.operators.len());
}

/// 🧪️ Installs one hand-authored `flow.extension` manifest so the shared operator registry this
/// crate's lib test links has something registered. The registry is EMPTY by construction here —
/// operator sets are contributed by the `flow-extension-*` plugin crates, which this artifact
/// deliberately never depends on (an artifact must not link a geometry kernel) — so a law about the
/// catalogue has to bring its own fixture, exactly as flow-core's own tests do
/// (`🌊️flow/🗿️artifacts/🌊️flow/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`).
fn install_catalogue_fixture_extension() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let manifest = semio_framework_os_flow::FlowExtensionManifest {
            schema: "flow.extension".into(),
            id: "math".into(),
            name: "Math".into(),
            version: "0.0.0-test-fixture".into(),
            activation_events: vec!["onStartup".into()],
            contributes: semio_framework_os_flow::FlowExtensionContributes {
                schemas: vec![],
                operators: vec![semio_framework_os_flow::neural::OperatorInfo { id: "math.add".into(), extension: "math".into(), name: "Add".into(), abbreviation: "Add".into(), ..Default::default() }],
                widgets: vec![],
                commands: vec![],
                settings: vec![],
            },
        };
        let manifest_json = semio_framework_os_flow::os_pack::json::to_json_string(&manifest);
        semio_framework_os_flow::install_flow_extension_manifest("generation2d-catalogue-test-fixture", &manifest_json).expect("fixture extension admission");
    });
}

/// 🛍️ …and the catalogue the scene no longer carries is exactly what this app publishes on the
/// reserved `framework.section.catalogue` retained surface, once per app instance.
#[semio_framework_async_macros::async_test]
async fn the_app_catalogue_section_carries_the_registered_operators() {
    install_catalogue_fixture_extension();
    let catalogue: serde_json::Value = serde_json::from_str(&semio_framework_os_flow::flow_app_catalogue_json()).expect("app catalogue json");
    let operators = catalogue.get("operators").and_then(|value| value.as_array()).expect("app catalogue operators");
    assert!(operators.iter().any(|operator| operator.get("id").and_then(|id| id.as_str()) == Some("math.add")), "{operators:?}");
    let sections = catalogue.get("sections").and_then(|value| value.as_array()).expect("app catalogue sections");
    assert!(sections.iter().any(|section| section.get("id").and_then(|id| id.as_str()) == Some("math")), "{sections:?}");
    assert!(sections.iter().any(|section| section.get("id").and_then(|id| id.as_str()) == Some("inputs")), "the static widget sections must merge into the app catalogue; {sections:?}");
}

#[test]
fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, GENERATION2D_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}
