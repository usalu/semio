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

use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
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
    ui_text, ActionDescriptor, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect,
    DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UiNode,
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
pub async fn reset_wires_document_effect(document: &WiresSnapshot) -> Effect {
    let pack = <WiresSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("wires document spr encode is infallible for a fresh, edit-free envelope");
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
pub async fn wires_select_action_args(ids: &[String], granularity: &str, merge: &str) -> Value {
    let targets: Vec<Value> = ids.iter().map(|id| json!({ "granularity": granularity, "id": id })).collect();
    json!({ "domainId": WIRES_INTERACTION_GRAPH, "targets": serde_json::to_string(&targets).unwrap_or_default(), "merge": merge, "method": "pick" })
}

/// 🕹️ Wraps [`wires_select_action_args`] into the redispatch effect a canvas gesture's own `handle`
/// returns — `dispatch_action` intercepts the six framework interaction verbs BEFORE routing to
/// `ArtifactApp::handle`, so a plain config mutation can no longer express a selection change; the app
/// asks the host to redispatch `interactionSelect` instead (master doc: "surfaces do geometric
/// hit-testing and emit one batched `interactionSelect`").
pub async fn wires_select_effect(ids: &[String], granularity: &str, merge: &str) -> Effect {
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
        WiresCommand::CanvasPointerUp(_) => Ok(Emit::config(vec![WiresConfigMutation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }])),
        WiresCommand::SetLocale(payload) if payload.value.len() <= WIRES_RETAINED_RAW_BYTES => Ok(Emit::config(vec![WiresConfigMutation::SetLocale { value: payload.value.clone() }])),
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
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::wires::MINDMAP_WIRES_SCHEMA;
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
        WiresConfigMutation::SetDrag { node_id, .. } => node_id.as_ref().map_or(0, String::len),
        WiresConfigMutation::SetLocale { value } => value.len(),
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
            let mut post = base.clone();
            let inverse = match &mutation {
                WiresConfigMutation::SetDrag { node_id, last_x, last_y } => {
                    let inverse = WiresConfigMutation::SetDrag { node_id: base.drag_node_id.clone(), last_x: base.drag_last_x, last_y: base.drag_last_y };
                    post.drag_node_id = node_id.clone(); post.drag_last_x = *last_x; post.drag_last_y = *last_y;
                    inverse
                }
                WiresConfigMutation::SetLocale { value } => { let inverse = WiresConfigMutation::SetLocale { value: base.locale.clone() }; post.locale = value.clone(); inverse }
            };
            self.candidate = Some((post, vec![inverse], mutation));
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

    const DIALECT: Dialect = crate::artifacts::wires::WIRES_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::wires::MINDMAP_WIRES_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(WiresConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<ReasoningWiresPlayApp>,
        owner_file: "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.reasoning.wires@1/*#editor",
        document_schema: "reasoning.wires.fixture",
        factory: "WiresRetainedCommandJobFactory",
        factory_type: WiresRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 16, 1, 16_384, 7_500),
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
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            operation_context,
            request.completion,
            WiresCommand::command_id,
            WIRES_RETAINED_RAW_BYTES,
            WIRES_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::wires::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> WiresSnapshot {
        crate::artifacts::wires::empty_wires_snapshot()
    }

    /// 🏷️ Supplied wholesale by `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &WiresCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ `deleteSelection` reads the "graph" interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    async fn handle(
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

    async fn render(body_key: &str, doc: &ArtifactView<'_, WiresSnapshot>, cfg: &ConfigView<'_, WiresConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::editor::wires::terminology::WiresLabels>(&cfg.snapshot.locale);
        match body_key {
            WIRES_PLAY_BODY_COMPOSITE => edit::windows::canvas::render(&crate::artifacts::wires::wires_working_board(document), &document.wires_fixture),
            WIRES_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            WIRES_PLAY_BODY_CATALOGUE => catalogue_panel::render(&document.wires_fixture, labels),
            WIRES_PLAY_BODY_PROPERTIES => inspection_panel::render(document),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
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
pub async fn create_wires_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::wires::WIRES_DIALECT)
        .document(["semio", "reasoning", "mindmap", "wires"])
        .artifact_kind(crate::artifacts::wires::artifact_kind())
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
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as new_test_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type WiresApp = VcsArtifactApp<EditorApp<ReasoningWiresPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn new_app() -> WiresApp {
        new_test_app::<EditorApp<ReasoningWiresPlayApp>>()
    }

    /// 🧪️ Framework testkit gap (SDK GAP, see this ticket's `📓️w0-f-report.md` handoff #3):
    /// `assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take `fn() -> App`,
    /// unchanged for this ticket, while `create_wires_app` now returns `AppDefinition` — this tiny
    /// local wrapper bridges the two shapes with an empty `examples` list (dropped per `create_wires_app`'s
    /// own doc comment).
    async fn wires_manifest_for_testkit() -> App {
        App { definition: create_wires_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — required to resolve the "graph" interaction
    /// domain's declaration when dispatching a framework-injected verb like `interactionSelect`.
    pub async fn app_with_registry() -> WiresApp {
        new_app_with_registry::<EditorApp<ReasoningWiresPlayApp>>(wires_manifest_for_testkit)
    }

    /// 🧪️ An app pre-loaded with the metabolism example document, for tests exercising a populated board.
    pub async fn metabolism_app() -> WiresApp {
        let mut app = new_app();
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
        let envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", document, None);
        let files = store::print_document_pack(&envelope).expect("print document pack");
        app.load_document_pack(&files).expect("load metabolism");
        app
    }

    pub async fn dispatch(app: &mut WiresApp, command: WiresCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut WiresApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::wires::testkit::{metabolism_app, new_app, render};
    use semio_framework_plugin::EditorApp;

    const RETAINED_ROUTES: &str = include_str!("🧪️fixtures/retained-command-routes.json");

    #[test]
    fn retained_route_fixture_matches_the_exact_factory_and_fail_closed_census() {
        use semio_framework_plugin::ArtifactOwnedToolJobFactory;
        let fixture: Value = serde_json::from_str(RETAINED_ROUTES).expect("Wires retained route fixture decodes through serde_json");
        assert_eq!(fixture.get("maximumRawBytes").and_then(Value::as_u64), Some(WIRES_RETAINED_RAW_BYTES as u64));
        assert_eq!(fixture.get("maximumWorkItems").and_then(Value::as_u64), Some(WIRES_RETAINED_WORK_ITEMS as u64));
        let routes = fixture.get("routes").and_then(Value::as_array).expect("routes");
        let migrated = routes
            .iter()
            .filter(|route| route.get("disposition").and_then(Value::as_str) == Some("migrated"))
            .map(|route| route.get("id").and_then(Value::as_str).expect("route id"))
            .collect::<Vec<_>>();
        assert_eq!(migrated, WIRES_RETAINED_TOOL_IDS);
        assert_eq!(routes.len(), 10);
        assert_eq!(<WiresRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, WIRES_RETAINED_PUBLICATION_CONTRACTS);
        assert!(WIRES_RETAINED_PUBLICATION_CONTRACTS.iter().all(|row| row.lanes == [ArtifactToolPublicationLane::Config]));
        assert!(routes.iter().filter(|route| route.get("disposition").and_then(Value::as_str) == Some("batch-only-pending-rewrite")).all(|route| route.get("lanes").and_then(Value::as_array).is_some_and(Vec::is_empty)));
    }

    #[test]
    fn config_preparation_rejects_wrong_lane_and_oversized_locale() {
        use store::ArtifactStoreOneItemPreparationFactory;
        let factory = WiresConfigPreparationFactory;
        assert!(factory.preflight(&WiresConfigMutation::SetLocale { value: "de-DE".into() }, None, store::HistoryLane::Document).is_ok());
        assert!(factory.preflight(&WiresConfigMutation::SetLocale { value: "de-DE".into() }, None, store::HistoryLane::Interaction).is_err());
        assert!(factory.preflight(&WiresConfigMutation::SetLocale { value: "x".repeat(WIRES_RETAINED_RAW_BYTES + 1) }, None, store::HistoryLane::Document).is_err());
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 10, "every WiresCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
    /// per-row from the `app_commands!` table's `"id" as "wire-key"` declarations rather than derived
    /// (several rows genuinely diverge from a naive kebab-case of the id: `setLocale` → `locale`,
    /// `setActiveExample` → `active-example`, and all three `canvasPointer*` rows drop the `canvas-`
    /// prefix). This is what a missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the
    /// record prints with no keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keys = [
            ("setActiveExample", "active-example"),
            ("addNode", "add-node"),
            ("addRelationship", "add-relationship"),
            ("deleteSelection", "delete-selection"),
            ("forceLayout", "force-layout"),
            ("reorganize", "reorganize"),
            ("canvasPointerMove", "pointer-move"),
            ("canvasPointerDown", "pointer-down"),
            ("canvasPointerUp", "pointer-up"),
            ("setLocale", "locale"),
        ];
        for command in every_command() {
            let id = command.command_id();
            let expected = expected_keys.iter().find(|(row_id, _)| *row_id == id).map(|(_, key)| *key).unwrap_or_else(|| panic!("no expected wire key recorded for command {id}"));
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ The wire bytes/text pinned from the pre-merge 7-crate baseline (see the ticket's
    /// `🧪️wire-baseline-before.txt`) — a regression here is a real format break, not a fixture mismatch.
    /// `setSelection`/`documentSelect` dissolved into the framework's own "graph" interaction domain
    /// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) and no longer exist as `WiresCommand`
    /// rows, which shifts every later row's binary ordinal by 2 — `CanvasPointerUp`'s and `SetLocale`'s
    /// pinned hex below are updated for the new ordinals (8 and 9); `SetActiveExample` is unaffected
    /// (ordinal 0, before the deleted rows).
    #[semio_framework_async_macros::async_test]
    async fn commands_keep_their_pre_migration_wire_bytes() {
        let node = dsl::to_dsl_value(&serde_json::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).unwrap();
        let _ = node;
        let cases: [(WiresCommand, &str, &str); 3] = [
            (WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "metabolism".into() }), "active-example active-example example-id=metabolism", "0100010a6d657461626f6c69736d01000600"),
            (WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}), "pointer-up pointer-up", "01080000"),
            (WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), "locale locale value=de-DE", "0109010564652d444501000600"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<WiresCommand> {
        vec![
            WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "metabolism".into() }),
            WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }),
            WiresCommand::AddRelationship(add_relationship::AddRelationship { kind: "owns".into() }),
            WiresCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            WiresCommand::ForceLayout(force_layout::ForceLayout {}),
            WiresCommand::Reorganize(reorganize::Reorganize {}),
            WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1.5, y: -2.5 }),
            WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }),
            WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
            WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Interaction
    /// 🕹️ The "graph" domain is declared `HierarchyProvider::Flat`, single-select/pick/replace-only,
    /// and scoped to the canvas window (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    #[semio_framework_async_macros::async_test]
    async fn graph_interaction_domain_is_declared_flat_and_scoped_to_the_canvas_window() {
        let definition = create_wires_app();
        let graph = definition.interactions.iter().find(|interaction| interaction.id == WIRES_INTERACTION_GRAPH).expect("graph interaction domain declared");
        assert!(matches!(graph.hierarchy, HierarchyProvider::Flat));
        assert_eq!(graph.granularities.len(), 2);
        assert!(!graph.selection.transitive, "graph has no hierarchy to close a transitive selection over");
        let canvas_window = definition.window_kinds.iter().find(|window| window.id == WIRES_PLAY_WINDOW_CANVAS).expect("canvas window kind declared");
        assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == WIRES_INTERACTION_GRAPH), "canvas window must reference the graph interaction domain");
    }

    /// 🕹️ `wires_select_action_args` shapes the exact JSON the framework's `interactionSelect` action
    /// expects: `domainId`/`targets` (a JSON-stringified `Vec<InteractionTarget>`)/`merge`/`method`.
    #[semio_framework_async_macros::async_test]
    async fn wires_select_action_args_shapes_interaction_select_payload() {
        let args = wires_select_action_args(&["node-1".to_string()], WIRES_GRANULARITY_NODE, "replace");
        assert_eq!(args["domainId"], WIRES_INTERACTION_GRAPH);
        assert_eq!(args["merge"], "replace");
        assert_eq!(args["method"], "pick");
        assert!(args["targets"].as_str().expect("targets json").contains("node-1"));
        assert!(args["targets"].as_str().expect("targets json").contains(WIRES_GRANULARITY_NODE));
    }
    //#endregion 🔖️Interaction

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_wires_app()).expect("app definition json");
        assert!(json.contains(WIRES_PLAY_WINDOW_CANVAS), "window kind missing from the manifest: {json}");
        assert!(json.contains(edit::WIRES_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [WIRES_PLAY_BODY_DOCUMENT, WIRES_PLAY_BODY_CATALOGUE, WIRES_PLAY_BODY_PROPERTIES] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("graph.wires"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn wires_labels_resolve_native_by_default() {
        let mut app = metabolism_app();
        let json = render(&mut app, WIRES_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Identities") && json.contains("Relationships"));
        let catalogue_json = render(&mut app, WIRES_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Identity kinds"));
        assert!(catalogue_json.contains("Relationship kinds"));
    }

    #[semio_framework_async_macros::async_test]
    async fn metabolism_board_fixture_uses_mindmap_schema() {
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
        let board = crate::artifacts::wires::wires_working_board(&document);
        assert_eq!(board.get("schema").and_then(|value| value.as_str()), Some(crate::artifacts::wires::MINDMAP_BOARD_SCHEMA));
        assert_eq!(crate::artifacts::wires::schema::fixture_nodes(&board).len(), 7);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = new_app();
        assert!(render(&mut app, "reasoning.wires.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(
            &mut app,
            WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }),
            |app| crate::artifacts::wires::schema::fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).len(),
            0,
            1,
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<EditorApp<ReasoningWiresPlayApp>, usize>(WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), |app| {
            crate::artifacts::wires::schema::fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).len()
        });
    }

    /// 🧪️ The definitional merge proof: A adds a node while B renames another node — disjoint edits
    /// on one backbone that must both survive on both instances (impossible under whole-document LWW).
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_graph_edits_via_backbone() {
        use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
        use semio_framework_plugin::testkit::meta;
        use semio_framework_plugin::PluginApp;
        use store::MemoryBackbone;

        let mut instance_a = new_app();
        let mut instance_b = new_app();
        // Seed both from an identical base projection carrying node-1/node-2 (as initial state, not
        // as edits) so the only edits on the channel are A's and B's disjoint ones.
        let seed_node = |id: &str| dsl::to_dsl_value(&serde_json::json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": id, "handles": [] })).expect("seed node");
        let mut base = crate::artifacts::wires::empty_wires_snapshot();
        base = store::apply_mutation(&base, &crate::artifacts::wires::mutations::create_node(seed_node("node-1"))).expect("valid mutation").0;
        base = store::apply_mutation(&base, &crate::artifacts::wires::mutations::create_node(seed_node("node-2"))).expect("valid mutation").0;
        let base_envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", base, None);
        let base_files = store::print_document_pack(&base_envelope).expect("print document pack");
        instance_a.load_document_pack(&base_files).expect("load a");
        instance_b.load_document_pack(&base_files).expect("load b");
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://mindmap-convergence", "mem://mindmap-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        // A adds node-3 (a new node); B moves node-2 (a PatchNode) — disjoint edits on the graph.
        instance_a.dispatch_typed(WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), &meta("actor-a")).expect("a adds node");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-2".into()), x: 0.0, y: 0.0 }), &meta("actor-b")).expect("b down");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 50.0, y: 60.0 }), &meta("actor-b")).expect("b move");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}), &meta("actor-b")).expect("b up");

        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.snapshot().expect("projection a");
        let projection_b = instance_b.snapshot().expect("projection b");
        // A's added node-3 survives on both.
        assert!(find_board_node(&projection_a, "node-3").is_some(), "A keeps its own node");
        assert!(find_board_node(&projection_b, "node-3").is_some(), "B converges on A's node");
        // B's move of node-2 survives on both.
        let x_of = |document: &WiresSnapshot| find_board_node(document, "node-2").map(|node| crate::artifacts::wires::schema::node_position(&node)).unwrap().0;
        assert_eq!(x_of(&projection_a), 50.0, "A converges on B's move");
        assert_eq!(x_of(&projection_b), 50.0, "B keeps its own move");
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
