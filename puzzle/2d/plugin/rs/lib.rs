//! 🧩 Puzzle 2D plugin — declarative puzzle 2d play app bundled as a hot-swappable WASM component.

use puzzle_2d::Puzzle2dExtension;
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App,
    Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const PUZZLE2D_PLAY_APP_ID: &str = "puzzle2d-play";
const PUZZLE2D_PLAY_CONTROLLER_ID: &str = "puzzle2d-play";
const PUZZLE2D_PLAY_SURFACE_ID: &str = "puzzle2d.play.composite";
const PUZZLE2D_PLAY_BODY_COMPOSITE: &str = "puzzle2d.play.composite";
const PUZZLE2D_PLAY_BODY_LAYERS: &str = "puzzle2d.play.layers";
const PUZZLE2D_PLAY_BODY_CATALOGUE: &str = "puzzle2d.play.catalogue";
const PUZZLE2D_PLAY_BODY_PROPERTIES: &str = "puzzle2d.play.properties";
const PUZZLE2D_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
const PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID: &str = "concrete-forest";
const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../example/concrete-forest.2d.json");

static NODE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle2dPlayEnvelope {
    fixture: Value,
    #[serde(default)]
    runtime: Puzzle2dPlayRuntime,
}

fn default_empty_fixture() -> Value {
    json!({
        "schema": PUZZLE2D_FIXTURE_SCHEMA,
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [],
        "edges": [],
        "wires": []
    })
}

fn default_envelope() -> Puzzle2dPlayEnvelope {
    Puzzle2dPlayEnvelope {
        fixture: default_empty_fixture(),
        runtime: Puzzle2dPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> Puzzle2dPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Puzzle2dPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn puzzle2d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PUZZLE2D_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("id"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}

fn fixture_camera(fixture: &Value) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
}

fn fixture_nodes(fixture: &Value) -> &[Value] {
    fixture
        .get("nodes")
        .and_then(|value| value.as_array())
        .map(|values| values.as_slice())
        .unwrap_or(&[])
}

fn fixture_edges(fixture: &Value) -> &[Value] {
    fixture
        .get("edges")
        .and_then(|value| value.as_array())
        .map(|values| values.as_slice())
        .unwrap_or(&[])
}

fn kind_catalog_entries<'a>(fixture: &'a Value, key: &str) -> Option<&'a [Value]> {
    fixture
        .get("meta")
        .and_then(|value| value.get("kindCatalogs"))
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_array())
        .map(|values| values.as_slice())
}

fn new_node_id(prefix: &str) -> String {
    let serial = NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{serial}")
}

fn add_node_to_fixture(fixture: &mut Value, kind: Option<&str>) {
    let Some(obj) = fixture.as_object_mut() else {
        return;
    };
    let nodes = obj.entry("nodes".to_string()).or_insert_with(|| json!([]));
    let Some(nodes) = nodes.as_array_mut() else {
        return;
    };
    let node_kind = kind.unwrap_or("node");
    let id = new_node_id("node");
    nodes.push(json!({
        "id": id,
        "nodeKind": node_kind,
        "shape": "circle",
        "x": 0.0,
        "y": 0.0,
        "radius": 24.0,
        "text": id,
        "handles": []
    }));
}

fn delete_selection_from_fixture(fixture: &mut Value, selected: &[String]) {
    if selected.is_empty() {
        return;
    }
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let node_ids: HashSet<String> = fixture_nodes(fixture)
        .iter()
        .filter_map(|node| node.get("id").and_then(|value| value.as_str()))
        .filter(|id| selected.contains(id))
        .map(str::to_string)
        .collect();
    let handle_ids: HashSet<String> = fixture_nodes(fixture)
        .iter()
        .flat_map(|node| {
            node.get("handles")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
                .filter_map(|handle| handle.get("id").and_then(|value| value.as_str()))
        })
        .filter(|id| selected.contains(id))
        .map(str::to_string)
        .collect();
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
        *nodes = nodes
            .iter()
            .filter(|node| {
                node.get("id")
                    .and_then(|value| value.as_str())
                    .is_none_or(|id| !node_ids.contains(id))
            })
            .map(|node| {
                let mut next = node.clone();
                if let Some(handles) = next.get_mut("handles").and_then(|value| value.as_array_mut()) {
                    handles.retain(|handle| {
                        handle
                            .get("id")
                            .and_then(|value| value.as_str())
                            .is_none_or(|id| !handle_ids.contains(id))
                    });
                }
                next
            })
            .collect();
    }
    if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
        edges.retain(|edge| {
            let id_ok = edge
                .get("id")
                .and_then(|value| value.as_str())
                .is_none_or(|id| !selected.contains(id));
            let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("");
            let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("");
            id_ok && !node_ids.contains(source) && !node_ids.contains(target) && !handle_ids.contains(source) && !handle_ids.contains(target)
        });
    }
}

