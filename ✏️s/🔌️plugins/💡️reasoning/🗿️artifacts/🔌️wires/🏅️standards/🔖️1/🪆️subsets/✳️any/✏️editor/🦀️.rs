//! ✏️ Wires editor — the read-write counterpart of `👁️viewer` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `ReasoningWiresPlayApp` implements
//! `ArtifactEditor`; `EditorApp<ReasoningWiresPlayApp>` (framework SDK) is the sole runtime
//! `ArtifactApp` adapter.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/✏️edit/🪟️windows/🕸️canvas`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view
//! state in `🦀️config.rs`, shared document helpers in the artifact's `🧬️schema`, derived reads in its
//! `🧬️schema/💡️inferences`, and plugin registration (below — dissolved from the former `⚙️engine`, ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). This file is a routing table: `handle` →
//! `WiresCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.
//!
//! B1: `ReasoningWiresPlayApp` is a unit struct — every former `WiresPlayRuntime` field (selection,
//! in-flight drag) lives in `crate::editor::wires::config::WiresConfig`, written via
//! `crate::editor::wires::config::WiresConfigMutation`s (real `backwards`, no ad hoc runtime `RefCell`);
//! every action dispatches through the single typed `WiresCommand` channel via `ArtifactEditor::handle`.

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use crate::editor::wires::commands::add_node;
use crate::editor::wires::commands::add_relationship;
use crate::editor::wires::commands::delete_selection;
use crate::editor::wires::commands::set_active_example;
use crate::editor::wires::commands::set_locale;
use crate::editor::wires::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up};
use crate::editor::wires::commands::{force_layout, reorganize};
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::editor::wires::modes::edit;
use crate::editor::wires::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use semio_framework::kernel::Effect;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect,
    DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
    INTERACTION_SELECT_ACTION_ID,
};
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
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
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
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}


/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "replace the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/🧬️set-active-example::set_active_example` uses instead of a banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub fn reset_wires_document_effect(document: &WiresSnapshot) -> Effect {
    let pack = <WiresSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::MINDMAP_WIRES_SCHEMA, "reasoning-wires", document.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("wires document spr encode is infallible for a fresh, edit-free envelope");
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
    /// vocabularies; `setLocale`/`locale` is the row that proves it. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum WiresCommand for WiresSnapshot, WiresMutation, WiresConfig, WiresConfigMutation {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "addNode" as "add-node" => add_node::AddNode,
        "addRelationship" as "add-relationship" => add_relationship::AddRelationship,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "forceLayout" as "force-layout" => force_layout::ForceLayout,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "canvasPointerMove" as "pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerDown" as "pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerUp" as "pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ReasoningWiresPlayApp
/// 🧪️ B1: unit struct — every former `WiresPlayRuntime` field now lives in `WiresConfig`, written
/// through `WiresConfigMutation`s.
#[derive(Default)]
pub struct ReasoningWiresPlayApp;

//#region 🧵️RetainedCommands
const WIRES_RETAINED_TOOL_IDS: &[&str] = &["canvasPointerUp", "setLocale"];
const WIRES_RETAINED_PAYLOAD_SCHEMA: &str = "reasoning.wires.tool-command.v1";
const WIRES_RETAINED_RAW_BYTES: usize = 8_192;
const WIRES_RETAINED_WORK_ITEMS: usize = 1;
const WIRES_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
];

fn wires_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(WIRES_RETAINED_RAW_BYTES, 16, WIRES_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

fn wires_retained_extent(command: &WiresCommand, _snapshot: &WiresSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        WiresCommand::CanvasPointerUp(_) => Some(WIRES_RETAINED_WORK_ITEMS),
        WiresCommand::SetLocale(payload) if payload.value.len() <= WIRES_RETAINED_RAW_BYTES => Some(WIRES_RETAINED_WORK_ITEMS),
        _ => None,
    }
}

