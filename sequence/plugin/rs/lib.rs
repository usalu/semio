//! 🔗 Sequence plugin — declarative sequence play app bundled as a hot-swappable WASM component.

use mathematical_graph_port_directed_dag::{DagFixture, DagLayoutOptions, DagLayoutOrientation};
use sequence_core::{default_fixture, SequenceFixture, SequenceHost, SequenceStep};
use semio_framework_plugin::{SurfaceKind, PanelGroup, 
    build_node_graph_scene, build_text_editor_scene, create_default_layout, tool_button, tool_collection,
    tool_toggle, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text,
    App, CommandDescriptor, NodeGraphScene, PluginApp, PluginBundle, TextEditorScene, ToolCategory, ToolNode, UiControlNode,
    UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiToggleNode, UiTreeItemNode, UiTreeNode,
    UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const SEQUENCE_PLAY_APP_ID: &str = "sequence-play";
const SEQUENCE_PLAY_SURFACE_MAIN: &str = "sequence.play.main";
const SEQUENCE_PLAY_SURFACE_SCRIPT: &str = "sequence.play.script";
const SEQUENCE_PLAY_SURFACE_COMPILED: &str = "sequence.play.compiled-dag";
const SEQUENCE_PLAY_BODY_MAIN: &str = "sequence.play.main";
const SEQUENCE_PLAY_BODY_SCRIPT: &str = "sequence.play.script";
const SEQUENCE_PLAY_BODY_COMPILED: &str = "sequence.play.compiled-dag";
const SEQUENCE_PLAY_BODY_DOCUMENT: &str = "sequence.play.document";
const SEQUENCE_PLAY_BODY_CATALOGUE: &str = "sequence.play.catalogue";
const SEQUENCE_PLAY_BODY_INSPECTOR: &str = "sequence.play.inspection";
const SEQUENCE_PLAY_WINDOW_MAIN: &str = "sequence-main";
const SEQUENCE_PLAY_WINDOW_SCRIPT: &str = "sequence-script";
const SEQUENCE_PLAY_WINDOW_COMPILED: &str = "sequence-compiled-dag";
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SequencePlayRuntime {
    #[serde(default)]
    selected_step_ids: Vec<String>,
    #[serde(default)]
    last_run_json: String,
    #[serde(default)]
    orientation: DagLayoutOrientation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SequencePlayEnvelope {
    fixture: SequenceFixture,
    #[serde(default)]
    runtime: SequencePlayRuntime,
    #[serde(default)]
    undo_stack: Vec<SequenceFixture>,
    #[serde(default)]
    redo_stack: Vec<SequenceFixture>,
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
fn default_envelope() -> SequencePlayEnvelope {
    SequencePlayEnvelope {
        fixture: default_fixture(),
        runtime: SequencePlayRuntime::default(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
    }
}

fn parse_envelope(document_json: &str) -> SequencePlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &SequencePlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn push_undo(envelope: &mut SequencePlayEnvelope) {
    envelope.undo_stack.push(envelope.fixture.clone());
    if envelope.undo_stack.len() > 32 {
        envelope.undo_stack.remove(0);
    }
    envelope.redo_stack.clear();
}

fn sequence_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: SEQUENCE_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn host_from_envelope(envelope: &SequencePlayEnvelope) -> SequenceHost {
    SequenceHost::from_fixture(envelope.fixture.clone())
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "next".into()))
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
//#endregion 🔖DocumentHelpers

//#region 🔖TreeHelpers
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
fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "control.if" | "control.while" | "control.repeat")
}

fn control_slots(kind: &str) -> &'static [&'static str] {
    match kind {
        "control.if" => &["then", "else"],
        "control.while" | "control.repeat" => &["body"],
        _ => &[],
    }
}

