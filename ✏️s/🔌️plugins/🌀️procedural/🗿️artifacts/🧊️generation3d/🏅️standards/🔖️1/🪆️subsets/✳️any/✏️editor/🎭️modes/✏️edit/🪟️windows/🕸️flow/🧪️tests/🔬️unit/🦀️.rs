use super::*;
use crate::editor::generation3d::unit_tests::context::{app_with_registry, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await.contains("node-graph"));
}

/// 🛍️ The scene names its operators by KIND ID (inside `fixtureJson`) and carries only the document's
/// own neuron kinds as operator records — never the registered catalogue (~100 KB, three times the
/// fixed 32 KiB per-surface admission; ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). Those records
/// are how the canvas instantiates the graph instead of `FlowHostSnapshot::default()`'s placeholder slider.
#[semio_framework_async_macros::async_test]
async fn main_graph_scene_exports_flow_backed_node_graph_fields() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<NodeGraphScene>(&json).expect("node-graph scene decodes off the rendered surface");
    assert!(scene.host_snapshot_json.as_deref().is_some_and(|host_snapshot| host_snapshot.contains("flow.host_snapshot")));
    let capabilities = scene.capabilities_json.clone().unwrap_or_default();
    assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
    assert!(!scene.nodes.is_empty(), "the open document's nodes must reach the scene");
    assert!(!scene.nodes.iter().any(|node| node.id == "slider"), "the engine default fixture must not replace the open document");
    assert!(
        scene.operators.len() < 32,
        "operators must be document-derived, not the registered catalogue, carries {}",
        scene.operators.len()
    );
    assert!(
        scene.operators.iter().any(|operator| operator.id.contains("brep.") || operator.id.contains("math.")),
        "the open document's neuron kinds must reach the scene as operator records: {:?}",
        scene.operators.iter().map(|operator| operator.id.as_str()).collect::<Vec<_>>()
    );
}

/// 🛍️ …and the catalogue the scene no longer carries is exactly what this app publishes on the
/// reserved `framework.section.catalogue` retained surface, once per app instance.
#[semio_framework_async_macros::async_test]
async fn the_app_catalogue_section_carries_the_registered_operators() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let catalogue: serde_json::Value = serde_json::from_str(&semio_framework_os_flow::flow_app_catalogue_json()).expect("app catalogue json");
    let operators = catalogue.get("operators").and_then(|value| value.as_array()).expect("operators array");
    assert!(operators.iter().any(|operator| operator.get("id").and_then(|value| value.as_str()).is_some_and(|id| id.contains("math.add") || id.contains("brep."))));
    assert!(catalogue.get("sections").and_then(|value| value.as_array()).is_some_and(|sections| !sections.is_empty()));
}

//#region 🔖️GraphOutline
/// 🕸️ The law both implementations of this window answer to: one committed flow fixture in, the exact
/// node/port/wire rows and the exact interaction bindings out. Language-agnostic on purpose — the row
/// vocabulary is the `🟦️.ts` twin's too (`Generation3dFlowOutline`).
const GRAPH_OUTLINE_LAW: &str = include_str!("../../🧫️fixtures/🔬️unit/🔣️.json");

fn law_outline_projection() -> serde_json::Value {
    let law: serde_json::Value = serde_json::from_str(GRAPH_OUTLINE_LAW).expect("graph outline law json");
    let fixture = semio_framework_os_flow::FlowHost::parse_host_snapshot_json(&law["fixture"].to_string()).expect("law fixture parses");
    let (nodes, edges) = with_host(&fixture, |host| dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
    let labels = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::default());
    let outline = graph_outline(&TreeWindows::unhosted(), &nodes, &edges, None, labels).expect("outline builds");
    fixture.retire_cold();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(outline)).expect("outline projects");
    serde_json::from_str(&projection).expect("outline projection json")
}

