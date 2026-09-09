//! ✏️ Wires document editing with gestures owned by each concrete canvas window.

#[path = "🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🦀️.rs"]
pub mod window_transient;
#[path = "🧵️retained/🦀️.rs"]
mod retained;

use crate::editor::wires::commands::add_node;
use crate::editor::wires::commands::add_relationship;
use crate::editor::wires::commands::delete_selection;
use crate::editor::wires::commands::set_active_example;
use crate::editor::wires::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up};
use crate::editor::wires::commands::{force_layout, reorganize};
use crate::editor::wires::modes::edit;
use crate::editor::wires::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::op::WiresMutation;
use crate::WiresSnapshot;
use semio_framework::kernel::Effect;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor,
    EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
    INTERACTION_SELECT_ACTION_ID,
};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use serde_json::{json, Value};
use store::EngineHandles;

//#region 🔖️Constants
pub const WIRES_PLAY_APP_ID: &str = "reasoning-wires-play";
pub use catalogue_panel::WIRES_PLAY_BODY_CATALOGUE;
pub use document_panel::WIRES_PLAY_BODY_DOCUMENT;
pub use edit::windows::canvas::{WIRES_PLAY_BODY_COMPOSITE, WIRES_PLAY_WINDOW_CANVAS};
pub use inspection_panel::WIRES_PLAY_BODY_PROPERTIES;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn wires_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(WIRES_PLAY_APP_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.label-capacity", "wires label exceeds its fixed capacity"))
}

/// 📝️ Admits text into an action payload.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "replace the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/🧬️set-active-example::set_active_example` uses instead of a banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub fn reset_wires_document_effect(document: &WiresSnapshot) -> Effect {
    let pack = <WiresSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("reasoning-wires", crate::MINDMAP_WIRES_SCHEMA));
    Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ The one framework-owned interaction domain wires declares — identities (nodes) and
/// relationships (edges) on the mindmap canvas plus the document tree's identity/relationship rows
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). `Flat`: the mindmap graph
/// (`infinite_board_normal_undirected`) is a normal undirected identity/relationship graph — no
/// parent/child structure exists anywhere in `WiresSnapshot`/the fixture schema to build a topology
/// from, unlike writer's AST or procedural's DAG, so this crate's own migration deliberately disagrees
/// with the original per-crate brief's "Topology over parent links" guess.
pub const WIRES_INTERACTION_GRAPH: &str = "graph";
pub const WIRES_GRANULARITY_NODE: &str = "node";
pub const WIRES_GRANULARITY_EDGE: &str = "edge";

/// 🕹️ Builds `interactionSelect`'s JSON args for one merge over `ids` at `granularity` — shared by
/// the canvas pointer/add commands (wrapped into a `Effect::DispatchAction`) and any document-tree
/// row whose click should select a real canvas identity/relationship.
pub fn wires_select_action_args(ids: &[String], granularity: &str, merge: &str) -> Value {
    let targets: Vec<Value> = ids.iter().map(|id| json!({ "granularity": granularity, "id": id })).collect();
    json!({ "domainId": WIRES_INTERACTION_GRAPH, "targets": serde_json::to_string(&targets).unwrap_or_default(), "merge": merge, "method": "pick" })
}

/// 🕹️ Wraps [`wires_select_action_args`] into the redispatch effect a canvas gesture's own `handle`
/// returns — `dispatch_action` intercepts the six framework interaction verbs BEFORE routing to
/// `ArtifactApp::handle`, so a plain config mutation can no longer express a selection change; the app
/// asks the host to redispatch `interactionSelect` instead (master doc: "surfaces do geometric
/// hit-testing and emit one batched `interactionSelect`").
pub fn wires_select_effect(ids: &[String], granularity: &str, merge: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(112), action: INTERACTION_SELECT_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(wires_select_action_args(ids, granularity, merge))), delay_ms: 0 }
}
//#endregion 🔖️Interaction

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `ReasoningWiresPlayApp::Command` — the SOLE dispatch surface for this app's behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum WiresCommand for WiresSnapshot, WiresMutation, NoConfig, NoConfigMutation {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "addNode" as "add-node" => add_node::AddNode,
        "addRelationship" as "add-relationship" => add_relationship::AddRelationship,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "forceLayout" as "force-layout" => force_layout::ForceLayout,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "canvasPointerMove" as "pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerDown" as "pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerUp" as "pointer-up" => canvas_pointer_up::CanvasPointerUp,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ReasoningWiresPlayApp
