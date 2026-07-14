//! 🔗 Sequence plugin — declarative sequence play app bundled as a hot-swappable WASM component.

use infinite_board_port_directed_dag::{DagFixture, DagLayoutOptions, DagLayoutOrientation};
use sequence_core::{
    default_fixture, sequence_fixture_ops, SequenceFixture, SequenceHost, SequenceOp, SequenceStep, SlotRef,
    SEQUENCE_FIXTURE_SCHEMA,
};
use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_node_graph_scene, build_text_editor_scene, create_default_layout,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text,
    ActionArgDef, ActionArgOption, ActionEmit, App, ActionDescriptor, AppLabelsOverlay, DocumentApp, DocumentView, NodeGraphScene, TextEditorScene,
    UiControlNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiToggleNode,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

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
/// 🎛️ Ephemeral view state (selection, last run output, layout orientation) held in the app struct,
/// never in the document — so it stays out of undo history and off the op channel.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SequencePlayRuntime {
    selected_step_ids: Vec<String>,
    last_run_json: String,
    orientation: DagLayoutOrientation,
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
fn sequence_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: SEQUENCE_PLAY_APP_ID.into(),
        action: action.into(),
        args,
    }
}

/// 🧰 Builds a {@link SequenceHost} seeded from a projection so an action can mutate it (with all the
/// host's cycle/slot/layout logic) and then diff the result into typed ops.
fn host_from_fixture(fixture: &SequenceFixture) -> SequenceHost {
    SequenceHost::from_fixture(fixture.clone())
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
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
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
    let mut item = tree_item_with_action(
        format!("sequence-play-document.step.{}", step.id),
        format!("{} ({})", step.id, step.kind),
        Some(step.kind.clone()),
        sequence_action("setSelection", Some(json!({ "ids": [step.id.clone()] }))),
    );
    if is_control_kind(&step.kind) {
        item.control = Some(UiControlNode::Toggle(UiToggleNode {
            id: format!("sequence-play-document.collapse.{}", step.id),
            icon_id: if step.collapsed { "chevron-right" } else { "chevron-down" }.into(),
            pressed: !step.collapsed,
            text: None,
            on_change: sequence_action("setStepCollapsed", Some(json!({ "id": step.id }))),
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
                    action: None,
                    hover_action: None,
                    unhover_action: None,
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

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the sequence app; one field per label makes every locale combination compile-checked.
struct SequenceLabels {
    steps: &'static str,
    flow_edges: &'static str,
    select_prompt: &'static str,
    step_not_found: &'static str,
    kind: &'static str,
    params: &'static str,
    id: &'static str,
    step: &'static str,
    action_set_state: &'static str,
    action_log_print: &'static str,
    action_if: &'static str,
    action_while: &'static str,
    action_add: &'static str,
    add_to: &'static str,
    run: &'static str,
    stop: &'static str,
    reorganize: &'static str,
    layout: &'static str,
    left_to_right: &'static str,
    top_to_bottom: &'static str,
    window_main: &'static str,
    window_script: &'static str,
    window_compiled: &'static str,
}

const SEQUENCE_LABELS_NATIVE_EN: SequenceLabels = SequenceLabels {
    steps: "Steps",
    flow_edges: "Flow edges",
    select_prompt: "Select a step in the canvas or document.",
    step_not_found: "Step not found",
    kind: "Kind",
    params: "Params",
    id: "Id",
    step: "Step",
    action_set_state: "Set state",
    action_log_print: "Print log",
    action_if: "If",
    action_while: "While",
    action_add: "Add",
    add_to: "Add to",
    run: "Run",
    stop: "Stop",
    reorganize: "Reorganize",
    layout: "Layout",
    left_to_right: "Left to right",
    top_to_bottom: "Top to bottom",
    window_main: "Sequence",
    window_script: "Script",
    window_compiled: "DSL",
};

const SEQUENCE_LABELS_NATIVE_DE: SequenceLabels = SequenceLabels {
    steps: "Schritte",
    flow_edges: "Ablaufkanten",
    select_prompt: "Wähle einen Schritt in der Zeichenfläche oder im Dokument aus.",
    step_not_found: "Schritt nicht gefunden",
    kind: "Art",
    params: "Parameter",
    id: "ID",
    step: "Schritt",
    action_set_state: "Zustand setzen",
    action_log_print: "Log ausgeben",
    action_if: "Wenn",
    action_while: "Solange",
    action_add: "Addieren",
    add_to: "Hinzufügen zu",
    run: "Ausführen",
    stop: "Stopp",
    reorganize: "Neu anordnen",
    layout: "Layout",
    left_to_right: "Links nach rechts",
    top_to_bottom: "Oben nach unten",
    window_main: "Sequenz",
    window_script: "Skript",
    window_compiled: "DSL",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; this app has no terminology variants.
fn sequence_labels(view_state: &ViewState) -> &'static SequenceLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &SEQUENCE_LABELS_NATIVE_DE } else { &SEQUENCE_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn build_document_tree(fixture: &SequenceFixture, selected: &[String], labels: &SequenceLabels) -> UiNode {
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
                action: None,
        hover_action: None,
        unhover_action: None,
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
                label: Some(labels.steps.into()),
                default_open: Some(true),
                items: if step_items.is_empty() {
                    vec![tree_item("sequence-play-document.steps.empty", "(none)")]
                } else {
                    step_items
                },
            },
            UiTreeSectionNode {
                id: "sequence-play-document.edges".into(),
                label: Some(labels.flow_edges.into()),
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
        drop_action: None,
    })
}

fn build_catalogue_tree(fixture: &SequenceFixture, labels: &SequenceLabels) -> UiNode {
    let actions = [
        ("state.set", labels.action_set_state),
        ("log.print", labels.action_log_print),
        ("control.if", labels.action_if),
        ("control.while", labels.action_while),
        ("math.add", labels.action_add),
    ];
    let mut items: Vec<UiTreeItemNode> = actions
        .iter()
        .map(|(kind, label)| {
            tree_item_with_action(
                format!("sequence-play-catalogue.action.{kind}"),
                *label,
                Some((*kind).into()),
                sequence_action("addStep", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    for owner in fixture.steps.iter().filter(|step| is_control_kind(&step.kind)) {
        for slot_name in control_slots(&owner.kind) {
            items.push(tree_item_with_action(
                format!("sequence-play-catalogue.slot.{}.{}", owner.id, slot_name),
                format!("{} {} → {slot_name}", labels.add_to, owner.id),
                Some(format!("{slot_name} @ {}", owner.id)),
                sequence_action(
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
        drop_action: None,
    })
}

fn build_inspector_tree(fixture: &SequenceFixture, selected: &[String], labels: &SequenceLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "sequence-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.select_prompt)],
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
            children: vec![ui_text(labels.step_not_found)],
        }]);
    }
    let step_ids: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let mut fields = vec![
        ui_inspector_readonly_field("sequence-play-inspector.kind", labels.kind, steps[0].kind.clone()),
        ui_inspector_readonly_field(
            "sequence-play-inspector.params",
            labels.params,
            serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into()),
        ),
    ];
    if step_ids.len() == 1 {
        fields.insert(
            0,
            ui_inspector_readonly_field("sequence-play-inspector.id", labels.id, step_ids[0].clone()),
        );
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "sequence-play-inspector.step".into(),
        label: labels.step.into(),
        default_open: None,
        fields,
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(fixture: &SequenceFixture, runtime: &SequencePlayRuntime) -> UiNode {
    let mut host = host_from_fixture(fixture);
    host.layout_expanded_slots();
    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let selection_json = if runtime.selected_step_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&runtime.selected_step_ids).ok()
    };
    build_node_graph_scene(
        SEQUENCE_PLAY_SURFACE_MAIN,
        SEQUENCE_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            selection_json,
            context_menu_json: Some(
                r#"[{"id":"delete-selection","label":"Delete selection","action":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
            ),
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_script(fixture: &SequenceFixture, runtime: &SequencePlayRuntime) -> UiNode {
    let host = host_from_fixture(fixture);
    let mut text = host.compile_text();
    if !runtime.last_run_json.is_empty() {
        text.push_str("\n\n# run result\n");
        text.push_str(&runtime.last_run_json);
    }
    build_text_editor_scene(
        SEQUENCE_PLAY_SURFACE_SCRIPT,
        SEQUENCE_PLAY_APP_ID,
        TextEditorScene::base(text, Some("imperative".into()), None),
    )
}

fn render_compiled_dag(fixture: &SequenceFixture) -> UiNode {
    let host = host_from_fixture(fixture);
    build_text_editor_scene(
        SEQUENCE_PLAY_SURFACE_COMPILED,
        SEQUENCE_PLAY_APP_ID,
        TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None),
    )
}
//#endregion 🔖Render

//#region 🔖SequencePlayApp
#[derive(Default)]
struct SequencePlayApp {
    runtime: SequencePlayRuntime,
}

impl SequencePlayApp {
    /// 🔀 Runs a host mutation seeded from the current projection and diffs the result into typed ops.
    fn ops_from_host_mutation(
        &self,
        fixture: &SequenceFixture,
        mutate: impl FnOnce(&mut SequenceHost),
    ) -> Vec<SequenceOp> {
        let mut host = host_from_fixture(fixture);
        mutate(&mut host);
        sequence_fixture_ops(fixture, &host.fixture)
    }
}

impl DocumentApp for SequencePlayApp {
    type Projection = SequenceFixture;
    type Op = SequenceOp;

    fn app_id(&self) -> &str {
        SEQUENCE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        SEQUENCE_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> SequenceFixture {
        default_fixture()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, SequenceFixture>,
        _view_state: &ViewState,
    ) -> ActionEmit<SequenceOp> {
        let fixture = doc.projection;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no ops.
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                self.runtime.selected_step_ids = node_graph_selection_ids(args);
                ActionEmit::default()
            }
            "nodeGraphHover" => ActionEmit::default(),
            "graphPointerDown" => {
                self.runtime.selected_step_ids.clear();
                ActionEmit::default()
            }
            "setOrientation" => {
                let orientation = args.and_then(|value| value.get("orientation")).and_then(|value| value.as_str());
                self.runtime.orientation = match orientation {
                    Some("topBottom") => DagLayoutOrientation::TopBottom,
                    Some("leftRight") => DagLayoutOrientation::LeftRight,
                    _ => return ActionEmit::default(),
                };
                ActionEmit::default()
            }
            "run" => {
                let result = host_from_fixture(fixture).run();
                self.runtime.last_run_json = serde_json::to_string(&result).unwrap_or_default();
                ActionEmit::default()
            }
            "stop" => {
                self.runtime.last_run_json.clear();
                ActionEmit::default()
            }
            // 📷 Camera — a coalesced scalar op so a pan/zoom gesture is one undo step.
            "nodeGraphViewport" => {
                if let Some(camera) = args
                    .and_then(|value| value.get("viewportJson"))
                    .and_then(|value| value.as_str())
                    .and_then(|json| serde_json::from_str(json).ok())
                {
                    return ActionEmit::amend(vec![SequenceOp::SetCamera { camera }], "camera");
                }
                ActionEmit::default()
            }
            // ✏️ Operations — compute the target fixture via the host, emit granular ops.
            "addStep" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print").to_string();
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let mut host = host_from_fixture(fixture);
                let id = host.add_step(&kind, x, y);
                self.runtime.selected_step_ids = vec![id];
                ActionEmit::ops(sequence_fixture_ops(fixture, &host.fixture))
            }
            "addStepToSlot" | "addStepDropped" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print").to_string();
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let picked = args
                    .and_then(|value| value.get("pickedStepId"))
                    .or_else(|| args.and_then(|value| value.get("owner")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let owner = args.and_then(|value| value.get("owner")).and_then(|value| value.as_str()).map(str::to_string);
                let slot = args.and_then(|value| value.get("slotName")).and_then(|value| value.as_str()).map(str::to_string);
                let is_slot = action == "addStepToSlot";
                let mut host = host_from_fixture(fixture);
                let id = if is_slot {
                    match (owner, slot) {
                        (Some(owner), Some(slot)) => host.add_step_in_slot(&kind, x, y, Some(SlotRef { owner, name: slot })),
                        _ => host.add_step(&kind, x, y),
                    }
                } else {
                    host.add_step_dropped(&kind, x, y, picked.as_deref())
                };
                self.runtime.selected_step_ids = vec![id];
                ActionEmit::ops(sequence_fixture_ops(fixture, &host.fixture))
            }
            "removeStep" => {
                let step_id = args
                    .and_then(|value| value.get("id"))
                    .or_else(|| args.and_then(|value| value.get("stepId")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let Some(step_id) = step_id else { return ActionEmit::default() };
                self.runtime.selected_step_ids.retain(|id| id != &step_id);
                ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                    host.remove_step(&step_id);
                }))
            }
            "deleteSelection" => {
                let selected = self.runtime.selected_step_ids.clone();
                let ops = self.ops_from_host_mutation(fixture, |host| {
                    for step_id in &selected {
                        host.remove_step(step_id);
                    }
                });
                if !ops.is_empty() {
                    self.runtime.selected_step_ids.clear();
                }
                ActionEmit::ops(ops)
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if fixture.steps.iter().any(|step| step.id == node_id) {
                        let ops = self.ops_from_host_mutation(fixture, |host| {
                            let mut next = host.fixture.clone();
                            if let Some(step) = next.steps.iter_mut().find(|step| step.id == node_id) {
                                step.x = x;
                                step.y = y;
                            }
                            let _ = host.replace_fixture(next);
                        });
                        return ActionEmit::ops(ops);
                    }
                }
                ActionEmit::default()
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                if let (Some(from), Some(to)) = (from, to) {
                    return ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                        let _ = host.connect_steps(&from, &to);
                    }));
                }
                ActionEmit::default()
            }
            "disconnectSteps" => {
                let from_id = args.and_then(|value| value.get("fromId")).and_then(|value| value.as_str()).map(str::to_string);
                let to_id = args.and_then(|value| value.get("toId")).and_then(|value| value.as_str()).map(str::to_string);
                if let (Some(from_id), Some(to_id)) = (from_id, to_id) {
                    return ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                        host.disconnect_steps(&from_id, &to_id);
                    }));
                }
                ActionEmit::default()
            }
            "setStepParams" => {
                let step_id = args
                    .and_then(|value| value.get("id"))
                    .or_else(|| args.and_then(|value| value.get("stepId")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let params = args.and_then(|value| value.get("params")).map(|value| value.to_string());
                if let (Some(step_id), Some(params)) = (step_id, params) {
                    return ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                        let _ = host.set_step_params_json(&step_id, &params);
                    }));
                }
                ActionEmit::default()
            }
            "setStepCollapsed" => {
                let step_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                let Some(step_id) = step_id else { return ActionEmit::default() };
                let collapsed = fixture
                    .steps
                    .iter()
                    .find(|step| step.id == step_id)
                    .map(|step| !step.collapsed)
                    .unwrap_or(true);
                ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                    host.set_step_collapsed(&step_id, collapsed);
                }))
            }
            "reorganize" => {
                let orientation = self.runtime.orientation;
                ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                    let opts = DagLayoutOptions { orientation, ..DagLayoutOptions::default() };
                    let _ = host.reorganize(&opts);
                }))
            }
            "nodeGraphEdit" => {
                let sub_ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                let selected = self.runtime.selected_step_ids.clone();
                let mut cleared = false;
                let ops = self.ops_from_host_mutation(fixture, |host| {
                    for op in &sub_ops {
                        match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture) = op
                                    .get("fixtureJson")
                                    .and_then(|value| value.as_str())
                                    .and_then(|json| serde_json::from_str::<SequenceFixture>(json).ok())
                                {
                                    let _ = host.replace_fixture(fixture);
                                }
                            }
                            "deleteSelection" => {
                                for step_id in &selected {
                                    if host.remove_step(step_id) {
                                        cleared = true;
                                    }
                                }
                            }
                            "connect" => {
                                let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                                let to = op.get("targetNodeId").and_then(|value| value.as_str());
                                if let (Some(from), Some(to)) = (from, to) {
                                    let _ = host.connect_steps(from, to);
                                }
                            }
                            _ => {}
                        }
                    }
                });
                if cleared {
                    self.runtime.selected_step_ids.clear();
                }
                ActionEmit::ops(ops)
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, SequenceFixture>, view_state: &ViewState) -> UiNode {
        let fixture = doc.projection;
        let labels = sequence_labels(view_state);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => render_main_graph(fixture, &self.runtime),
            SEQUENCE_PLAY_BODY_SCRIPT => render_script(fixture, &self.runtime),
            SEQUENCE_PLAY_BODY_COMPILED => render_compiled_dag(fixture),
            SEQUENCE_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &self.runtime.selected_step_ids, labels),
            SEQUENCE_PLAY_BODY_CATALOGUE => build_catalogue_tree(fixture, labels),
            SEQUENCE_PLAY_BODY_INSPECTOR => build_inspector_tree(fixture, &self.runtime.selected_step_ids, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = sequence_labels(view_state);
        AppLabelsOverlay {
            app_label: None,
            window_kind_labels: HashMap::from([
                (SEQUENCE_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string()),
                (SEQUENCE_PLAY_WINDOW_SCRIPT.to_string(), labels.window_script.to_string()),
                (SEQUENCE_PLAY_WINDOW_COMPILED.to_string(), labels.window_compiled.to_string()),
            ]),
            panel_tab_labels: HashMap::new(),
            mode_labels: HashMap::new(),
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
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .operation("addStep", "Add Step")
            .operation("addStepToSlot", "Add Step To Slot")
            .operation("addStepDropped", "Add Step Dropped")
            .operation("removeStep", "Remove Step")
            .operation("deleteSelection", "Delete Selection")
            .operation("moveMediaNode", "Move Step")
            .operation("connectMediaPorts", "Connect Steps")
            .operation("disconnectSteps", "Disconnect Steps")
            .operation("setStepParams", "Set Step Params")
            .operation("setStepCollapsed", "Set Step Collapsed")
            .operation("reorganize", "Reorganize")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("nodeGraphViewport", "Node Graph Viewport")
            // 👁️ Ephemeral view state — selection, run output, layout orientation.
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Node Graph Select")
            .view_action("nodeGraphHover", "Node Graph Hover")
            .view_action("graphPointerDown", "Graph Pointer Down")
            .view_action("setOrientation", "Set Orientation")
            .view_action("run", "Run")
            .view_action("stop", "Stop")
            // 📝 Staged argument forms for the panel-visible create + layout actions.
            .action_args("addStep", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("state.set", "Set State"),
                    ActionArgOption::new("log.print", "Print"),
                    ActionArgOption::new("control.if", "If"),
                    ActionArgOption::new("control.while", "While"),
                    ActionArgOption::new("math.add", "Add"),
                ]).default_value("log.print"),
            ])
            .action_args("setOrientation", vec![
                ActionArgDef::select("orientation", "Orientation", vec![
                    ActionArgOption::new("leftRight", "Left to Right"),
                    ActionArgOption::new("topBottom", "Top to Bottom"),
                ]).required(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_fixture()).unwrap())
    .program("sequence", "Sequence", "graph")
}