fn build_step_tree_item(step: &SequenceStep, fixture: &SequenceFixture) -> UiTreeItemNode {
    let mut item = tree_item_with_command(
        format!("sequence-play-document.step.{}", step.id),
        format!("{} ({})", step.id, step.kind),
        Some(step.kind.clone()),
        sequence_cmd("setSelection", Some(json!({ "ids": [step.id.clone()] }))),
    );
    if is_control_kind(&step.kind) {
        item.control = Some(UiControlNode::Toggle(UiToggleNode {
            id: format!("sequence-play-document.collapse.{}", step.id),
            icon_id: if step.collapsed { "chevron-right" } else { "chevron-down" }.into(),
            pressed: !step.collapsed,
            text: None,
            on_change: sequence_cmd("setStepCollapsed", Some(json!({ "id": step.id }))),
        }));
        let slot_items: Vec<UiTreeItemNode> = control_slots(&step.kind)
            .iter()
            .map(|slot_name| {
                let nested: Vec<UiTreeItemNode> = fixture
                    .steps
                    .iter()
                    .filter(|entry| {
                        entry.slot.as_ref().is_some_and(|slot| slot.owner == step.id && slot.name == *slot_name)
                    })
                    .map(|entry| build_step_tree_item(entry, fixture))
                    .collect();
                UiTreeItemNode {
                    id: format!("sequence-play-document.slot.{}.{}", step.id, slot_name),
                    label: (*slot_name).into(),
                    description: Some(format!("{} slot", step.id)),
                    icon_id: Some("folder".into()),
                    selected: None,
                    default_open: Some(true),
                    command: None,
                    hover_command: None,
                    unhover_command: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: if nested.is_empty() { None } else { Some(nested) },
                    control: None,
                    is_hidden: if step.collapsed { Some(true) } else { None },
                }
            })
            .collect();
        if !slot_items.is_empty() {
            item.items = Some(slot_items);
        }
        item.default_open = Some(!step.collapsed);
    }
    item
}
//#endregion 🔖TreeHelpers

//#region 🔖Panels
fn build_document_tree(fixture: &SequenceFixture, selected: &[String]) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = fixture
        .steps
        .iter()
        .filter(|step| step.slot.is_none())
        .map(|step| build_step_tree_item(step, fixture))
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture
        .edges
        .iter()
        .map(|edge| {
            UiTreeItemNode {
                id: format!("sequence-play-document.edge.{}", edge.id),
                label: format!("{} → {}", edge.from, edge.to),
                description: Some(edge.id.clone()),
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
                id: "sequence-play-document.steps".into(),
                label: Some("Steps".into()),
                default_open: Some(true),
                items: if step_items.is_empty() {
                    vec![tree_item("sequence-play-document.steps.empty", "(none)")]
                } else {
                    step_items
                },
            },
            UiTreeSectionNode {
                id: "sequence-play-document.edges".into(),
                label: Some("Flow edges".into()),
                default_open: Some(false),
                items: if edge_items.is_empty() {
                    vec![tree_item("sequence-play-document.edges.empty", "(none)")]
                } else {
                    edge_items
                },
            },
        ],
        selected_ids: Some(selected.iter().map(|id| format!("sequence-play-document.step.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree(fixture: &SequenceFixture) -> UiNode {
    let actions = [
        ("state.set", "Set state"),
        ("log.print", "Print log"),
        ("control.if", "If"),
        ("control.while", "While"),
        ("math.add", "Add"),
    ];
    let mut items: Vec<UiTreeItemNode> = actions
        .iter()
        .map(|(kind, label)| {
            tree_item_with_command(
                format!("sequence-play-catalogue.action.{kind}"),
                *label,
                Some((*kind).into()),
                sequence_cmd("addStep", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    for owner in fixture.steps.iter().filter(|step| is_control_kind(&step.kind)) {
        for slot_name in control_slots(&owner.kind) {
            items.push(tree_item_with_command(
                format!("sequence-play-catalogue.slot.{}.{}", owner.id, slot_name),
                format!("Add to {} → {slot_name}", owner.id),
                Some(format!("{slot_name} @ {}", owner.id)),
                sequence_cmd(
                    "addStepToSlot",
                    Some(json!({
                        "kind": "log.print",
                        "owner": owner.id,
                        "slotName": slot_name,
                    })),
                ),
            ));
        }
    }
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "sequence-play-catalogue.actions".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(fixture: &SequenceFixture, selected: &[String]) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "sequence-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Select a step in the canvas or document.")],
        }]);
    }
    let steps: Vec<&SequenceStep> = selected
        .iter()
        .filter_map(|id| fixture.steps.iter().find(|step| &step.id == id))
        .collect();
    if steps.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "sequence-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Step not found")],
        }]);
    }
    let step_ids: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let mut fields = vec![
        ui_inspector_readonly_field("sequence-play-inspector.kind", "Kind", steps[0].kind.clone()),
        ui_inspector_readonly_field(
            "sequence-play-inspector.params",
            "Params",
            serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into()),
        ),
    ];
    if step_ids.len() == 1 {
        fields.insert(
            0,
            ui_inspector_readonly_field("sequence-play-inspector.id", "Id", step_ids[0].clone()),
        );
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "sequence-play-inspector.step".into(),
        label: "Step".into(),
        default_open: None,
        fields,
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(envelope: &SequencePlayEnvelope) -> UiNode {
    let mut host = host_from_envelope(envelope);
    host.layout_expanded_slots();
    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&envelope.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let selection_json = if envelope.runtime.selected_step_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&envelope.runtime.selected_step_ids).ok()
    };
    build_node_graph_scene(
        SEQUENCE_PLAY_SURFACE_MAIN,
        SEQUENCE_PLAY_APP_ID,
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

fn render_script(envelope: &SequencePlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    let mut text = host.compile_text();
    if !envelope.runtime.last_run_json.is_empty() {
        text.push_str("\n\n# run result\n");
        text.push_str(&envelope.runtime.last_run_json);
    }
    build_text_editor_scene(
        SEQUENCE_PLAY_SURFACE_SCRIPT,
        SEQUENCE_PLAY_APP_ID,
        TextEditorScene::base(text, Some("imperative".into()), None),
    )
}

fn render_compiled_dag(envelope: &SequencePlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    build_text_editor_scene(
        SEQUENCE_PLAY_SURFACE_COMPILED,
        SEQUENCE_PLAY_APP_ID,
        TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None),
    )
}
//#endregion 🔖Render

