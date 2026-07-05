//! 🧠 Mindmap Wires plugin — declarative WIRES play app bundled as a hot-swappable WASM component.

use puzzle_2d::Puzzle2dExtension;
use reasoning_mindmap_wires::{DefaultWiresExtension, RelationshipKind};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App,
    Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const WIRES_PLAY_APP_ID: &str = "reasoning-wires-play";
const WIRES_PLAY_CONTROLLER_ID: &str = "reasoning-wires-play";
const WIRES_PLAY_SURFACE_ID: &str = "reasoning.wires.composite";
const WIRES_PLAY_BODY_COMPOSITE: &str = "reasoning.wires.composite";
const WIRES_PLAY_BODY_HIERARCHY: &str = "reasoning.wires.hierarchy";
const WIRES_PLAY_BODY_CATALOGUE: &str = "reasoning.wires.catalogue";
const WIRES_PLAY_BODY_PROPERTIES: &str = "reasoning.wires.properties";
const WIRES_FIXTURE_SCHEMA: &str = "reasoning.wires.fixture";
const PUZZLE2D_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = "metabolism";
const METABOLISM_WIRES_EXAMPLE_JSON: &str = include_str!("../../example/metabolism.wires.json");

const WIRES_HIERARCHY_IDENTITY_PREFIX: &str = "wires-play-hierarchy.identity.";
const WIRES_HIERARCHY_RELATIONSHIP_PREFIX: &str = "wires-play-hierarchy.relationship.";
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReasoningWiresPlayEnvelope {
    wires_fixture: Value,
    board_fixture: Value,
    #[serde(default)]
    selected_ids: Vec<String>,
}

fn default_empty_wires_fixture() -> Value {
    json!({
        "schema": WIRES_FIXTURE_SCHEMA,
        "identities": [],
        "relationships": [],
        "board": default_empty_board_fixture()
    })
}

fn default_empty_board_fixture() -> Value {
    json!({
        "schema": PUZZLE2D_FIXTURE_SCHEMA,
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [],
        "edges": [],
        "wires": []
    })
}

fn default_envelope() -> ReasoningWiresPlayEnvelope {
    ReasoningWiresPlayEnvelope {
        wires_fixture: default_empty_wires_fixture(),
        board_fixture: default_empty_board_fixture(),
        selected_ids: Vec::new(),
    }
}

fn parse_envelope(document_json: &str) -> ReasoningWiresPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &ReasoningWiresPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn wires_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: WIRES_PLAY_CONTROLLER_ID.into(),
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

fn wires_identities(wires: &Value) -> &[Value] {
    wires
        .get("identities")
        .and_then(|value| value.as_array())
        .map(|values| values.as_slice())
        .unwrap_or(&[])
}

fn wires_relationships(wires: &Value) -> &[Value] {
    wires
        .get("relationships")
        .and_then(|value| value.as_array())
        .map(|values| values.as_slice())
        .unwrap_or(&[])
}

fn wires_fixture_board(wires: &Value) -> Value {
    let mut board = wires.get("board").cloned().unwrap_or_else(default_empty_board_fixture);
    if let Some(obj) = board.as_object_mut() {
        obj.insert("schema".into(), json!(PUZZLE2D_FIXTURE_SCHEMA));
        if !obj.contains_key("wires") {
            obj.insert("wires".into(), json!([]));
        }
        if let Some(nodes) = obj.get_mut("nodes").and_then(|value| value.as_array_mut()) {
            for node in nodes {
                if let Some(node_obj) = node.as_object_mut() {
                    if !node_obj.contains_key("handles") {
                        node_obj.insert("handles".to_string(), json!([]));
                    }
                }
            }
        }
    }
    board
}

fn envelope_from_wires_fixture(wires: Value) -> ReasoningWiresPlayEnvelope {
    ReasoningWiresPlayEnvelope {
        board_fixture: wires_fixture_board(&wires),
        wires_fixture: wires,
        selected_ids: Vec::new(),
    }
}

fn identity_label(wires: &Value, identity_id: u64) -> Option<String> {
    wires_identities(wires)
        .iter()
        .find(|identity| identity.get("identityId").and_then(|value| value.as_u64()) == Some(identity_id))
        .and_then(|identity| identity.get("label").and_then(|value| value.as_str()))
        .map(str::to_string)
}

