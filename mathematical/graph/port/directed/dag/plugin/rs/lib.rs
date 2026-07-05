//! 🔀 DAG plugin — declarative DAG play app bundled as a hot-swappable WASM component.

use mathematical_graph_port_directed_dag::{
    dag_fixture_to_wire_literal, dag_node_kind_tag, fit_node_size, note_widget_size, preview_widget_size,
    stepper_widget_height, stepper_widget_width, would_create_cycle, DagCamera, DagFixture, DagFixtureEdge, DagHost,
    DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, DagStepperField, IoPortSpec,
};
use semio_framework_plugin::{
    build_node_graph_scene, build_text_editor_scene, create_default_layout, ui_declarative_sections_to_tree,
    ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field,
    ui_text, App, CommandDescriptor, NodeGraphScene, PluginApp, PluginBundle, TextEditorScene, UiControlNode,
    UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::sync::LazyLock;

//#region 🔖Constants
const DAG_PLAY_APP_ID: &str = "dag-play";
const DAG_PLAY_SURFACE_MAIN: &str = "dag.play.main";
const DAG_PLAY_SURFACE_COMPILED: &str = "dag.play.compiled-dag";
const DAG_PLAY_BODY_MAIN: &str = "dag.play.main";
const DAG_PLAY_BODY_COMPILED: &str = "dag.play.compiled-dag";
const DAG_PLAY_BODY_HIERARCHY: &str = "dag.play.hierarchy";
const DAG_PLAY_BODY_CATALOGUE: &str = "dag.play.catalogue";
const DAG_PLAY_BODY_INSPECTOR: &str = "dag.play.inspection";
const DAG_PLAY_WINDOW_MAIN: &str = "dag-main";
const DAG_PLAY_WINDOW_COMPILED: &str = "dag-compiled-dag";
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DagPlayRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default)]
    undo_fixtures: Vec<DagFixture>,
    #[serde(default)]
    redo_fixtures: Vec<DagFixture>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DagPlayEnvelope {
    fixture: DagFixture,
    #[serde(default)]
    runtime: DagPlayRuntime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<MediaGraphPortRecord>,
    outputs: Vec<MediaGraphPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_envelope() -> DagPlayEnvelope {
    DagPlayEnvelope {
        fixture: DagFixture::default(),
        runtime: DagPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> DagPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &DagPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn dag_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: DAG_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn snapshot_dag(runtime: &mut DagPlayRuntime, fixture: &DagFixture) {
    runtime.undo_fixtures.push(fixture.clone());
    runtime.redo_fixtures.clear();
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<MediaGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| MediaGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node
                .inputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
            outputs: node
                .outputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
        })
        .collect();
    let edges: Vec<MediaGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            MediaGraphEdgeRecord {
                id: edge.id.clone(),
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
            }
        })
        .collect();
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
    )
}

fn next_node_id(fixture: &DagFixture) -> String {
    let max = fixture
        .nodes
        .iter()
        .filter_map(|node| node.id.strip_prefix('n').and_then(|suffix| suffix.parse::<u64>().ok()))
        .max()
        .unwrap_or(0);
    format!("n{}", max + 1)
}