//#region 🔖Tools
fn orientation_arg(orientation: DagLayoutOrientation) -> Value {
    match orientation {
        DagLayoutOrientation::LeftRight => json!({ "orientation": "leftRight" }),
        DagLayoutOrientation::TopBottom => json!({ "orientation": "topBottom" }),
    }
}

fn edit_tools(envelope: &SequencePlayEnvelope) -> Vec<ToolNode> {
    let orientation = envelope.runtime.orientation;
    vec![
        tool_collection(
            "sequence-tools-execution",
            "play",
            "Run",
            vec![
                tool_button("sequence-tools-run", "play", "Run", sequence_cmd("run", None)),
                tool_button("sequence-tools-stop", "square", "Stop", sequence_cmd("stop", None)),
            ],
        )
        .with_category(ToolCategory::Commands),
        tool_button(
            "sequence-tools-reorganize",
            "refresh-cw",
            "Reorganize",
            sequence_cmd("reorganize", None),
        )
        .with_category(ToolCategory::Commands),
        tool_collection(
            "sequence-tools-orientation",
            "layout-grid",
            "Layout",
            vec![
                tool_toggle(
                    "sequence-tools-orientation-lr",
                    "arrow-right",
                    "Left to right",
                    orientation == DagLayoutOrientation::LeftRight,
                    sequence_cmd("setOrientation", Some(orientation_arg(DagLayoutOrientation::LeftRight))),
                ),
                tool_toggle(
                    "sequence-tools-orientation-tb",
                    "arrow-down",
                    "Top to bottom",
                    orientation == DagLayoutOrientation::TopBottom,
                    sequence_cmd("setOrientation", Some(orientation_arg(DagLayoutOrientation::TopBottom))),
                ),
            ],
        )
        .with_category(ToolCategory::Tools),
    ]
}
//#endregion 🔖Tools

//#region 🔖SequencePlayApp
struct SequencePlayApp;

