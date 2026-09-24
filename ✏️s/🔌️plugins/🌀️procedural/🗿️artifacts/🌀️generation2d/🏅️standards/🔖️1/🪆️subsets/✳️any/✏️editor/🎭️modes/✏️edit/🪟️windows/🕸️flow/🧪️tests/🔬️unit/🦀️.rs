use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_main_graph_scene() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_MAIN).await;
    close(app);
    assert!(rendered.contains("node-graph"), "{rendered}");
}

/// 🛍️ The generation2d twin of the generation3d law: the registered operator catalogue is app-static
/// and rides the reserved `framework.section.catalogue` surface. The scene carries DOCUMENT-DERIVED
/// operator records only (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1), plus the flow-plugin-style
/// `NodeGraphInteractionDomain` and live selection (ticket 26/09/23/FLOW-AND-PROCEDURAL-FEATURE-COMPLETE).
#[semio_framework_async_macros::async_test]
async fn main_graph_scene_exports_flow_backed_node_graph_fields() {
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION2D_PLAY_BODY_MAIN).await;
    close(app);
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<NodeGraphScene>(&json).expect("node-graph scene decodes off the rendered surface");
    assert!(
        scene.host_snapshot_json.as_deref().is_some_and(|host_snapshot| host_snapshot.contains("flow.host_snapshot")),
        "flow-backed scene must carry a host snapshot"
    );
    assert!(scene.capabilities_json.as_deref().is_some_and(|capabilities| capabilities.contains("flow")));
    let domain = scene.interaction_domain.as_ref().expect("flow window must stamp NodeGraphInteractionDomain");
    assert_eq!(domain.id, crate::editor::generation2d::GENERATION2D_INTERACTION_DOMAIN);
    assert!(!domain.node_target_prefix.is_empty());
    assert!(!domain.edge_target_prefix.is_empty());
    assert!(!domain.handle_target_prefix.is_empty());
    assert!(
        scene.operators.iter().any(|operator| operator.id == "draw.shape.rect"),
        "operators must be document-derived for the open graph's non-core kinds, carries {:?}",
        scene.operators.iter().map(|operator| operator.id.as_str()).collect::<Vec<_>>()
    );
    assert!(scene.operators.len() < 32, "operators must be document-derived, not the registered catalogue, carries {}", scene.operators.len());
}

/// 🎯️ A live selection handed to `render` is painted onto the scene — the twin of the flow plugin
/// main window's `selection` field.
#[test]
fn render_paints_the_selection_it_is_handed() {
    let document = crate::Generation2dSnapshot::default();
    let config = config::Generation2dMainWindowConfig::default();
    let mut session = FlowEvalSession::new();
    let selection = vec!["slider".into(), "rect".into()];
    let node = render(&document, &config, &session, &selection).expect("empty fixture still renders a node-graph scene");
    let tree = semio_framework_plugin::built_to_component_tree(node);
    let encoded = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("tree json");
    session.begin_close();
    while !session.terminal_is_empty() {
        let _ = session.close_step(usize::MAX, usize::MAX);
    }
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<NodeGraphScene>(&encoded).expect("scene");
    assert_eq!(scene.selection, selection);
}

/// 🧪️ Installs one hand-authored `flow.extension` manifest so the shared operator registry this
/// crate's lib test links has something registered. The registry is EMPTY by construction here —
/// operator sets are contributed by the `flow-extension-*` plugin crates, which this artifact
/// deliberately never depends on (an artifact must not link a geometry kernel) — so a law about the
/// catalogue has to bring its own fixture, exactly as flow-core's own tests do
/// (`🌊️flow/🗿️artifacts/🌊️flow/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`).
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