fn default_node_for_kind(kind: &str, id: &str, x: f64, y: f64) -> DagNodeSpec {
    let mut node = match kind {
        "slider" => DagNodeSpec {
            id: id.into(),
            name: "Slider".into(),
            abbreviation: "Sld".into(),
            icon: "emoji:🎚️".into(),
            x,
            y,
            kind: DagNodeKind::Slider {
                min: 0.0,
                max: 10.0,
                step: 0.1,
                value: 3.0,
                output: IoPortSpec::named("N", "Num", "number", "Number"),
            },
            ..Default::default()
        },
        "select" => DagNodeSpec {
            id: id.into(),
            name: "Select".into(),
            abbreviation: "Sel".into(),
            icon: "emoji:📋".into(),
            x,
            y,
            kind: DagNodeKind::Select {
                options: vec!["A".into(), "B".into(), "C".into()],
                selected: 0,
                output: IoPortSpec::named("V", "Val", "value", "Value"),
            },
            ..Default::default()
        },
        "screen" => DagNodeSpec {
            id: id.into(),
            name: "Screen".into(),
            abbreviation: "Scr".into(),
            icon: "emoji:🖥️".into(),
            x,
            y,
            kind: DagNodeKind::Screen {
                media: None,
                input: IoPortSpec::named("I", "In", "in", "Input"),
            },
            ..Default::default()
        },
        "note" => {
            let text = String::new();
            let (width, height) = note_widget_size(&text);
            DagNodeSpec {
                id: id.into(),
                name: "Note".into(),
                abbreviation: "Note".into(),
                icon: "emoji:📝".into(),
                x,
                y,
                width,
                height,
                kind: DagNodeKind::Note {
                    text,
                    output: IoPortSpec::named("T", "Txt", "text", "Text"),
                },
                ..Default::default()
            }
        }
        "preview" => {
            let (width, height) = preview_widget_size(&DagPreviewContent::Scalar { text: String::new() }, &BTreeSet::new());
            DagNodeSpec {
                id: id.into(),
                name: "Preview".into(),
                abbreviation: "Prv".into(),
                icon: "emoji:👁️".into(),
                x,
                y,
                width,
                height,
                kind: DagNodeKind::Preview {
                    content: DagPreviewContent::Scalar { text: String::new() },
                    expanded: BTreeSet::new(),
                    input: IoPortSpec::named("I", "In", "in", "Input"),
                },
                ..Default::default()
            }
        }
        "stepper" => {
            let fields = vec![DagStepperField {
                key: "value".into(),
                label: "Value".into(),
                value: 0.0,
                step: 1.0,
            }];
            DagNodeSpec {
                id: id.into(),
                name: "Stepper".into(),
                abbreviation: "Stp".into(),
                icon: "emoji:🎚️".into(),
                x,
                y,
                width: stepper_widget_width(),
                height: stepper_widget_height(fields.len()),
                kind: DagNodeKind::Stepper {
                    fields,
                    output: IoPortSpec::named("N", "Num", "number", "Number"),
                },
                ..Default::default()
            }
        }
        _ => DagNodeSpec {
            id: id.into(),
            name: "Computation".into(),
            abbreviation: "Cmp".into(),
            icon: "emoji:⚙️".into(),
            x,
            y,
            operator_kind: Some("math.add".into()),
            kind: DagNodeKind::Computation {
                inputs: vec![IoPortSpec::named("A", "A", "a", "A"), IoPortSpec::named("B", "B", "b", "B")],
                outputs: vec![IoPortSpec::named("R", "R", "result", "Result")],
                variadic_inputs: false,
                variadic_outputs: false,
            },
            ..Default::default()
        },
    };
    fit_node_size(&mut node);
    node
}

fn connect_ports(fixture: &mut DagFixture, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<(), String> {
    let existing: Vec<(String, String)> = fixture
        .edges
        .iter()
        .filter_map(|edge| {
            let (from, _) = split_endpoint(&edge.source);
            let (to, _) = split_endpoint(&edge.target);
            Some((from, to))
        })
        .collect();
    if would_create_cycle(&existing, source_node_id, target_node_id) {
        return Err("connection would create cycle".into());
    }
    let edge_id = format!(
        "e{}",
        fixture
            .edges
            .iter()
            .filter_map(|edge| edge.id.strip_prefix('e').and_then(|suffix| suffix.parse::<u64>().ok()))
            .max()
            .unwrap_or(0)
            + 1
    );
    fixture.edges.push(DagFixtureEdge {
        id: edge_id,
        source: format!("{source_node_id}:{source_port_id}"),
        target: format!("{target_node_id}:{target_port_id}"),
        ..Default::default()
    });
    Ok(())
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
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
    }
}