impl PluginApp for SequencePlayApp {
    fn app_id(&self) -> &str {
        SEQUENCE_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("sequence envelope json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        let mut host = host_from_envelope(&envelope);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                envelope.runtime.selected_step_ids = node_graph_selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "nodeGraphHover" => return Vec::new(),
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str(viewport_json) {
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
                                if let Ok(fixture) = serde_json::from_str::<SequenceFixture>(fixture_json) {
                                    push_undo(&mut envelope);
                                    envelope.fixture = fixture;
                                    changed = true;
                                }
                            }
                        }
                        "deleteSelection" => {
                            for step_id in envelope.runtime.selected_step_ids.clone() {
                                push_undo(&mut envelope);
                                if host.remove_step(&step_id) {
                                    changed = true;
                                }
                            }
                            if changed {
                                envelope.fixture = host.fixture.clone();
                                envelope.runtime.selected_step_ids.clear();
                            }
                        }
                        "connect" => {
                            let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                            let to = op.get("targetNodeId").and_then(|value| value.as_str());
                            if let (Some(from), Some(to)) = (from, to) {
                                push_undo(&mut envelope);
                                if host.connect_steps(from, to).is_ok() {
                                    envelope.fixture = host.fixture.clone();
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
                for step_id in envelope.runtime.selected_step_ids.clone() {
                    push_undo(&mut envelope);
                    if host.remove_step(&step_id) {
                        envelope.fixture = host.fixture;
                        envelope.runtime.selected_step_ids.retain(|id| id != &step_id);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "graphPointerDown" => {
                envelope.runtime.selected_step_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if envelope.fixture.steps.iter().any(|step| step.id == node_id) {
                        push_undo(&mut envelope);
                        if let Some(step) = envelope.fixture.steps.iter_mut().find(|step| step.id == node_id) {
                            step.x = x;
                            step.y = y;
                            host.replace_fixture(envelope.fixture.clone()).ok();
                            envelope.fixture = host.fixture;
                            return vec![set_document_op(&envelope)];
                        }
                    }
                }
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str());
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str());
                if let (Some(from), Some(to)) = (from, to) {
                    push_undo(&mut envelope);
                    if host.connect_steps(from, to).is_ok() {
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addStep" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print");
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                push_undo(&mut envelope);
                let id = host.add_step(kind, x, y);
                envelope.fixture = host.fixture;
                envelope.runtime.selected_step_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "addStepToSlot" | "addStepDropped" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print");
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let picked = args
                    .and_then(|value| value.get("pickedStepId"))
                    .or_else(|| args.and_then(|value| value.get("owner")))
                    .and_then(|value| value.as_str());
                push_undo(&mut envelope);
                let id = if command == "addStepToSlot" {
                    let owner = args.and_then(|value| value.get("owner")).and_then(|value| value.as_str());
                    let slot = args.and_then(|value| value.get("slotName")).and_then(|value| value.as_str());
                    match (owner, slot) {
                        (Some(owner), Some(slot)) => host.add_step_in_slot(
                            kind,
                            x,
                            y,
                            Some(sequence_core::SlotRef {
                                owner: owner.into(),
                                name: slot.into(),
                            }),
                        ),
                        _ => host.add_step(kind, x, y),
                    }
                } else {
                    host.add_step_dropped(kind, x, y, picked)
                };
                envelope.fixture = host.fixture;
                envelope.runtime.selected_step_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "removeStep" => {
                let step_id = args
                    .and_then(|value| value.get("id"))
                    .or_else(|| args.and_then(|value| value.get("stepId")))
                    .and_then(|value| value.as_str());
                if let Some(step_id) = step_id {
                    push_undo(&mut envelope);
                    if host.remove_step(step_id) {
                        envelope.fixture = host.fixture;
                        envelope.runtime.selected_step_ids.retain(|id| id != step_id);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setStepParams" => {
                let step_id = args
                    .and_then(|value| value.get("id"))
                    .or_else(|| args.and_then(|value| value.get("stepId")))
                    .and_then(|value| value.as_str());
                let params = args.and_then(|value| value.get("params"));
                if let (Some(step_id), Some(params)) = (step_id, params) {
                    push_undo(&mut envelope);
                    if host.set_step_params_json(step_id, &params.to_string()).is_ok() {
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setStepCollapsed" => {
                let step_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                if let Some(step_id) = step_id {
                    let collapsed = envelope
                        .fixture
                        .steps
                        .iter()
                        .find(|step| step.id == step_id)
                        .map(|step| !step.collapsed)
                        .unwrap_or(true);
                    push_undo(&mut envelope);
                    if host.set_step_collapsed(step_id, collapsed) {
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "disconnectSteps" => {
                let from_id = args.and_then(|value| value.get("fromId")).and_then(|value| value.as_str());
                let to_id = args.and_then(|value| value.get("toId")).and_then(|value| value.as_str());
                if let (Some(from_id), Some(to_id)) = (from_id, to_id) {
                    push_undo(&mut envelope);
                    if host.disconnect_steps(from_id, to_id) {
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "run" => {
                let result = host.run();
                envelope.runtime.last_run_json = serde_json::to_string(&result).unwrap_or_default();
                return vec![set_document_op(&envelope)];
            }
            "stop" => {
                envelope.runtime.last_run_json.clear();
                return vec![set_document_op(&envelope)];
            }
            "reorganize" => {
                let opts = DagLayoutOptions {
                    orientation: envelope.runtime.orientation,
                    ..DagLayoutOptions::default()
                };
                push_undo(&mut envelope);
                if host.reorganize(&opts).is_ok() {
                    envelope.fixture = host.fixture;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setOrientation" => {
                let orientation = args.and_then(|value| value.get("orientation")).and_then(|value| value.as_str());
                let orientation = match orientation {
                    Some("topBottom") => DagLayoutOrientation::TopBottom,
                    Some("leftRight") => DagLayoutOrientation::LeftRight,
                    _ => return Vec::new(),
                };
                envelope.runtime.orientation = orientation;
                return vec![set_document_op(&envelope)];
            }
            "undo" => {
                if let Some(previous) = envelope.undo_stack.pop() {
                    envelope.redo_stack.push(envelope.fixture.clone());
                    envelope.fixture = previous;
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                if let Some(next) = envelope.redo_stack.pop() {
                    envelope.undo_stack.push(envelope.fixture.clone());
                    envelope.fixture = next;
                    return vec![set_document_op(&envelope)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn tools(&self, document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        edit_tools(&parse_envelope(document_json))
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => render_main_graph(&envelope),
            SEQUENCE_PLAY_BODY_SCRIPT => render_script(&envelope),
            SEQUENCE_PLAY_BODY_COMPILED => render_compiled_dag(&envelope),
            SEQUENCE_PLAY_BODY_DOCUMENT => build_document_tree(&envelope.fixture, &envelope.runtime.selected_step_ids),
            SEQUENCE_PLAY_BODY_CATALOGUE => build_catalogue_tree(&envelope.fixture),
            SEQUENCE_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_step_ids),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn node_graph_selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("nodeIds"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_else(|| selection_ids(args))
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
//#endregion 🔖SequencePlayApp

//#region 🔖Manifest
fn create_sequence_app() -> App {
    App::from_builder(
        App::builder(SEQUENCE_PLAY_APP_ID, "Sequence").document(["semio", "sequence"])
            .icon_id("sequence")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(SEQUENCE_PLAY_WINDOW_MAIN, "Sequence", SEQUENCE_PLAY_BODY_MAIN, SurfaceKind::NodeGraph)
            .window_kind(SEQUENCE_PLAY_WINDOW_SCRIPT, "Script", SEQUENCE_PLAY_BODY_SCRIPT, SurfaceKind::TextEditor)
            .window_kind(SEQUENCE_PLAY_WINDOW_COMPILED, "DSL", SEQUENCE_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph)
            .default_layout(create_default_layout(
                &[
                    SEQUENCE_PLAY_WINDOW_MAIN.into(),
                    SEQUENCE_PLAY_WINDOW_SCRIPT.into(),
                    SEQUENCE_PLAY_WINDOW_COMPILED.into(),
                ],
                "row",
                Some(&[50.0, 25.0, 25.0]),
                Some(&["Sequence".into(), "Script".into(), "DSL".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                SEQUENCE_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                SEQUENCE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                SEQUENCE_PLAY_BODY_INSPECTOR,
            )
            .mode_tools("edit", edit_tools(&default_envelope()))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_envelope()).unwrap())
    .program("sequence", "Sequence", "graph")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("sequence", "Sequence", "0.1.0").register_app(create_sequence_app(), || Box::new(SequencePlayApp))
}

semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let app = SequencePlayApp;
        let document = app.initial_document_json();
        let node = app.render(SEQUENCE_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_script_editor() {
        let app = SequencePlayApp;
        let document = app.initial_document_json();
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn default_fixture_has_steps() {
        let envelope = default_envelope();
        assert_eq!(envelope.fixture.steps.len(), 2);
    }

    #[test]
    fn add_step_command_appends_step() {
        let mut app = SequencePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addStep", Some(&json!({ "kind": "log.print" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated_op: Value = serde_json::from_str(&ops[0]).unwrap();
        let updated: SequencePlayEnvelope = serde_json::from_value(updated_op["document"].clone()).unwrap();
        assert!(updated.fixture.steps.len() > 2);
    }

    #[test]
    fn run_stores_result_in_runtime() {
        let mut app = SequencePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("run", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: SequencePlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert!(!updated.runtime.last_run_json.is_empty());
    }

    #[test]
    fn remove_step_command_deletes_step() {
        let mut app = SequencePlayApp;
        let envelope = default_envelope();
        let step_id = envelope.fixture.steps[0].id.clone();
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command_patch_ops("removeStep", Some(&json!({ "id": step_id })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: SequencePlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert!(updated.fixture.steps.iter().all(|step| step.id != step_id));
    }

    #[test]
    fn footer_tools_include_run_stop_reorganize_and_orientation() {
        let app = SequencePlayApp;
        let document = app.initial_document_json();
        let tools = app.tools(&document, &ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("\"id\":\"sequence-tools-run\""));
        assert!(json.contains("\"id\":\"sequence-tools-stop\""));
        assert!(json.contains("\"id\":\"sequence-tools-reorganize\""));
        assert!(json.contains("\"id\":\"sequence-tools-orientation-lr\""));
        assert!(json.contains("\"id\":\"sequence-tools-orientation-tb\""));
    }

    #[test]
    fn set_orientation_command_flips_toggle_state() {
        let mut app = SequencePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setOrientation", Some(&json!({ "orientation": "topBottom" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: SequencePlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert_eq!(updated.runtime.orientation, DagLayoutOrientation::TopBottom);
        let tools = app.tools(&serde_json::to_string(&updated).unwrap(), &ViewState::default());
        let tools_json = serde_json::to_string(&tools).unwrap();
        assert!(tools_json.contains(r#""id":"sequence-tools-orientation-tb""#));
        let lr_pressed = tools_json
            .split(r#""id":"sequence-tools-orientation-lr""#)
            .nth(1)
            .and_then(|rest| rest.split_once("\"pressed\":"))
            .map(|(_, rest)| rest.starts_with("false"))
            .unwrap_or(false);
        let tb_pressed = tools_json
            .split(r#""id":"sequence-tools-orientation-tb""#)
            .nth(1)
            .and_then(|rest| rest.split_once("\"pressed\":"))
            .map(|(_, rest)| rest.starts_with("true"))
            .unwrap_or(false);
        assert!(lr_pressed, "left-to-right toggle should be unpressed, got {tools_json}");
        assert!(tb_pressed, "top-to-bottom toggle should be pressed, got {tools_json}");
    }

    #[test]
    fn reorganize_command_spreads_step_positions_apart() {
        let mut app = SequencePlayApp;
        let mut envelope = default_envelope();
        for step in envelope.fixture.steps.iter_mut() {
            step.x = 0.0;
            step.y = 0.0;
        }
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command_patch_ops("reorganize", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: SequencePlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        let xs: Vec<f64> = updated.fixture.steps.iter().map(|step| step.x).collect();
        assert!(xs.iter().any(|x| *x != 0.0), "reorganize should spread steps apart, got {xs:?}");
    }

    #[test]
    fn stop_command_clears_last_run_result() {
        let mut app = SequencePlayApp;
        let document = app.initial_document_json();
        let ran = app.handle_command_patch_ops("run", None, &document, &ViewState::default());
        let ran_document = serde_json::from_str::<Value>(&ran[0]).unwrap()["document"].to_string();
        let ops = app.handle_command_patch_ops("stop", None, &ran_document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: SequencePlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert!(updated.runtime.last_run_json.is_empty());
    }
}
