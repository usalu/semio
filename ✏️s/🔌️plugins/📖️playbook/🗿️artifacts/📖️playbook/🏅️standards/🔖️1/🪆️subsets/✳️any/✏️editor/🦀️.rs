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
use crate::editor::playbook::modes::builder::windows::activity as activity_window;
use crate::editor::playbook::modes::builder::windows::builder as builder_window;
use crate::editor::playbook::modes::builder::windows::changes as changes_window;
use crate::editor::playbook::modes::builder::windows::files as files_window;
use crate::editor::playbook::modes::builder::windows::source as source_window;
use crate::editor::playbook::modes::builder::windows::steps as steps_window;
use crate::op::PlaybookMutation;
use crate::schema::default_block;
use crate::{artifact_kind, playbook_add_block_leaves, playbook_add_step_leaves, playbook_composed_spec, playbook_flow_content, playbook_flow_emit, steps_from_flow_content, PlaybookSnapshot, PlaybookSpec, PlaybookStep, PLAYBOOK_DIALECT, PLAYBOOK_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants
pub use builder_window::PLAYBOOK_PLAY_BODY_BUILDER;
pub use builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER;
pub use changes_window::PLAYBOOK_PLAY_BODY_CHANGES;
pub use changes_window::PLAYBOOK_PLAY_WINDOW_CHANGES;
pub use files_window::{PLAYBOOK_PLAY_BODY_FILES, PLAYBOOK_PLAY_WINDOW_FILES};
pub use activity_window::{PLAYBOOK_PLAY_BODY_ACTIVITY, PLAYBOOK_PLAY_WINDOW_ACTIVITY};
pub use source_window::{PLAYBOOK_PLAY_BODY_SOURCE, PLAYBOOK_PLAY_WINDOW_SOURCE};
pub use steps_window::{PLAYBOOK_PLAY_BODY_STEPS, PLAYBOOK_PLAY_WINDOW_STEPS};

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
/// this app declares: `HierarchyProvider::Topology` over the composed playbook's step/block nesting (steps
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
/// after every document dispatch (`revalidate_interaction_on_document_change`), replacing the
/// deleted hand-rolled prune in `remove_block::handle`.
fn playbook_blocks_topology(spec: &PlaybookSpec) -> DomainTopology {
    let mut ordered = Vec::new();
    for step in &spec.steps {
        ordered.push(TopologyNode { id: step.id.clone(), granularity: PLAYBOOK_INTERACTION_GRANULARITY_STEP.into(), parent: None });
        for block in &step.blocks {
            ordered.push(TopologyNode { id: block.id.clone(), granularity: PLAYBOOK_INTERACTION_GRANULARITY_BLOCK.into(), parent: Some(step.id.clone()) });
        }
    }
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

//#region 🔖️ChildLaneEmit
/// 🌊️ Publishes child intent `leaves` as ONE edit of the playbook's `flow` child (design §20.15); no leaf is the empty emit.
pub fn playbook_child_leaves_emit<D>(snapshot: &PlaybookSnapshot, leaves: Vec<SemioFlowMutation>) -> Emit<PlaybookMutation, PlaybookConfigMutation, D> {
    if leaves.is_empty() {
        return Emit::default();
    }
    Emit { child_preparations: std::collections::VecDeque::from([playbook_flow_emit(snapshot, leaves)]), ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }
}

/// 🆔️ An id `doc`'s admitted operation mints under `prefix`: content-addressed over the admission's authoring seed and operation
/// id, so two writers — or two sessions of one — never mint the same id at one base.
pub fn playbook_minted_id(doc: &ArtifactView<'_, PlaybookSnapshot>, prefix: &str) -> Result<String, Fault> {
    let operation = doc.operation()?;
    Ok(store::content_id(prefix, format!("{}\u{1f}{}", operation.authoring_seed, operation.operation_id).as_bytes()))
}
//#endregion 🔖️ChildLaneEmit

//#region 🔖️PlaybookPlayApp
/// 🧪️ B1: unit struct — the former app-struct `RefCell<Vec<String>>` selection now lives in
/// `PlaybookConfig` (see `ArtifactEditor::Config`), written through `PlaybookConfigMutation`s.
#[derive(Default)]
pub struct PlaybookPlayApp;

/// 🧬️ The whole-document replacement `setActiveExample` emits. `store::os_store::text_format::empty_document_spr` (never a
/// minted `create_document_envelope`) is what keeps the guest off the `terminal shell reached Drop
/// before its app-owned bounded retirement authority detached` trap on this path; the framework
/// re-stamps the log with the live mount's identity before hydration sees it
/// (`store::os_store::text_format::stamp_document_spr_identity`), so no app ever states its own mount.
pub fn reset_playbook_document_effect(document: &PlaybookSnapshot) -> semio_framework_plugin::Effect {
    let pack = <PlaybookSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("playbook", PLAYBOOK_DOCUMENT_SCHEMA));
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
/// `updatePlaybook` emits the absolute `change-title` once per dispatch (palette, agent, a blur-committed field): one
/// edit per committed title, never a coalesced typing burst.
pub const PLAYBOOK_RETAINED_TOOL_IDS: &[&str] = &["setContributions", "setActiveExample", "addStep", "removeStep", "moveStep", "addBlock", "removeBlock", "moveBlock", "updatePlaybook"];
const PLAYBOOK_RETAINED_PAYLOAD_SCHEMA: &str = "playbook.program.tool-command.v1";
const PLAYBOOK_RETAINED_RAW_BYTES: usize = 8_192;
const PLAYBOOK_RETAINED_WORK_ITEMS: usize = 64;

/// 🚦️ Per-tool publication lanes, read straight off the command bodies: `setContributions` writes the config store, the six
/// structural verbs edit the `flow` child's lane only (design §20.15), the title edit emits the parent's `change-title`.
pub const PLAYBOOK_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "addBlock", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "removeBlock", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "moveBlock", lanes: &[ArtifactToolPublicationLane::Child] },
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
fn playbook_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<PlaybookCommand, Fault> {
    let entries: &[(String, semio_framework_value::DslValue)] = match args {
        Some(semio_framework_value::DslValue::Object(object)) => object.as_slice(),
        _ => &[],
    };
    let lookup = |keys: &[&str]| keys.iter().find_map(|key| entries.iter().find(|(name, _)| name == key).map(|(_, value)| value));
    let text = |keys: &[&str], fallback: &str| match lookup(keys) {
        Some(semio_framework_value::DslValue::String(raw)) if !raw.is_empty() => raw.clone(),
        Some(semio_framework_value::DslValue::String(_)) | None => fallback.to_string(),
        Some(other) => semio_framework_pack_json::to_json_string(other),
    };
    let index = |keys: &[&str]| match lookup(keys) {
        Some(semio_framework_value::DslValue::Number(value)) => value.as_f64().max(0.0) as usize,
        Some(semio_framework_value::DslValue::String(raw)) => raw.trim().parse::<usize>().unwrap_or_default(),
        _ => 0,
    };
    let optional_text = |keys: &[&str]| match lookup(keys) {
        Some(semio_framework_value::DslValue::String(raw)) if !raw.is_empty() => Some(raw.clone()),
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
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{other}' is not one of this app's declared verbs"))),
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
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PlaybookPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, NoDraftMutation>, Fault> {
    let children = context.map(|context| semio_framework_plugin::ChildContentView::clone(&context.children)).unwrap_or_default();
    command.dispatch(&ArtifactView::with_children(snapshot, history, children).bound_to_operation(operation.clone()), &ConfigView { snapshot: config, window: None })
}

pub struct PlaybookRetainedCommandJobFactory {
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
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: PLAYBOOK_RETAINED_RAW_BYTES, checkpoint_resume: false);

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

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct PlaybookOneItemPreparationFactory<P, M>(std::marker::PhantomData<fn() -> (P, M)>);

impl<P, M> Default for PlaybookOneItemPreparationFactory<P, M> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

struct PlaybookOneItemPreparation<P: 'static, M: 'static> {
    owners: store::OneItemOwners<P, M>,
    phase: u8,
    retained_bytes: usize,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

// 🌉️ `protocol::json::to_json_string` (`pack`'s `ToValue`-keyed helper) has no streaming-writer
// analog of `serde_json::to_writer` — it materializes the full JSON text before the length is
// known. `PLAYBOOK_STORE_MAXIMUM_BYTES` is small (32KiB) so this is an accepted trade-off, not a
// bounded/incremental check anymore.
fn playbook_bounded_serialized_bytes<T: semio_framework_value::ToValue>(value: &T) -> Result<usize, String> {
    let bytes = semio_framework_pack_json::to_json_string(value).len();
    if bytes > PLAYBOOK_STORE_MAXIMUM_BYTES {
        return Err("Playbook retained Store value exceeds its fixed envelope".to_string());
    }
    Ok(bytes)
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M> for PlaybookOneItemPreparationFactory<P, M>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::ToValue + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<M>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::admit_artifact_batch_digest(edit, grant) }

    fn preflight(&self, mutation: &M, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Playbook retained preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, playbook_bounded_serialized_bytes(mutation)?))
    }

    fn begin_demand(&self, _mutation: &M, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<PlaybookOneItemPreparation<P, M>>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };

        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "preparation rejected original publication authority"), request));
        }
        Ok((Box::new(PlaybookOneItemPreparation {
            owners: store::OneItemOwners::from_request(request),
            phase: 0,
            retained_bytes: 0,
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M> for PlaybookOneItemPreparation<P, M>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Playbook retained preparation lost its exact base root"))?;
                let retained_bytes = playbook_bounded_serialized_bytes(base.get()).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Playbook retained Store value exceeds its fixed envelope"))?;
                if grant.maximum_copy_bytes < retained_bytes {
                    return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
                }
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Playbook retained preparation lost its mutation owner"))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff_cold(mutation.diff(base.get()).diff(), base.get()).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation could not apply its original mutation"))?;
                let forward = self.owners.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Playbook retained preparation lost its mutation owner"))?;
                *self.owners.candidate = Some((post, inverse, forward));
                self.retained_bytes = retained_bytes;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: retained_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: retained_bytes, ..Default::default() }))
            }
            1 => {
                let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Playbook retained preparation lost its Store authority"))?;
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Playbook retained preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️OneItemPreparation

impl ArtifactEditor for PlaybookPlayApp {
    /// 🧩️ The roster the composed `s.stdio.semio@v1/flow` child opens through. A `NoMembers` editor cannot materialise the child
    /// `genesis_child_pack` derives, so every whole-document load would fail its archive closure leg before it is opened.
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

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(PlaybookOneItemPreparationFactory::<Self::Config, Self::ConfigMutation>::default()))
    }

    /// 📬️ The ARTIFACT lane's publication authority for the parent's own `change-title` (`updatePlaybook`): a retained tool whose
    /// contract states `Artifact` has nowhere to stage its edit without it.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// ♻️ The exact store owners and disposers every lane of this editor retires through. The app
    /// declared none at all, so the first real document publication answered
    /// `returned snapshot read requires its exact owned-snapshot retirement factory` and the whole
    /// interaction chain stopped one step short of the document.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
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

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !PLAYBOOK_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id || playbook_retained_extent(&original.command, &original.snapshot, &original.interaction_state) != Some(1) {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "Playbook command does not match its exact registered tool or exceeds its declared extent").into());
        }
        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, PlaybookCommand::command_id, PLAYBOOK_RETAINED_RAW_BYTES, PLAYBOOK_RETAINED_WORK_ITEMS, false, |request| BoundedArtifactCommandWork::new(tool_id, playbook_retained_reduce, playbook_retained_extent))
    }

    fn app_schema() -> Option<semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::playbook::config::schema::app_schema_descriptor())
    }

    /// 🪆️ The one `flow` member a loaded parent restores — see `crate::playbook_child_restore_projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::playbook_child_restore_projection(snapshot)
    }

    /// 🌱️ The `flow` member's genesis, answered from this plugin's own catalogue — see `crate::genesis_playbook_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        Ok(crate::genesis_playbook_child_pack(snapshot, slot, child_id))
    }

    /// 📣️ The localized notices of this artifact's refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        crate::playbook_fault_notices()
    }

    fn reference_birth()->Option<fn()->Result<PlaybookSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(crate::empty_playbook_snapshot())
    })}

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(playbook_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &PlaybookCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<PlaybookCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
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

    /// 🕹️ `blocks` domain: `HierarchyProvider::Topology` from the composed playbook's step/block nesting — see
    /// `playbook_blocks_topology`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
        let spec = playbook_composed_spec(doc.snapshot, &doc.children).map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("{}: {}", fault.code.0, fault.message)))?;
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PLAYBOOK_INTERACTION_BLOCKS.to_string(), playbook_blocks_topology(&spec));
        Ok(InteractionTopology { domains })
    }

    /// 🎞️ `"chapters:in"` (Text×Document, `Many`) — decodes a `writer`-shaped chapter payload (see
    /// `writer_engine::WriterChapterPayload`/`PlaybookChapterPayload`) and inserts it as a `"note"` block (free-form `text` field,
    /// non-interactive) into a dedicated `"imported"` step, created on first import and reused on every later one (idempotent step
    /// creation) — ONE edit of the `flow` child either way.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, PlaybookSnapshot>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "chapters:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "chapters:in importer only accepts a Structured payload".into()));
        };
        let chapter: PlaybookChapterPayload = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let refused = |fault: Fault| MediaError::Payload(port.to_string(), format!("{}: {}", fault.code.0, fault.message));
        let content = playbook_flow_content(doc.snapshot, &doc.children).map_err(refused)?;
        let steps = steps_from_flow_content(&content).map_err(|message| MediaError::Payload(port.to_string(), message))?;
        let mut block = default_block(format!("chapter-{}", steps.iter().map(|step| step.blocks.len()).sum::<usize>() + 1), "note");
        block.label = chapter.title;
        block.text = Some(chapter.text);
        let leaves = if steps.iter().any(|step| step.id == PLAYBOOK_IMPORTED_STEP_ID) {
            playbook_add_block_leaves(&content, PLAYBOOK_IMPORTED_STEP_ID, block, None)
        } else {
            playbook_add_step_leaves(&content, &PlaybookStep { id: PLAYBOOK_IMPORTED_STEP_ID.into(), title: "Imported".into(), description: None, blocks: vec![block] })
        }
        .map_err(refused)?;
        Ok(playbook_child_leaves_emit(doc.snapshot, leaves))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, PlaybookSnapshot>, cfg: &ConfigView<'_, PlaybookConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let spec = || playbook_composed_spec(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new(fault.code.0, fault.message));
        match body_key {
            PLAYBOOK_PLAY_BODY_BUILDER => Ok(semio_framework_plugin::built_to_component_tree(builder_window::render(&spec()?, cfg.snapshot)?)),
            PLAYBOOK_PLAY_BODY_CHANGES => Ok(semio_framework_plugin::built_to_component_tree(changes_window::render(&spec()?)?)),
            PLAYBOOK_PLAY_BODY_STEPS => Ok(semio_framework_plugin::built_to_component_tree(steps_window::render(&spec()?)?)),
            PLAYBOOK_PLAY_BODY_ACTIVITY => Ok(semio_framework_plugin::built_to_component_tree(activity_window::render(&spec()?)?)),
            PLAYBOOK_PLAY_BODY_SOURCE => Ok(semio_framework_plugin::built_to_component_tree(source_window::render(&spec()?)?)),
            PLAYBOOK_PLAY_BODY_FILES => Ok(semio_framework_plugin::built_to_component_tree(files_window::render(&spec()?, crate::editor::playbook::terminology::playbook_play_labels(view_state))?)),
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
        .window_kind_def(steps_window::definition())
        .window_kind_def(changes_window::definition())
        .window_kind_def(activity_window::definition())
        .window_kind_def(source_window::definition())
        .window_kind_def(files_window::definition())
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
        .action_with(
            semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Beispiel setzen"), ActionKind::View, "file")
                .with_args(vec![ActionArgDef::text("exampleId", LocalizedLabel::native("Example", "Beispiel")).default_value(&crate::examples::demo::ID)]),
        )
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_args("addBlock", vec![
            ActionArgDef::select(
                "kind",
                LocalizedLabel::native("Kind", "Art"),
                crate::PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
            )
            .default_value(&"text"),
        ])
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
        .config(PlaybookPlayApp::config_spec())
        .io(playbook_io())
        .action_describe("addStep", LocalizedLabel::native("Appends a new, empty step to the end of the playbook.", "Hängt dem Playbook am Ende einen neuen, leeren Schritt an."))
        .action_describe("removeStep", LocalizedLabel::native("Removes one step by id from the playbook together with every block it contains.", "Entfernt einen Schritt anhand seiner Id samt aller enthaltenen Bausteine aus dem Playbook."))
        .action_describe("moveStep", LocalizedLabel::native("Moves one step to a new position (index) in the playbook.", "Verschiebt einen Schritt an eine neue Position (Index) im Playbook."))
        .action_describe("addBlock", LocalizedLabel::native("Adds a new block of the given kind (such as a procedural building component) to the named step, or to the default step when none is named.", "Fügt einem Schritt einen neuen Baustein der angegebenen Art (etwa ein prozedurales Bauteil) hinzu, ohne Angabe dem Standardschritt."))
        .action_describe("removeBlock", LocalizedLabel::native("Removes one block by id from the given step of the playbook.", "Entfernt einen Baustein anhand seiner Id aus dem angegebenen Schritt des Playbooks."))
        .action_describe("moveBlock", LocalizedLabel::native("Moves one block from its step to a position (index) in another step or the same one.", "Verschiebt einen Baustein aus seinem Schritt an eine Position (Index) in einem anderen oder demselben Schritt."))
        .action_describe("updatePlaybook", LocalizedLabel::native("Sets the playbook's title in one undo step; an empty value clears it.", "Legt den Titel des Playbooks in einem Rückgängig-Schritt fest; ein leerer Wert entfernt ihn."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole playbook with the bundled demo playbook, or with an empty playbook for any other example id.", "Ersetzt das gesamte Playbook durch das mitgelieferte Demo-Playbook, bei jeder anderen Beispiel-Id durch ein leeres Playbook."))
        .action_destructive("setActiveExample")
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests

//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::schema;


//#endregion 🪢️TaxonomyMounts

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{PlaybookSnapshot, PlaybookSpec, artifact_kind, empty_playbook_snapshot};
use crate::{PLAYBOOK_FLOW_SLOT, PLAYBOOK_GENESIS_FLOW_ID, flow_content_snapshot_from_steps, steps_from_flow_content};
use semio_framework_artifact_playbook_playbook as playbook;
use semio_framework_plugin::{ChildContentView, Fault, FaultCode, FaultOrigin};
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️ComposeOnRead
pub const PLAYBOOK_FLOW_UNAVAILABLE: &str = "playbook.flow.unavailable";
pub const PLAYBOOK_FLOW_DIALECT: &str = "playbook.flow.dialect";
pub const PLAYBOOK_FLOW_CONTENT: &str = "playbook.flow.content";
pub const PLAYBOOK_FLOW_PROJECTION: &str = "playbook.flow.projection";
pub const PLAYBOOK_STEP_MISSING: &str = "playbook.step.missing";
pub const PLAYBOOK_STEP_DUPLICATE: &str = "playbook.step.duplicate";
pub const PLAYBOOK_BLOCK_MISSING: &str = "playbook.block.missing";
pub const PLAYBOOK_BLOCK_DUPLICATE: &str = "playbook.block.duplicate";

pub fn playbook_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

/// 📣️ The localized notice of every refusal this artifact raises (design §20.12), declared by its editor and viewer.
pub fn playbook_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    use semio_framework_ui_locale::LocalizedLabel;
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 8]> = std::sync::LazyLock::new(|| {
        [
            (PLAYBOOK_FLOW_UNAVAILABLE, LocalizedLabel::native("The playbook's steps are not loaded yet.", "Die Schritte des Playbooks sind noch nicht geladen.")),
            (PLAYBOOK_FLOW_DIALECT, LocalizedLabel::native("The playbook's steps are stored in an unsupported format.", "Die Schritte des Playbooks liegen in einem nicht unterstützten Format vor.")),
            (PLAYBOOK_FLOW_CONTENT, LocalizedLabel::native("A step of the playbook cannot be read.", "Ein Schritt des Playbooks kann nicht gelesen werden.")),
            (PLAYBOOK_FLOW_PROJECTION, LocalizedLabel::native("The playbook's steps cannot be restored.", "Die Schritte des Playbooks können nicht wiederhergestellt werden.")),
            (PLAYBOOK_STEP_MISSING, LocalizedLabel::native("The step no longer exists.", "Der Schritt existiert nicht mehr.")),
            (PLAYBOOK_STEP_DUPLICATE, LocalizedLabel::native("A step with this id already exists.", "Ein Schritt mit dieser Id existiert bereits.")),
            (PLAYBOOK_BLOCK_MISSING, LocalizedLabel::native("The block no longer exists.", "Der Baustein existiert nicht mehr.")),
            (PLAYBOOK_BLOCK_DUPLICATE, LocalizedLabel::native("A block with this id already exists in the step.", "Ein Baustein mit dieser Id existiert im Schritt bereits.")),
        ]
    });
    &*NOTICES
}

/// 🪆️ The loaded-parent child projection: the one `flow` child `snapshot` names, so a reload restores exactly that member.
pub fn playbook_child_restore_projection(snapshot: &PlaybookSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| playbook_fault(PLAYBOOK_FLOW_PROJECTION, error.to_string()))
}

