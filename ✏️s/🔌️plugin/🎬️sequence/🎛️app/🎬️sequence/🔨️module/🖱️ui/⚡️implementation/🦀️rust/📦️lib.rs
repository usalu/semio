//! 🖥️ Sequence app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! pivot — `SequencePlayApp` is a unit struct; every former `SequencePlayRuntime` field (selection,
//! last-run output, layout orientation, node-graph viewport camera) now lives in
//! `sequence_engine::SequenceConfig`, written via `sequence_op::SequenceConfigOperation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `sequence_protocol::SequenceCommand` channel via `DocumentApp::handle`.

use infinite_board_port_directed_dag::{DagFixture, DagLayoutOptions, DagLayoutOrientation};
use semio_framework_plugin::{
    app_labels, build_node_graph_scene, build_text_editor_scene, create_default_layout, localized_label_map, tree_item_desc, tree_item_with_action, ui_declarative_sections_to_tree,
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, AppIo, ActionArgDef, ActionArgOption, ActionDescriptor, App, AppActionRegistry, AppLabelsOverlay, AppLabelsOverlayExt, ConfigFieldShape, ConfigFieldSpec, ConfigSpec, ConfigView,
    ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView, DslValue, Emit, LocaleLabels, Media, MediaError, MediaPayload, NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup,
    PanelTreeBuilder, ArtifactKindSpec, SurfaceKind, TextEditorScene, UiControlNode, UiInspectorFieldGroup, UiNode, UiPresence, UiToggleNode, UiTreeItemNode, NodeGraphNodeRecord, NodeGraphEdgeRecord, NodeGraphPortRecord, NodeGraphViewport,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use sequence::{default_fixture, SequenceFixture, SequenceStep, SlotRef, StepParams, SEQUENCE_FIXTURE_SCHEMA};
use sequence_engine::{control_slots, is_control_kind, sequence_example_json, SequenceConfig, SequenceHost};
use sequence_op::{sequence_fixture_operations, SequenceConfigOperation, SequenceOperation};
use sequence_protocol::SequenceCommand;
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

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s pilot helpers.
fn is_de_locale(cfg: &SequenceConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &SequenceConfig) -> &'static L {
    if is_de_locale(cfg) {
        L::locale_labels_de()
    } else {
        L::locale_labels_en()
    }
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn sequence_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: SEQUENCE_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 🧰️ Builds a {@link SequenceHost} seeded from a projection so a command can mutate it (with all the
/// host's cycle/slot/layout logic) and then diff the result into typed operations.
fn host_from_fixture(fixture: &SequenceFixture) -> SequenceHost {
    SequenceHost::from_fixture(fixture.clone())
}

/// 🔀️ Runs a host mutation seeded from `fixture` and diffs the result into typed operations — a free
/// function (not a method) since `SequencePlayApp` is a unit struct with nothing to borrow.
fn ops_from_host_mutation(fixture: &SequenceFixture, mutate: impl FnOnce(&mut SequenceHost)) -> Vec<SequenceOperation> {
    let mut host = host_from_fixture(fixture);
    mutate(&mut host);
    sequence_fixture_operations(fixture, &host.fixture)
}

/// 🌳️ `SequenceConfig::orientation`'s string encoding <-> the DAG kernel's real
/// `DagLayoutOrientation` — see `SequenceConfig::orientation`'s doc comment for why the config field
/// itself stays a string.
fn orientation_from_config(value: &str) -> DagLayoutOrientation {
    match value {
        "topBottom" => DagLayoutOrientation::TopBottom,
        _ => DagLayoutOrientation::LeftRight,
    }
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), "next".into()))
}

fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
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
            ("moveStep", "Move Step", "Schritt verschieben"),
            ("connectSteps", "Connect Steps", "Schritte verbinden"),
            ("disconnectSteps", "Disconnect Steps", "Schritte trennen"),
            ("setStepParams", "Set Step Params", "Schrittparameter festlegen"),
            ("setStepCollapsed", "Set Step Collapsed", "Schritt einklappen"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
            ("setViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("setOrientation", "Set Orientation", "Ausrichtung festlegen"),
            ("run", "Run", "Ausführen"),
            ("stop", "Stop", "Stopp"),
            ("setLocale", "Set Locale", "Sprache festlegen"),
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
fn render_main_graph(fixture: &SequenceFixture, config: &SequenceConfig) -> UiNode {
    let mut host = host_from_fixture(fixture);
    host.layout_expanded_slots();
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let selection = config.selected_step_ids.clone();
    build_node_graph_scene(
        SEQUENCE_PLAY_SURFACE_MAIN,
        SEQUENCE_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            selection,
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}

fn render_script(fixture: &SequenceFixture, config: &SequenceConfig) -> UiNode {
    let host = host_from_fixture(fixture);
    let mut text = host.compile_text();
    if !config.last_run_json.is_empty() {
        text.push_str("\n\n# run result\n");
        text.push_str(&config.last_run_json);
    }
    build_text_editor_scene(SEQUENCE_PLAY_SURFACE_SCRIPT, SEQUENCE_PLAY_APP_ID, TextEditorScene::base(text, Some("imperative".into()), None))
}

fn render_compiled_dag(fixture: &SequenceFixture) -> UiNode {
    let host = host_from_fixture(fixture);
    build_text_editor_scene(SEQUENCE_PLAY_SURFACE_COMPILED, SEQUENCE_PLAY_APP_ID, TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🔖️SequencePlayApp
/// 🧪️ B1: unit struct — every former `SequencePlayRuntime` field now lives in
/// `sequence_engine::SequenceConfig` (see `DocumentApp::Config`), written through
/// `sequence_op::SequenceConfigOperation`s.
#[derive(Default)]
pub struct SequencePlayApp;

impl DocumentApp for SequencePlayApp {
    type Projection = SequenceFixture;
    type Operation = SequenceOperation;
    type Config = SequenceConfig;
    type ConfigOperation = SequenceConfigOperation;
    type Command = SequenceCommand;

    fn app_id(&self) -> &str {
        SEQUENCE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        SEQUENCE_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> SequenceFixture {
        default_fixture()
    }

    fn io(&self) -> Option<AppIo> {
        Some(sequence_engine::sequence_io())
    }

    /// 🎞️ `steps:in` (Wave-2 port recipe): inserts incoming computation results as a new step at the
    /// far right of the flow — an object payload becomes that step's params verbatim, a bare
    /// scalar/array is wrapped under a single `"value"` key. Never mutates anything directly (matches
    /// every other `import_media` override): the caller (a headless runner or the UI) applies the
    /// returned `StepsAdd` through the ordinary, undoable document store.
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, SequenceFixture>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, MediaError> {
        if port != "steps:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "steps:in importer only accepts a Structured (JSON) payload".into()));
        };
        let value: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let params_value = if value.is_object() { value } else { json!({ "value": value }) };
        let params: StepParams = serde_json::from_value(params_value).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let fixture = doc.projection;
        let id = sequence_engine::next_available_step_id(fixture);
        let x = fixture.steps.iter().map(|step| step.x).fold(0.0_f64, f64::max) + if fixture.steps.is_empty() { 0.0 } else { 280.0 };
        let step = SequenceStep { id, kind: "computation.import".into(), params, x, y: 0.0, slot: None, collapsed: false };
        Ok(Emit::operations(vec![SequenceOperation::StepsAdd { index: fixture.steps.len(), item: step }]))
    }

    /// 🏷️ Maps each `SequenceCommand` variant back to the action id it was declared under in
    /// `create_sequence_app`.
    fn command_id(&self, command: &SequenceCommand) -> &str {
        match command {
            SequenceCommand::AddStep { .. } => "addStep",
            SequenceCommand::AddStepToSlot { .. } => "addStepToSlot",
            SequenceCommand::AddStepDropped { .. } => "addStepDropped",
            SequenceCommand::RemoveStep { .. } => "removeStep",
            SequenceCommand::DeleteSelection => "deleteSelection",
            SequenceCommand::MoveStep { .. } => "moveStep",
            SequenceCommand::ConnectSteps { .. } => "connectSteps",
            SequenceCommand::DisconnectSteps { .. } => "disconnectSteps",
            SequenceCommand::SetStepParams { .. } => "setStepParams",
            SequenceCommand::SetStepCollapsed { .. } => "setStepCollapsed",
            SequenceCommand::Reorganize => "reorganize",
            SequenceCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            SequenceCommand::SetSelection { .. } => "setSelection",
            SequenceCommand::SetOrientation { .. } => "setOrientation",
            SequenceCommand::Run => "run",
            SequenceCommand::Stop => "stop",
            SequenceCommand::SetViewport { .. } => "setViewport",
            SequenceCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &SequenceCommand, doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>) -> Emit<SequenceOperation, SequenceConfigOperation> {
        let fixture = doc.projection;
        let config = cfg.projection;
        match command {
            SequenceCommand::AddStep { kind, x, y } => {
                let mut host = host_from_fixture(fixture);
                let id = host.add_step(kind, *x, *y);
                Emit { document_operations: sequence_fixture_operations(fixture, &host.fixture), config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: vec![id] }], ..Default::default() }
            }
            SequenceCommand::AddStepToSlot { kind, x, y, owner, slot_name } => {
                let mut host = host_from_fixture(fixture);
                let id = host.add_step_in_slot(kind, *x, *y, Some(SlotRef { owner: owner.clone(), name: slot_name.clone() }));
                Emit { document_operations: sequence_fixture_operations(fixture, &host.fixture), config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: vec![id] }], ..Default::default() }
            }
            SequenceCommand::AddStepDropped { kind, x, y, picked_step_id } => {
                let mut host = host_from_fixture(fixture);
                let id = host.add_step_dropped(kind, *x, *y, picked_step_id.as_deref());
                Emit { document_operations: sequence_fixture_operations(fixture, &host.fixture), config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: vec![id] }], ..Default::default() }
            }
            SequenceCommand::RemoveStep { id } => {
                let ops = ops_from_host_mutation(fixture, |host| {
                    host.remove_step(id);
                });
                if ops.is_empty() {
                    Emit::default()
                } else {
                    let step_ids = config.selected_step_ids.iter().filter(|selected| *selected != id).cloned().collect();
                    Emit { document_operations: ops, config_operations: vec![SequenceConfigOperation::SetSelection { step_ids }], ..Default::default() }
                }
            }
            SequenceCommand::DeleteSelection => {
                let selected = config.selected_step_ids.clone();
                let ops = ops_from_host_mutation(fixture, |host| {
                    for step_id in &selected {
                        host.remove_step(step_id);
                    }
                });
                if ops.is_empty() {
                    Emit::default()
                } else {
                    Emit { document_operations: ops, config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: Vec::new() }], ..Default::default() }
                }
            }
            SequenceCommand::MoveStep { node_id, x, y } => {
                if !fixture.steps.iter().any(|step| &step.id == node_id) {
                    return Emit::default();
                }
                Emit::operations(ops_from_host_mutation(fixture, |host| {
                    let mut next = host.fixture.clone();
                    if let Some(step) = next.steps.iter_mut().find(|step| &step.id == node_id) {
                        step.x = *x;
                        step.y = *y;
                    }
                    let _ = host.replace_fixture(next);
                }))
            }
            SequenceCommand::ConnectSteps { source_node_id, target_node_id } => Emit::operations(ops_from_host_mutation(fixture, |host| {
                let _ = host.connect_steps(source_node_id, target_node_id);
            })),
            SequenceCommand::DisconnectSteps { from_id, to_id } => Emit::operations(ops_from_host_mutation(fixture, |host| {
                host.disconnect_steps(from_id, to_id);
            })),
            SequenceCommand::SetStepParams { id, params_json } => Emit::operations(ops_from_host_mutation(fixture, |host| {
                let _ = host.set_step_params_json(id, params_json);
            })),
            SequenceCommand::SetStepCollapsed { id } => {
                let collapsed = fixture.steps.iter().find(|step| &step.id == id).map(|step| !step.collapsed).unwrap_or(true);
                Emit::operations(ops_from_host_mutation(fixture, |host| {
                    host.set_step_collapsed(id, collapsed);
                }))
            }
            SequenceCommand::Reorganize => {
                let orientation = orientation_from_config(&config.orientation);
                Emit::operations(ops_from_host_mutation(fixture, |host| {
                    let opts = DagLayoutOptions { orientation, ..DagLayoutOptions::default() };
                    let _ = host.reorganize(&opts);
                }))
            }
            SequenceCommand::NodeGraphEdit { operations_json } => {
                let sub_operations: Vec<Value> = serde_json::from_str(operations_json).unwrap_or_default();
                let selected = config.selected_step_ids.clone();
                let mut cleared = false;
                let ops = ops_from_host_mutation(fixture, |host| {
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
                    Emit { document_operations: ops, config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: Vec::new() }], ..Default::default() }
                } else {
                    Emit::operations(ops)
                }
            }
            SequenceCommand::SetSelection { step_ids } => Emit::config(vec![SequenceConfigOperation::SetSelection { step_ids: step_ids.clone() }]),
            SequenceCommand::SetOrientation { value } => Emit::config(vec![SequenceConfigOperation::SetOrientation { value: value.clone() }]),
            SequenceCommand::Run => {
                let result = host_from_fixture(fixture).run();
                let json = serde_json::to_string(&result).unwrap_or_default();
                Emit::config(vec![SequenceConfigOperation::SetLastRun { json }])
            }
            SequenceCommand::Stop => Emit::config(vec![SequenceConfigOperation::SetLastRun { json: String::new() }]),
            SequenceCommand::SetViewport { camera } => Emit::config(vec![SequenceConfigOperation::SetCamera { camera: camera.clone() }]),
            SequenceCommand::SetLocale { value } => Emit::config(vec![SequenceConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    /// 🧮️ This app's typed configuration spec — the layout orientation `reorganize` reads.
    fn config_spec(&self) -> ConfigSpec {
        ConfigSpec {
            fields: vec![ConfigFieldSpec {
                key: "orientation".into(),
                label: "Layout Orientation".into(),
                shape: ConfigFieldShape::Select { options: vec!["leftRight".into(), "topBottom".into()] },
                default: Some(DslValue::String("leftRight".into())),
            }],
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>) -> UiNode {
        let fixture = doc.projection;
        let config = cfg.projection;
        let labels = resolve_labels::<SequenceLabels>(config);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => render_main_graph(fixture, config),
            SEQUENCE_PLAY_BODY_SCRIPT => render_script(fixture, config),
            SEQUENCE_PLAY_BODY_COMPILED => render_compiled_dag(fixture),
            SEQUENCE_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &config.selected_step_ids, labels),
            SEQUENCE_PLAY_BODY_CATALOGUE => build_catalogue_tree(fixture, labels),
            SEQUENCE_PLAY_BODY_INSPECTOR => build_inspector_tree(fixture, &config.selected_step_ids, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, cfg: &ConfigView<'_, SequenceConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<SequenceLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(SEQUENCE_PLAY_WINDOW_MAIN, labels.window_main)
            .window_kind_label(SEQUENCE_PLAY_WINDOW_SCRIPT, labels.window_script)
            .window_kind_label(SEQUENCE_PLAY_WINDOW_COMPILED, labels.window_compiled)
            .action_labels(sequence_action_labels(is_de))
            .utility_labels(sequence_utility_labels(is_de))
    }

    fn context_menu(&self, request: &ContextMenuRequest, _doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = is_de_locale(cfg.projection);
        let selected = cfg.projection.selected_step_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
        let mut menu = Menu::of(registry);
        if let Some(spec) = node_graph_delete_selection_spec("Delete selection", is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::Direct) {
            menu = menu.item(spec);
        }
        menu.build()
    }
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
            .mode("edit", "Edit", "square-pen")
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
            .operation("moveStep", "Move Step")
            .operation("connectSteps", "Connect Steps")
            .operation("disconnectSteps", "Disconnect Steps")
            .operation("setStepParams", "Set Step Params")
            .operation("setStepCollapsed", "Set Step Collapsed")
            .operation("reorganize", "Reorganize")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .view_action("setViewport", "Node Graph Viewport")
            // 👁️ Ephemeral view state — selection, run output, layout orientation, locale.
            .view_action("setSelection", "Set Selection")
            .view_action("setOrientation", "Set Orientation")
            .view_action("run", "Run")
            .view_action("stop", "Stop")
            .view_action("setLocale", "Set Locale")
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
            .keybinding("mod+shift+z", "redo")
            .config(SequencePlayApp.config_spec())
            .io(sequence_engine::sequence_io()),
    )
    .example("demo", "Demo", sequence_example_json(), "flask-conical")
    .workflow("sequence", "Sequence", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, ViewState, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<SequencePlayApp> {
        testkit::new_app::<SequencePlayApp>()
    }

    fn new_app_with_registry() -> VcsDocumentApp<SequencePlayApp> {
        testkit::new_app_with_registry::<SequencePlayApp>(create_sequence_app)
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
    fn add_step_command_appends_step() {
        let mut app = new_app();
        app.dispatch_typed(SequenceCommand::AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 }, &testkit::meta("local")).expect("add");
        assert!(app.projection().expect("projection").steps.len() > 2);
    }

    #[test]
    fn run_stores_result_and_renders_in_script() {
        let mut app = new_app();
        let result = app.dispatch_typed(SequenceCommand::Run, &testkit::meta("local")).expect("run");
        assert!(result.operations.is_empty(), "run is config-only and emits no document operations");
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("run result"));
    }

    /// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceOperation` (no VCS edit, no
    /// undo entry) and instead write straight into the config store.
    #[test]
    fn set_viewport_writes_config_not_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(SequenceCommand::SetViewport { camera: sequence::SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } }, &testkit::meta("local")).expect("viewport pan/zoom");
        assert!(result.operations.is_empty(), "setViewport must not emit a VCS operation");
        let node = app.render(SEQUENCE_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        assert_eq!(payload["nodeGraph"]["viewport"]["zoom"], json!(2.0));
    }

    #[test]
    fn remove_step_command_deletes_step() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        app.dispatch_typed(SequenceCommand::RemoveStep { id: step_id.clone() }, &testkit::meta("local")).expect("remove");
        assert!(app.projection().expect("projection").steps.iter().all(|step| step.id != step_id));
    }

    #[test]
    fn set_orientation_command_changes_reorganize_layout_axis() {
        let mut app = new_app();
        app.dispatch_typed(SequenceCommand::SetOrientation { value: "topBottom".into() }, &testkit::meta("local")).expect("orientation");
        let ids: Vec<String> = app.projection().expect("projection").steps.iter().map(|step| step.id.clone()).collect();
        for id in &ids {
            app.dispatch_typed(SequenceCommand::MoveStep { node_id: id.clone(), x: 0.0, y: 0.0 }, &testkit::meta("local")).expect("move");
        }
        app.dispatch_typed(SequenceCommand::Reorganize, &testkit::meta("local")).expect("reorganize");
        let ys: Vec<f64> = app.projection().expect("projection").steps.iter().map(|step| step.y).collect();
        assert!(ys.iter().any(|y| *y != 0.0), "topBottom orientation should spread steps vertically, got {ys:?}");
    }

    #[test]
    fn reorganize_command_spreads_step_positions_apart() {
        let mut app = new_app();
        let ids: Vec<String> = app.projection().expect("projection").steps.iter().map(|step| step.id.clone()).collect();
        for id in &ids {
            app.dispatch_typed(SequenceCommand::MoveStep { node_id: id.clone(), x: 0.0, y: 0.0 }, &testkit::meta("local")).expect("move");
        }
        app.dispatch_typed(SequenceCommand::Reorganize, &testkit::meta("local")).expect("reorganize");
        let xs: Vec<f64> = app.projection().expect("projection").steps.iter().map(|step| step.x).collect();
        assert!(xs.iter().any(|x| *x != 0.0), "reorganize should spread steps apart, got {xs:?}");
    }

    #[test]
    fn stop_command_clears_last_run_result() {
        let mut app = new_app();
        app.dispatch_typed(SequenceCommand::Run, &testkit::meta("local")).expect("run");
        app.dispatch_typed(SequenceCommand::Stop, &testkit::meta("local")).expect("stop");
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        assert!(!serde_json::to_string(&node).unwrap().contains("run result"));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(&mut app, SequenceCommand::AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 }, |app| app.projection().expect("projection").steps.len(), 2, 3);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
    /// apply DISJOINT edits (A moves step-1, B moves step-2), and exchanging operations over a `MemoryBackbone`
    /// converges both sides onto an identical projection.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<SequencePlayApp, _>(
            "mem://sequence-convergence",
            SequenceCommand::MoveStep { node_id: "step-1".into(), x: 111.0, y: 0.0 },
            SequenceCommand::MoveStep { node_id: "step-2".into(), x: 222.0, y: 0.0 },
            |app| app.projection().expect("projection"),
        );
    }

    #[test]
    fn sequence_labels_render_native_english_by_default() {
        let mut app = new_app();
        let document_json = serde_json::to_string(&app.render(SEQUENCE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(document_json.contains("\"Steps\""));
        assert!(document_json.contains("\"Flow edges\""));
        let action_labels = app.app_labels().action_labels;
        for (id, label) in [("run", "Run"), ("stop", "Stop"), ("reorganize", "Reorganize")] {
            assert_eq!(action_labels.get(id).map(String::as_str), Some(label), "{id} action label");
        }
    }

    #[test]
    fn sequence_labels_render_german_locale() {
        let mut app = new_app();
        app.dispatch_typed(SequenceCommand::SetLocale { value: "de".into() }, &testkit::meta("local")).expect("locale");
        let document_json = serde_json::to_string(&app.render(SEQUENCE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(document_json.contains("Schritte"));
        assert!(document_json.contains("Ablaufkanten"));
        assert!(!document_json.contains("\"Steps\""));
        let action_labels = app.app_labels().action_labels;
        for (id, label) in [("run", "Ausführen"), ("stop", "Stopp"), ("reorganize", "Neu anordnen")] {
            assert_eq!(action_labels.get(id).map(String::as_str), Some(label), "{id} action label");
        }
    }

    //#region 🔖️PortTests
    #[test]
    fn sequence_io_declares_steps_in_and_document_ports() {
        let ports = SequencePlayApp.io().expect("io").all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        assert!(ports.iter().any(|port| port.id == "steps:in"));
    }

    #[test]
    fn import_media_steps_in_inserts_a_new_step_from_an_object_payload() {
        let mut app = new_app_with_registry();
        let before = app.projection().expect("projection").steps.len();
        let media = Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "computation.value".into(), json: json!({ "message": "from upstream" }).to_string() } };
        app.import_media("steps:in", &media, &testkit::meta("local")).expect("import steps:in");
        let after = app.projection().expect("projection");
        assert_eq!(after.steps.len(), before + 1);
        let imported = after.steps.last().expect("imported step");
        assert_eq!(imported.kind, "computation.import");
        assert_eq!(imported.params.get("message").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("from upstream"));
    }

    #[test]
    fn import_media_steps_in_wraps_a_bare_scalar_payload() {
        let mut app = new_app_with_registry();
        let media = Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "computation.value".into(), json: "42".into() } };
        app.import_media("steps:in", &media, &testkit::meta("local")).expect("import steps:in");
        let after = app.projection().expect("projection");
        let imported = after.steps.last().expect("imported step");
        assert_eq!(imported.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(42.0));
    }

    #[test]
    fn import_media_rejects_unknown_port() {
        let mut app = new_app_with_registry();
        let media = Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "computation.value".into(), json: "{}".into() } };
        assert!(app.import_media("not-a-port", &media, &testkit::meta("local")).is_err());
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