/// 🪟️ Stateless editor whose captured window owns each pointer gesture.
#[derive(Default)]
pub struct ReasoningWiresPlayApp;

//#region 🧵️RetainedCommands
const WIRES_RETAINED_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp"];
const WIRES_RETAINED_PAYLOAD_SCHEMA: &str = "reasoning.wires.tool-command.v1";
const WIRES_RETAINED_RAW_BYTES: usize = 8_192;
const WIRES_RETAINED_WORK_ITEMS: usize = 1_048_576;
const WIRES_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
];

fn wires_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(WIRES_RETAINED_RAW_BYTES, 16, WIRES_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

fn wires_retained_extent(command: &WiresCommand, _snapshot: &WiresSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        WiresCommand::CanvasPointerUp(_) => Some(WIRES_RETAINED_WORK_ITEMS),
        WiresCommand::CanvasPointerDown(payload) if payload.id.as_ref().is_none_or(|id| id.len() <= 1_024) && payload.x.is_finite() && payload.y.is_finite() => Some(WIRES_RETAINED_WORK_ITEMS),
        WiresCommand::CanvasPointerMove(payload) if payload.x.is_finite() && payload.y.is_finite() => Some(1),
        _ => None,
    }
}

struct WiresWindowDragWork {
    tool_id: &'static str,
    node_cursor: usize,
    field_cursor: usize,
    visited: usize,
    consumed: bool,
    matched_node: Option<bool>,
    node_x: Option<f64>,
    node_y: Option<f64>,
}

impl WiresWindowDragWork {
    fn drag_mutation(
        window: &semio_framework_plugin::WindowTransientSnapshot,
        node_id: Option<String>,
        start_x: f64,
        start_y: f64,
        last_x: f64,
        last_y: f64,
        zoom: f64,
    ) -> semio_framework_plugin::WindowTransientMutation {
        semio_framework_plugin::WindowTransientMutation::of::<window_transient::WiresCanvasTransientOwner>(
            window.window_id(),
            window_transient::SetDrag { node_id, start_x, start_y, last_x, last_y, zoom }.into(),
        )
    }

    fn clear(window: &semio_framework_plugin::WindowTransientSnapshot) -> semio_framework_plugin::WindowTransientMutation {
        Self::drag_mutation(window, None, 0.0, 0.0, 0.0, 0.0, 1.0)
    }

    fn complete_down(
        &mut self,
        window: &semio_framework_plugin::WindowTransientSnapshot,
        node_id: Option<String>,
        x: f64,
        y: f64,
        zoom: f64,
    ) -> ArtifactCommandWorkStep<EditorApp<ReasoningWiresPlayApp>> {
        self.consumed = true;
        let effects = node_id.as_ref().map(|id| vec![wires_select_effect(std::slice::from_ref(id), WIRES_GRANULARITY_NODE, "replace")]).unwrap_or_default();
        let mutation = Self::drag_mutation(window, node_id, x, y, x, y, zoom);
        ArtifactCommandWorkStep::CompleteWithEphemeral { emit: Emit { effects, ..Default::default() }, ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![mutation] } }
    }

    fn complete_clear(&mut self, window: &semio_framework_plugin::WindowTransientSnapshot) -> ArtifactCommandWorkStep<EditorApp<ReasoningWiresPlayApp>> {
        self.consumed = true;
        ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit: Emit::default(),
            ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![Self::clear(window)] },
        }
    }
}