fn set_fixture_camera(fixture: &mut Value, camera: &Value) {
    if let Some(obj) = fixture.as_object_mut() {
        obj.insert("camera".to_string(), camera.clone());
    }
}

fn puzzle_extension_id() -> &'static str {
    let _extension = Puzzle2dExtension;
    "puzzle.2d"
}
//#endregion 🔖Envelope

//#region 🔖Canvas
fn render_canvas(fixture: &Value) -> UiNode {
    let (camera_x, camera_y, zoom) = fixture_camera(fixture);
    build_canvas_2d_scene(
        PUZZLE2D_PLAY_SURFACE_ID,
        PUZZLE2D_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x,
            camera_y,
            zoom,
            layers_json: serde_json::to_string(fixture_nodes(fixture)).unwrap_or_else(|_| "[]".into()),
        },
    )
}
//#endregion 🔖Canvas

//#region 🔖HierarchyPanel
fn tree_item_with_command(id: impl Into<String>, label: impl Into<String>, description: Option<String>, command: CommandDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        default_open: None,
        command: Some(command),
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn node_label(node: &Value) -> String {
    node.get("text")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| node.get("id").and_then(|value| value.as_str()))
        .unwrap_or("node")
        .into()
}

fn edge_label(edge: &Value, fixture: &Value) -> String {
    let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("?");
    let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("?");
    let source_label = fixture_nodes(fixture)
        .iter()
        .find(|node| node.get("id").and_then(|value| value.as_str()) == Some(source))
        .map(node_label)
        .unwrap_or_else(|| source.into());
    let target_label = fixture_nodes(fixture)
        .iter()
        .find(|node| node.get("id").and_then(|value| value.as_str()) == Some(target))
        .map(node_label)
        .unwrap_or_else(|| target.into());
    format!("{source_label} → {target_label}")
}

fn hierarchy_tree_selected_ids(fixture: &Value, selected: &[String]) -> Vec<String> {
    selected
        .iter()
        .filter_map(|id| {
            if fixture_nodes(fixture)
                .iter()
                .any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
            {
                return Some(format!("puzzle2d-play-hierarchy.node.{id}"));
            }
            if fixture_edges(fixture)
                .iter()
                .any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
            {
                return Some(format!("puzzle2d-play-hierarchy.edge.{id}"));
            }
            None
        })
        .collect()
}

fn render_hierarchy_panel(envelope: &Puzzle2dPlayEnvelope) -> UiNode {
    let fixture = &envelope.fixture;
    let node_items: Vec<UiTreeItemNode> = fixture_nodes(fixture)
        .iter()
        .filter_map(|node| {
            let id = node.get("id")?.as_str()?;
            Some(tree_item_with_command(
                format!("puzzle2d-play-hierarchy.node.{id}"),
                node_label(node),
                node.get("nodeKind").and_then(|value| value.as_str()).map(str::to_string),
                puzzle2d_cmd("setSelection", Some(json!({ "ids": [id] }))),
            ))
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture_edges(fixture)
        .iter()
        .filter_map(|edge| {
            let id = edge.get("id")?.as_str()?;
            Some(tree_item_with_command(
                format!("puzzle2d-play-hierarchy.edge.{id}"),
                edge_label(edge, fixture),
                edge.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                puzzle2d_cmd("setSelection", Some(json!({ "ids": [id] }))),
            ))
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "puzzle2d-play-hierarchy.nodes".into(),
                label: Some("Nodes".into()),
                default_open: Some(true),
                items: if node_items.is_empty() {
                    vec![UiTreeItemNode {
                        id: "puzzle2d-play-hierarchy.nodes.empty".into(),
                        label: "(none)".into(),
                        description: None,
                        icon_id: None,
                        selected: None,
                        default_open: None,
                        command: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    }]
                } else {
                    node_items
                },
            },
            UiTreeSectionNode {
                id: "puzzle2d-play-hierarchy.edges".into(),
                label: Some("Edges".into()),
                default_open: Some(false),
                items: if edge_items.is_empty() {
                    vec![UiTreeItemNode {
                        id: "puzzle2d-play-hierarchy.edges.empty".into(),
                        label: "(none)".into(),
                        description: None,
                        icon_id: None,
                        selected: None,
                        default_open: None,
                        command: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    }]
                } else {
                    edge_items
                },
            },
        ],
        selected_ids: Some(hierarchy_tree_selected_ids(fixture, &envelope.runtime.selected_ids)),
        highlighted_ids: None,
        selection_change: Some(puzzle2d_cmd("setSelection", None)),
    })
}
//#endregion 🔖HierarchyPanel

