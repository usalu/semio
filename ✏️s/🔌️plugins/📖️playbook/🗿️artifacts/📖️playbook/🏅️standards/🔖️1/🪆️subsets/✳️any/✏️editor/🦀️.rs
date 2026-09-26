//! ✏️ Playbook editor — the mutation-capable surface for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `PlaybookPlayApp` implements
//! `ArtifactEditor`, never the runtime `ArtifactApp` directly — `EditorApp<PlaybookPlayApp>`
//! (framework SDK) is the sole runtime adapter `PluginBuilder::editor` wires up.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/🏗️builder/🪟️windows/🏗️builder`, labels in `🗣️terminology`, view state in `🎚️config`,
//! shared compute in `⚙️engine`. This file is a routing table: `handle` → `PlaybookCommand::dispatch`,
//! `render` → body-key → node, plus `import_media`'s `"chapters:in"` importer (an editor-level override,
//! not a command).

use crate::editor::playbook::commands::{add_block, add_step, move_block, move_step, remove_block, remove_step, set_active_example, set_contributions, update_playbook};
use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::engine::{playbook_io, PlaybookChapterPayload};
use crate::editor::playbook::modes::builder;
use crate::editor::playbook::modes::builder::windows::builder as builder_window;
use crate::flatten_playbook_blocks;
use crate::op::{AddStep, PlaybookMutation};
use crate::schema::default_block;
use crate::{artifact_kind, PlaybookSnapshot, PlaybookStep, PLAYBOOK_DIALECT, PLAYBOOK_DOCUMENT_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionKind, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView,
    CommandDefinition, ConfigView, Dialect, DomainTopology, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, Media,
    MediaError, MediaPayload, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
use store::EngineHandles;

//#region 🔖️Constants
pub use builder_window::PLAYBOOK_PLAY_BODY_BUILDER;
pub use builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER;

/// 📥️ The step `"chapters:in"` imports land in — created on first import, reused on every later one.
const PLAYBOOK_IMPORTED_STEP_ID: &str = "imported";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `PlaybookPlayApp::Command` — the SOLE dispatch surface for playbook's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the codec uses) — copied verbatim off the pre-migration
    /// `playbook_protocol::PlaybookCommand`'s `#[dsl(key)]` attributes. **Row order is the binary
    /// variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum PlaybookCommand for PlaybookSnapshot, PlaybookMutation, PlaybookConfig, PlaybookConfigMutation {
        "addStep" as "add-step" => add_step::AddStep,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "moveStep" as "move-step" => move_step::MoveStep,
        "addBlock" as "add-block" => add_block::AddBlock,
        "removeBlock" as "remove-block" => remove_block::RemoveBlock,
        "moveBlock" as "move-block" => move_block::MoveBlock,
        "updatePlaybook" as "update-playbook" => update_playbook::UpdatePlaybook,
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Interaction
/// 🕹️ "blocks" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own step/block nesting (steps
/// are the "step" granularity, blocks are the "block" granularity, default) — replaces the deleted
/// `PlaybookConfig::selected_ids`/`set-selection` command/`PlaybookPresence::selected_ids`. Pick-only
/// (the block-list builder is a flat clickable list, no canvas marquee surface); not transitive —
/// the pre-migration `selected_ids` never auto-expanded a step selection onto its blocks, so this
/// keeps that exact semantic instead of inventing new cascading-selection behavior.
pub const PLAYBOOK_INTERACTION_BLOCKS: &str = "blocks";
pub const PLAYBOOK_INTERACTION_GRANULARITY_BLOCK: &str = "block";
pub const PLAYBOOK_INTERACTION_GRANULARITY_STEP: &str = "step";

/// 🌳️ `blocks` domain topology from the document's own step/block nesting — step ids and block ids
/// share the same flat id namespace `PlaybookConfig::selected_ids` used to (`remove-block`'s old manual
/// prune matched on either), so `validate_state` prunes a deleted step's OR block's id automatically
/// after every document dispatch (`revalidate_interaction_state_after_document_change`), replacing the
/// deleted hand-rolled prune in `remove_block::handle`.
fn playbook_blocks_topology(spec: &PlaybookSnapshot) -> DomainTopology {
    let mut ordered = Vec::new();
    for step in spec.steps() {
        ordered.push(TopologyNode { id: step.id.clone(), granularity: PLAYBOOK_INTERACTION_GRANULARITY_STEP.into(), parent: None });
        for block in step.blocks {
            ordered.push(TopologyNode { id: block.id.clone(), granularity: PLAYBOOK_INTERACTION_GRANULARITY_BLOCK.into(), parent: Some(step.id.clone()) });
        }
    }
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

//#region 🔖️PlaybookPlayApp
/// 🧪️ B1: unit struct — the former app-struct `RefCell<Vec<String>>` selection now lives in
/// `PlaybookConfig` (see `ArtifactEditor::Config`), written through `PlaybookConfigMutation`s.
#[derive(Default)]
pub struct PlaybookPlayApp;

/// 🧬️ The whole-document replacement `setActiveExample` emits. `store::empty_document_spr` (never a
/// minted `create_document_envelope`) is what keeps the guest off the `terminal shell reached Drop
/// before its app-owned bounded retirement authority detached` trap on this path; the framework
/// re-stamps the log with the live mount's identity before hydration sees it
/// (`store::stamp_document_spr_identity`), so no app ever states its own mount.
pub fn reset_playbook_document_effect(document: &PlaybookSnapshot) -> semio_framework_plugin::Effect {
    let pack = <PlaybookSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("playbook", PLAYBOOK_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

//#region 🧵️RetainedCommands
/// 🧵️ Every verb the shell may dispatch is a retained tool. `validate_ui_dispatch_classification`
/// refuses anything not classified `Migrated`, and `Migrated` only survives the guest's
/// `interactive-job.catalog-incomplete` boot check when this list, the publication contracts, the
/// extent function and the `bounded_first_step_tool_proofs!` block below all name the same ids
/// against ONE registered factory type. The six structural verbs were `BatchOnlyPendingRewrite`, so
/// the Builder window's whole palette was hard dead in the shell
/// (`UI dispatch rejected action:addStep with interactive-job classification BatchOnlyPendingRewrite`).
/// `updatePlaybook` emits `Emit::amend`: the retained Artifact lane carries its coalesce key onto the
/// store publication, so the title field stays one undo step per typing burst.
const PLAYBOOK_RETAINED_TOOL_IDS: &[&str] = &["setContributions", "setActiveExample", "addStep", "removeStep", "moveStep", "addBlock", "removeBlock", "moveBlock", "updatePlaybook"];
const PLAYBOOK_RETAINED_PAYLOAD_SCHEMA: &str = "playbook.program.tool-command.v1";
const PLAYBOOK_RETAINED_RAW_BYTES: usize = 8_192;
const PLAYBOOK_RETAINED_WORK_ITEMS: usize = 64;

/// 🚦️ Per-tool publication lanes, read straight off the command bodies: `setContributions` writes
/// the config store, the six structural verbs and the coalesced title edit emit `artifact_mutations` only.
const PLAYBOOK_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "updatePlaybook", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn playbook_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(PLAYBOOK_RETAINED_RAW_BYTES, 64, PLAYBOOK_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

fn playbook_retained_extent(command: &PlaybookCommand, _snapshot: &PlaybookSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        PlaybookCommand::SetContributions(payload) => payload.json.len(),
        PlaybookCommand::SetActiveExample(payload) => payload.example_id.len(),
        PlaybookCommand::UpdatePlaybook(payload) => payload.value.len(),
        _ => 0,
    };
    (bytes <= PLAYBOOK_RETAINED_RAW_BYTES && PLAYBOOK_RETAINED_TOOL_IDS.contains(&command.command_id())).then_some(1)
}

/// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into the typed `PlaybookCommand` every
/// dispatch path already speaks. `ArtifactEditor::command_from_action`'s default refuses EVERY id
/// (`app.command.unsupported`), so without this bridge no Builder-window palette row could ever
/// reach `PlaybookCommand::dispatch`.
fn playbook_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<PlaybookCommand, Fault> {
    let entries: &[(String, dsl::DslValue)] = match args {
        Some(dsl::DslValue::Object(object)) => object.as_slice(),
        _ => &[],
    };
    let lookup = |keys: &[&str]| keys.iter().find_map(|key| entries.iter().find(|(name, _)| name == key).map(|(_, value)| value));
    let text = |keys: &[&str], fallback: &str| match lookup(keys) {
        Some(dsl::DslValue::String(raw)) if !raw.is_empty() => raw.clone(),
        Some(dsl::DslValue::String(_)) | None => fallback.to_string(),
        Some(other) => dsl::json::to_json_string(other),
    };
    let index = |keys: &[&str]| match lookup(keys) {
        Some(dsl::DslValue::Number(value)) => value.as_f64().max(0.0) as usize,
        Some(dsl::DslValue::String(raw)) => raw.trim().parse::<usize>().unwrap_or_default(),
        _ => 0,
    };
    let optional_text = |keys: &[&str]| match lookup(keys) {
        Some(dsl::DslValue::String(raw)) if !raw.is_empty() => Some(raw.clone()),
        _ => None,
    };
    match action {
        "addStep" => Ok(PlaybookCommand::AddStep(add_step::AddStep {})),
        "removeStep" => Ok(PlaybookCommand::RemoveStep(remove_step::RemoveStep { step_id: text(&["stepId", "step_id", "id", "value"], "") })),
        "moveStep" => Ok(PlaybookCommand::MoveStep(move_step::MoveStep { step_id: text(&["stepId", "step_id", "id", "value"], ""), index: index(&["index"]) })),
        "addBlock" => Ok(PlaybookCommand::AddBlock(add_block::AddBlock { kind: text(&["kind", "value"], "note"), step_id: optional_text(&["stepId", "step_id"]) })),
        "removeBlock" => Ok(PlaybookCommand::RemoveBlock(remove_block::RemoveBlock { step_id: text(&["stepId", "step_id"], ""), block_id: text(&["blockId", "block_id", "id", "value"], "") })),
        "moveBlock" => Ok(PlaybookCommand::MoveBlock(move_block::MoveBlock {
            block_id: text(&["blockId", "block_id", "id"], ""),
            from_step_id: text(&["fromStepId", "from_step_id"], ""),
            to_step_id: text(&["toStepId", "to_step_id"], ""),
            index: index(&["index"]),
        })),
        "updatePlaybook" => Ok(PlaybookCommand::UpdatePlaybook(update_playbook::UpdatePlaybook { value: text(&["value", "title"], "") })),
        "setContributions" => Ok(PlaybookCommand::SetContributions(set_contributions::SetContributions { json: text(&["json", "value"], "{}") })),
        "setActiveExample" => Ok(PlaybookCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text(&["exampleId", "example_id", "id", "value"], crate::examples::demo::ID) })),
        other => Err(Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("playbook.unhandled-action"),
            format!("action '{other}' is not one of this app's declared verbs"),
        )),
    }
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn playbook_retained_reduce(
    command: &PlaybookCommand,
    snapshot: &PlaybookSnapshot,
    config: &PlaybookConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PlaybookPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

struct PlaybookRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl PlaybookRetainedCommandJobFactory {
    fn new(controller: &str) -> Self {
        Self { keys: PLAYBOOK_RETAINED_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl ToolJobFactory for PlaybookRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<PlaybookPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<PlaybookPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        PLAYBOOK_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        playbook_retained_contract()
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
        if input.declared_bytes() > PLAYBOOK_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Playbook retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for PlaybookRetainedCommandJobFactory {
    type Owner = EditorApp<PlaybookPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PLAYBOOK_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PLAYBOOK_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = PLAYBOOK_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️OneItemPreparation
const PLAYBOOK_STORE_MAXIMUM_BYTES: usize = 32_768;

struct PlaybookOneItemPreparationFactory<P, M>(std::marker::PhantomData<fn() -> (P, M)>);

impl<P, M> Default for PlaybookOneItemPreparationFactory<P, M> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

struct PlaybookOneItemPreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(P, Vec<M>, M, usize)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

// 🌉️ `protocol::json::to_json_string` (`pack`'s `ToValue`-keyed helper) has no streaming-writer
// analog of `serde_json::to_writer` — it materializes the full JSON text before the length is
// known. `PLAYBOOK_STORE_MAXIMUM_BYTES` is small (32KiB) so this is an accepted trade-off, not a
// bounded/incremental check anymore.
fn playbook_bounded_serialized_bytes<T: protocol::ToValue>(value: &T) -> Result<usize, String> {
    let bytes = protocol::json::to_json_string(value).len();
    if bytes > PLAYBOOK_STORE_MAXIMUM_BYTES {
        return Err("Playbook retained Store value exceeds its fixed envelope".to_string());
    }
    Ok(bytes)
}

fn playbook_one_item_edit<M>(forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("playbook-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for PlaybookOneItemPreparationFactory<P, M>
where
    P: Clone + protocol::ToValue + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::ToValue + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn preflight(&self, mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Playbook retained preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: playbook_bounded_serialized_bytes(mutation)? })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(PlaybookOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for PlaybookOneItemPreparation<P, M>
where
    P: Clone + protocol::ToValue + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::ToValue + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() || self.phase >= 2 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            0 => {
                let base = self.base.as_ref().ok_or_else(|| "Playbook retained preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Playbook retained preparation lost its mutation owner".to_string())?;
                let retained_bytes = playbook_bounded_serialized_bytes(base.get())?;
                let inverse = mutation.inverse(base.get());
                let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
                self.candidate = Some((post, inverse, mutation, retained_bytes));
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: retained_bytes as u64, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, mutation, retained_bytes) = self.candidate.take().ok_or_else(|| "Playbook retained preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Playbook retained preparation lost its Store authority".to_string())?;
                let prepared = authority.prepare_one_item(playbook_one_item_edit(mutation, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
                self.phase = 2;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: retained_bytes as u64, digest: prepared.edit_digest() };
                self.prepared = Some(prepared);
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
        }
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Playbook retained preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️OneItemPreparation

impl ArtifactEditor for PlaybookPlayApp {
    /// 🧩️ The roster both composed `s.stdio.semio` children (`document`, `flow`) open through. A
    /// `NoMembers` editor cannot materialise the children `genesis_child_pack` derives, so every
    /// whole-document load fails its archive closure leg before any of them is opened.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = PlaybookSnapshot;
    type Mutation = PlaybookMutation;
    type Config = PlaybookConfig;
    type ConfigMutation = PlaybookConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::app::NoTransient;
    type TransientMutation = semio_framework_plugin::app::NoTransientMutation;

    type Command = PlaybookCommand;

    const DIALECT: Dialect = PLAYBOOK_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PLAYBOOK_DOCUMENT_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(PlaybookOneItemPreparationFactory::<Self::Config, Self::ConfigMutation>::default()))
    }

    /// 📬️ The ARTIFACT lane's publication authority. Without it the six structural verbs reach the
    /// typed operation and die there: the app owns a config-lane preparation only, and a retained
    /// tool whose contract states `Artifact` has nowhere to stage its edit.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("playbook-artifact-retained", PLAYBOOK_STORE_MAXIMUM_BYTES))
    }

    /// ♻️ The exact store owners and disposers every lane of this editor retires through. The app
    /// declared none at all, so the first real document publication answered
    /// `returned snapshot read requires its exact owned-snapshot retirement factory` and the whole
    /// interaction chain stopped one step short of the document.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    /// 👥️ A disposer is not a retirement owner. `PresenceStore::local_read` fails closed with
    /// `presence local read requires a live exact local retirement owner` while
    /// `local_retirement_factory` is `None`, so the FIRST command whose ephemeral leg reads local
    /// presence dies — observed at `#playbook` boot as
    /// `setContributions command failed: playbook presence local read requires a live exact local
    /// retirement owner` (ticket 26/09/19 play-grid strict acceptance). `Presence = NoPresence` here,
    /// so the framework's own `NoPresence`-typed owners are exactly right; an app with a REAL
    /// presence type needs a bounded owner over that type instead, as 🪐️space's Home does
    /// (`HomePresenceRetirementFactory`, ticket 26/09/18 S10 §2.9a — the same defect, same week).
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<PlaybookPlayApp>,
        owner_file: "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.playbook.playbook@1/*#editor",
        artifact_schema: "playbook.program",
        factory: "PlaybookRetainedCommandJobFactory",
        factory_type: PlaybookRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 64, 64, 16_384, 7_500),
        tools: ["setContributions", "setActiveExample", "addStep", "removeStep", "moveStep", "addBlock", "removeBlock", "moveBlock", "updatePlaybook"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(PlaybookRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PLAYBOOK_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || playbook_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("playbook-retained-command-tool-mismatch-or-capacity"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, playbook_retained_reduce, playbook_retained_extent));
        let operation = AppOperationContext {
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
                operation,
                completion: request.completion,
            },
            PlaybookCommand::command_id,
            PLAYBOOK_RETAINED_RAW_BYTES,
            PLAYBOOK_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::playbook::config::schema::app_schema_descriptor())
    }

    /// 🏗️ Admits the whole-document replacement `reset_playbook_document_effect` emits for every
    /// example switch. The trait default refuses the envelope, so the host answers every
    /// `setActiveExample` with `artifact-store.persisted-initializer-refused` at the archive-load
    /// boundary.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, PLAYBOOK_DOCUMENT_SCHEMA, operation, generation))
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
        crate::genesis_playbook_child_pack(snapshot, slot, child_id)
    }

    fn initial_snapshot() -> PlaybookSnapshot {
        crate::empty_playbook_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(playbook_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &PlaybookCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<PlaybookCommand, Fault> {
        playbook_command_from_action(action, args)
    }

    fn handle(
        command: &PlaybookCommand,
        doc: &ArtifactView<'_, PlaybookSnapshot>,
        cfg: &ConfigView<'_, PlaybookConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ `blocks` domain: `HierarchyProvider::Topology` from the document's own step/block nesting —
    /// see `playbook_blocks_topology`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PLAYBOOK_INTERACTION_BLOCKS.to_string(), playbook_blocks_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    /// 🎞️ `"chapters:in"` (Text×Document, `Many`) — decodes a `writer`-shaped chapter payload (see
    /// `writer_engine::WriterChapterPayload`/`PlaybookChapterPayload`) and inserts it as a `"note"` block
    /// (free-form `text` field, non-interactive) into a dedicated `"imported"` step, created on first
    /// import and reused on every later one (idempotent step creation).
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, PlaybookSnapshot>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "chapters:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "chapters:in importer only accepts a Structured payload".into()));
        };
        let chapter: PlaybookChapterPayload = protocol::json::from_json_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let spec = doc.snapshot;
        let mut operations = Vec::new();
        if !spec.steps().iter().any(|step| step.id == PLAYBOOK_IMPORTED_STEP_ID) {
            operations.push(PlaybookMutation::AddStep(AddStep { step: PlaybookStep { id: PLAYBOOK_IMPORTED_STEP_ID.into(), title: "Imported".into(), description: None, blocks: Vec::new() }, index: None }));
        }
        let block_id = format!("chapter-{}", flatten_playbook_blocks(spec).len() + 1);
        let mut block = default_block(block_id, "note");
        block.label = chapter.title;
        block.text = Some(chapter.text);
        operations.push(crate::op::add_block_operation(PLAYBOOK_IMPORTED_STEP_ID, block, None));
        Ok(Emit::mutations(operations))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, PlaybookSnapshot>, cfg: &ConfigView<'_, PlaybookConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            PLAYBOOK_PLAY_BODY_BUILDER => Ok(semio_framework_plugin::built_to_component_tree(builder_window::render(doc.snapshot, cfg.snapshot)?)),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️PlaybookPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_playbook_play_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(PLAYBOOK_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
        .document(["semio", "playbook"])
        .artifact_kind(artifact_kind())
        .mode_def(builder::definition())
        .default_mode_id(builder::PLAYBOOK_PLAY_MODE_BUILDER)
        .window_kind_def(builder_window::definition())
        .default_layout(builder::layout())
        .mutation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
        .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
        .action_destructive("removeStep")
        .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
        .mutation("addBlock", LocalizedLabel::native("Add Block", "Baustein hinzufügen"))
        .mutation("removeBlock", LocalizedLabel::native("Remove Block", "Baustein entfernen"))
        .action_destructive("removeBlock")
        .mutation("moveBlock", LocalizedLabel::native("Move Block", "Baustein verschieben"))
        .mutation("updatePlaybook", LocalizedLabel::native("Update Playbook", "Playbook aktualisieren"))
        .action_interactive_job("addStep", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeStep", InteractiveJobClassification::Migrated)
        .action_interactive_job("moveStep", InteractiveJobClassification::Migrated)
        .action_interactive_job("addBlock", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeBlock", InteractiveJobClassification::Migrated)
        .action_interactive_job("moveBlock", InteractiveJobClassification::Migrated)
        .action_interactive_job("updatePlaybook", InteractiveJobClassification::Migrated)
        .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
        // 🧬️ The example picker's verb. The subset registers `crate::examples::demo`, so the shell
        // dispatches this at boot and on every navbar pick; with no declaration at all every one of
        // those was dropped `undeclared-action` before it reached the app.
        .action_with(
            semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Beispiel setzen"), ActionKind::View, "file")
                .with_args(vec![ActionArgDef::text("exampleId", LocalizedLabel::native("Example", "Beispiel")).default_value(&crate::examples::demo::ID)]),
        )
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        // 📝️ Staged argument form for the panel-visible create action (block kind is a choice).
        .action_args("addBlock", vec![
            ActionArgDef::select(
                "kind",
                LocalizedLabel::native("Kind", "Art"),
                crate::PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
            )
            .default_value(&"text"),
        ])
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "blocks" interaction
        // domain — two granularities ("block" default, "step"), `HierarchyProvider::Topology` from
        // the document's own step/block nesting (`playbook_blocks_topology`/
        // `PlaybookPlayApp::interaction_topology`). Selection is pick-only (no canvas marquee
        // surface exists for this domain); the framework auto-injects interactionSelect/
        // interactionHover/clearSelection/selectAll/setSelectionMode/setInteractionGranularity,
        // replacing the deleted `setSelection` view action.
        .interaction(InteractionDefinition {
            id: PLAYBOOK_INTERACTION_BLOCKS.into(),
            label: LocalizedLabel::native("Blocks", "Bausteine"),
            granularities: vec![
                GranularityDefinition { id: PLAYBOOK_INTERACTION_GRANULARITY_BLOCK.into(), label: LocalizedLabel::native("Block", "Baustein"), icon_id: "square".into() },
                GranularityDefinition { id: PLAYBOOK_INTERACTION_GRANULARITY_STEP.into(), label: LocalizedLabel::native("Step", "Schritt"), icon_id: "list-ordered".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                transitive: false,
                broadcast: true,
            },
        })
        .window_kind_interactions(PLAYBOOK_PLAY_WINDOW_BUILDER, vec![InteractionRef::new(PLAYBOOK_INTERACTION_BLOCKS)])
        // 🎯️ Typed channel surface (mirrors `writer_ui::create_writer_app`'s identical wiring) —
        // `crate::editor::playbook::engine::playbook_io()` is the single source of truth for both
        // the trait's `io()` override and this manifest declaration.
        .config(PlaybookPlayApp::config_spec())
        .io(playbook_io())
        .action_describe("addStep", LocalizedLabel::native("Appends a new, empty step to the end of the playbook.", "Hängt dem Playbook am Ende einen neuen, leeren Schritt an."))
        .action_describe("removeStep", LocalizedLabel::native("Removes one step by id from the playbook together with every block it contains.", "Entfernt einen Schritt anhand seiner Id samt aller enthaltenen Bausteine aus dem Playbook."))
        .action_describe("moveStep", LocalizedLabel::native("Moves one step to a new position (index) in the playbook.", "Verschiebt einen Schritt an eine neue Position (Index) im Playbook."))
        .action_describe("addBlock", LocalizedLabel::native("Adds a new block of the given kind (such as a procedural building component) to the named step, or to the default step when none is named.", "Fügt einem Schritt einen neuen Baustein der angegebenen Art (etwa ein prozedurales Bauteil) hinzu, ohne Angabe dem Standardschritt."))
        .action_describe("removeBlock", LocalizedLabel::native("Removes one block by id from the given step of the playbook.", "Entfernt einen Baustein anhand seiner Id aus dem angegebenen Schritt des Playbooks."))
        .action_describe("moveBlock", LocalizedLabel::native("Moves one block from its step to a position (index) in another step or the same one.", "Verschiebt einen Baustein aus seinem Schritt an eine Position (Index) in einem anderen oder demselben Schritt."))
        .action_describe("updatePlaybook", LocalizedLabel::native("Sets the playbook's title; an empty value clears it, and consecutive edits merge into one undo step.", "Legt den Titel des Playbooks fest; ein leerer Wert entfernt ihn, aufeinanderfolgende Änderungen werden zu einem Rückgängig-Schritt zusammengefasst."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole playbook with the bundled demo playbook, or with an empty playbook for any other example id.", "Ersetzt das gesamte Playbook durch das mitgelieferte Demo-Playbook, bei jeder anderen Beispiel-Id durch ein leeres Playbook."))
        .action_destructive("setActiveExample")
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
#[path = "👥️presence/🧬️schema/🦀️.rs"]
pub mod schema;
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