impl ArtifactCommandWork<EditorApp<ReasoningWiresPlayApp>> for WiresWindowDragWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &WiresCommand, snapshot: &WiresSnapshot, interaction: &protocol::InteractionState, context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<ReasoningWiresPlayApp>>>) -> Option<usize> {
        let context = context?;
        context.window_transient.as_ref()?.get::<window_transient::WiresCanvasTransientOwner>()?;
        if matches!(command, WiresCommand::CanvasPointerDown(_)) {
            context
                .window_config
                .as_ref()?
                .get::<edit::windows::canvas::config::WiresCanvasWindowConfigOwner>()?;
        }
        (command.command_id() == self.tool_id).then(|| wires_retained_extent(command, snapshot, interaction)).flatten()
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<ReasoningWiresPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<ReasoningWiresPlayApp>>, Fault> {
        if self.consumed || self.visited >= WIRES_RETAINED_WORK_ITEMS {
            return Err(Fault::from("wires-window-drag-work-capacity"));
        }
        self.visited += 1;
        let window = input.context.and_then(|context| context.window_transient.as_ref()).ok_or_else(|| Fault::from("wires-drag-requires-window"))?;
        let transient = window.get::<window_transient::WiresCanvasTransientOwner>().ok_or_else(|| Fault::from("wires-drag-window-kind"))?;
        match input.command {
            WiresCommand::CanvasPointerDown(payload) => {
                let Some(id) = payload.id.as_ref() else {
                    return Ok(self.complete_clear(window));
                };
                let window_config = input
                    .context
                    .and_then(|context| context.window_config.as_ref())
                    .and_then(|snapshot| snapshot.get::<edit::windows::canvas::config::WiresCanvasWindowConfigOwner>())
                    .ok_or_else(|| Fault::from("wires-drag-requires-window-config"))?;
                let zoom = window_config.camera.zoom;
                if !zoom.is_finite() || zoom <= 0.0 {
                    return Err(Fault::from("wires-drag-camera-invalid"));
                }
                let scene = input.snapshot.content.local_owner::<crate::WiresWorkingScene>().ok_or_else(|| Fault::from("wires-drag-child-not-materialized"))?;
                let Some(node) = scene.nodes.get(self.node_cursor) else {
                    return Ok(self.complete_clear(window));
                };
                if let dsl::DslValue::Object(fields) = node {
                    if let Some((key, value)) = fields.get(self.field_cursor) {
                        self.field_cursor += 1;
                        if key == "id" && value.as_str() == Some(id.as_str()) {
                            return Ok(self.complete_down(window, Some(id.clone()), payload.x, payload.y, zoom));
                        }
                        return Ok(ArtifactCommandWorkStep::Progress { stage: "canvas-hit", preview: &[] });
                    }
                }
                self.node_cursor += 1;
                self.field_cursor = 0;
                Ok(ArtifactCommandWorkStep::Progress { stage: "canvas-hit", preview: &[] })
            }
            WiresCommand::CanvasPointerMove(payload) => {
                let Some(id) = transient.drag_node_id.as_ref() else {
                    self.consumed = true;
                    return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
                };
                if id.len() > 1_024 {
                    return Err(Fault::from("wires-drag-node-capacity"));
                }
                if !transient.drag_zoom.is_finite() || transient.drag_zoom <= 0.0 {
                    return Err(Fault::from("wires-drag-camera-invalid"));
                }
                self.consumed = true;
                let gesture = Self::drag_mutation(
                    window,
                    Some(id.clone()),
                    transient.drag_start_x,
                    transient.drag_start_y,
                    payload.x,
                    payload.y,
                    transient.drag_zoom,
                );
                Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                    emit: Emit::default(),
                    ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![gesture] },
                })
            }
            WiresCommand::CanvasPointerUp(_) => {
                let Some(id) = transient.drag_node_id.as_ref() else {
                    return Ok(self.complete_clear(window));
                };
                if id.len() > 1_024 || !transient.drag_zoom.is_finite() || transient.drag_zoom <= 0.0 {
                    return Err(Fault::from("wires-drag-transient-invalid"));
                }
                let scene = input.snapshot.content.local_owner::<crate::WiresWorkingScene>().ok_or_else(|| Fault::from("wires-drag-child-not-materialized"))?;
                let Some(node) = scene.nodes.get(self.node_cursor) else {
                    return Ok(self.complete_clear(window));
                };
                if let dsl::DslValue::Object(fields) = node {
                    if let Some((key, value)) = fields.get(self.field_cursor) {
                        self.field_cursor += 1;
                        match key.as_str() {
                            "id" if self.matched_node.is_none() => self.matched_node = Some(value.as_str() == Some(id.as_str())),
                            "x" if self.node_x.is_none() => self.node_x = Some(value.as_f64().unwrap_or(0.0)),
                            "y" if self.node_y.is_none() => self.node_y = Some(value.as_f64().unwrap_or(0.0)),
                            _ => {}
                        }
                        return Ok(ArtifactCommandWorkStep::Progress { stage: "canvas-move-target", preview: &[] });
                    }
                    if self.matched_node == Some(true) {
                        let current_x = self.node_x.ok_or_else(|| Fault::from("wires-drag-position-missing"))?;
                        let current_y = self.node_y.ok_or_else(|| Fault::from("wires-drag-position-missing"))?;
                        let x = current_x + (transient.drag_last_x - transient.drag_start_x) / transient.drag_zoom;
                        let y = current_y + (transient.drag_last_y - transient.drag_start_y) / transient.drag_zoom;
                        if !x.is_finite() || !y.is_finite() {
                            return Err(Fault::from("wires-drag-position-non-finite"));
                        }
                        self.consumed = true;
                        return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                            emit: Emit { artifact_mutations: vec![crate::mutations::move_node(id.clone(), x, y)], ..Default::default() },
                            ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![Self::clear(window)] },
                        });
                    }
                }
                self.node_cursor += 1;
                self.field_cursor = 0;
                self.matched_node = None;
                self.node_x = None;
                self.node_y = None;
                Ok(ArtifactCommandWorkStep::Progress { stage: "canvas-move-target", preview: &[] })
            }
            _ => Err(Fault::from("wires-window-drag-route")),
        }
    }
}

