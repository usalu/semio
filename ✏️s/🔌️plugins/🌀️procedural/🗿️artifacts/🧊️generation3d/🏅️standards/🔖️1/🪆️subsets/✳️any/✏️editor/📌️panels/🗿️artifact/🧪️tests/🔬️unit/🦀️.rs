use super::*;
use crate::editor::generation3d::modes::edit::windows::flow::{graph_outline, GENERATION_3D_PLAY_OUTLINE_NODES, GENERATION_3D_PLAY_OUTLINE_WIRES};
use crate::editor::generation3d::unit_tests::context;
use crate::editor::generation3d::unit_tests::context::{app_with_registry, render as render_body};
use crate::standards::v1::subsets::any::schema::{dag_host_snapshot_to_workflow, with_host};
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};
use semio_framework_ui::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

const DOCUMENT_ROWS_LAW: &str = include_str!("../../🧫️fixtures/🔬️unit/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let rendered = render_body(&mut app, GENERATION_3D_PLAY_BODY_ARTIFACT).await;
    let fixture_widgets: Vec<String> = context::snapshot(&app).host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    let first = fixture_widgets.first().expect("default fixture has at least one widget");
    assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
}

/// 🧾️ The law fixture's document tree, projected through the same retiring projection the flow
/// window's outline law uses, so both trees are read as one renderer-neutral shape.
fn law_document_projection() -> serde_json::Value {
    let law: serde_json::Value = serde_json::from_str(DOCUMENT_ROWS_LAW).expect("document rows law json");
    let fixture = semio_framework_os_flow::FlowHost::parse_host_snapshot_json(&law["hostSnapshot"].to_string()).expect("law fixture parses");
    let (nodes, edges) = with_host(&fixture, |host| dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
    let labels = crate::editor::generation3d::terminology::generation3d_labels(&ViewModel::default());
    let outline = graph_outline(&TreeWindows::unhosted(), &nodes, &edges, None, labels).expect("document tree builds");
    fixture.retire_cold();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(outline)).expect("document tree projects");
    serde_json::from_str(&projection).expect("document projection json")
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

/// ⏎️ LAW: every document row is a `graph`-domain pick target in its own right — but it pays no
/// argument arena for it. The row declares its `granularity` and the TREE carries the one
/// `interactionSelect` binding the whole outline shares, which is what lets a document of any size
/// stay clickable (a per-row argument map refused past the arena page, ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, and left the Inspection panel on its "no selection" branch).
#[test]
fn document_rows_are_domain_pick_targets_without_a_per_row_binding() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let law: serde_json::Value = serde_json::from_str(DOCUMENT_ROWS_LAW).expect("document rows law json");
    let projection = law_document_projection();
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some(crate::editor::generation3d::GENERATION_3D_INTERACTION_DOMAIN));
    let root_activate = projection["bindings"].as_array().cloned().unwrap_or_default().iter().filter(|binding| binding["trigger"].as_str() == Some("activate")).count();
    assert_eq!(root_activate, 1, "the tree root carries exactly one interactionSelect binding: {projection}");
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
        assert_eq!(row.3, vec!["hoverPreview".to_string()], "node {} keeps only its channel-addressed hover binding", row.0);
        assert_eq!(row.4.as_deref(), Some("node"), "node {} declares its pick granularity", row.0);
    }
    let wires = law_rows(&sections[1]);
    let expected_wires: Vec<String> = law["expected"]["wires"].as_array().expect("law wires").iter().map(|wire| wire["id"].as_str().unwrap_or_default().to_string()).collect();
    assert_eq!(wires.iter().map(|row| row.0.clone()).collect::<Vec<_>>(), expected_wires, "wire rows");
    for row in &wires {
        assert_eq!(row.4.as_deref(), Some("edge"), "wire {} declares its pick granularity", row.0);
        assert!(row.3.is_empty(), "wire {} carries no per-row binding", row.0);
    }
}

//#region 🪟️WindowLaws
fn port(id: &str) -> NodeGraphPortRecord {
    NodeGraphPortRecord { id: id.into(), label: None, code: None, abbreviation: None, full_name: None, artifact_kind: None, value_type: None }
}

fn node(index: usize) -> NodeGraphNodeRecord {
    NodeGraphNodeRecord {
        id: format!("widget-{index:03}"),
        label: Some(format!("Widget {index}")),
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
        inputs: vec![port(&format!("widget-{index:03}@in")), port(&format!("widget-{index:03}@aux"))],
        outputs: vec![port(&format!("widget-{index:03}@out"))],
        instance_id: None,
        plugin_id: None,
        app_id: None,
        icon: None,
    }
}

fn wire(index: usize) -> NodeGraphEdgeRecord {
    NodeGraphEdgeRecord {
        id: format!("synapse-{index:03}"),
        source_node_id: format!("widget-{index:03}"),
        source_port_id: "out".into(),
        target_node_id: format!("widget-{:03}", index + 1),
        target_port_id: "in".into(),
        label: None,
    }
}

/// 🌾️ A document an order of magnitude past any viewport: 240 widgets and 200 wires.
fn oversized_graph() -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    ((0..240).map(node).collect(), (0..200).map(wire).collect())
}

