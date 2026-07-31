//! 🖥️ Sequence app — DocumentApp impl, render, manifest (constitutional: ui).

use infinite_board_port_directed_dag::{DagFixture, DagLayoutOptions, DagLayoutOrientation};
use semio_framework_plugin::{
    app_labels, build_node_graph_scene, build_text_editor_scene, create_default_layout, is_de_locale, localized_label_map, resolve_labels, selection_ids as sdk_selection_ids, tree_item_desc, tree_item_with_action, ui_declarative_sections_to_tree,
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDescriptor, ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup,
    PanelTreeBuilder, ArtifactKindSpec, SurfaceKind, TextEditorScene, UiControlNode, UiInspectorFieldGroup, UiNode, UiPresence, UiToggleNode, UiTreeItemNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use sequence::{default_fixture, SequenceCamera, SequenceFixture, SequenceStep, SlotRef, SEQUENCE_FIXTURE_SCHEMA};
use sequence_engine::{control_slots, is_control_kind, sequence_example_json, SequenceHost};
use sequence_op::{sequence_fixture_operations, SequenceOperation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
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
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🎛️ Ephemeral view state (selection, last run output, layout orientation) held in the app struct,
/// never in the document — so it stays out of undo history and off the operation channel.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SequencePlayRuntime {
    selected_step_ids: Vec<String>,
    last_run_json: String,
    orientation: DagLayoutOrientation,
    /// 🎥️ The node-graph viewport pan/zoom — session-only view state (never a VCS-tracked document
    /// field): see `"nodeGraphViewport"` in `handle_action` below.
    camera: SequenceCamera,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowDiagramPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<WorkflowDiagramPortRecord>,
    outputs: Vec<WorkflowDiagramPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
fn sequence_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: SEQUENCE_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🧰️ Builds a {@link SequenceHost} seeded from a projection so an action can mutate it (with all the
/// host's cycle/slot/layout logic) and then diff the result into typed operations.
fn host_from_fixture(fixture: &SequenceFixture) -> SequenceHost {
    SequenceHost::from_fixture(fixture.clone())
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), "next".into()))
}

fn fixture_to_workflow(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<WorkflowNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| WorkflowNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| WorkflowDiagramPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()) }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| WorkflowDiagramPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()) }).collect(),
        })
        .collect();
    let edges: Vec<WorkflowEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            WorkflowEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id }
        })
        .collect();
    (serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()))
}

/// 🗣️ Localizes a control-flow slot name ("then"/"else"/"body") for tree display; unknown slot names fall back to the raw id.
fn slot_label<'a>(slot_name: &'a str, labels: &'a SequenceLabels) -> &'a str {
    match slot_name {
        "then" => labels.slot_then,
        "else" => labels.slot_else,
        "body" => labels.slot_body,
        other => other,
    }
}