fn tree_item_with_description(id: impl Into<String>, label: impl Into<String>, description: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: Some(description.into()),
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
    }
}

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
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn build_hierarchy_tree(fixture: &DagFixture, selected: &[String]) -> UiNode {
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_command(
                format!("dag-play-hierarchy.node.{}", node.id),
                if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                Some(dag_node_kind_tag(&node.kind).into()),
                dag_cmd("setSelection", Some(json!({ "ids": [node.id.clone()] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture
        .edges
        .iter()
        .map(|edge| {
            tree_item_with_description(
                format!("dag-play-hierarchy.edge.{}", edge.id),
                format!("{} → {}", edge.source, edge.target),
                edge.id.clone(),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "dag-play-hierarchy.nodes".into(),
                label: Some("Nodes".into()),
                default_open: Some(true),
                items: if node_items.is_empty() {
                    vec![tree_item("dag-play-hierarchy.nodes.empty", "(none)")]
                } else {
                    node_items
                },
            },
            UiTreeSectionNode {
                id: "dag-play-hierarchy.edges".into(),
                label: Some("Edges".into()),
                default_open: Some(false),
                items: if edge_items.is_empty() {
                    vec![tree_item("dag-play-hierarchy.edges.empty", "(none)")]
                } else {
                    edge_items
                },
            },
        ],
        selected_ids: Some(selected.iter().map(|id| format!("dag-play-hierarchy.node.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let kinds = [
        ("computation", "Computation"),
        ("slider", "Slider"),
        ("stepper", "Stepper"),
        ("select", "Select"),
        ("note", "Note"),
        ("preview", "Preview"),
        ("screen", "Screen"),
    ];
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "dag-play-catalogue.node-kinds".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items: kinds
                .iter()
                .map(|(kind, label)| {
                    tree_item_with_command(
                        format!("dag-play-catalogue.kind.{kind}"),
                        *label,
                        Some((*kind).into()),
                        dag_cmd("addNode", Some(json!({ "kind": kind }))),
                    )
                })
                .collect(),
        }],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn inspector_number_field(node_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
            commit: None,
            on_change: dag_cmd("patchDagNodes", Some(json!({ "nodeIds": node_ids, "field": field }))),
        }),
    })
}

fn inspector_text_field(node_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: Some("blur".into()),
            on_change: dag_cmd("patchDagNodes", Some(json!({ "nodeIds": node_ids, "field": field }))),
        }),
    })
}