fn register_sequence_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "sequence",
    label: "Sequence",
    version: "0.1.0",
    setup: register_sequence_exports,
    apps: [ create_sequence_app => SequencePlayApp ],
}
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
    use vcs::{Backbone, BackboneMessage, MemoryBackbone};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<SequencePlayApp> {
        VcsDocumentApp::new(SequencePlayApp::default())
    }

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app();
        let node = app.render(SEQUENCE_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn renders_script_editor() {
        let mut app = new_app();
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("text-editor"));
    }

    #[test]
    fn default_fixture_has_steps() {
        assert_eq!(default_fixture().steps.len(), 2);
    }

    #[test]
    fn add_step_action_appends_step() {
        let mut app = new_app();
        app.handle_action("addStep", Some(&json!({ "kind": "log.print" })), &ViewState::default(), &meta("local")).expect("add");
        assert!(app.projection().expect("projection").steps.len() > 2);
    }

    #[test]
    fn run_stores_result_and_renders_in_script() {
        let mut app = new_app();
        let result = app.handle_action("run", None, &ViewState::default(), &meta("local")).expect("run");
        assert!(result.operations.is_empty(), "run is a view action and emits no ops");
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("run result"));
    }

    #[test]
    fn remove_step_action_deletes_step() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        app.handle_action("removeStep", Some(&json!({ "id": step_id })), &ViewState::default(), &meta("local")).expect("remove");
        assert!(app.projection().expect("projection").steps.iter().all(|step| step.id != step_id));
    }

    #[test]
    fn footer_tools_include_run_stop_reorganize_and_orientation() {
        let mut app = new_app();
        let json = serde_json::to_string(&app.tools(&ViewState::default())).unwrap();
        for id in ["sequence-tools-run", "sequence-tools-stop", "sequence-tools-reorganize", "sequence-tools-orientation-lr", "sequence-tools-orientation-tb"] {
            assert!(json.contains(&format!("\"id\":\"{id}\"")), "tools expose {id}");
        }
    }

    #[test]
    fn set_orientation_action_flips_toggle_state() {
        let mut app = new_app();
        app.handle_action("setOrientation", Some(&json!({ "orientation": "topBottom" })), &ViewState::default(), &meta("local")).expect("orientation");
        let tools_json = serde_json::to_string(&app.tools(&ViewState::default())).unwrap();
        let tb_pressed = tools_json
            .split(r#""id":"sequence-tools-orientation-tb""#)
            .nth(1)
            .and_then(|rest| rest.split_once("\"pressed\":"))
            .map(|(_, rest)| rest.starts_with("true"))
            .unwrap_or(false);
        assert!(tb_pressed, "top-to-bottom toggle should be pressed, got {tools_json}");
    }

    #[test]
    fn reorganize_action_spreads_step_positions_apart() {
        let mut app = new_app();
        // Collapse both steps onto the origin, then reorganize.
        let ids: Vec<String> = app.projection().expect("projection").steps.iter().map(|step| step.id.clone()).collect();
        for id in &ids {
            app.handle_action("moveMediaNode", Some(&json!({ "nodeId": id, "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta("local")).expect("move");
        }
        app.handle_action("reorganize", None, &ViewState::default(), &meta("local")).expect("reorganize");
        let xs: Vec<f64> = app.projection().expect("projection").steps.iter().map(|step| step.x).collect();
        assert!(xs.iter().any(|x| *x != 0.0), "reorganize should spread steps apart, got {xs:?}");
    }

    #[test]
    fn stop_action_clears_last_run_result() {
        let mut app = new_app();
        app.handle_action("run", None, &ViewState::default(), &meta("local")).expect("run");
        app.handle_action("stop", None, &ViewState::default(), &meta("local")).expect("stop");
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        assert!(!serde_json::to_string(&node).unwrap().contains("run result"));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        app.handle_action("addStep", Some(&json!({ "kind": "log.print" })), &ViewState::default(), &meta("local")).expect("add");
        assert_eq!(app.projection().expect("projection").steps.len(), 3);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").steps.len(), 2);
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").steps.len(), 3);
    }

    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://sequence-convergence", "mem://sequence-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        // A moves step-1; B moves step-2 — disjoint step patches.
        instance_a
            .handle_action("moveMediaNode", Some(&json!({ "nodeId": "step-1", "x": 111.0, "y": 0.0 })), &ViewState::default(), &meta("actor-a"))
            .expect("a moves step-1");
        instance_b
            .handle_action("moveMediaNode", Some(&json!({ "nodeId": "step-2", "x": 222.0, "y": 0.0 })), &ViewState::default(), &meta("actor-b"))
            .expect("b moves step-2");

        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        let x_of = |fixture: &SequenceFixture, id: &str| fixture.steps.iter().find(|step| step.id == id).map(|step| step.x).unwrap();
        assert_eq!(x_of(&projection_a, "step-1"), 111.0, "A keeps its own edit");
        assert_eq!(x_of(&projection_a, "step-2"), 222.0, "A converges on B's edit");
        assert_eq!(x_of(&projection_b, "step-1"), 111.0, "B converges on A's edit");
        assert_eq!(x_of(&projection_b, "step-2"), 222.0, "B keeps its own edit");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        let mut sender = new_app();
        let (near, mut far) = MemoryBackbone::pair("mem://sequence-doc", "mem://sequence-doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        sender
            .handle_action("moveMediaNode", Some(&json!({ "nodeId": "step-1", "x": 99.0, "y": 0.0 })), &ViewState::default(), &meta("local"))
            .expect("move");
        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Ops { envelopes: ops } = message {
                envelopes.extend(ops);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied op on the channel");
        let operations_json = serde_json::to_string(&envelopes).expect("serialize");
        let mut receiver = new_app();
        receiver.ingest_operations(&operations_json).expect("ingest once");
        receiver.ingest_operations(&operations_json).expect("ingest twice");
        let step_x = receiver.projection().expect("projection").steps.iter().find(|step| step.id == "step-1").unwrap().x;
        assert_eq!(step_x, 99.0, "feeding the same op twice must not double-apply");
    }

    #[test]
    fn sequence_labels_render_native_english_by_default() {
        let mut app = new_app();
        let document_json = serde_json::to_string(&app.render(SEQUENCE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(document_json.contains("\"Steps\""));
        assert!(document_json.contains("\"Flow edges\""));
        let tools_json = serde_json::to_string(&app.tools(&ViewState::default())).unwrap();
        for label in ["\"Run\"", "\"Stop\"", "\"Reorganize\"", "\"Left to right\"", "\"Top to bottom\""] {
            assert!(tools_json.contains(label), "tools expose {label}");
        }
    }

    #[test]
    fn sequence_labels_render_german_locale() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_json = serde_json::to_string(&app.render(SEQUENCE_PLAY_BODY_DOCUMENT, None, &view_state).expect("render")).unwrap();
        assert!(document_json.contains("Schritte"));
        assert!(document_json.contains("Ablaufkanten"));
        assert!(!document_json.contains("\"Steps\""));
        let tools_json = serde_json::to_string(&app.tools(&view_state)).unwrap();
        for label in ["Ausführen", "Stopp", "Neu anordnen", "Links nach rechts", "Oben nach unten"] {
            assert!(tools_json.contains(label), "tools expose {label}");
        }
    }
}
