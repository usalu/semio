//! 🌊 Flow plugin — declarative flow play app bundled as a hot-swappable WASM component.

use flow_core::{dag::DagFixture, CameraJson, FlowFixture, FlowHost, Widget};
use semio_framework_plugin::{
    build_node_graph_scene, build_text_editor_scene, create_default_layout, ui_declarative_sections_to_tree,
    ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field,
    ui_text, App, CommandDescriptor, NodeGraphScene, PluginApp, PluginBundle, TextEditorScene, UiControlNode,
    UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID,     FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const FLOW_PLAY_APP_ID: &str = "flow-play";
const FLOW_PLAY_SURFACE_MAIN: &str = "flow.play.main";
const FLOW_PLAY_SURFACE_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_BODY_MAIN: &str = "flow.play.main";
const FLOW_PLAY_BODY_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_BODY_HIERARCHY: &str = "flow.play.hierarchy";
const FLOW_PLAY_BODY_CATALOGUE: &str = "flow.play.catalogue";
const FLOW_PLAY_BODY_INSPECTOR: &str = "flow.play.inspection";
const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";
const FLOW_PLAY_WINDOW_COMPILED: &str = "flow-compiled-dag";
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlowPlayRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default)]
    undo_fixtures: Vec<FlowFixture>,
    #[serde(default)]
    redo_fixtures: Vec<FlowFixture>,
    #[serde(default)]
    last_eval_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlowPlayEnvelope {
    fixture: FlowFixture,
    #[serde(default)]
    runtime: FlowPlayRuntime,
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
fn default_envelope() -> FlowPlayEnvelope {
    FlowPlayEnvelope {
        fixture: FlowFixture::default(),
        runtime: FlowPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> FlowPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &FlowPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn flow_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: FLOW_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn host_from_envelope(envelope: &FlowPlayEnvelope) -> FlowHost {
    FlowHost::from_fixture(envelope.fixture.clone())
}

fn snapshot_fixture(runtime: &mut FlowPlayRuntime, fixture: &FlowFixture) {
    runtime.undo_fixtures.push(fixture.clone());
    runtime.redo_fixtures.clear();
}

fn sync_host_selection(host: &mut FlowHost, selected: &[String]) {
    if selected.is_empty() {
        let _ = host.dag.cancel_area_select();
    } else {
        host.dag.set_selection(selected);
    }
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

fn widget_kind_label(widget: &Widget) -> &'static str {
    match widget {
        Widget::Neuron { .. } => "neuron",
        Widget::InputSlider { .. } => "inputSlider",
        Widget::InputStepper { .. } => "inputStepper",
        Widget::InputNote { .. } => "inputNote",
        Widget::InputImage { .. } => "inputImage",
        Widget::Variable { .. } => "variable",
        Widget::OutputPreview { .. } => "outputPreview",
        Widget::OutputAction { .. } => "outputAction",
        Widget::OutputExport { .. } => "outputExport",
        Widget::Cluster { .. } => "cluster",
    }
}

fn widget_tree_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { id, neuronKind, .. } => format!("{id} ({neuronKind})"),
        Widget::InputSlider { id, .. } => format!("{id} (slider)"),
        Widget::InputNote { id, .. } => format!("{id} (note)"),
        Widget::OutputPreview { id, .. } => format!("{id} (preview)"),
        Widget::Variable { id, name, .. } => format!("{id} ({name})"),
        widget => format!("{} ({})", widget_id(widget), widget_kind_label(widget)),
    }
}

fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputStepper { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
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
fn build_hierarchy_tree(fixture: &FlowFixture, selected: &[String]) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            tree_item_with_command(
                format!("flow-play-hierarchy.widget.{}", widget_id(widget)),
                widget_tree_label(widget),
                Some(widget_kind_label(widget).into()),
                flow_cmd("setSelection", Some(json!({ "ids": [widget_id(widget)] }))),
            )
        })
        .collect();
    let synapse_items: Vec<UiTreeItemNode> = fixture
        .synapses
        .iter()
        .map(|synapse| {
            UiTreeItemNode {
                id: format!("flow-play-hierarchy.synapse.{}", synapse.id),
                label: format!("{} → {}", synapse.from, synapse.to),
                description: Some(format!("{} → {}", synapse.from_port, synapse.to_port)),
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
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "flow-play-hierarchy.widgets".into(),
                label: Some("Widgets".into()),
                default_open: Some(true),
                items: if widget_items.is_empty() {
                    vec![tree_item("flow-play-hierarchy.widgets.empty", "(none)")]
                } else {
                    widget_items
                },
            },
            UiTreeSectionNode {
                id: "flow-play-hierarchy.synapses".into(),
                label: Some("Synapses".into()),
                default_open: Some(false),
                items: if synapse_items.is_empty() {
                    vec![tree_item("flow-play-hierarchy.synapses.empty", "(none)")]
                } else {
                    synapse_items
                },
            },
        ],
        selected_ids: Some(selected.iter().map(|id| format!("flow-play-hierarchy.widget.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree(envelope: &FlowPlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    let sections: Vec<Value> = host
        .catalogue_json()
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default();
    let tree_sections: Vec<UiTreeSectionNode> = sections
        .iter()
        .filter_map(|section| {
            let id = section.get("id")?.as_str()?.to_string();
            let title = section
                .get("title")
                .and_then(|value| value.as_str())
                .unwrap_or(&id)
                .to_string();
            let items: Vec<UiTreeItemNode> = section
                .get("items")
                .and_then(|value| value.as_array())
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| {
                            let kind = entry.get("kind")?.as_str()?;
                            let label = entry
                                .get("name")
                                .or_else(|| entry.get("abbreviation"))
                                .and_then(|value| value.as_str())
                                .unwrap_or(kind);
                            let command = if kind == "neuron" {
                                flow_cmd(
                                    "addWidget",
                                    Some(json!({
                                        "kind": "neuron",
                                        "neuronKind": entry.get("neuronKind").and_then(|value| value.as_str()).unwrap_or(kind),
                                    })),
                                )
                            } else {
                                flow_cmd("addWidget", Some(json!({ "kind": kind })))
                            };
                            Some(tree_item_with_command(
                                format!("flow-play-catalogue.{id}.{kind}.{label}"),
                                label,
                                Some(kind.to_string()),
                                command,
                            ))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Some(UiTreeSectionNode {
                id: format!("flow-play-catalogue.{id}"),
                label: Some(title),
                default_open: Some(true),
                items,
            })
        })
        .collect();
    if tree_sections.is_empty() {
        return build_catalogue_tree_fallback();
    }
    UiNode::Tree(UiTreeNode {
        sections: tree_sections,
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree_fallback() -> UiNode {
    let sources = [("inputSlider", "Slider"), ("inputStepper", "Stepper"), ("inputNote", "Note")];
    let components = [("math.add", "Add"), ("logic.and", "And"), ("text.concat", "Concat")];
    let sinks = [("outputPreview", "Preview"), ("outputExport", "Export")];
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "flow-play-catalogue.sources".into(),
                label: Some("Sources".into()),
                default_open: Some(true),
                items: sources
                    .iter()
                    .map(|(kind, label)| {
                        tree_item_with_command(
                            format!("flow-play-catalogue.source.{kind}"),
                            *label,
                            Some((*kind).into()),
                            flow_cmd("addWidget", Some(json!({ "kind": kind }))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "flow-play-catalogue.components".into(),
                label: Some("Components".into()),
                default_open: Some(true),
                items: components
                    .iter()
                    .map(|(kind, label)| {
                        tree_item_with_command(
                            format!("flow-play-catalogue.component.{kind}"),
                            *label,
                            Some((*kind).into()),
                            flow_cmd("addWidget", Some(json!({ "kind": "neuron", "neuronKind": kind }))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "flow-play-catalogue.sinks".into(),
                label: Some("Sinks".into()),
                default_open: Some(false),
                items: sinks
                    .iter()
                    .map(|(kind, label)| {
                        tree_item_with_command(
                            format!("flow-play-catalogue.sink.{kind}"),
                            *label,
                            Some((*kind).into()),
                            flow_cmd("addWidget", Some(json!({ "kind": kind }))),
                        )
                    })
                    .collect(),
            },
        ],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(fixture: &FlowFixture, selected: &[String]) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "flow-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Select a widget in the canvas or hierarchy.")],
        }]);
    }
    let widgets: Vec<&Widget> = selected
        .iter()
        .filter_map(|id| fixture.widgets.iter().find(|widget| widget_id(widget) == id))
        .collect();
    if widgets.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "flow-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Widget not found")],
        }]);
    }
    let widget_ids: Vec<String> = widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if widgets.iter().all(|widget| matches!(widget, Widget::InputSlider { .. })) {
        let mixed = ui_inspector_mixed_number(&widgets.iter().map(|widget| match widget { Widget::InputSlider { value, .. } => *value, _ => 0.0 }).collect::<Vec<_>>());
        groups.push(UiInspectorFieldGroup {
            id: "flow-play-inspector.kind.inputSlider".into(),
            label: "inputSlider".into(),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.slider-value".into(),
                label: "Value".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "flow-play-inspector.slider-value.input".into(),
                    input_kind: "number".into(),
                    value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
                    placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                    commit: None,
                    on_change: flow_cmd("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "value" }))),
                }),
            })],
        });
    }
    if widgets.iter().all(|widget| matches!(widget, Widget::InputNote { .. })) {
        let mixed = ui_inspector_mixed_text(&widgets.iter().map(|widget| match widget { Widget::InputNote { text, .. } => text.clone(), _ => String::new() }).collect::<Vec<_>>());
        groups.push(UiInspectorFieldGroup {
            id: "flow-play-inspector.kind.inputNote".into(),
            label: "inputNote".into(),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.note-text".into(),
                label: "Text".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "flow-play-inspector.note-text.input".into(),
                    input_kind: "text".into(),
                    value: mixed.value,
                    placeholder: mixed.placeholder,
                    commit: Some("blur".into()),
                    on_change: flow_cmd("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "text" }))),
                }),
            })],
        });
    }
    let kind_mixed = ui_inspector_mixed_text(&widgets.iter().map(|widget| widget_kind_label(widget).to_string()).collect::<Vec<_>>());
    let mut base_fields = vec![ui_inspector_readonly_field(
        "flow-play-inspector.kind",
        "Kind",
        if kind_mixed.placeholder.is_none() { widget_kind_label(widgets[0]).to_string() } else { "—".into() },
    )];
    if widget_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.id".into(),
                label: "Id".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "flow-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: widget_ids[0].clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: flow_cmd("renameFlowWidget", Some(json!({ "oldId": widget_ids[0] }))),
                }),
            }),
        );
    }
    groups.push(UiInspectorFieldGroup {
        id: "flow-play-inspector.base".into(),
        label: "Widget".into(),
        default_open: None,
        fields: base_fields,
    });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(envelope: &FlowPlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&envelope.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let fixture_json = serde_json::to_string(&envelope.fixture).ok();
    let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
    };
    build_node_graph_scene(
        FLOW_PLAY_SURFACE_MAIN,
        FLOW_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators_json: None,
            context_menu_json: Some(
                r#"[{"id":"delete-selection","label":"Delete selection","command":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
            ),
            find_items_json: None,
            capabilities_json: Some(r#"{"engine":"flow","spotlight":true,"noteEdit":true,"clusters":true,"previewToggle":true}"#.into()),
            lod_json: Some(r#"{"automatic":true}"#.into()),
            fixture_json,
            selection_json,
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_compiled_dag(envelope: &FlowPlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    build_text_editor_scene(
        FLOW_PLAY_SURFACE_COMPILED,
        FLOW_PLAY_APP_ID,
        TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None),
    )
}
//#endregion 🔖Render

//#region 🔖FlowPlayApp
struct FlowPlayApp {
    host: Option<FlowHost>,
}

impl FlowPlayApp {
    fn host_for(&mut self, envelope: &FlowPlayEnvelope) -> &mut FlowHost {
        let replace = self
            .host
            .as_ref()
            .map_or(true, |host| host.fixture != envelope.fixture);
        if replace {
            self.host = Some(FlowHost::from_fixture(envelope.fixture.clone()));
        }
        self.host.as_mut().expect("flow host")
    }
}

impl PluginApp for FlowPlayApp {
    fn app_id(&self) -> &str {
        FLOW_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("flow envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        let host = self.host_for(&envelope);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                let ids = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok())
                    .unwrap_or_else(|| selection_ids(args));
                envelope.runtime.selected_node_ids = ids;
                return vec![set_document_op(&envelope)];
            }
            "graphPointerDown" => {
                envelope.runtime.selected_node_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "moveMediaNode" => {
                snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if host.move_widget(node_id, x, y).is_ok() {
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "undo" => {
                if host.undo() {
                    envelope.fixture = host.fixture.clone();
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                if host.redo() {
                    envelope.fixture = host.fixture.clone();
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "evaluate" => {
                host.clear_computing_widget_ids();
                if let Ok(eval_json) = host.evaluate() {
                    envelope.fixture = host.fixture.clone();
                    envelope.runtime.last_eval_json = eval_json.clone();
                    host.apply_eval_outputs_json(&eval_json);
                    host.clear_computing_widget_ids();
                    return vec![set_document_op(&envelope)];
                }
            }
            "removeWidget" => {
                let widget_id = args
                    .and_then(|value| value.get("widgetId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str());
                if let Some(widget_id) = widget_id {
                    snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                    if host.remove_widget(widget_id).is_ok() {
                        envelope.runtime.selected_node_ids.retain(|id| id != widget_id);
                        if let Ok(eval_json) = host.evaluate() {
                            envelope.runtime.last_eval_json = eval_json;
                        }
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "deleteSelection" => {
                snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                sync_host_selection(host, &envelope.runtime.selected_node_ids);
                if host.delete_selection().is_ok() {
                    envelope.runtime.selected_node_ids.clear();
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "disconnect" => {
                let synapse_id = args
                    .and_then(|value| value.get("synapseId"))
                    .or_else(|| args.and_then(|value| value.get("edgeId")))
                    .and_then(|value| value.as_str());
                if let Some(synapse_id) = synapse_id {
                    snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                    if host.disconnect(synapse_id).is_ok() {
                        if let Ok(eval_json) = host.evaluate() {
                            envelope.runtime.last_eval_json = eval_json;
                        }
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str());
                let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str());
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str());
                let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str());
                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                    snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                    if host.connect_ports(from, from_port, to, to_port).is_ok() {
                        if let Ok(eval_json) = host.evaluate() {
                            envelope.runtime.last_eval_json = eval_json;
                        }
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addWidget" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => {
                        let neuron_kind = args.and_then(|value| value.get("neuronKind")).and_then(|value| value.as_str()).unwrap_or("math.add");
                        json!({ "kind": "neuron", "neuronKind": neuron_kind }).to_string()
                    }
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                if let Ok(id) = host.add_widget(&descriptor, x, y) {
                    envelope.runtime.selected_node_ids = vec![id];
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "reorganize" => {
                snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "patchFlowWidgets" => {
                snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                let widget_ids: Vec<String> = args
                    .and_then(|value| value.get("widgetIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value"));
                for widget in envelope.fixture.widgets.iter_mut() {
                    if !widget_ids.contains(&widget_id(widget).to_string()) {
                        continue;
                    }
                    match (field, widget) {
                        ("value", Widget::InputSlider { value: ref mut slider_value, .. }) => {
                            if let Some(value) = raw_value.and_then(|value| value.as_f64()) {
                                *slider_value = value;
                            }
                        }
                        ("text", Widget::InputNote { text: ref mut note_text, .. }) => {
                            if let Some(value) = raw_value.and_then(|value| value.as_str()) {
                                *note_text = value.into();
                            }
                        }
                        _ => {}
                    }
                }
                host.replace_fixture(envelope.fixture.clone());
                if let Ok(eval_json) = host.evaluate() {
                    envelope.runtime.last_eval_json = eval_json;
                }
                envelope.fixture = host.fixture.clone();
                return vec![set_document_op(&envelope)];
            }
            "renameFlowWidget" => {
                snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                let old_id = args.and_then(|value| value.get("oldId")).and_then(|value| value.as_str());
                let new_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                if let (Some(old_id), Some(new_id)) = (old_id, new_id) {
                    let trimmed = new_id.trim();
                    if !trimmed.is_empty() && trimmed != old_id && !envelope.fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
                        for widget in envelope.fixture.widgets.iter_mut() {
                            if widget_id(widget) == old_id {
                                match widget {
                                    Widget::Neuron { id, .. }
                                    | Widget::InputSlider { id, .. }
                                    | Widget::InputStepper { id, .. }
                                    | Widget::InputNote { id, .. }
                                    | Widget::InputImage { id, .. }
                                    | Widget::Variable { id, .. }
                                    | Widget::OutputPreview { id, .. }
                                    | Widget::OutputAction { id, .. }
                                    | Widget::OutputExport { id, .. }
                                    | Widget::Cluster { id, .. } => *id = trimmed.to_string(),
                                }
                            }
                        }
                        for synapse in envelope.fixture.synapses.iter_mut() {
                            if synapse.from == old_id {
                                synapse.from = trimmed.into();
                            }
                            if synapse.to == old_id {
                                synapse.to = trimmed.into();
                            }
                        }
                        envelope.runtime.selected_node_ids = vec![trimmed.into()];
                        host.replace_fixture(envelope.fixture.clone());
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "nodeGraphHover" => {
                return Vec::new();
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<CameraJson>(viewport_json) {
                        envelope.fixture.camera = camera;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "nodeGraphEdit" | "spotlightCommit" => {
                let ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                if ops.is_empty() && command == "spotlightCommit" {
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
                let mut changed = false;
                for op in ops {
                    let op_name = op.get("op").and_then(|value| value.as_str()).unwrap_or("");
                    match op_name {
                        "setFixture" => {
                            if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Ok(fixture) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                    snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                                    envelope.fixture = fixture.clone();
                                    host.replace_fixture(fixture);
                                    changed = true;
                                }
                            }
                        }
                        "deleteSelection" => {
                            snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                            sync_host_selection(host, &envelope.runtime.selected_node_ids);
                            if host.delete_selection().is_ok() {
                                envelope.runtime.selected_node_ids.clear();
                                changed = true;
                            }
                        }
                        "connect" => {
                            let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                            let to = op.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                snapshot_fixture(&mut envelope.runtime, &envelope.fixture);
                                if host.connect_ports(from, from_port, to, to_port).is_ok() {
                                    changed = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            FLOW_PLAY_BODY_MAIN => render_main_graph(&envelope),
            FLOW_PLAY_BODY_COMPILED => render_compiled_dag(&envelope),
            FLOW_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope.fixture, &envelope.runtime.selected_node_ids),
            FLOW_PLAY_BODY_CATALOGUE => build_catalogue_tree(&envelope),
            FLOW_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_node_ids),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}
//#endregion 🔖FlowPlayApp

//#region 🔖Manifest
fn create_flow_app() -> App {
    App::from_builder(
        App::builder(FLOW_PLAY_APP_ID, "Flow")
            .icon_id("flow")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(FLOW_PLAY_WINDOW_MAIN, "Flow", FLOW_PLAY_BODY_MAIN)
            .window_kind(FLOW_PLAY_WINDOW_COMPILED, "DSL", FLOW_PLAY_BODY_COMPILED)
            .default_layout(create_default_layout(
                &[FLOW_PLAY_WINDOW_MAIN.into(), FLOW_PLAY_WINDOW_COMPILED.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Flow".into(), "DSL".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                FLOW_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                FLOW_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                FLOW_PLAY_BODY_INSPECTOR,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_envelope()).unwrap())
    .program("flow", "Flow", "graph")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("flow", "Flow", "0.1.0").register_app(create_flow_app(), || Box::new(FlowPlayApp { host: None }))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_compiled_wire_editor() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_COMPILED, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn default_fixture_has_widgets() {
        let envelope = default_envelope();
        assert!(!envelope.fixture.widgets.is_empty());
    }

    #[test]
    fn hierarchy_lists_widgets() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("flow-play-hierarchy.widgets"));
    }

    #[test]
    fn evaluate_updates_preview_state() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let ops = app.handle_command("evaluate", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: FlowPlayEnvelope = serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert!(!updated.runtime.last_eval_json.is_empty());
    }

    #[test]
    fn undo_restores_fixture_after_add_widget() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let before: FlowPlayEnvelope = serde_json::from_str(&document).unwrap();
        let count_before = before.fixture.widgets.len();
        let ops = app.handle_command(
            "addWidget",
            Some(&json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let after_add = serde_json::to_string(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"]).unwrap();
        let undo_ops = app.handle_command("undo", None, &after_add, &ViewState::default());
        let restored: FlowPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&undo_ops[0]).unwrap()["document"].clone()).unwrap();
        assert_eq!(restored.fixture.widgets.len(), count_before);
    }
}