/// 🌊️ The `flow` child content `snapshot` names, read through `children` (design §20.15: readers compose on read, never through
/// the handle). A child that is not composed, or composed as anything but `s.stdio.semio@v1/flow`, is a named fault.
pub fn playbook_flow_content<'a>(snapshot: &PlaybookSnapshot, children: &'a ChildContentView) -> Result<store::os_store::snapshot_read::SnapshotReadRef<'a, SemioFlowSnapshot>, Fault> {
    let child_id = snapshot.flow.child_id.as_str();
    match children.dialect(PLAYBOOK_FLOW_SLOT, child_id) {
        Some(dialect) if dialect.artifact_kind == "s.stdio.semio" && dialect.standard == "v1" && dialect.subset == "flow" => children.typed_read::<SemioFlowSnapshot>(PLAYBOOK_FLOW_SLOT, child_id),
        Some(dialect) => Err(playbook_fault(PLAYBOOK_FLOW_DIALECT, format!("the playbook flow child {child_id:?} is {}@{}/{}, not s.stdio.semio@v1/flow", dialect.artifact_kind, dialect.standard, dialect.subset))),
        None => Err(playbook_fault(PLAYBOOK_FLOW_UNAVAILABLE, format!("the playbook flow child {child_id:?} is not composed"))),
    }
}