fn build_inspector_tree(fixture: &DagFixture, selected: &[String]) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Select a node in the hierarchy.")],
        }]);
    }
    let nodes: Vec<&DagNodeSpec> = selected
        .iter()
        .filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id))
        .collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Node not found")],
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if nodes.iter().all(|node| matches!(node.kind, DagNodeKind::Slider { .. })) {
        groups.push(UiInspectorFieldGroup {
            id: "dag-play-inspector.kind.slider".into(),
            label: "slider".into(),
            default_open: None,
            fields: vec![
                inspector_number_field(&node_ids, "dag-play-inspector.slider-value", "Value", &nodes.iter().map(|node| match &node.kind { DagNodeKind::Slider { value, .. } => *value, _ => 0.0 }).collect::<Vec<_>>(), "value"),
                inspector_number_field(&node_ids, "dag-play-inspector.slider-min", "Min", &nodes.iter().map(|node| match &node.kind { DagNodeKind::Slider { min, .. } => *min, _ => 0.0 }).collect::<Vec<_>>(), "min"),
                inspector_number_field(&node_ids, "dag-play-inspector.slider-max", "Max", &nodes.iter().map(|node| match &node.kind { DagNodeKind::Slider { max, .. } => *max, _ => 0.0 }).collect::<Vec<_>>(), "max"),
            ],
        });
    }
    let mut base_fields = vec![
        inspector_text_field(&node_ids, "dag-play-inspector.name", "Name", &nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>(), "name"),
        ui_inspector_readonly_field(
            "dag-play-inspector.kind",
            "Kind",
            if nodes.iter().map(|node| dag_node_kind_tag(&node.kind)).collect::<std::collections::HashSet<_>>().len() == 1 {
                dag_node_kind_tag(&nodes[0].kind).to_string()
            } else {
                "—".into()
            },
        ),
    ];
    if node_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                id: "dag-play-inspector.id".into(),
                label: "Id".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "dag-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: node_ids[0].clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: dag_cmd("renameDagNode", Some(json!({ "oldId": node_ids[0] }))),
                }),
            }),
        );
    } else {
        base_fields.insert(0, ui_inspector_readonly_field("dag-play-inspector.id", "Id", &format!("{} selected", node_ids.len())));
    }
    groups.push(UiInspectorFieldGroup {
        id: "dag-play-inspector.base".into(),
        label: "Node".into(),
        default_open: None,
        fields: base_fields,
    });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(envelope: &DagPlayEnvelope) -> UiNode {
    let fixture = &envelope.fixture;
    let (nodes_json, edges_json) = fixture_to_media_graph(fixture);
    let viewport_json = serde_json::to_string(&fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
    };
    build_node_graph_scene(
        DAG_PLAY_SURFACE_MAIN,
        DAG_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            selection_json,
            context_menu_json: Some(
                r#"[{"id":"delete-selection","label":"Delete selection","command":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
            ),
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_compiled_dag(fixture: &DagFixture) -> UiNode {
    build_text_editor_scene(
        DAG_PLAY_SURFACE_COMPILED,
        DAG_PLAY_APP_ID,
        TextEditorScene::base(dag_fixture_to_wire_literal(fixture), Some("wire".into()), None),
    )
}
//#endregion 🔖Render

//#region 🔖DagPlayApp
struct DagPlayApp;

impl PluginApp for DagPlayApp {
    fn app_id(&self) -> &str {
        DAG_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("dag envelope json")
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
                        envelope = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                let ids = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok())
                    .or_else(|| {
                        args.and_then(|value| value.get("ids").or_else(|| value.get("nodeId").map(|_| value.get("nodeId")).flatten()))
                            .and_then(|value| {
                                if value.is_array() {
                                    serde_json::from_value(value.clone()).ok()
                                } else if let Some(id) = value.as_str() {
                                    Some(vec![id.to_string()])
                                } else {
                                    None
                                }
                            })
                    })
                    .or_else(|| {
                        args.and_then(|value| value.get("nodeId"))
                            .and_then(|value| value.as_str())
                            .map(|id| vec![id.to_string()])
                    })
                    .unwrap_or_default();
                envelope.runtime.selected_node_ids = ids;
                return vec![set_document_op(&envelope)];
            }
            "nodeGraphHover" => return Vec::new(),
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<DagCamera>(viewport_json) {
                        envelope.fixture.camera = camera;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "nodeGraphEdit" => {
                let ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut changed = false;
                for op in ops {
                    match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Ok(fixture) = serde_json::from_str::<DagFixture>(fixture_json) {
                                    snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                                    envelope.fixture = fixture;
                                    changed = true;
                                }
                            }
                        }
                        "deleteSelection" => {
                            if !envelope.runtime.selected_node_ids.is_empty() {
                                snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                                let ids: Vec<String> = envelope.runtime.selected_node_ids.clone();
                                envelope.fixture.nodes.retain(|node| !ids.contains(&node.id));
                                envelope.fixture.edges.retain(|edge| {
                                    let (from, _) = split_endpoint(&edge.source);
                                    let (to, _) = split_endpoint(&edge.target);
                                    !ids.iter().any(|id| id == from || id == to)
                                });
                                envelope.runtime.selected_node_ids.clear();
                                changed = true;
                            }
                        }
                        "connect" => {
                            let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                            let to = op.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) =
                                (from, from_port, to, to_port)
                            {
                                snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                                if connect_ports(&mut envelope.fixture, from, from_port, to, to_port).is_ok() {
                                    changed = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    return vec![set_document_op(&envelope)];
                }
            }
            "deleteSelection" => {
                if !envelope.runtime.selected_node_ids.is_empty() {
                    snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                    let ids: Vec<String> = envelope.runtime.selected_node_ids.clone();
                    envelope.fixture.nodes.retain(|node| !ids.contains(&node.id));
                    envelope.fixture.edges.retain(|edge| {
                        let (from, _) = split_endpoint(&edge.source);
                        let (to, _) = split_endpoint(&edge.target);
                        !ids.iter().any(|id| id == from || id == to)
                    });
                    envelope.runtime.selected_node_ids.clear();
                    return vec![set_document_op(&envelope)];
                }
            }
            "graphPointerDown" => {
                envelope.runtime.selected_node_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "undo" => {
                if let Some(previous) = envelope.runtime.undo_fixtures.pop() {
                    envelope.runtime.redo_fixtures.push(envelope.fixture.clone());
                    envelope.fixture = previous;
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                if let Some(next) = envelope.runtime.redo_fixtures.pop() {
                    envelope.runtime.undo_fixtures.push(envelope.fixture.clone());
                    envelope.fixture = next;
                    return vec![set_document_op(&envelope)];
                }
            }
            "renameDagNode" => {
                let old_id = args.and_then(|value| value.get("oldId")).and_then(|value| value.as_str());
                let new_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                if let (Some(old_id), Some(new_id)) = (old_id, new_id) {
                    let trimmed = new_id.trim();
                    if !trimmed.is_empty()
                        && trimmed != old_id
                        && !envelope.fixture.nodes.iter().any(|node| node.id == trimmed)
                    {
                        snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                        for node in &mut envelope.fixture.nodes {
                            if node.id == old_id {
                                node.id = trimmed.into();
                            }
                        }
                        for edge in &mut envelope.fixture.edges {
                            let (from_node, from_port) = split_endpoint(&edge.source);
                            let (to_node, to_port) = split_endpoint(&edge.target);
                            if from_node == old_id {
                                edge.source = format!("{trimmed}:{from_port}");
                            }
                            if to_node == old_id {
                                edge.target = format!("{trimmed}:{to_port}");
                            }
                        }
                        envelope.runtime.selected_node_ids = vec![trimmed.into()];
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "removeNode" => {
                let node_id = args
                    .and_then(|value| value.get("nodeId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str());
                if let Some(node_id) = node_id {
                    snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                    envelope.fixture.nodes.retain(|node| node.id != node_id);
                    envelope.fixture.edges.retain(|edge| {
                        let (from, _) = split_endpoint(&edge.source);
                        let (to, _) = split_endpoint(&edge.target);
                        from != node_id && to != node_id
                    });
                    envelope.runtime.selected_node_ids.retain(|id| id != node_id);
                    return vec![set_document_op(&envelope)];
                }
            }
            "disconnect" => {
                let edge_id = args
                    .and_then(|value| value.get("edgeId"))
                    .or_else(|| args.and_then(|value| value.get("synapseId")))
                    .and_then(|value| value.as_str());
                if let Some(edge_id) = edge_id {
                    snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                    envelope.fixture.edges.retain(|edge| edge.id != edge_id);
                    return vec![set_document_op(&envelope)];
                }
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if let Ok(mut host) = DagHost::load_fixture_json(&serde_json::to_string(&envelope.fixture).unwrap()) {
                        let _ = host.set_widget_position(node_id, x, y);
                        if let Ok(json) = host.fixture_json() {
                            if let Ok(fixture) = serde_json::from_str(&json) {
                                envelope.fixture = fixture;
                                return vec![set_document_op(&envelope)];
                            }
                        }
                    }
                }
            }
            "connectMediaPorts" => {
                let source_node_id = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str());
                let source_port_id = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str());
                let target_node_id = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str());
                let target_port_id = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str());
                if let (Some(from), Some(from_port), Some(to), Some(to_port)) =
                    (source_node_id, source_port_id, target_node_id, target_port_id)
                {
                    snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                    if connect_ports(&mut envelope.fixture, from, from_port, to, to_port).is_ok() {
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("computation");
                let id = next_node_id(&envelope.fixture);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                envelope.fixture.nodes.push(default_node_for_kind(kind, &id, x, y));
                envelope.runtime.selected_node_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "reorganize" => {
                snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                if let Ok(mut host) = DagHost::load_fixture_json(&serde_json::to_string(&envelope.fixture).unwrap()) {
                    let _ = host.reorganize(&DagLayoutOptions::default());
                    if let Ok(json) = host.fixture_json() {
                        if let Ok(fixture) = serde_json::from_str(&json) {
                            envelope.fixture = fixture;
                            return vec![set_document_op(&envelope)];
                        }
                    }
                }
            }
            "patchDagNodes" => {
                snapshot_dag(&mut envelope.runtime, &envelope.fixture);
                let node_ids: Vec<String> = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value"));
                for node in envelope.fixture.nodes.iter_mut() {
                    if !node_ids.contains(&node.id) {
                        continue;
                    }
                    match field {
                        "name" => {
                            if let Some(value) = raw_value.and_then(|value| value.as_str()) {
                                node.name = value.into();
                            }
                        }
                        "value" if matches!(node.kind, DagNodeKind::Slider { .. }) => {
                            if let (Some(value), DagNodeKind::Slider { value: ref mut slider_value, .. }) =
                                (raw_value.and_then(|value| value.as_f64()), &mut node.kind)
                            {
                                *slider_value = value;
                            }
                        }
                        "min" if matches!(node.kind, DagNodeKind::Slider { .. }) => {
                            if let (Some(value), DagNodeKind::Slider { min: ref mut min_value, .. }) =
                                (raw_value.and_then(|value| value.as_f64()), &mut node.kind)
                            {
                                *min_value = value;
                            }
                        }
                        "max" if matches!(node.kind, DagNodeKind::Slider { .. }) => {
                            if let (Some(value), DagNodeKind::Slider { max: ref mut max_value, .. }) =
                                (raw_value.and_then(|value| value.as_f64()), &mut node.kind)
                            {
                                *max_value = value;
                            }
                        }
                        _ => {}
                    }
                    fit_node_size(node);
                }
                return vec![set_document_op(&envelope)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            DAG_PLAY_BODY_MAIN => render_main_graph(&envelope),
            DAG_PLAY_BODY_COMPILED => render_compiled_dag(&envelope.fixture),
            DAG_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope.fixture, &envelope.runtime.selected_node_ids),
            DAG_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            DAG_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_node_ids),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖DagPlayApp

//#region 🔖Manifest
fn create_dag_app() -> App {
    App::from_builder(
        App::builder(DAG_PLAY_APP_ID, "DAG").hierarchy(["semio", "mathematical", "graph", "port", "directed", "dag"])
            .icon_id("dag")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(DAG_PLAY_WINDOW_MAIN, "DAG", DAG_PLAY_BODY_MAIN)
            .window_kind(DAG_PLAY_WINDOW_COMPILED, "DSL", DAG_PLAY_BODY_COMPILED)
            .default_layout(create_default_layout(
                &[DAG_PLAY_WINDOW_MAIN.into(), DAG_PLAY_WINDOW_COMPILED.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["DAG".into(), "DSL".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                DAG_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                DAG_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                DAG_PLAY_BODY_INSPECTOR,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_envelope()).unwrap())
    .program("dag", "DAG", "graph")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("dag", "DAG", "0.1.0").register_app(create_dag_app(), || Box::new(DagPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let app = DagPlayApp;
        let document = serde_json::to_string(&default_envelope()).unwrap();
        let node = app.render(DAG_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_compiled_dag_text_editor() {
        let app = DagPlayApp;
        let document = serde_json::to_string(&default_envelope()).unwrap();
        let node = app.render(DAG_PLAY_BODY_COMPILED, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn add_node_command_updates_fixture() {
        let mut app = DagPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("addNode", Some(&json!({ "kind": "slider" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated_op: Value = serde_json::from_str(&ops[0]).unwrap();
        let updated: DagPlayEnvelope = serde_json::from_value(updated_op["document"].clone()).unwrap();
        assert!(updated.fixture.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })));
    }

    #[test]
    fn inspector_shows_selected_node() {
        let mut envelope = default_envelope();
        let node_id = envelope.fixture.nodes.first().map(|node| node.id.clone()).unwrap_or_else(|| "n1".into());
        envelope.runtime.selected_node_ids = vec![node_id];
        let app = DagPlayApp;
        let node = app.render(DAG_PLAY_BODY_INSPECTOR, &serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("field"));
    }

    #[test]
    fn rename_dag_node_updates_fixture() {
        let mut app = DagPlayApp;
        let document = app.initial_document_json();
        let envelope: DagPlayEnvelope = serde_json::from_str(&document).unwrap();
        let old_id = envelope.fixture.nodes.first().map(|node| node.id.clone()).expect("node");
        let ops = app.handle_command(
            "renameDagNode",
            Some(&json!({ "oldId": old_id, "value": "renamed-node" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let updated: DagPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert!(updated.fixture.nodes.iter().any(|node| node.id == "renamed-node"));
    }

    #[test]
    fn remove_node_deletes_from_fixture() {
        let mut app = DagPlayApp;
        let document = app.initial_document_json();
        let envelope: DagPlayEnvelope = serde_json::from_str(&document).unwrap();
        let node_id = envelope.fixture.nodes.first().map(|node| node.id.clone()).expect("node");
        let ops = app.handle_command("removeNode", Some(&json!({ "nodeId": node_id })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: DagPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert!(updated.fixture.nodes.iter().all(|node| node.id != node_id));
    }
}