fn relationship_kind_display_name(kind: &str) -> &str {
    match kind {
        "owns" => "Owns",
        "is" => "Is",
        "references" => "References",
        "has" => "Has",
        _ => kind,
    }
}

fn wires_relationship_hierarchy_label(wires: &Value, edge_id: &str) -> Option<String> {
    let relationship = wires_relationships(wires).iter().find(|row| {
        row.get("edgeId").and_then(|value| value.as_str()) == Some(edge_id)
    })?;
    let kind = relationship.get("kind")?.as_str()?;
    let source_id = relationship.get("sourceIdentityId")?.as_u64()?;
    let target_id = relationship.get("targetIdentityId")?.as_u64()?;
    let source = identity_label(wires, source_id)?;
    let target = identity_label(wires, target_id)?;
    Some(format!(
        "{}: {source} → {target}",
        relationship_kind_display_name(kind)
    ))
}

fn wires_identity_kind_name(wires: &Value, identity_kind_id: &str) -> Option<String> {
    wires
        .get("kindCatalogs")
        .and_then(|value| value.get("identityKinds"))
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .chain(
            wires
                .get("board")
                .and_then(|value| value.get("meta"))
                .and_then(|value| value.get("kindCatalogs"))
                .and_then(|value| value.get("identityKinds"))
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten(),
        )
        .find(|row| row.get("id").and_then(|value| value.as_str()) == Some(identity_kind_id))
        .and_then(|row| row.get("name").and_then(|value| value.as_str()))
        .map(str::to_string)
}

fn wires_kind_catalog_entries(wires: &Value, key: &str) -> Vec<Value> {
    wires
        .get("kindCatalogs")
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_array())
        .cloned()
        .or_else(|| {
            wires
                .get("board")
                .and_then(|value| value.get("meta"))
                .and_then(|value| value.get("kindCatalogs"))
                .and_then(|value| value.get(key))
                .and_then(|value| value.as_array())
                .cloned()
        })
        .unwrap_or_default()
}

fn hierarchy_tree_selected_ids(board: &Value, selected: &[String]) -> Vec<String> {
    selected
        .iter()
        .filter_map(|id| {
            if fixture_nodes(board)
                .iter()
                .any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
            {
                return Some(format!("{WIRES_HIERARCHY_IDENTITY_PREFIX}{id}"));
            }
            if fixture_edges(board)
                .iter()
                .any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
            {
                return Some(format!("{WIRES_HIERARCHY_RELATIONSHIP_PREFIX}{id}"));
            }
            None
        })
        .collect()
}
//#endregion 🔖Envelope

//#region 🔖Canvas
fn relationship_edge_layers(wires: &Value, board: &Value) -> Vec<Value> {
    let mut layers = Vec::new();
    for relationship in wires_relationships(wires) {
        let edge_id = relationship.get("edgeId").and_then(|value| value.as_str()).unwrap_or("");
        if edge_id.is_empty() {
            continue;
        }
        let edge = fixture_edges(board).iter().find(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(edge_id));
        if let Some(edge) = edge {
            layers.push(edge.clone());
        } else {
            layers.push(json!({
                "id": edge_id,
                "kind": "edge",
                "edgeKind": relationship.get("kind").cloned().unwrap_or_else(|| json!("relationship")),
                "source": relationship.get("sourceIdentityId").map(|value| value.to_string()).unwrap_or_default(),
                "target": relationship.get("targetIdentityId").map(|value| value.to_string()).unwrap_or_default(),
            }));
        }
    }
    layers
}

