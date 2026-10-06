use super::*;
use crate::app_fixture::{app_with_registry, render_body};

const KEYBOARD_REACHABILITY_FIXTURE_JSON: &str = include_str!("../../../../../../../🧫️fixtures/⌨️keyboard-reachability.json");

#[test]
fn flow_graph_node_status_is_localized() {
    let english = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let german = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) });
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

#[semio_framework_async_macros::async_test]
async fn the_node_graph_canvas_declares_the_activate_binding_the_keyboard_snapshot_states() {
    let _serial = crate::test_serial::lock();
    let fixture: serde_json::Value = serde_json::from_str(KEYBOARD_REACHABILITY_FIXTURE_JSON).expect("keyboard fixture");
    let row = fixture["surfaceBindings"].as_array().expect("surfaceBindings").iter().find(|row| row["surface"].as_str() == Some(GENERATION_3D_PLAY_SURFACE_MAIN)).expect("the node-graph canvas has a surface-binding row");
    let mut app = app_with_registry().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await;
    let action = row["action"].as_str().expect("action");
    assert!(json.contains(action), "the rendered canvas must bind {action}");
    assert!(json.contains(crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID), "the binding must be addressed at this app");
    assert!(json.contains(row["trigger"].as_str().expect("trigger")), "the binding must carry the fixture's trigger");
    assert!(json.contains(row["shortcut"].as_str().expect("shortcut")), "the canvas must advertise its chord as aria-keyshortcuts");
}

const GRAPH_OUTLINE_LAW: &str = include_str!("../../🧫️fixtures/🔬️unit/🔣️.json");

fn law_outline_projection() -> serde_json::Value {
    let law: serde_json::Value = serde_json::from_str(GRAPH_OUTLINE_LAW).expect("graph outline law json");
    let fixture = semio_framework_os_flow::FlowHost::parse_host_snapshot_json(&law["hostSnapshot"].to_string()).expect("law fixture parses");
    let (nodes, edges) = with_host(&fixture, |host| dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
    let labels = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
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
    let _serial = crate::test_serial::lock();
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
    let _serial = crate::test_serial::lock();
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