fn build_step_tree_item(step: &SequenceStep, fixture: &SequenceFixture, labels: &SequenceLabels) -> UiTreeItemNode {
    let mut item = tree_item_with_action(format!("sequence-play-document.step.{}", step.id), format!("{} ({})", step.id, step.kind), Some(step.kind.clone()), sequence_action("setSelection", Some(json!({ "ids": [step.id.clone()] }))));
    if is_control_kind(&step.kind) {
        item.control = Some(UiControlNode::Toggle(UiToggleNode {
            id: format!("sequence-play-document.collapse.{}", step.id),
            icon_id: if step.collapsed { "chevron-right" } else { "chevron-down" }.into(),
            presence: UiPresence::selected(!step.collapsed),
            text: None,
            on_change: sequence_action("setStepCollapsed", Some(json!({ "id": step.id }))),
            menu: None,
        }));
        let slot_items: Vec<UiTreeItemNode> = control_slots(&step.kind)
            .iter()
            .map(|slot_name| {
                let nested: Vec<UiTreeItemNode> = fixture.steps.iter().filter(|entry| entry.slot.as_ref().is_some_and(|slot| slot.owner == step.id && slot.name == *slot_name)).map(|entry| build_step_tree_item(entry, fixture, labels)).collect();
                UiTreeItemNode {
                    id: format!("sequence-play-document.slot.{}.{}", step.id, slot_name),
                    label: slot_label(slot_name, labels).into(),
                    description: Some(format!("{} {}", step.id, labels.slot)),
                    icon_id: Some("folder".into()),
                    presence: UiPresence::default(),
                    default_open: Some(true),
                    action: None,
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: if nested.is_empty() { None } else { Some(nested) },
                    control: None,
                    dimmed: if step.collapsed { Some(true) } else { None },
                    menu: None,
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
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the sequence app; one field per label makes every locale combination compile-checked.
    struct SequenceLabels {
        steps: &'static str = en: "Steps", de: "Schritte";
        flow_edges: &'static str = en: "Flow edges", de: "Ablaufkanten";
        select_prompt: &'static str = en: "Select a step in the canvas or document.", de: "Wähle einen Schritt in der Zeichenfläche oder im Dokument aus.";
        step_not_found: &'static str = en: "Step not found", de: "Schritt nicht gefunden";
        kind: &'static str = en: "Kind", de: "Art";
        params: &'static str = en: "Params", de: "Parameter";
        id: &'static str = en: "Id", de: "ID";
        step: &'static str = en: "Step", de: "Schritt";
        action_set_state: &'static str = en: "Set state", de: "Zustand setzen";
        action_log_print: &'static str = en: "Print log", de: "Log ausgeben";
        action_if: &'static str = en: "If", de: "Wenn";
        action_while: &'static str = en: "While", de: "Solange";
        action_add: &'static str = en: "Add", de: "Addieren";
        add_to: &'static str = en: "Add to", de: "Hinzufügen zu";
        run: &'static str = en: "Run", de: "Ausführen";
        stop: &'static str = en: "Stop", de: "Stopp";
        reorganize: &'static str = en: "Reorganize", de: "Neu anordnen";
        layout: &'static str = en: "Layout", de: "Layout";
        left_to_right: &'static str = en: "Left to right", de: "Links nach rechts";
        top_to_bottom: &'static str = en: "Top to bottom", de: "Oben nach unten";
        window_main: &'static str = en: "Sequence", de: "Sequenz";
        window_script: &'static str = en: "Script", de: "Skript";
        window_compiled: &'static str = en: "DSL", de: "DSL";
        none: &'static str = en: "(none)", de: "(keine)";
        slot: &'static str = en: "slot", de: "Slot";
        slot_then: &'static str = en: "Then", de: "Dann";
        slot_else: &'static str = en: "Else", de: "Sonst";
        slot_body: &'static str = en: "Body", de: "Rumpf";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_sequence_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn sequence_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(
        is_de,
        &[
            ("addStep", "Add Step", "Schritt hinzufügen"),
            ("addStepToSlot", "Add Step To Slot", "Schritt zu Slot hinzufügen"),
            ("addStepDropped", "Add Step Dropped", "Schritt per Ablegen hinzufügen"),
            ("removeStep", "Remove Step", "Schritt entfernen"),
            ("deleteSelection", "Delete Selection", "Auswahl löschen"),
            ("moveMediaNode", "Move Step", "Schritt verschieben"),
            ("connectMediaPorts", "Connect Steps", "Schritte verbinden"),
            ("disconnectSteps", "Disconnect Steps", "Schritte trennen"),
            ("setStepParams", "Set Step Params", "Schrittparameter festlegen"),
            ("setStepCollapsed", "Set Step Collapsed", "Schritt einklappen"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
            ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("selectNode", "Select Node", "Knoten auswählen"),
            ("nodeGraphSelect", "Node Graph Select", "Knotengraph-Auswahl"),
            ("nodeGraphHover", "Node Graph Hover", "Knotengraph-Hover"),
            ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
            ("setOrientation", "Set Orientation", "Ausrichtung festlegen"),
            ("run", "Run", "Ausführen"),
            ("stop", "Stop", "Stopp"),
        ],
    )
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_sequence_app`;
/// this manifest declares none, so this is an empty overlay kept for parity with the shared `app_labels` wiring.
fn sequence_utility_labels(_is_de: bool) -> HashMap<String, String> {
    HashMap::new()
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
fn build_document_tree(fixture: &SequenceFixture, selected: &[String], labels: &SequenceLabels) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = fixture.steps.iter().filter(|step| step.slot.is_none()).map(|step| build_step_tree_item(step, fixture, labels)).collect();
    let edge_items: Vec<UiTreeItemNode> = fixture.edges.iter().map(|edge| tree_item_desc(format!("sequence-play-document.edge.{}", edge.id), format!("{} → {}", edge.from, edge.to), Some(edge.id.clone()))).collect();
    PanelTreeBuilder::new("sequence-play-document")
        .section_or_placeholder("sequence-play-document.steps", Some(labels.steps.into()), true, step_items, labels.none)
        .section_or_placeholder("sequence-play-document.edges", Some(labels.flow_edges.into()), false, edge_items, labels.none)
        .selected(selected.iter().map(|id| format!("sequence-play-document.step.{id}")).collect())
        .build()
}

fn build_catalogue_tree(fixture: &SequenceFixture, labels: &SequenceLabels) -> UiNode {
    let actions = [("state.set", labels.action_set_state), ("log.print", labels.action_log_print), ("control.if", labels.action_if), ("control.while", labels.action_while), ("math.add", labels.action_add)];
    let mut items: Vec<UiTreeItemNode> = actions.iter().map(|(kind, label)| tree_item_with_action(format!("sequence-play-catalogue.action.{kind}"), *label, Some((*kind).into()), sequence_action("addStep", Some(json!({ "kind": kind }))))).collect();
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
    PanelTreeBuilder::new("sequence-play-catalogue").section("sequence-play-catalogue.actions", Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()), true, items).selected(vec![]).build()
}

fn build_inspector_tree(fixture: &SequenceFixture, selected: &[String], labels: &SequenceLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "sequence-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.select_prompt)],
            menu: None,
        }]);
    }
    let steps: Vec<&SequenceStep> = selected.iter().filter_map(|id| fixture.steps.iter().find(|step| &step.id == id)).collect();
    if steps.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "sequence-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.step_not_found)],
            menu: None,
        }]);
    }
    let step_ids: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let mut fields = vec![
        ui_inspector_readonly_field("sequence-play-inspector.kind", labels.kind, steps[0].kind.clone()),
        ui_inspector_readonly_field("sequence-play-inspector.params", labels.params, serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into())),
    ];
    if step_ids.len() == 1 {
        fields.insert(0, ui_inspector_readonly_field("sequence-play-inspector.id", labels.id, step_ids[0].clone()));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "sequence-play-inspector.step".into(), label: labels.step.into(), default_open: None, fields }])
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_main_graph(fixture: &SequenceFixture, runtime: &SequencePlayRuntime) -> UiNode {
    let mut host = host_from_fixture(fixture);
    host.layout_expanded_slots();
    let (nodes_json, edges_json) = fixture_to_workflow(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&runtime.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let selection_json = if runtime.selected_step_ids.is_empty() { None } else { serde_json::to_string(&runtime.selected_step_ids).ok() };
    build_node_graph_scene(
        SEQUENCE_PLAY_SURFACE_MAIN,
        SEQUENCE_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            selection_json,
            context_menu_json: Some(r#"[{"id":"delete-selection","label":"Delete selection","icon":"trash","action":"nodeGraphEdit","args":{"operations":[{"operation":"deleteSelection"}]},"destructive":true}]"#.into()),
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
    build_text_editor_scene(SEQUENCE_PLAY_SURFACE_SCRIPT, SEQUENCE_PLAY_APP_ID, TextEditorScene::base(text, Some("imperative".into()), None))
}

fn render_compiled_dag(fixture: &SequenceFixture) -> UiNode {
    let host = host_from_fixture(fixture);
    build_text_editor_scene(SEQUENCE_PLAY_SURFACE_COMPILED, SEQUENCE_PLAY_APP_ID, TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🔖️SequencePlayApp
#[derive(Default)]
pub struct SequencePlayApp {
    runtime: SequencePlayRuntime,
}

impl SequencePlayApp {
    /// 🔀️ Runs a host mutation seeded from the current projection and diffs the result into typed operations.
    fn ops_from_host_mutation(&self, fixture: &SequenceFixture, mutate: impl FnOnce(&mut SequenceHost)) -> Vec<SequenceOperation> {
        let mut host = host_from_fixture(fixture);
        mutate(&mut host);
        sequence_fixture_operations(fixture, &host.fixture)
    }
}

impl DocumentApp for SequencePlayApp {
    type Projection = SequenceFixture;
    type Operation = SequenceOperation;

    fn app_id(&self) -> &str {
        SEQUENCE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        SEQUENCE_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> SequenceFixture {
        default_fixture()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, SequenceFixture>, _view_state: &ViewState) -> ActionEmit<SequenceOperation> {
        let fixture = doc.projection;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no operations.
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
            // 👁️ View action: the node-graph viewport never touches the document — it's written
            // straight into `self.runtime`, session-only, no VCS edit, no undo entry.
            "nodeGraphViewport" => {
                if let Some(camera) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()).and_then(|json| serde_json::from_str(json).ok()) {
                    self.runtime.camera = camera;
                }
                ActionEmit::default()
            }
            // ✏️ Operations — compute the target fixture via the host, emit granular operations.
            "addStep" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print").to_string();
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let mut host = host_from_fixture(fixture);
                let id = host.add_step(&kind, x, y);
                self.runtime.selected_step_ids = vec![id];
                ActionEmit::operations(sequence_fixture_operations(fixture, &host.fixture))
            }
            "addStepToSlot" | "addStepDropped" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print").to_string();
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let picked = args.and_then(|value| value.get("pickedStepId")).or_else(|| args.and_then(|value| value.get("owner"))).and_then(|value| value.as_str()).map(str::to_string);
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
                ActionEmit::operations(sequence_fixture_operations(fixture, &host.fixture))
            }
            "removeStep" => {
                let step_id = args.and_then(|value| value.get("id")).or_else(|| args.and_then(|value| value.get("stepId"))).and_then(|value| value.as_str()).map(str::to_string);
                let Some(step_id) = step_id else { return ActionEmit::default() };
                self.runtime.selected_step_ids.retain(|id| id != &step_id);
                ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                    host.remove_step(&step_id);
                }))
            }
            "deleteSelection" => {
                let selected = self.runtime.selected_step_ids.clone();
                let operations = self.ops_from_host_mutation(fixture, |host| {
                    for step_id in &selected {
                        host.remove_step(step_id);
                    }
                });
                if !operations.is_empty() {
                    self.runtime.selected_step_ids.clear();
                }
                ActionEmit::operations(operations)
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if fixture.steps.iter().any(|step| step.id == node_id) {
                        let operations = self.ops_from_host_mutation(fixture, |host| {
                            let mut next = host.fixture.clone();
                            if let Some(step) = next.steps.iter_mut().find(|step| step.id == node_id) {
                                step.x = x;
                                step.y = y;
                            }
                            let _ = host.replace_fixture(next);
                        });
                        return ActionEmit::operations(operations);
                    }
                }
                ActionEmit::default()
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                if let (Some(from), Some(to)) = (from, to) {
                    return ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                        let _ = host.connect_steps(&from, &to);
                    }));
                }
                ActionEmit::default()
            }
            "disconnectSteps" => {
                let from_id = args.and_then(|value| value.get("fromId")).and_then(|value| value.as_str()).map(str::to_string);
                let to_id = args.and_then(|value| value.get("toId")).and_then(|value| value.as_str()).map(str::to_string);
                if let (Some(from_id), Some(to_id)) = (from_id, to_id) {
                    return ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                        host.disconnect_steps(&from_id, &to_id);
                    }));
                }
                ActionEmit::default()
            }
            "setStepParams" => {
                let step_id = args.and_then(|value| value.get("id")).or_else(|| args.and_then(|value| value.get("stepId"))).and_then(|value| value.as_str()).map(str::to_string);
                let params = args.and_then(|value| value.get("params")).map(|value| value.to_string());
                if let (Some(step_id), Some(params)) = (step_id, params) {
                    return ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                        let _ = host.set_step_params_json(&step_id, &params);
                    }));
                }
                ActionEmit::default()
            }
            "setStepCollapsed" => {
                let step_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                let Some(step_id) = step_id else { return ActionEmit::default() };
                let collapsed = fixture.steps.iter().find(|step| step.id == step_id).map(|step| !step.collapsed).unwrap_or(true);
                ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                    host.set_step_collapsed(&step_id, collapsed);
                }))
            }
            "reorganize" => {
                let orientation = self.runtime.orientation;
                ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                    let opts = DagLayoutOptions { orientation, ..DagLayoutOptions::default() };
                    let _ = host.reorganize(&opts);
                }))
            }
            "nodeGraphEdit" => {
                let sub_operations = args.and_then(|value| value.get("operations")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let selected = self.runtime.selected_step_ids.clone();
                let mut cleared = false;
                let operations = self.ops_from_host_mutation(fixture, |host| {
                    for operation in &sub_operations {
                        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<SequenceFixture>(json).ok()) {
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
                                let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                                let to = operation.get("targetNodeId").and_then(|value| value.as_str());
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
                ActionEmit::operations(operations)
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, SequenceFixture>, view_state: &ViewState) -> UiNode {
        let fixture = doc.projection;
        let labels = resolve_labels::<SequenceLabels>(view_state);
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
        let labels = resolve_labels::<SequenceLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(SEQUENCE_PLAY_WINDOW_MAIN, labels.window_main)
            .window_kind_label(SEQUENCE_PLAY_WINDOW_SCRIPT, labels.window_script)
            .window_kind_label(SEQUENCE_PLAY_WINDOW_COMPILED, labels.window_compiled)
            .action_labels(sequence_action_labels(is_de))
            .utility_labels(sequence_utility_labels(is_de))
    }
}

fn node_graph_selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("nodeIds")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_else(|| selection_ids(args))
}

/// 🎯️ Falls back to a singular `nodeId` key when the SDK's `ids`-array parsing comes up empty — this
/// app's node-graph pointer actions address a step by `nodeId`, not `ids`.
fn selection_ids(args: Option<&Value>) -> Vec<String> {
    let ids = sdk_selection_ids(args);
    if !ids.is_empty() {
        return ids;
    }
    args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()]).unwrap_or_default()
}
//#endregion 🔖️SequencePlayApp

//#region 🔖️Manifest
pub fn create_sequence_app() -> App {
    App::from_builder(
        App::builder(SEQUENCE_PLAY_APP_ID, "Sequence").document(["semio", "sequence"])
            .artifact_kind(ArtifactKindSpec {
                id: "computation.sequence".into(),
                name: "Sequence".into(),
                source_format: "sequence.fixture".into(),
                component_kind: "sequence".into(),
                dimension: "graph".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Sequence },
                schema: "sequence.fixture".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("sequence")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(SEQUENCE_PLAY_WINDOW_MAIN, "Sequence", SEQUENCE_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "list-ordered")
            .window_kind(SEQUENCE_PLAY_WINDOW_SCRIPT, "Script", SEQUENCE_PLAY_BODY_SCRIPT, SurfaceKind::TextEditor, "file-code")
            .window_kind(SEQUENCE_PLAY_WINDOW_COMPILED, "DSL", SEQUENCE_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph, "code")
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
            .view_action("nodeGraphViewport", "Node Graph Viewport")
            // 👁️ Ephemeral view state — selection, run output, layout orientation.
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Node Graph Select")
            .view_action("nodeGraphHover", "Node Graph Hover")
            .view_action("graphPointerDown", "Graph Pointer Down")
            .view_action("setOrientation", "Set Orientation")
            .view_action("run", "Run")
            .view_action("stop", "Stop")
            // 📝️ Staged argument forms for the panel-visible create + layout actions.
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
    .example("demo", "Demo", sequence_example_json())
    .workflow("sequence", "Sequence", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<SequencePlayApp> {
        testkit::new_app::<SequencePlayApp>()
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
        app.handle_action("addStep", Some(&json!({ "kind": "log.print" })), &ViewState::default(), &testkit::meta("local")).expect("add");
        assert!(app.projection().expect("projection").steps.len() > 2);
    }

    #[test]
    fn run_stores_result_and_renders_in_script() {
        let mut app = new_app();
        let result = app.handle_action("run", None, &ViewState::default(), &testkit::meta("local")).expect("run");
        assert!(result.operations.is_empty(), "run is a view action and emits no operations");
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("run result"));
    }

    /// 🎥️ `"nodeGraphViewport"` is a View action — it must never emit a `SequenceOperation` (no VCS
    /// edit, no undo entry) and instead write straight into `self.runtime`.
    #[test]
    fn node_graph_viewport_writes_runtime_not_operations() {
        let mut app = new_app();
        let result = app
            .handle_action("nodeGraphViewport", Some(&json!({ "viewportJson": r#"{"x":5.0,"y":6.0,"zoom":2.0}"# })), &ViewState::default(), &testkit::meta("local"))
            .expect("viewport pan/zoom");
        assert!(result.operations.is_empty(), "nodeGraphViewport must not emit a VCS operation");
        let node = app.render(SEQUENCE_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let viewport: Value = serde_json::from_str(payload["nodeGraph"]["viewportJson"].as_str().unwrap()).unwrap();
        assert_eq!(viewport["zoom"], json!(2.0));
    }

    #[test]
    fn remove_step_action_deletes_step() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        app.handle_action("removeStep", Some(&json!({ "id": step_id })), &ViewState::default(), &testkit::meta("local")).expect("remove");
        assert!(app.projection().expect("projection").steps.iter().all(|step| step.id != step_id));
    }

    // 🧰️ `footer_tools_include_run_stop_reorganize_and_orientation` asserted on `VcsDocumentApp::tools()`,
    // which no longer exists (utility bars are derived by the renderer from the utility registry now — see
    // `sequence_utility_labels` above; this manifest declares no utilities, so run/stop/reorganize have
    // no utility-bar equivalent to assert on). Its behavioral coverage lives on in
    // `run_stores_result_and_renders_in_script`, `stop_action_clears_last_run_result`, and
    // `reorganize_action_spreads_step_positions_apart`.

    #[test]
    fn set_orientation_action_changes_reorganize_layout_axis() {
        let mut app = new_app();
        app.handle_action("setOrientation", Some(&json!({ "orientation": "topBottom" })), &ViewState::default(), &testkit::meta("local")).expect("orientation");
        let ids: Vec<String> = app.projection().expect("projection").steps.iter().map(|step| step.id.clone()).collect();
        for id in &ids {
            app.handle_action("moveMediaNode", Some(&json!({ "nodeId": id, "x": 0.0, "y": 0.0 })), &ViewState::default(), &testkit::meta("local")).expect("move");
        }
        app.handle_action("reorganize", None, &ViewState::default(), &testkit::meta("local")).expect("reorganize");
        let ys: Vec<f64> = app.projection().expect("projection").steps.iter().map(|step| step.y).collect();
        assert!(ys.iter().any(|y| *y != 0.0), "topBottom orientation should spread steps vertically, got {ys:?}");
    }

    #[test]
    fn reorganize_action_spreads_step_positions_apart() {
        let mut app = new_app();
        // Collapse both steps onto the origin, then reorganize.
        let ids: Vec<String> = app.projection().expect("projection").steps.iter().map(|step| step.id.clone()).collect();
        for id in &ids {
            app.handle_action("moveMediaNode", Some(&json!({ "nodeId": id, "x": 0.0, "y": 0.0 })), &ViewState::default(), &testkit::meta("local")).expect("move");
        }
        app.handle_action("reorganize", None, &ViewState::default(), &testkit::meta("local")).expect("reorganize");
        let xs: Vec<f64> = app.projection().expect("projection").steps.iter().map(|step| step.x).collect();
        assert!(xs.iter().any(|x| *x != 0.0), "reorganize should spread steps apart, got {xs:?}");
    }

    #[test]
    fn stop_action_clears_last_run_result() {
        let mut app = new_app();
        app.handle_action("run", None, &ViewState::default(), &testkit::meta("local")).expect("run");
        app.handle_action("stop", None, &ViewState::default(), &testkit::meta("local")).expect("stop");
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        assert!(!serde_json::to_string(&node).unwrap().contains("run result"));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(&mut app, "addStep", Some(&json!({ "kind": "log.print" })), |app| app.projection().expect("projection").steps.len(), 2, 3);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
    /// apply DISJOINT edits (A moves step-1, B moves step-2), and exchanging operations over a `MemoryBackbone`
    /// converges both sides onto an identical projection.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<SequencePlayApp, _>(
            "mem://sequence-convergence",
            ("moveMediaNode", Some(&json!({ "nodeId": "step-1", "x": 111.0, "y": 0.0 }))),
            ("moveMediaNode", Some(&json!({ "nodeId": "step-2", "x": 222.0, "y": 0.0 }))),
            |app| app.projection().expect("projection"),
        );
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<SequencePlayApp, _>("moveMediaNode", Some(&json!({ "nodeId": "step-1", "x": 99.0, "y": 0.0 })), |app| app.projection().expect("projection").steps.iter().find(|step| step.id == "step-1").unwrap().x);
    }

    #[test]
    fn sequence_labels_render_native_english_by_default() {
        let mut app = new_app();
        let document_json = serde_json::to_string(&app.render(SEQUENCE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(document_json.contains("\"Steps\""));
        assert!(document_json.contains("\"Flow edges\""));
        // 🧰️ Run/stop/reorganize no longer render as utility bar utilities (see note on the removed
        // `footer_tools_include_run_stop_reorganize_and_orientation` test above) — their locale
        // translation now surfaces only through the action-label overlay.
        let action_labels = app.app_labels(&ViewState::default()).action_labels;
        for (id, label) in [("run", "Run"), ("stop", "Stop"), ("reorganize", "Reorganize")] {
            assert_eq!(action_labels.get(id).map(String::as_str), Some(label), "{id} action label");
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
        let action_labels = app.app_labels(&view_state).action_labels;
        for (id, label) in [("run", "Ausführen"), ("stop", "Stopp"), ("reorganize", "Neu anordnen")] {
            assert_eq!(action_labels.get(id).map(String::as_str), Some(label), "{id} action label");
        }
    }
}
//#endregion 🧪️Tests