/// 🧮️ The playbook every reader works on: the parent's own fields composed with the steps of its `flow` child.
pub fn playbook_composed_spec(snapshot: &PlaybookSnapshot, children: &ChildContentView) -> Result<PlaybookSpec, Fault> {
    let content = playbook_flow_content(snapshot, children)?;
    let steps = steps_from_flow_content(&content).map_err(|message| playbook_fault(PLAYBOOK_FLOW_CONTENT, message))?;
    Ok(PlaybookSpec { schema: snapshot.schema.clone(), id: snapshot.id.clone(), version: snapshot.version.clone(), title: snapshot.title.clone(), steps })
}

/// 🌱️ The flow content a document's `flow` child `child_id` is composed from when no archive member carries it — answered by
/// id from this plugin's own catalogue (the empty playbook's flow, every built-in example's), never from the parent.
pub fn playbook_genesis_flow(child_id: &str) -> Option<SemioFlowSnapshot> {
    match child_id {
        PLAYBOOK_GENESIS_FLOW_ID => Some(flow_content_snapshot_from_steps(&playbook::empty_playbook_snapshot().steps)),
        crate::examples::demo::FLOW_ID => crate::examples::demo::flow().ok(),
        _ => None,
    }
}

/// 🧩️ The genesis pack of `snapshot`'s `flow` child ([`playbook_genesis_flow`]) for a whole-document load whose archive carries
/// no such member.
pub fn genesis_playbook_child_pack(snapshot: &PlaybookSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    if slot != PLAYBOOK_FLOW_SLOT || child_id != snapshot.flow.child_id {
        return None;
    }
    playbook_genesis_flow(child_id).map(|content| <SemioFlowSnapshot as ArtifactPack>::encode_pack(&content))
}
//#endregion 🔖️ComposeOnRead
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{PlaybookBlock, PlaybookSnapshot, PlaybookStep};
use crate::{PLAYBOOK_BLOCK_PARAM_PREFIX, PLAYBOOK_FLOW_SLOT, PLAYBOOK_STEP_NODE_KIND, playbook_block_param_key, playbook_chain, playbook_sequence_edge, playbook_step_from_node, playbook_step_node, playbook_step_order};
use crate::{PLAYBOOK_BLOCK_DUPLICATE, PLAYBOOK_BLOCK_MISSING, PLAYBOOK_FLOW_CONTENT, PLAYBOOK_STEP_DUPLICATE, PLAYBOOK_STEP_MISSING, playbook_fault};
use semio_framework_plugin::Fault;
use semio_s_artifact_stdio_semio_flow::{schema::mutations::insert_edge, schema::mutations::insert_node, schema::mutations::remove_edge, schema::mutations::remove_node, schema::mutations::remove_node_param, schema::mutations::set_node_param, schema::mutations::SemioFlowMutation};
use semio_s_artifact_stdio_semio_flow::{schema::snapshot::FlowNode as SemioFlowNode, schema::snapshot::SemioFlowSnapshot};
use std::collections::BTreeSet;