//#region 🔖CataloguePanel
fn catalog_kind_label(entry: &Value) -> String {
    entry
        .get("name")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| entry.get("id").and_then(|value| value.as_str()))
        .unwrap_or("kind")
        .into()
}

fn inferred_kind_entries(fixture: &Value, field: &str) -> Vec<Value> {
    let mut ids = BTreeSet::new();
    match field {
        "nodes" => {
            for node in fixture_nodes(fixture) {
                if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        "handles" => {
            for node in fixture_nodes(fixture) {
                if let Some(handles) = node.get("handles").and_then(|value| value.as_array()) {
                    for handle in handles {
                        if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                            ids.insert(kind.to_string());
                        }
                    }
                }
            }
        }
        "edges" => {
            for edge in fixture_edges(fixture) {
                if let Some(kind) = edge.get("edgeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        _ => {}
    }
    ids.into_iter()
        .map(|id| json!({ "id": id, "name": id }))
        .collect()
}

fn kind_catalog_section(section_id: &str, label: &str, entries: &[Value]) -> UiTreeSectionNode {
    let items: Vec<UiTreeItemNode> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            UiTreeItemNode {
                id: format!("{section_id}.{index}.{kind_id}"),
                label: catalog_kind_label(entry),
                description: Some(kind_id.into()),
                icon_id: None,
                selected: None,
                default_open: None,
                command: Some(puzzle2d_cmd("addNode", Some(json!({ "kind": kind_id })))),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    UiTreeSectionNode {
        id: section_id.into(),
        label: Some(label.into()),
        default_open: Some(true),
        items: if items.is_empty() {
            vec![UiTreeItemNode {
                id: format!("{section_id}.empty"),
                label: "(none)".into(),
                description: None,
                icon_id: None,
                selected: None,
                default_open: None,
                command: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }]
        } else {
            items
        },
    }
}

fn render_catalogue_panel(fixture: &Value) -> UiNode {
    let inferred_nodes = inferred_kind_entries(fixture, "nodes");
    let inferred_handles = inferred_kind_entries(fixture, "handles");
    let inferred_edges = inferred_kind_entries(fixture, "edges");
    let node_entries = kind_catalog_entries(fixture, "nodes").unwrap_or(inferred_nodes.as_slice());
    let handle_entries = kind_catalog_entries(fixture, "handles").unwrap_or(inferred_handles.as_slice());
    let edge_entries = kind_catalog_entries(fixture, "edges").unwrap_or(inferred_edges.as_slice());
    UiNode::Tree(UiTreeNode {
        sections: vec![
            kind_catalog_section("puzzle2d-play-kinds.nodes", "Nodes", &node_entries),
            kind_catalog_section("puzzle2d-play-kinds.handles", "Handles", &handle_entries),
            kind_catalog_section("puzzle2d-play-kinds.edges", "Edges", &edge_entries),
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}
//#endregion 🔖CataloguePanel

//#region 🔖InspectorPanel
fn render_properties_panel(envelope: &Puzzle2dPlayEnvelope) -> UiNode {
    let selected_nodes: Vec<&Value> = envelope
        .runtime
        .selected_ids
        .iter()
        .filter_map(|id| {
            fixture_nodes(&envelope.fixture)
                .iter()
                .find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
        })
        .collect();
    if selected_nodes.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {PUZZLE2D_FIXTURE_SCHEMA}")),
            ui_text(format!("Extension: {}", puzzle_extension_id())),
            ui_text(format!("Nodes: {}", fixture_nodes(&envelope.fixture).len())),
            ui_text(format!("Edges: {}", fixture_edges(&envelope.fixture).len())),
        ]);
    }
    let node = selected_nodes[0];
    ui_stack_vertical(vec![
        ui_inspector_readonly_field(
            "puzzle2d-play-inspector.id",
            "Id",
            node.get("id")
                .and_then(|value| value.as_str())
                .unwrap_or("")
                .to_string(),
        ),
        ui_inspector_readonly_field(
            "puzzle2d-play-inspector.node-kind",
            "Node Kind",
            node.get("nodeKind")
                .and_then(|value| value.as_str())
                .unwrap_or("—")
                .to_string(),
        ),
        ui_inspector_readonly_field(
            "puzzle2d-play-inspector.x",
            "X",
            node.get("x")
                .and_then(|value| value.as_f64())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".into()),
        ),
        ui_inspector_readonly_field(
            "puzzle2d-play-inspector.y",
            "Y",
            node.get("y")
                .and_then(|value| value.as_f64())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".into()),
        ),
    ])
}
//#endregion 🔖InspectorPanel

//#region 🔖Puzzle2dPlayApp
struct Puzzle2dPlayApp;

impl PluginApp for Puzzle2dPlayApp {
    fn app_id(&self) -> &str {
        PUZZLE2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("puzzle2d envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "hierarchySelect" => {
                envelope.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                add_node_to_fixture(&mut envelope.fixture, kind);
                return vec![set_document_op(&envelope)];
            }
            "deleteSelection" => {
                delete_selection_from_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                envelope.runtime.selected_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    set_fixture_camera(&mut envelope.fixture, camera);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope.fixture = if example_id.is_empty() || example_id == "empty" {
                    default_empty_fixture()
                } else if example_id == PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID || example_id == "concrete" {
                    serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture())
                } else {
                    default_empty_fixture()
                };
                envelope.runtime.selected_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            PUZZLE2D_PLAY_BODY_COMPOSITE => render_canvas(&envelope.fixture),
            PUZZLE2D_PLAY_BODY_LAYERS => render_hierarchy_panel(&envelope),
            PUZZLE2D_PLAY_BODY_CATALOGUE => render_catalogue_panel(&envelope.fixture),
            PUZZLE2D_PLAY_BODY_PROPERTIES => render_properties_panel(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Puzzle2dPlayApp

//#region 🔖AppFactory
fn create_puzzle2d_app() -> App {
    App::from_builder(
        App::builder(PUZZLE2D_PLAY_APP_ID, "Puzzle 2D")
            .icon_id("puzzle2d")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind("puzzle2d-composite", "Canvas", PUZZLE2D_PLAY_BODY_COMPOSITE)
            .panel_tab("framework.panel.hierarchy", FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, "workbench", PUZZLE2D_PLAY_BODY_LAYERS)
            .panel_tab("framework.panel.catalogue", FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "workbench", PUZZLE2D_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "details", PUZZLE2D_PLAY_BODY_PROPERTIES)
            .default_layout(create_default_layout(
                &["puzzle2d-composite".into()],
                "row",
                Some(&[100.0]),
                Some(&["Canvas".into()]),
            )),
    )
    .example("empty", "Empty", serde_json::to_string(&default_envelope()).unwrap())
    .example(
        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
        "Concrete Forest",
        serde_json::to_string(&Puzzle2dPlayEnvelope {
            fixture: serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture()),
            runtime: Puzzle2dPlayRuntime::default(),
        })
        .unwrap(),
    )
    .program("puzzle2d", "Puzzle 2D", "layout")
}

fn puzzle2d_bundle() -> PluginBundle {
    PluginBundle::new("puzzle2d", "Puzzle 2D", "0.1.0").register_app(create_puzzle2d_app(), || Box::new(Puzzle2dPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(puzzle2d_bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_canvas_scene() {
        let app = Puzzle2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PUZZLE2D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn hierarchy_panel_lists_nodes_section() {
        let app = Puzzle2dPlayApp;
        let envelope = Puzzle2dPlayEnvelope {
            fixture: serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap(),
            runtime: Puzzle2dPlayRuntime::default(),
        };
        let document = serde_json::to_string(&envelope).unwrap();
        let node = app.render(PUZZLE2D_PLAY_BODY_LAYERS, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("puzzle2d-play-hierarchy.nodes"));
        assert!(json.contains("seed-left-001"));
    }

    #[test]
    fn add_node_command_appends_node() {
        let mut app = Puzzle2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("addNode", Some(&json!({ "kind": "node" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert_eq!(envelope.fixture.get("nodes").and_then(|value| value.as_array()).map(|values| values.len()), Some(1));
    }

    fn apply_document_op(document_json: &str, op_json: &str) -> Puzzle2dPlayEnvelope {
        let mut envelope = parse_envelope(document_json);
        if let Ok(op) = serde_json::from_str::<Value>(op_json) {
            if op.get("op").and_then(|value| value.as_str()) == Some("setDocument") {
                if let Some(document) = op.get("document") {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        envelope = parsed;
                    }
                }
            }
        }
        envelope
    }
}
//#endregion 🧪Tests