fn render_canvas(board: &Value, wires: &Value) -> UiNode {
    let (camera_x, camera_y, zoom) = fixture_camera(board);
    let mut layers: Vec<Value> = fixture_nodes(board).iter().cloned().collect();
    layers.extend(fixture_edges(board).iter().cloned());
    layers.extend(relationship_edge_layers(wires, board));
    build_canvas_2d_scene(
        WIRES_PLAY_SURFACE_ID,
        WIRES_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x,
            camera_y,
            zoom,
            layers_json: serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()),
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
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn render_hierarchy_panel(envelope: &ReasoningWiresPlayEnvelope) -> UiNode {
    let wires = &envelope.wires_fixture;
    let board = &envelope.board_fixture;
    let identity_items: Vec<UiTreeItemNode> = wires_identities(wires)
        .iter()
        .filter_map(|identity| {
            let node_id = identity.get("nodeId")?.as_str()?;
            let label = identity.get("label")?.as_str()?;
            let identity_kind = identity.get("identityKind").and_then(|value| value.as_str());
            let description = identity_kind
                .and_then(|kind| wires_identity_kind_name(wires, kind))
                .filter(|kind_name| kind_name != label);
            Some(tree_item_with_command(
                format!("{WIRES_HIERARCHY_IDENTITY_PREFIX}{node_id}"),
                label,
                description,
                wires_cmd("setSelection", Some(json!({ "ids": [node_id] }))),
            ))
        })
        .collect();
    let relationship_items: Vec<UiTreeItemNode> = fixture_edges(board)
        .iter()
        .filter_map(|edge| {
            let edge_id = edge.get("id")?.as_str()?;
            Some(tree_item_with_command(
                format!("{WIRES_HIERARCHY_RELATIONSHIP_PREFIX}{edge_id}"),
                wires_relationship_hierarchy_label(wires, edge_id).unwrap_or_else(|| edge_id.into()),
                None,
                wires_cmd("setSelection", Some(json!({ "ids": [edge_id] }))),
            ))
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "wires-play-hierarchy.identities".into(),
                label: Some("Identities".into()),
                default_open: Some(true),
                items: if identity_items.is_empty() {
                    vec![UiTreeItemNode {
                        id: "wires-play-hierarchy.identities.empty".into(),
                        label: "(none)".into(),
                        description: None,
                        icon_id: None,
                        selected: None,
                        default_open: None,
                        command: None,
                        hover_command: None,
                        unhover_command: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    }]
                } else {
                    identity_items
                },
            },
            UiTreeSectionNode {
                id: "wires-play-hierarchy.relationships".into(),
                label: Some("Relationships".into()),
                default_open: Some(false),
                items: if relationship_items.is_empty() {
                    vec![UiTreeItemNode {
                        id: "wires-play-hierarchy.relationships.empty".into(),
                        label: "(none)".into(),
                        description: None,
                        icon_id: None,
                        selected: None,
                        default_open: None,
                        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    }]
                } else {
                    relationship_items
                },
            },
        ],
        selected_ids: Some(hierarchy_tree_selected_ids(board, &envelope.selected_ids)),
        highlighted_ids: None,
        selection_change: Some(wires_cmd("setSelection", None)),
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

fn kind_catalog_section(section_id: &str, label: &str, entries: &[Value]) -> UiTreeSectionNode {
    let items: Vec<UiTreeItemNode> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            let command = match section_id {
                "wires-play-kinds.identity-kinds" => wires_cmd("addNode", Some(json!({ "kind": kind_id }))),
                "wires-play-kinds.relationship-kinds" => {
                    wires_cmd("addRelationship", Some(json!({ "kind": kind_id })))
                }
                _ => wires_cmd("addNode", Some(json!({ "kind": kind_id }))),
            };
            UiTreeItemNode {
                id: format!("{section_id}.{index}.{kind_id}"),
                label: catalog_kind_label(entry),
                description: Some(kind_id.into()),
                icon_id: None,
                selected: None,
                default_open: None,
                command: Some(command),
                hover_command: None,
                unhover_command: None,
                actions: None,
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
        hover_command: None,
        unhover_command: None,
        actions: None,
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

fn render_catalogue_panel(wires: &Value) -> UiNode {
    let identity_entries = wires_kind_catalog_entries(wires, "identityKinds");
    let relationship_entries = wires_kind_catalog_entries(wires, "relationshipKinds");
    UiNode::Tree(UiTreeNode {
        sections: vec![
            kind_catalog_section("wires-play-kinds.identity-kinds", "Identity kinds", &identity_entries),
            kind_catalog_section("wires-play-kinds.relationship-kinds", "Relationship kinds", &relationship_entries),
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}
//#endregion 🔖CataloguePanel

//#region 🔖InspectorPanel
fn render_properties_panel(envelope: &ReasoningWiresPlayEnvelope) -> UiNode {
    let selected_nodes: Vec<&Value> = envelope
        .selected_ids
        .iter()
        .filter_map(|id| {
            fixture_nodes(&envelope.board_fixture)
                .iter()
                .find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
        })
        .collect();
    if selected_nodes.is_empty() {
        let extension = DefaultWiresExtension::from_fixture_json(&envelope.wires_fixture.to_string()).ok();
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {WIRES_FIXTURE_SCHEMA}")),
            ui_text(format!(
                "Identities: {}",
                extension.as_ref().map(|ext| ext.mindmap.topics.len()).unwrap_or(0)
            )),
            ui_text(format!(
                "Relationships: {}",
                extension.as_ref().map(|ext| ext.relationships.len()).unwrap_or(0)
            )),
            ui_text(format!("Board nodes: {}", fixture_nodes(&envelope.board_fixture).len())),
        ]);
    }
    let node = selected_nodes[0];
    let identity = wires_identities(&envelope.wires_fixture)
        .iter()
        .find(|identity| identity.get("nodeId").and_then(|value| value.as_str()) == node.get("id").and_then(|value| value.as_str()));
    ui_stack_vertical(vec![
        ui_inspector_readonly_field(
            "wires-play-inspector.id",
            "Id",
            node.get("id")
                .and_then(|value| value.as_str())
                .unwrap_or("")
                .to_string(),
        ),
        ui_inspector_readonly_field(
            "wires-play-inspector.identity-label",
            "Identity",
            identity
                .and_then(|row| row.get("label"))
                .and_then(|value| value.as_str())
                .unwrap_or("—")
                .to_string(),
        ),
        ui_inspector_readonly_field(
            "wires-play-inspector.node-kind",
            "Identity Kind",
            node.get("nodeKind")
                .and_then(|value| value.as_str())
                .unwrap_or("—")
                .to_string(),
        ),
        ui_inspector_readonly_field(
            "wires-play-inspector.x",
            "X",
            node.get("x")
                .and_then(|value| value.as_f64())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".into()),
        ),
        ui_inspector_readonly_field(
            "wires-play-inspector.y",
            "Y",
            node.get("y")
                .and_then(|value| value.as_f64())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".into()),
        ),
    ])
}
//#endregion 🔖InspectorPanel

//#region 🔖WiresPlayApp
struct WiresPlayApp;

impl PluginApp for WiresPlayApp {
    fn app_id(&self) -> &str {
        WIRES_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("wires envelope json")
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
                envelope.selected_ids = selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    default_envelope()
                } else if example_id == WIRES_PLAY_EXAMPLE_METABOLISM_ID {
                    envelope_from_wires_fixture(
                        serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_wires_fixture()),
                    )
                } else {
                    default_envelope()
                };
                return vec![set_document_op(&envelope)];
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("identity");
                let id = format!("node-{}", envelope.board_fixture.get("nodes").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0) + 1);
                if let Some(nodes) = envelope.board_fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
                    nodes.push(json!({
                        "id": id,
                        "nodeKind": kind,
                        "shape": "circle",
                        "x": 0.0,
                        "y": 0.0,
                        "radius": 24.0,
                        "text": id,
                        "handles": []
                    }));
                }
                envelope.selected_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "addRelationship" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("owns");
                let edge_id = format!("edge-{}", fixture_edges(&envelope.board_fixture).len() + 1);
                if let Some(edges) = envelope.board_fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                    edges.push(json!({
                        "id": edge_id,
                        "edgeKind": format!("wires.{kind}"),
                        "source": "node-1",
                        "target": "node-2"
                    }));
                }
                if let Some(relationships) = envelope.wires_fixture.get_mut("relationships").and_then(|value| value.as_array_mut()) {
                    relationships.push(json!({
                        "edgeId": edge_id,
                        "kind": kind,
                        "sourceIdentityId": 1,
                        "targetIdentityId": 2
                    }));
                }
                envelope.selected_ids = vec![edge_id];
                return vec![set_document_op(&envelope)];
            }
            "deleteSelection" => {
                let selected: std::collections::HashSet<&str> = envelope.selected_ids.iter().map(String::as_str).collect();
                if let Some(nodes) = envelope.board_fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
                    nodes.retain(|node| {
                        node.get("id")
                            .and_then(|value| value.as_str())
                            .is_none_or(|id| !selected.contains(id))
                    });
                }
                if let Some(edges) = envelope.board_fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                    edges.retain(|edge| {
                        edge.get("id")
                            .and_then(|value| value.as_str())
                            .is_none_or(|id| !selected.contains(id))
                    });
                }
                envelope.selected_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            WIRES_PLAY_BODY_COMPOSITE => render_canvas(&envelope.board_fixture, &envelope.wires_fixture),
            WIRES_PLAY_BODY_HIERARCHY => render_hierarchy_panel(&envelope),
            WIRES_PLAY_BODY_CATALOGUE => render_catalogue_panel(&envelope.wires_fixture),
            WIRES_PLAY_BODY_PROPERTIES => render_properties_panel(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖WiresPlayApp

//#region 🔖AppFactory
fn create_wires_app() -> App {
    App::from_builder(
        App::builder(WIRES_PLAY_APP_ID, "Mindmap Wires")
            .icon_id("reasoning-wires")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind("reasoning-wires-composite", "Canvas", WIRES_PLAY_BODY_COMPOSITE)
            .panel_tab("framework.panel.hierarchy", FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, "workbench", WIRES_PLAY_BODY_HIERARCHY)
            .panel_tab("framework.panel.catalogue", FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "workbench", WIRES_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "details", WIRES_PLAY_BODY_PROPERTIES)
            .default_layout(create_default_layout(
                &["reasoning-wires-composite".into()],
                "row",
                Some(&[100.0]),
                Some(&["Canvas".into()]),
            )),
    )
    .example("empty", "Empty", serde_json::to_string(&default_envelope()).unwrap())
    .example(
        WIRES_PLAY_EXAMPLE_METABOLISM_ID,
        "Metabolism",
        serde_json::to_string(&envelope_from_wires_fixture(
            serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_wires_fixture()),
        ))
        .unwrap(),
    )
    .program("reasoning-wires", "Mindmap Wires", "graph")
}