//#region 🔖️ChildLane
/// 🧬️ The `ChildEmit` that publishes `leaves` on `snapshot`'s `flow` child — the ONE lane every content edit takes.
pub fn playbook_flow_emit(snapshot: &PlaybookSnapshot, leaves: Vec<SemioFlowMutation>) -> semio_framework_plugin::app::ChildEmitPreparation {
    semio_framework_plugin::app::ChildEmitPreparation::of::<SemioFlowSnapshot, _>(PLAYBOOK_FLOW_SLOT, snapshot.flow.child_id.as_str(), leaves)
}

/// 🪢️ The leaves that make `content`'s step chain read `order`: every `sequence` edge the new chain drops is removed first, every
/// one it gains inserted after, so a reorder keeps every node and its identity.
pub fn playbook_chain_leaves(content: &SemioFlowSnapshot, order: &[&str]) -> Vec<SemioFlowMutation> {
    let wanted: BTreeSet<(&str, &str)> = order.windows(2).map(|pair| (pair[0], pair[1])).collect();
    let present: BTreeSet<(&str, &str)> = playbook_chain(content).map(|edge| (edge.from.node.as_str(), edge.to.node.as_str())).collect();
    let removed = playbook_chain(content).filter(|edge| !wanted.contains(&(edge.from.node.as_str(), edge.to.node.as_str()))).map(|edge| SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: edge.id.clone() }));
    let inserted = order.windows(2).filter(|pair| !present.contains(&(pair[0], pair[1]))).map(|pair| SemioFlowMutation::InsertEdge(insert_edge::InsertEdge::new(playbook_sequence_edge(pair[0], pair[1]))));
    removed.chain(inserted).collect()
}