fn law_rows(section: &serde_json::Value) -> Vec<(String, Option<String>, Vec<String>, Vec<String>, Option<String>)> {
    section["children"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(|row| {
            (
                row["key"].as_str().unwrap_or_default().to_string(),
                row["component"]["label"].as_str().map(str::to_string),
                row["children"].as_array().cloned().unwrap_or_default().iter().map(|port| port["key"].as_str().unwrap_or_default().to_string()).collect(),
                row["bindings"].as_array().cloned().unwrap_or_default().iter().map(|binding| binding["trigger"].as_str().unwrap_or_default().to_string()).collect(),
                row["component"]["granularity"].as_str().map(str::to_string),
            )
        })
        .collect()
}

#[test]
fn flow_graph_outline_answers_its_language_agnostic_law() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let law: serde_json::Value = serde_json::from_str(GRAPH_OUTLINE_LAW).expect("graph outline law json");
    let projection = law_outline_projection();
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some(GENERATION_3D_INTERACTION_DOMAIN));
    let sections = projection["children"].as_array().cloned().unwrap_or_default();
    assert_eq!(sections.len(), 2, "the outline carries exactly a nodes and a wires section");
    let nodes = law_rows(&sections[0]);
    let expected_nodes = law["expected"]["nodes"].as_array().expect("law nodes");
    assert_eq!(nodes.len(), expected_nodes.len(), "node rows: {nodes:?}");
    for (row, expected) in nodes.iter().zip(expected_nodes) {
        assert_eq!(row.0, expected["id"].as_str().unwrap_or_default(), "node row id");
        assert_eq!(row.1.as_deref(), expected["label"].as_str(), "node row label");
        let expected_ports: Vec<String> = expected["ports"].as_array().expect("law ports").iter().map(|port| port.as_str().unwrap_or_default().to_string()).collect();
        assert_eq!(row.2, expected_ports, "node {} port rows", row.0);
        assert_eq!(row.3, vec!["hoverPreview".to_string()], "node {} interaction bindings", row.0);
        assert_eq!(row.4.as_deref(), Some("node"), "node {} pick granularity", row.0);
    }
    let wires: Vec<String> = law_rows(&sections[1]).iter().map(|row| row.0.clone()).collect();
    let expected_wires: Vec<String> = law["expected"]["wires"].as_array().expect("law wires").iter().map(|wire| wire["id"].as_str().unwrap_or_default().to_string()).collect();
    assert_eq!(wires, expected_wires, "wire rows");
}

/// 🕹️ Both interaction verbs the framework injects for every `.interaction(...)` app still reach every
/// node row — but they reach it from two different places now. Selection is the tree's: ONE
/// `interactionSelect` binding on the root, which the host completes with the clicked row's own
/// `granularity` and key, so a row costs no argument arena and a document of any size stays
/// clickable. Hover stays the row's own: it is channel-addressed, which the domain binding alone
/// cannot express.
#[test]
fn flow_graph_rows_pick_through_the_tree_domain_and_hover_on_their_own() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let projection = law_outline_projection();
    let root = projection["bindings"].as_array().cloned().unwrap_or_default();
    let select = root.iter().find(|binding| binding["trigger"] == "activate").expect("the tree root's interactionSelect binding");
    assert_eq!(root.iter().filter(|binding| binding["trigger"] == "activate").count(), 1, "exactly one tree-level interactionSelect: {projection}");
    assert!(select["action"].to_string().contains(semio_framework_plugin::INTERACTION_SELECT_ACTION_ID), "{select}");
    assert!(select["args"].to_string().contains(GENERATION_3D_INTERACTION_DOMAIN), "{select}");
    let row = projection["children"][0]["children"][0].clone();
    assert_eq!(row["component"]["granularity"].as_str(), Some("node"), "{row}");
    let bindings = row["bindings"].as_array().cloned().unwrap_or_default();
    assert!(!bindings.iter().any(|binding| binding["trigger"] == "activate"), "a pick row carries no per-row activate binding: {row}");
    let hover = bindings.iter().find(|binding| binding["trigger"] == "hoverPreview").expect("hoverPreview binding");
    assert!(hover["action"].to_string().contains(semio_framework_plugin::INTERACTION_HOVER_ACTION_ID), "{hover}");
    let hover_args = hover["args"].to_string();
    assert!(hover_args.contains(GENERATION_3D_INTERACTION_CHANNEL), "{hover_args}");
    assert!(hover_args.contains(row["key"].as_str().unwrap_or_default()), "{hover_args}");
}