fn wires_bundle() -> PluginBundle {
    PluginBundle::new("reasoning-wires", "Mindmap Wires", "0.1.0").register_app(create_wires_app(), || Box::new(WiresPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(wires_bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_canvas_scene() {
        let app = WiresPlayApp;
        let document = app.initial_document_json();
        let node = app.render(WIRES_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn hierarchy_has_identities_section() {
        let app = WiresPlayApp;
        let envelope = envelope_from_wires_fixture(
            serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).expect("metabolism fixture"),
        );
        let document = serde_json::to_string(&envelope).unwrap();
        let node = app.render(WIRES_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("wires-play-hierarchy.identities"));
        assert!(json.contains("Metabolism"));
    }

    #[test]
    fn metabolism_fixture_hydrates_extension() {
        let ext = DefaultWiresExtension::from_fixture_json(METABOLISM_WIRES_EXAMPLE_JSON).expect("metabolism fixture");
        assert_eq!(ext.mindmap.topics.len(), 7);
        assert_eq!(ext.relationships.len(), 9);
    }

    #[test]
    fn relationship_kind_labels_match_fixture() {
        assert_eq!(RelationshipKind::Owns.label(), "owns");
        assert_eq!(relationship_kind_display_name("is"), "Is");
    }

    #[test]
    fn wires_fixture_board_uses_puzzle_schema() {
        let _extension = Puzzle2dExtension;
        let wires: Value = serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).unwrap();
        let board = wires_fixture_board(&wires);
        assert_eq!(
            board.get("schema").and_then(|value| value.as_str()),
            Some(PUZZLE2D_FIXTURE_SCHEMA)
        );
        assert_eq!(fixture_nodes(&board).len(), 7);
    }
}
//#endregion 🧪Tests