pub fn playbook_step_node_of<'a>(content: &'a SemioFlowSnapshot, step_id: &str) -> Result<&'a SemioFlowNode, Fault> {
    content.nodes.iter().find(|node| node.id == step_id && node.kind == PLAYBOOK_STEP_NODE_KIND).ok_or_else(|| playbook_fault(PLAYBOOK_STEP_MISSING, format!("Step \"{step_id}\" does not exist.")))
}

/// 📋️ Step `step_id` of `content`, decoded; a missing step or an undecodable block param is a named fault.
pub fn playbook_step_of(content: &SemioFlowSnapshot, step_id: &str) -> Result<PlaybookStep, Fault> {
    playbook_step_from_node(playbook_step_node_of(content, step_id)?).map_err(|message| playbook_fault(PLAYBOOK_FLOW_CONTENT, message))
}

/// 🗝️ Step node `step_id`'s block params as `(param position, block id)` rows, in block order.
pub fn playbook_block_rows<'a>(node: &'a SemioFlowNode) -> Vec<(usize, &'a str)> {
    node.params.iter().enumerate().filter_map(|(position, param)| param.key.strip_prefix(PLAYBOOK_BLOCK_PARAM_PREFIX).map(|id| (position, id))).collect()
}

/// 🆕️ The concrete leaf that inserts `block` into step `step_id` at block position `index` (appended when `None` or past the
/// end); the param position it lands at is exact, so the inverse removal and every replay agree on the order.
pub fn playbook_insert_block_leaf(rows: &[(usize, &str)], step_id: &str, block: &PlaybookBlock, index: Option<usize>) -> Result<SemioFlowMutation, Fault> {
    if rows.iter().any(|(_, id)| *id == block.id) {
        return Err(playbook_fault(PLAYBOOK_BLOCK_DUPLICATE, format!("Block \"{}\" is already in step \"{step_id}\".", block.id)));
    }
    let at = index.and_then(|index| rows.get(index)).map(|(position, _)| *position);
    Ok(SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: step_id.into(), key: playbook_block_param_key(&block.id), value: semio_framework_pack_json::to_json_string(block), at }))
}

