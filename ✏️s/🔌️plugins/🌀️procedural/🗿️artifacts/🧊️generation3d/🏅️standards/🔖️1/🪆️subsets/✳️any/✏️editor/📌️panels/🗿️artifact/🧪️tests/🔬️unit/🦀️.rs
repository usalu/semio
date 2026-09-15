use super::*;
use crate::editor::generation3d::modes::edit::windows::flow::graph_outline;
use crate::editor::generation3d::unit_tests::context;
use crate::editor::generation3d::unit_tests::context::{app_with_registry, render as render_body};
use crate::standards::v1::subsets::any::schema::{dag_host_document_to_workflow, with_host};

const DOCUMENT_ROWS_LAW: &str = include_str!("../../🧫️fixtures/🔬️unit/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let rendered = render_body(&mut app, GENERATION_3D_PLAY_BODY_ARTIFACT).await;
    let fixture_widgets: Vec<String> = context::snapshot(&app).host_document.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    let first = fixture_widgets.first().expect("default fixture has at least one widget");
    assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
}

/// 🧾️ The law fixture's document tree, projected through the same retiring projection the flow
/// window's outline law uses, so both trees are read as one renderer-neutral shape.
fn law_document_projection() -> serde_json::Value {
    let law: serde_json::Value = serde_json::from_str(DOCUMENT_ROWS_LAW).expect("document rows law json");
    let fixture = semio_framework_os_flow::FlowHost::parse_host_document_json(&law["fixture"].to_string()).expect("law fixture parses");
    let (nodes, edges) = with_host(&snapshot, |host| dag_host_document_to_workflow(&host.dag.host_document));
    let labels = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::default());
    let outline = graph_outline(&nodes, &edges, None, labels).expect("document tree builds");
    fixture.retire_cold();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(outline)).expect("document tree projects");
    serde_json::from_str(&projection).expect("document projection json")
}

fn law_rows(section: &serde_json::Value) -> Vec<(String, Option<String>, Vec<String>, Vec<String>)> {
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
            )
        })
        .collect()
}

/// ⏎️ LAW: every document row is a `graph`-domain pick target in its own right. A row that carries only
/// the tree's `interactionDomain` is inert on the React renderer — a real click reached the row and
/// nothing was invoked (ticket 26/09/09/PROCEDURAL-3D-END-TO-END), which also left the Inspection
/// panel permanently on its "no selection" branch.
#[test]
fn document_rows_bind_both_framework_interaction_verbs() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let law: serde_json::Value = serde_json::from_str(DOCUMENT_ROWS_LAW).expect("document rows law json");
    let projection = law_document_projection();
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some(crate::editor::generation3d::GENERATION_3D_INTERACTION_DOMAIN));
    let sections = projection["children"].as_array().cloned().unwrap_or_default();
    assert_eq!(sections.len(), 2, "the document tree carries nodes and wires sections");
    let nodes = law_rows(&sections[0]);
    let expected_nodes = law["expected"]["nodes"].as_array().expect("law nodes");
    assert_eq!(nodes.len(), expected_nodes.len(), "node rows: {nodes:?}");
    for (row, expected) in nodes.iter().zip(expected_nodes) {
        assert_eq!(row.0, expected["id"].as_str().unwrap_or_default(), "node row id");
        assert_eq!(row.1.as_deref(), expected["label"].as_str(), "node row label");
        let expected_ports: Vec<String> = expected["ports"].as_array().expect("law ports").iter().map(|port| port.as_str().unwrap_or_default().to_string()).collect();
        assert_eq!(row.2, expected_ports, "node {} port rows", row.0);
        assert_eq!(row.3, vec!["activate".to_string(), "hoverPreview".to_string()], "node {} interaction bindings", row.0);
    }
    let wires: Vec<String> = law_rows(&sections[1]).iter().map(|row| row.0.clone()).collect();
    let expected_wires: Vec<String> = law["expected"]["wires"].as_array().expect("law wires").iter().map(|wire| wire["id"].as_str().unwrap_or_default().to_string()).collect();
    assert_eq!(wires, expected_wires, "wire rows");
}