struct WiresRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl WiresRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: WIRES_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for WiresRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<ReasoningWiresPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<ReasoningWiresPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WIRES_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        wires_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > WIRES_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Wires bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for WiresRetainedCommandJobFactory {
    type Owner = EditorApp<ReasoningWiresPlayApp>;
    const TOOL_IDS: &'static [&'static str] = WIRES_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::MINDMAP_WIRES_SCHEMA;

    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = WIRES_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

impl ArtifactEditor for ReasoningWiresPlayApp {
    type Snapshot = WiresSnapshot;
    type Mutation = WiresMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = WiresCommand;

    const DIALECT: Dialect = crate::WIRES_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::MINDMAP_WIRES_SCHEMA;

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::schema::retirement::document_store_owners())
    }
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(retained::factory())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }
    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }
    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<window_transient::WiresCanvasTransientOwner>()
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<edit::windows::canvas::config::WiresCanvasWindowConfigOwner>()
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<ReasoningWiresPlayApp>,
        owner_file: "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.reasoning.wires@1/*#editor",
        document_schema: "reasoning.wires.fixture",
        factory: "WiresRetainedCommandJobFactory",
        factory_type: WiresRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 16, 1_048_576, 16_384, 7_500),
        tools: ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(WiresRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !WIRES_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("wires-command-tool-mismatch"));
        }
        if wires_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("wires-command-payload-too-large"));
        }
        let work = Box::new(WiresWindowDragWork { tool_id: request.command.command_id(), node_cursor: 0, field_cursor: 0, visited: 0, consumed: false, matched_node: None, node_x: None, node_y: None });
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            WiresCommand::command_id,
            WIRES_RETAINED_RAW_BYTES,
            WIRES_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> WiresSnapshot {
        crate::empty_wires_snapshot()
    }

    /// 🏷️ Supplied wholesale by `app_commands!`'s generated `command_id()`.
    fn command_id(command: &WiresCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ `deleteSelection` reads the "graph" interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &WiresCommand,
        doc: &ArtifactView<'_, WiresSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<WiresMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            WiresCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, WiresSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let labels = semio_framework_plugin::resolve_labels::<crate::editor::wires::terminology::WiresLabels>(view_state);
        match body_key {
            WIRES_PLAY_BODY_COMPOSITE => {
                let window = edit::windows::canvas::config::current(cfg).cloned().unwrap_or_default();
                edit::windows::canvas::render(&crate::wires_working_board(document), &document.wires_fixture, &window)
            }
            WIRES_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            WIRES_PLAY_BODY_CATALOGUE => catalogue_panel::render(&document.wires_fixture, labels),
            WIRES_PLAY_BODY_PROPERTIES => inspection_panel::render(document, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "wires diagnostic admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, WiresSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        _interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        if body_key == WIRES_PLAY_BODY_COMPOSITE {
            let window = edit::windows::canvas::config::current(cfg).cloned().unwrap_or_default();
            let gesture = transient.window::<window_transient::WiresCanvasTransientOwner>().cloned().unwrap_or_default();
            let mut board = crate::wires_working_board(doc.snapshot);
            if let Some(node_id) = gesture.drag_node_id.as_deref() {
                if gesture.drag_zoom.is_finite() && gesture.drag_zoom > 0.0 {
                    let scene = crate::wires_working_scene(doc.snapshot);
                    if let Some(node) = scene.nodes.iter().find(|node| crate::schema::entity_id(node, "id") == Some(node_id)) {
                        let (current_x, current_y) = crate::schema::node_position(node);
                        let preview_x = current_x + (gesture.drag_last_x - gesture.drag_start_x) / gesture.drag_zoom;
                        let preview_y = current_y + (gesture.drag_last_y - gesture.drag_start_y) / gesture.drag_zoom;
                        if preview_x.is_finite() && preview_y.is_finite() {
                            crate::mutations::set_node_field(&mut board, node_id, "x", dsl::DslValue::float(preview_x));
                            crate::mutations::set_node_field(&mut board, node_id, "y", dsl::DslValue::float(preview_y));
                        }
                    }
                }
            }
            return edit::windows::canvas::render(&board, &doc.snapshot.wires_fixture, &window).map(semio_framework_plugin::built_to_component_tree);
        }
        Self::render(body_key, doc, cfg, view_state)
    }
}
//#endregion 🔖️ReasoningWiresPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
///
/// 🚧️ SDK GAP (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4/§7.4):
/// `EditorBuilder` has no `.example(...)`/`.workflow(...)` methods — `AppBuilder`'s `App { definition,
/// examples }` split means `.editor::<E>(def: AppDefinition)` only ever takes the definition, so the
/// old metabolism example registration and the `"reasoning-wires"` workflow tag are dropped here, not
/// silently lost. The subset's own `📚️examples/🎬️demo` facet is the documented replacement mechanism
/// for the former; `metabolism_wires_example_snapshot()` itself still lives on and is exercised
/// directly by this file's own tests below.
pub fn create_wires_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::WIRES_DIALECT)
        .document(["semio", "reasoning", "mindmap", "wires"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("reasoning-wires")
        .mode_def(edit::definition())
        .default_mode_id(edit::WIRES_PLAY_MODE_EDIT)
        .window_kind_def(edit::windows::canvas::definition())
        .default_layout(edit::layout())
        .panel_tab_def(document_panel::definition())
        .panel_tab_def(catalogue_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
        .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
        .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
        .mutation("addRelationship", LocalizedLabel::native("Add Relationship", "Beziehung hinzufügen"))
        .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
        .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
        .action_with(semio_framework_plugin::ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), semio_framework_plugin::ActionKind::Mutation, "rotate-cw"))
        .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
        // 👁️ Ephemeral view state — in-flight drag. Selection/hover are framework-owned now
        // (domain "graph") — no app-declared verbs; `interactionSelect`/`interactionHover`/
        // `clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity` auto-inject
        // below via `.interaction(...)`.
        .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
        .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))
        .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("addNode", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("addRelationship", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("deleteSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("forceLayout", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("reorganize", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
        // 🕹️ Domain "graph": identities (node) and relationships (edge) — `Flat` (the mindmap graph
        // has no parent/child structure to build a topology from, see `WIRES_INTERACTION_GRAPH`'s
        // doc comment); single-select, pick-only, replace-only merge (matches the pre-migration
        // click-to-select behaviour this crate hand-rolled).
        .interaction(InteractionDefinition {
            id: WIRES_INTERACTION_GRAPH.into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: WIRES_GRANULARITY_NODE.into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: WIRES_GRANULARITY_EDGE.into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            ],
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
        })
        .window_kind_interactions(WIRES_PLAY_WINDOW_CANVAS, vec![InteractionRef::new(WIRES_INTERACTION_GRAPH)])
        // 🎯️ Typed channel surface (B1 pure-trait conversion) — `config_spec()`'s single source of
        // truth (the trait default `ConfigSpec::empty()`: none of `NoConfig`'s fields are
        // user-visible settings, they're ephemeral view state) reused here rather than duplicated.
        .config(ReasoningWiresPlayApp::config_spec())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