pub fn playbook_remove_block_leaf(step_id: &str, block_id: &str) -> SemioFlowMutation {
    SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id: step_id.into(), key: playbook_block_param_key(block_id) })
}

/// ➕️ Appends `step`: its node, then its chain edge; a step id already present is refused.
pub fn playbook_add_step_leaves(content: &SemioFlowSnapshot, step: &PlaybookStep) -> Result<Vec<SemioFlowMutation>, Fault> {
    if content.nodes.iter().any(|node| node.id == step.id) {
        return Err(playbook_fault(PLAYBOOK_STEP_DUPLICATE, format!("Step \"{}\" already exists.", step.id)));
    }
    let mut order = playbook_step_order(content);
    let node = SemioFlowMutation::InsertNode(insert_node::InsertNode::new(playbook_step_node(step, order.len())));
    order.push(&step.id);
    Ok(std::iter::once(node).chain(playbook_chain_leaves(content, &order)).collect())
}

/// ➖️ Removes step `step_id`: its chain edges and the bridge over it first, then its node.
pub fn playbook_remove_step_leaves(content: &SemioFlowSnapshot, step_id: &str) -> Result<Vec<SemioFlowMutation>, Fault> {
    playbook_step_node_of(content, step_id)?;
    let order: Vec<&str> = playbook_step_order(content).into_iter().filter(|id| *id != step_id).collect();
    Ok(playbook_chain_leaves(content, &order).into_iter().chain(std::iter::once(SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: step_id.into() }))).collect())
}