fn wires_retained_reduce(
    command: &WiresCommand,
    _snapshot: &WiresSnapshot,
    _config: &WiresConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _operation: &AppOperationContext,
) -> Result<Emit<WiresMutation, WiresConfigMutation, NoDraftMutation>, Fault> {
    match command {
        WiresCommand::CanvasPointerUp(_) => Ok(Emit::config(vec![WiresConfigMutation::SetDrag(crate::editor::wires::config::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 })])),
        WiresCommand::SetLocale(payload) if payload.value.len() <= WIRES_RETAINED_RAW_BYTES => Ok(Emit::config(vec![WiresConfigMutation::SetLocale(crate::editor::wires::config::SetLocale { value: payload.value.clone() })])),
        _ => Err(Fault::from("wires-retained-route-mismatch")),
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

//#region 📬️ConfigStorePreparation
struct WiresConfigPreparationFactory;

struct WiresConfigPreparation {
    base: Option<store::SnapshotRead<WiresConfig>>,
    mutation: Option<WiresConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(WiresConfig, Vec<WiresConfigMutation>, WiresConfigMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<WiresConfig, WiresConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn wires_config_mutation_bytes(mutation: &WiresConfigMutation) -> usize {
    match mutation {
        WiresConfigMutation::SetDrag(payload) => payload.node_id.as_ref().map_or(0, String::len),
        WiresConfigMutation::SetLocale(payload) => payload.value.len(),
    }
}

fn wires_config_edit(forward: WiresConfigMutation, inverse: Vec<WiresConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<WiresConfigMutation> {
    let id = format!("wires-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<WiresConfig, WiresConfigMutation> for WiresConfigPreparationFactory {
    fn preflight(&self, mutation: &WiresConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || wires_config_mutation_bytes(mutation) > WIRES_RETAINED_RAW_BYTES || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Wires config preparation rejected its lane or bounded envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<WiresConfig, WiresConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<WiresConfig, WiresConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<WiresConfig, WiresConfigMutation>> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES || wires_config_mutation_bytes(&request.mutation) > WIRES_RETAINED_RAW_BYTES { return Err(request); }
        Ok(Box::new(WiresConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<WiresConfig, WiresConfigMutation> for WiresConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Wires config preparation lost its exact base root".to_string())?.get();
            if base.locale.len().saturating_add(base.drag_node_id.as_ref().map_or(0, String::len)) > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES { return Err("Wires config base exceeds retained byte capacity".into()); }
            let mutation = self.mutation.take().ok_or_else(|| "Wires config preparation lost its mutation owner".to_string())?;
            let post = protocol::Mutation::diff(&mutation, base).into_parts().0;
            let inverse = protocol::Mutation::inverse(&mutation, base);
            self.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 0, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Wires config preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Wires config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(wires_config_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<WiresConfig, WiresConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<WiresConfig, WiresConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if self.prepared.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Wires config preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            let bytes = authority.actor().len();
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigStorePreparation

impl ArtifactEditor for ReasoningWiresPlayApp {
    type Snapshot = WiresSnapshot;
    type Mutation = WiresMutation;
    type Config = WiresConfig;
    type ConfigMutation = WiresConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::wires::presence::WiresPresence;
    type PresenceMutation = crate::editor::wires::presence::WiresPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = WiresCommand;

    const DIALECT: Dialect = crate::WIRES_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::MINDMAP_WIRES_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(WiresConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<ReasoningWiresPlayApp>,
        owner_file: "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.reasoning.wires@1/*#editor",
        document_schema: "reasoning.wires.fixture",
        factory: "WiresRetainedCommandJobFactory",
        factory_type: WiresRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 16, 1, 16_384, 7_500),
        tools: ["canvasPointerUp", "setLocale"]
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
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, wires_retained_reduce, wires_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: None, operation: operation_context, completion: request.completion },
            WiresCommand::command_id,
            WIRES_RETAINED_RAW_BYTES,
            WIRES_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::wires::config::schema::app_schema_descriptor())
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
        cfg: &ConfigView<'_, WiresConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<WiresMutation, WiresConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            WiresCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, WiresSnapshot>, cfg: &ConfigView<'_, WiresConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::editor::wires::terminology::WiresLabels>(&cfg.snapshot.locale);
        match body_key {
            WIRES_PLAY_BODY_COMPOSITE => edit::windows::canvas::render(&crate::wires_working_board(document), &document.wires_fixture),
            WIRES_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            WIRES_PLAY_BODY_CATALOGUE => catalogue_panel::render(&document.wires_fixture, labels),
            WIRES_PLAY_BODY_PROPERTIES => inspection_panel::render(document, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "wires diagnostic admission failed")),
        }.map(semio_framework_plugin::built_to_component_tree)
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
        .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
        .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
        .mutation("addRelationship", LocalizedLabel::native("Add Relationship", "Beziehung hinzufügen"))
        .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
        .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
        .mutation("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"))
        .mutation("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
        // 👁️ Ephemeral view state — in-flight drag. Selection/hover are framework-owned now
        // (domain "graph") — no app-declared verbs; `interactionSelect`/`interactionHover`/
        // `clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity` auto-inject
        // below via `.interaction(...)`.
        .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
        .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"))
        .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
        .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("addNode", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("addRelationship", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("deleteSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("forceLayout", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("reorganize", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("canvasPointerMove", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("canvasPointerDown", InteractiveJobClassification::BatchOnlyPendingRewrite)
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
        // truth (the trait default `ConfigSpec::empty()`: none of `WiresConfig`'s fields are
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