/// 🚦 A node row's description is the localized `NodeEvalStatus` the flow session reported for that
/// widget — English and German, no default language.
///
/// 🗑️ `stale` is deliberately absent: `build_flow_status_json` stopped emitting it when the census
/// became monotone, so the word had no producer and the vocabulary dropped it rather than keep a
/// status a user could never be shown (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn flow_graph_node_status_is_localized() {
    let english = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::default());
    let german = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() });
    for (tag, en, de) in [("queued", "Queued", "In Warteschlange"), ("computing", "Computing", "Berechnen"), ("error", "Error", "Fehler"), ("blocked", "Blocked", "Blockiert"), ("ok", "Evaluated", "Ausgewertet")] {
        let status = format!(r#"{{"height":{{"status":"{tag}"}}}}"#);
        assert_eq!(node_status_label(Some(&status), "height", english), Some(en), "{tag}");
        assert_eq!(node_status_label(Some(&status), "height", german), Some(de), "{tag}");
    }
    let unknown = r#"{"height":{"status":"stale"}}"#.to_string();
    assert_eq!(node_status_label(Some(&unknown), "height", english), Some("Evaluated"), "a word the census cannot emit is not a status word");
    assert_eq!(node_status_label(Some(&unknown), "missing", english), None);
    assert_eq!(node_status_label(None, "height", english), None);
}

/// 🧱️ The Flow window body is the node-graph canvas only — the artifact tree lives on the Artifact panel.
#[semio_framework_async_macros::async_test]
async fn main_body_is_canvas_only_without_an_embedded_artifact_tree() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let projection: serde_json::Value = serde_json::from_str(&render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await).expect("body projection json");
    fn find_tree(node: &serde_json::Value) -> Option<&serde_json::Value> {
        if node["component"]["type"].as_str() == Some("tree") {
            return Some(node);
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(find_tree))
    }
    assert!(find_tree(&projection).is_none(), "the flow window must not embed an artifact tree");
    assert!(projection.to_string().contains("node-graph"), "the flow window body must carry the node-graph surface");
}
//#endregion 🔖️GraphOutline

/// ⏎️ LAW: the node-graph canvas carries the `activate` binding its `role="application"` promises,
/// and the chord it advertises, exactly as `🧫️fixtures/⌨️keyboard-reachability.json` states.
///
/// `SurfaceAccessibilityShell` (`🗣️Interpreter/🟦️.tsx`) installs its Enter/Space handler ONLY when the
/// record actually carries an activate binding, so a canvas without one announces itself as an
/// application that handles its own keys and then handles none. `aria-keyshortcuts` comes off
/// `AccessibilitySpec.shortcut`, which is the only way a user who cannot see the canvas learns the
/// chord exists.
#[semio_framework_async_macros::async_test]
async fn the_node_graph_canvas_declares_the_activate_binding_the_keyboard_fixture_states() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let fixture: serde_json::Value = serde_json::from_str(crate::editor::generation3d::unit_tests::KEYBOARD_REACHABILITY_FIXTURE_JSON).expect("keyboard fixture");
    let row = fixture["surfaceBindings"].as_array().expect("surfaceBindings").iter().find(|row| row["surface"].as_str() == Some(GENERATION_3D_PLAY_SURFACE_MAIN)).expect("the node-graph canvas has a surface-binding row");
    let mut app = app_with_registry().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await;
    let action = row["action"].as_str().expect("action");
    assert!(json.contains(action), "the rendered canvas must bind {action}");
    assert!(json.contains(crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID), "the binding must be addressed at this app");
    assert!(json.contains(row["trigger"].as_str().expect("trigger")), "the binding must carry the fixture's trigger");
    assert!(json.contains(row["shortcut"].as_str().expect("shortcut")), "the canvas must advertise its chord as aria-keyshortcuts");
    eprintln!("[DEBUG] node-graph canvas binds {action} on {} with shortcut {}", row["trigger"].as_str().unwrap_or_default(), row["shortcut"].as_str().unwrap_or_default());
}