fn outline_projection(windows: &TreeWindows<'_>, nodes: &[NodeGraphNodeRecord], edges: &[NodeGraphEdgeRecord]) -> serde_json::Value {
    let labels = crate::editor::generation3d::terminology::generation3d_labels(&ViewModel::default());
    let outline = graph_outline(windows, nodes, edges, None, labels).expect("outline builds");
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(outline)).expect("outline projects");
    serde_json::from_str(&projection).expect("outline projection json")
}

fn container(projection: &serde_json::Value, key: &str) -> serde_json::Value {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    walk(projection, key).unwrap_or_else(|| panic!("{key} is not in the outline: {projection}"))
}

fn window_of(node: &serde_json::Value) -> (u64, u64, usize) {
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{} stamps no window: {node}", node["key"]));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().map(Vec::len).unwrap_or_default(),
    )
}

fn window_view(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

/// ⚖️ LAW (a): an oversized document states its full extent and materialises at most its slice.
/// Every container — both sections and each node's nested port list — stamps `window.total ==
/// entries.len()`, so the host's scrollbar spans the whole graph rather than a page of it.
#[test]
fn an_oversized_document_stamps_every_containers_total() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let (nodes, edges) = oversized_graph();
    let projection = outline_projection(&TreeWindows::unhosted(), &nodes, &edges);
    let (node_total, _, node_rows) = window_of(&container(&projection, GENERATION_3D_PLAY_OUTLINE_NODES));
    assert_eq!(node_total as usize, nodes.len(), "the nodes section stamps the whole widget count");
    assert!(node_rows <= nodes.len(), "the nodes section materialises at most its slice: {node_rows}");
    let (wire_total, _, wire_rows) = window_of(&container(&projection, GENERATION_3D_PLAY_OUTLINE_WIRES));
    assert_eq!(wire_total as usize, edges.len(), "the wires section stamps the whole wire count");
    assert!(wire_rows <= edges.len(), "the wires section materialises at most its slice: {wire_rows}");
    let first_row = container(&projection, &nodes[0].id);
    let (port_total, _, port_rows) = window_of(&first_row);
    assert_eq!(port_total, 3, "a node row stamps its own port count");
    assert!(port_rows <= 3, "a node row materialises at most its port slice: {port_rows}");
}

/// ⚖️ LAW (d): nothing is ever summarised. A windowed outline publishes no `.more` key and no
/// `+n` label anywhere in the body.
#[test]
fn an_oversized_document_renders_no_continuation_row() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let (nodes, edges) = oversized_graph();
    let json = outline_projection(&TreeWindows::unhosted(), &nodes, &edges).to_string();
    assert!(!json.contains(".more\""), "a windowed outline has no continuation row");
    assert!(!json.contains(r#""label":"+"#), "a windowed outline publishes no `+n` label");
}

/// ⚖️ LAW (b): a closed container states its extent and costs nothing — `total` stamped, zero
/// children, which is what makes the first paint bounded no matter how large the document is.
#[test]
fn a_closed_document_section_stamps_its_total_with_no_children() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let (nodes, edges) = oversized_graph();
    let view = window_view(vec![TreeWindowRequest {
        body_key: GENERATION_3D_PLAY_BODY_ARTIFACT.into(),
        node_key: GENERATION_3D_PLAY_OUTLINE_NODES.into(),
        open: Some(false),
        offset: 0,
        rows: 0,
    }]);
    let projection = outline_projection(&TreeWindows::for_body(&view, GENERATION_3D_PLAY_BODY_ARTIFACT), &nodes, &edges);
    let (total, offset, rows) = window_of(&container(&projection, GENERATION_3D_PLAY_OUTLINE_NODES));
    assert_eq!(total as usize, nodes.len(), "a closed section still states its extent");
    assert_eq!(offset, 0, "a closed section starts at zero");
    assert_eq!(rows, 0, "a closed section materialises nothing");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)`, keyed by the
/// raw widget id — the guest answers the viewport, it does not re-derive a page.
#[test]
fn a_document_window_request_materialises_exactly_its_slice() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let (nodes, edges) = oversized_graph();
    let view = window_view(vec![TreeWindowRequest {
        body_key: GENERATION_3D_PLAY_BODY_ARTIFACT.into(),
        node_key: GENERATION_3D_PLAY_OUTLINE_NODES.into(),
        open: Some(true),
        offset: 100,
        rows: 12,
    }]);
    let projection = outline_projection(&TreeWindows::for_body(&view, GENERATION_3D_PLAY_BODY_ARTIFACT), &nodes, &edges);
    let section = container(&projection, GENERATION_3D_PLAY_OUTLINE_NODES);
    let rendered: Vec<String> = section["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect();
    let expected: Vec<String> = nodes.iter().skip(100).take(12).map(|node| node.id.clone()).collect();
    assert_eq!(rendered, expected, "the section materialises exactly entries [100, 112)");
    let (total, offset, _) = window_of(&section);
    assert_eq!(offset, 100, "the stamped offset is the requested one");
    assert_eq!(total as usize, nodes.len(), "the stamped total stays the whole document");
}
//#endregion 🪟️WindowLaws