/// ↔️ Moves step `step_id` to position `index` (clamped) by rewiring the chain only; a step already there yields no leaf.
pub fn playbook_move_step_leaves(content: &SemioFlowSnapshot, step_id: &str, index: usize) -> Result<Vec<SemioFlowMutation>, Fault> {
    playbook_step_node_of(content, step_id)?;
    let mut order: Vec<&str> = playbook_step_order(content).into_iter().filter(|id| *id != step_id).collect();
    order.insert(index.min(order.len()), step_id);
    Ok(playbook_chain_leaves(content, &order))
}

/// 🆕️ Inserts `block` into step `step_id` at `index` (appended when `None` or past the end): ONE `set-node-param` row.
pub fn playbook_add_block_leaves(content: &SemioFlowSnapshot, step_id: &str, block: PlaybookBlock, index: Option<usize>) -> Result<Vec<SemioFlowMutation>, Fault> {
    let node = playbook_step_node_of(content, step_id)?;
    Ok(vec![playbook_insert_block_leaf(&playbook_block_rows(node), step_id, &block, index)?])
}

/// 🚮️ Removes block `block_id` from step `step_id`: ONE `remove-node-param` row; a block the step does not hold is a named fault.
pub fn playbook_remove_block_leaves(content: &SemioFlowSnapshot, step_id: &str, block_id: &str) -> Result<Vec<SemioFlowMutation>, Fault> {
    let node = playbook_step_node_of(content, step_id)?;
    if !playbook_block_rows(node).iter().any(|(_, id)| *id == block_id) {
        return Err(playbook_fault(PLAYBOOK_BLOCK_MISSING, format!("Block \"{block_id}\" is not in step \"{step_id}\".")));
    }
    Ok(vec![playbook_remove_block_leaf(step_id, block_id)])
}

/// 🚚️ Moves block `block_id` from step `from` to block position `index` (clamped) of step `to`: a remove row then an insert row at
/// the exact target position, in the same step or across two; a block already there yields no row.
pub fn playbook_move_block_leaves(content: &SemioFlowSnapshot, block_id: &str, from: &str, to: &str, index: usize) -> Result<Vec<SemioFlowMutation>, Fault> {
    let source = playbook_step_node_of(content, from)?;
    let source_rows = playbook_block_rows(source);
    let position = source_rows.iter().position(|(_, id)| *id == block_id).ok_or_else(|| playbook_fault(PLAYBOOK_BLOCK_MISSING, format!("Block \"{block_id}\" is not in step \"{from}\".")))?;
    let block: PlaybookBlock = semio_framework_pack_json::from_json_str(&source.params[source_rows[position].0].value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| playbook_fault(PLAYBOOK_FLOW_CONTENT, format!("step {from:?}: {error}")))?;
    let target = playbook_step_node_of(content, to)?;
    let remaining: Vec<(usize, &str)> = if from == to {
        let removed = source_rows[position].0;
        source_rows.iter().filter(|(row, _)| *row != removed).map(|(row, id)| (if *row > removed { row - 1 } else { *row }, *id)).collect()
    } else {
        playbook_block_rows(target)
    };
    let index = index.min(remaining.len());
    if from == to && index == position {
        return Ok(Vec::new());
    }
    Ok(vec![playbook_remove_block_leaf(from, block_id), playbook_insert_block_leaf(&remaining, to, &block, Some(index))?])
}
//#endregion 🔖️ChildLane
}
//#endregion 🧩️ArtifactItems4
