//! 🖥️ FEM 3D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `Fem3dPlayApp` now
//! authors the `✏️editor` surface only — the read-only `👁️viewer` surface is a genuinely independent
//! sibling (`crate::viewer::fem3d::Fem3dViewer`), never constructed from this file.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config`, shared compute in the artifact's
//! `⚙️engine`. This file is a routing table: `handle` → `Fem3dCommand::dispatch`, `render` → body-key →
//! window, and a `🔖️Manifest` region that calls one passthrough per node (scalar `.mode(..)`/
//! `.window_kind(..)` calls stay inline — fem3d builds neither a `ModeDefinition` nor a
//! `WindowKindDefinition` object anywhere, see `modes::edit`'s and the window nodes' own doc comments).

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use crate::editor::fem3d::commands::{
    add_area_load, add_bar, add_combination, add_frame, add_load_case, add_material, add_member_udl, add_nodal_load, add_node, add_section, add_solid, add_support, remove_selection, set_active_example, set_analysis_settings, set_camera,
    set_result_display, set_self_weight,
};
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::editor::fem3d::modes::edit;
use crate::editor::fem3d::modes::edit::windows::{model as window_model, results as window_results};
use crate::model::{Dof, ElementResult};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    built_text_node, create_default_layout, ActionArgDef, ActionArgOption, AppDefinition, AppIo, AppOperationContext, AppRenderOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane,
    ArtifactView, ConfigSpec, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoDraft, NoDraftMutation, PluginCloseStep,
};
use dsl::json::Value;
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const FEM3D_APP_ID: &str = "fem3d-play";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem3dPlayApp::Command` — the SOLE dispatch surface for fem3d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — they are genuinely different vocabularies for 3 of these 18
    /// rows: `setActiveExample`/`active-example`, `setCamera`/`camera`, `setResultDisplay`/
    /// `result-display`. **Row order is the binary variant ordinal: appending is safe, reordering is a
    /// wire-format break.** Unlike fem2d, there is NO `setLocale`/`SetLocale` row — fem3d's pre-migration
    /// `Fem3dCommand` enum never had one (a pre-existing, intentional asymmetry between the two apps).
    pub enum Fem3dCommand for Fem3dSnapshot, Fem3dMutation, Fem3dConfig, Fem3dConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addFrame" as "add-frame" => add_frame::AddFrame,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addSolid" as "add-solid" => add_solid::AddSolid,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧾️ Every fem3d tool id, in `Fem3dCommand` declaration order — a bijection with the enum's 18 rows,
/// with `FEM3D_RETAINED_PUBLICATION_CONTRACTS`, and with the `.action_interactive_job(…, Migrated)` set
/// `create_fem3d_app` declares. `AppActionRegistry::tool_job_registration` enforces exactly that set
/// equality at construction time: a row missing here, or an action left `BatchOnlyPendingRewrite`,
/// faults the whole app with `interactive-job.catalog-incomplete` instead of silently going
/// dispatch-dead at the UI gate — which is what the pre-migration two-row list did to the other 16.
const FEM3D_RETAINED_TOOL_IDS: &[&str] = &[
    "addNode",
    "addBar",
    "addFrame",
    "addMaterial",
    "addSection",
    "addSupport",
    "addNodalLoad",
    "addMemberUdl",
    "addAreaLoad",
    "addSolid",
    "addLoadCase",
    "addCombination",
    "setSelfWeight",
    "setAnalysisSettings",
    "removeSelection",
    "setActiveExample",
    "setCamera",
    "setResultDisplay",
];
const FEM3D_RETAINED_PAYLOAD_SCHEMA: &str = "fem.3d.tool-command.v1";
const FEM3D_RETAINED_RAW_BYTES: usize = 65_536;
const FEM3D_RETAINED_DECODED_ITEMS: usize = 4_096;
const FEM3D_RETAINED_OUTPUT_BYTES: usize = 262_144;
const FEM3D_RETAINED_STEP_MICROS: u32 = 7_500;
/// 🎒️ Semantic work ceiling for one retained fem3d command: the document's own id-keyed collections
/// (`removeSelection` walks every one of them), so a model beyond this size falls out of the retained
/// path with `fem3d-command-payload-too-large` rather than blocking the interactive step budget.
const FEM3D_RETAINED_WORK_ITEMS: usize = 4_096;
const FEM3D_CONFIG_VALUE_BYTES: usize = 512;
const FEM3D_CONFIG_BASE_BYTES: usize = 512;
const FEM3D_CONFIG_STEP_BYTES: usize = 4_096;
/// 🎒️ Real bound for one Artifact-lane edit: the largest single `Fem3dMutation` any of the 15 document
/// tools emits is `CreateSolid` (outline + holes polygons) — 64 KiB is a genuine ceiling for that
/// encoded op, not a rubber stamp.
const FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
/// 🧾️ Which store lane each retained tool publishes on — the 15 document tools emit `Fem3dMutation`s
/// only, `setActiveExample` emits a whole-document `Effect::LoadDocument` plus the two config resets,
/// and the two view actions are config-only. `VcsArtifactApp` rejects any lane here that has no
/// one-item preparation factory with `interactive-job.publication-authority-missing`, which is why
/// `build_artifact_store_one_item_preparation_factory` is now implemented alongside the config one.
const FEM3D_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBar", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addNodalLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMemberUdl", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addAreaLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSolid", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSelfWeight", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAnalysisSettings", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setResultDisplay", lanes: &[ArtifactToolPublicationLane::Config] },
];

/// 🧷️ The single source of truth for this app's tool execution contract — `bounded_first_step_tool_proofs!`
/// calls this same fn, so the declared proof row and the registered factory can never disagree (they did
/// before: the proof declared `bounded_first_step` while the factory returned `resumable`, which made
/// `validate_tool_job_rows`' `registration.contract == row.contract` check reject even `setCamera`).
fn fem3d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(FEM3D_RETAINED_RAW_BYTES, FEM3D_RETAINED_DECODED_ITEMS, 1, FEM3D_RETAINED_OUTPUT_BYTES, FEM3D_RETAINED_STEP_MICROS)
}

/// 📏️ The config-lane payload bytes a command carries, `0` for every document-lane command — the two
/// view actions write an opaque host JSON blob straight into `Fem3dConfig`, so their envelope is
/// checked against `FEM3D_CONFIG_VALUE_BYTES` before the job is ever built.
fn fem3d_retained_config_value_bytes(command: &Fem3dCommand) -> usize {
    match command {
        Fem3dCommand::SetCamera(payload) => payload.json.len(),
        Fem3dCommand::SetResultDisplay(payload) => payload.mode.len().saturating_add(payload.source_id.as_ref().map_or(0, String::len)),
        _ => 0,
    }
}

fn fem3d_retained_extent(command: &Fem3dCommand, snapshot: &Fem3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !FEM3D_RETAINED_TOOL_IDS.contains(&command.command_id()) || fem3d_retained_config_value_bytes(command) > FEM3D_CONFIG_VALUE_BYTES {
        return None;
    }
    let collections = [
        snapshot.nodes.len(),
        snapshot.elements.len(),
        snapshot.materials.len(),
        snapshot.sections.len(),
        snapshot.solids.len(),
        snapshot.supports.len(),
        snapshot.load_cases.len(),
        snapshot.combinations.len(),
    ];
    let items = collections.into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= FEM3D_RETAINED_WORK_ITEMS).then_some(1)
}

/// 🎯️ One reducer for all 18 rows: `Fem3dCommand::dispatch` already routes each row to its own
/// `🎮️commands/*` handler, so the retained job reuses the exact same owned reducers the batch path
/// used — no second, drifting copy of any command body.
fn fem3d_retained_reduce(
    command: &Fem3dCommand,
    snapshot: &Fem3dSnapshot,
    config: &Fem3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct Fem3dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Fem3dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FEM3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Fem3dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Fem3dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Fem3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FEM3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        fem3d_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FEM3D_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("FEM3D retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for Fem3dRetainedCommandJobFactory {
    type Owner = EditorApp<Fem3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FEM3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::FEM_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = FEM3D_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️ArtifactStorePreparation
/// 📏️ One document mutation's retained envelope, measured on its own canonical binary op rather than
/// on a cloned snapshot — `OpBinary` is the exact shape the store publishes.
fn fem3d_artifact_mutation_retained_bytes(mutation: &Fem3dMutation) -> Result<usize, String> {
    protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "fem3d-artifact-mutation-encode-failed".to_string())
}

fn admit_fem3d_artifact_mutation(mutation: &Fem3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = fem3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("fem3d-artifact-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

/// 🧬️ Builds the single `protocol::Edit<Fem3dMutation>` the Artifact lane's `advance()` publishes —
/// the config lane's `fem3d_config_edit` twin, differing only in `M` and the edit-id prefix.
fn fem3d_artifact_edit(forward: Fem3dMutation, inverse: Vec<Fem3dMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Fem3dMutation> {
    let id = format!("fem3d-artifact-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
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

/// 📬️ Required by the Artifact publication lane: 15 of the 18 retained tools emit `Fem3dMutation`s, and
/// `VcsArtifactApp` rejects any tool whose declared lane has no one-item preparation factory with
/// `interactive-job.publication-authority-missing`.
struct Fem3dArtifactPreparationFactory;

struct Fem3dArtifactPreparation {
    base: Option<store::SnapshotRead<Fem3dSnapshot>>,
    mutation: Option<Fem3dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Fem3dSnapshot, Fem3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Fem3dSnapshot, Fem3dMutation> for Fem3dArtifactPreparationFactory {
    fn preflight(&self, mutation: &Fem3dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("fem3d-artifact-lane-or-description-envelope".into());
        }
        admit_fem3d_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Fem3dSnapshot, Fem3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Fem3dSnapshot, Fem3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Fem3dSnapshot, Fem3dMutation>> {
        let retained_bytes = fem3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Fem3dArtifactPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes, cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Fem3dSnapshot, Fem3dMutation> for Fem3dArtifactPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "fem3d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "fem3d-artifact-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "fem3d-artifact-authority-missing".to_string())?;
        let edit = fem3d_artifact_edit(mutation, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Fem3dSnapshot, Fem3dMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Fem3dSnapshot, Fem3dMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("fem3d-artifact-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ArtifactStorePreparation

//#region 📬️ConfigStorePreparation
struct Fem3dConfigPreparationFactory;

struct Fem3dConfigPreparation {
    base: Option<store::SnapshotRead<Fem3dConfig>>,
    mutation: Option<Fem3dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(Fem3dConfig, Fem3dConfigMutation, Fem3dConfigMutation)>,
    sealed_candidate: Option<(Fem3dConfig, protocol::Edit<Fem3dConfigMutation>)>,
    serialized_bytes: Option<usize>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Fem3dConfig, Fem3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn fem3d_config_retained_bytes(config: &Fem3dConfig) -> usize {
    config.result_source_id.as_ref().map_or(0, String::len).saturating_add(config.result_mode.len()).saturating_add(config.camera.json.len()).saturating_add(size_of::<u32>())
}

fn fem3d_config_edit(forward: Fem3dConfigMutation, inverse: Fem3dConfigMutation, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Fem3dConfigMutation> {
    let id = format!("fem3d-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse: vec![inverse],
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
}

struct Fem3dConfigByteCounter { bytes: usize }

impl std::io::Write for Fem3dConfigByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.saturating_add(bytes.len()) > FEM3D_CONFIG_STEP_BYTES { return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)); }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn fem3d_config_edit_bytes(edit: &protocol::Edit<Fem3dConfigMutation>) -> Result<usize, String> {
    let mut counter = Fem3dConfigByteCounter { bytes: 0 };
    use std::io::Write as _;
    counter.write_all(dsl::json::to_json_string(edit).as_bytes()).map_err(|_| "FEM3d config edit exceeds its serialized byte envelope".to_string())?;
    Ok(counter.bytes)
}

impl store::ArtifactStoreOneItemPreparationFactory<Fem3dConfig, Fem3dConfigMutation> for Fem3dConfigPreparationFactory {
    fn preflight(&self, mutation: &Fem3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let mutation_bytes = match mutation {
            Fem3dConfigMutation::SetCamera { camera } => camera.json.len(),
            Fem3dConfigMutation::SetResultDisplay { source_id, mode, .. } => source_id.as_ref().map_or(0, String::len).saturating_add(mode.len()),
            Fem3dConfigMutation::Snapshot { .. } => return Err("FEM3d config preparation rejects whole-snapshot publication".into()),
        };
        if lane != store::HistoryLane::Document || mutation_bytes > FEM3D_CONFIG_VALUE_BYTES || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("FEM3d config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 3, retained_bytes: FEM3D_CONFIG_STEP_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<Fem3dConfig, Fem3dConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Fem3dConfig, Fem3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Fem3dConfig, Fem3dConfigMutation>> {
        let mutation_bytes = match &request.mutation {
            Fem3dConfigMutation::SetCamera { camera } => camera.json.len(),
            Fem3dConfigMutation::SetResultDisplay { source_id, mode, .. } => source_id.as_ref().map_or(0, String::len).saturating_add(mode.len()),
            Fem3dConfigMutation::Snapshot { .. } => return Err(request),
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > FEM3D_CONFIG_VALUE_BYTES || request.description.as_ref().is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(Fem3dConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Fem3dConfig, Fem3dConfigMutation> for Fem3dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < FEM3D_CONFIG_STEP_BYTES || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() && self.sealed_candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "FEM3d config preparation lost its exact base root".to_string())?.get();
            if fem3d_config_retained_bytes(base) > FEM3D_CONFIG_BASE_BYTES { return Err("FEM3d config base exceeds retained byte capacity".into()); }
            let mutation = self.mutation.take().ok_or_else(|| "FEM3d config preparation lost its mutation owner".to_string())?;
            let mut post = base.clone();
            let inverse = match &mutation {
                Fem3dConfigMutation::SetCamera { camera } => Fem3dConfigMutation::SetCamera { camera: std::mem::replace(&mut post.camera, camera.clone()) },
                Fem3dConfigMutation::SetResultDisplay { source_id, mode, mode_index } => Fem3dConfigMutation::SetResultDisplay {
                    source_id: std::mem::replace(&mut post.result_source_id, source_id.clone()),
                    mode: std::mem::replace(&mut post.result_mode, mode.clone()),
                    mode_index: std::mem::replace(&mut post.result_mode_index, *mode_index),
                },
                Fem3dConfigMutation::Snapshot { .. } => return Err("FEM3d config preparation received a whole snapshot".into()),
            };
            self.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: fem3d_config_retained_bytes(base) as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if self.sealed_candidate.is_none() {
            let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "FEM3d config preparation lost its candidate".to_string())?;
            let authority = self.authority.as_ref().ok_or_else(|| "FEM3d config preparation lost its Store authority".to_string())?;
            self.sealed_candidate = Some((post, fem3d_config_edit(forward, inverse, self.description.take(), authority)));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.sealed_candidate.as_ref().ok_or_else(|| "FEM3d config preparation lost its semantic edit".to_string())?;
            let bytes = fem3d_config_edit_bytes(edit)?;
            if bytes.saturating_add(fem3d_config_retained_bytes(post)).saturating_add(512) > FEM3D_CONFIG_STEP_BYTES {
                return Err("FEM3d config publication exceeds the 4096-byte complete envelope".into());
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        let (post, edit) = self.sealed_candidate.take().ok_or_else(|| "FEM3d config preparation lost its validated edit".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "FEM3d config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Fem3dConfig, Fem3dConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Fem3dConfig, Fem3dConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if (self.prepared.is_some() || self.sealed_candidate.is_some() || self.candidate.is_some() || self.mutation.is_some() || self.description.is_some()) && grant.maximum_bytes < FEM3D_CONFIG_STEP_BYTES { return Ok(store::SnapshotRetirementStep::Blocked); }
        if self.prepared.take().is_some() || self.sealed_candidate.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: FEM3D_CONFIG_STEP_BYTES }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("FEM3d config preparation could not return its exact base root".into()); }
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
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.sealed_candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigStorePreparation

//#region 🔖️Fem3dResultsJson
/// 🎨️ Manual `crate::model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `crate::model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (the `🫀️core` kernel is
/// a cross-artifact shared crate, out of scope to touch here), so this hand-rolls the same shape
/// `dsl::json::to_json_string` would have produced, using `Dof`'s existing `{:?}` formatting. Single
/// consumer (`export_media`), so this lives here rather than in the artifact's `⚙️engine`.
fn fem3d_dof_json(dof: Dof) -> Value {
    dsl::json!(format!("{dof:?}"))
}

fn fem3d_element_result_json(result: &ElementResult) -> Value {
    match result {
        ElementResult::Bar { n } => dsl::json!({ "kind": "bar", "n": n }),
        ElementResult::Beam { stations } => {
            dsl::json!({ "kind": "beam", "stations": stations.iter().map(|s| dsl::json!({ "x": s.x, "n": s.n, "v": s.v, "m": s.m })).collect::<Vec<_>>() })
        }
        ElementResult::Plane { gauss } => {
            dsl::json!({ "kind": "plane", "gauss": gauss.iter().map(|g| dsl::json!({ "sxx": g.sxx, "syy": g.syy, "sxy": g.sxy, "vonMises": g.von_mises })).collect::<Vec<_>>() })
        }
        ElementResult::Plate { gauss } => {
            dsl::json!({ "kind": "plate", "gauss": gauss.iter().map(|g| dsl::json!({ "mx": g.mx, "my": g.my, "mxy": g.mxy })).collect::<Vec<_>>() })
        }
        ElementResult::Solid { gauss } => dsl::json!({
            "kind": "solid",
            "gauss": gauss.iter().map(|g| dsl::json!({ "sxx": g.sxx, "syy": g.syy, "szz": g.szz, "sxy": g.sxy, "syz": g.syz, "sxz": g.sxz, "vonMises": g.von_mises })).collect::<Vec<_>>(),
        }),
        ElementResult::Shell { gauss } => dsl::json!({
            "kind": "shell",
            "gauss": gauss.iter().map(|g| dsl::json!({ "nxx": g.nxx, "nyy": g.nyy, "nxy": g.nxy, "mxx": g.mxx, "myy": g.myy, "mxy": g.mxy, "vonMisesTop": g.von_mises_top, "vonMisesBottom": g.von_mises_bottom })).collect::<Vec<_>>(),
        }),
    }
}

fn fem3d_static_result_json(result: &crate::model::StaticResult) -> Value {
    dsl::json!({
        "displacements": result.displacements.iter().map(|d| dsl::json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| dsl::json!({ "nodeId": r.node_id, "dof": fem3d_dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| dsl::json!({ "id": id, "result": fem3d_element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

fn fem3d_results_map_json(results: &HashMap<String, crate::model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), fem3d_static_result_json(result))).collect())
}
//#endregion 🔖️Fem3dResultsJson

//#region 🔌️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.3d` × 3D-Any) plus `geometry:in` (importing an externally authored extruded-footprint outline
/// as a new `FemSolid` — see `import_media` above) and `results:out` (every load case/combination's
/// solved `crate::model::StaticResult`, pinned to the `computation.fem3d` artifact kind declared in
/// `crate::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub fn fem3d_io() -> AppIo {
    AppIo {
        document_schema: crate::FEM_3D_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
        ports: vec![fem3d_geometry_in_port(), fem3d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.fem".into(), name: "FEM 3D".into(), dimension: "3d".into(), component_kind: "fem3d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored extruded-footprint outline (polygon-with-holes,
/// base/height/layers), imported as a new `FemSolid`.
pub fn fem3d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `crate::model::StaticResult`, pinned to the
/// `computation.fem3d` artifact kind.
pub fn fem3d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        kind_id: Some("computation.fem3d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
//#endregion 🔌️Io

//#region 🎬️SceneRender
/// 🎬️ App-facing 3D scene-building bridge, moved out of the (now deleted) artifact `⚙️engine`: every fn
/// here references `crate::app_surface` (an app type) and/or returns scene JSON consumed only by the
/// model/results windows (`crate::editor::fem3d::modes::edit::windows::{model, results}`), per the
/// migration recipe's `DocumentHelpers` rule — a helper with 2+ window consumers belongs at the app
/// level, not duplicated per window.
#[cfg(test)]
use crate::fem3d_engine::mesh_preview;

/// 🧭️ Hamilton quaternion product `a * b`, both `[x,y,z,w]` — applying `b`'s rotation first, then `a`'s.
#[cfg(test)]
fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]
}

/// 🧭️ Rotation of `roll` radians about the LOCAL +Z axis — applied before `quat_z_to` reorients +Z to
/// the member direction, so this spins the box prism about its own long axis (matches `Frame3`'s roll).
#[cfg(test)]
fn quat_roll_z(roll: f64) -> [f64; 4] {
    let h = roll / 2.0;
    [0.0, 0.0, h.sin(), h.cos()]
}

/// 🧭️ Shortest-arc rotation taking local `+Z` (the `"box"` mesh's long axis) onto unit direction `dir`
/// — the standard "rotate A onto B" quaternion (`axis = cross(from,to)`, `angle = acos(dot(from,to))`),
/// specialized for `from = (0,0,1)` so `cross` reduces to `(-dir.y, dir.x, 0)`. Handles the antiparallel
/// case (`dir ≈ (0,0,-1)`) with a fixed 180° flip about the X axis, since `cross` degenerates to zero there.
#[cfg(test)]
fn quat_z_to(dir: [f64; 3]) -> [f64; 4] {
    let dot = dir[2].clamp(-1.0, 1.0);
    if dot > 0.999_999 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    if dot < -0.999_999 {
        return [1.0, 0.0, 0.0, 0.0];
    }
    let axis = [-dir[1], dir[0], 0.0];
    let axis_len = (axis[0] * axis[0] + axis[1] * axis[1]).sqrt();
    let axis_n = [axis[0] / axis_len, axis[1] / axis_len, 0.0];
    let half = dot.acos() / 2.0;
    let s = half.sin();
    [axis_n[0] * s, axis_n[1] * s, axis_n[2] * s, half.cos()]
}

/// 🧊️ Node-position resolver shared by every 3D instance/mesh builder: `displacements` (node id -> 6-DOF
/// values), when present, offsets a node's position by its solved displacement scaled by `deform_scale`.
#[cfg(test)]
fn fem3d_deformed_position(pos: [f64; 3], node_id: &str, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> [f64; 3] {
    let mut p = pos;
    if let Some(map) = displacements {
        if let Some(d) = map.get(node_id) {
            p[0] += d[Dof::Tx.index()] * deform_scale;
            p[1] += d[Dof::Ty.index()] * deform_scale;
            p[2] += d[Dof::Tz.index()] * deform_scale;
        }
    }
    p
}

/// 🧊️ Half-extent-ish scale of the small box instance drawn at each node.
#[cfg(test)]
const NODE_SIZE_3D: f64 = 0.05;
/// 🧊️ Cross-section (x/y) thickness of the oriented box prism drawn for each `Bar`/`Frame` member —
/// a fixed visual thickness, not the member's actual section dimensions (see `fem3d_structural_instances`).
#[cfg(test)]
const MEMBER_THICKNESS_3D: f64 = 0.05;

#[cfg(test)]
fn find_node_3d<'a>(nodes: &'a [crate::FemNode], id: &str) -> Option<&'a crate::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

#[cfg(test)]
fn fem3d_element_endpoints(element: &crate::FemElement) -> (&str, &str) {
    match element {
        crate::FemElement::Bar { start, end, .. } | crate::FemElement::Frame { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🧊️ One small box instance per node, plus one ORIENTED box prism per `Bar`/`Frame` member — position
/// at the (possibly deformed) midpoint, `scale=[t,t,length]` so the mesh's own long (local Z) axis
/// stretches along the member, `rotation` a quaternion aligning that axis to the member's direction
/// (composed with a `Frame`'s own `roll` about its own axis; `Bar`s have no roll).
#[cfg(test)]
fn fem3d_structural_instances(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let node_pos = |node: &crate::FemNode| fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);

    let mut instances: Vec<Value> = Vec::new();
    for node in &doc.nodes {
        let p = node_pos(node);
        instances.push(dsl::json!({
            "id": format!("node-{}", node.id),
            "meshId": "box",
            "position": p,
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [NODE_SIZE_3D, NODE_SIZE_3D, NODE_SIZE_3D],
            "label": node.id,
        }));
    }
    for element in &doc.elements {
        let (start, end) = fem3d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_3d(&doc.nodes, start), find_node_3d(&doc.nodes, end)) else { continue };
        let p1 = node_pos(n1);
        let p2 = node_pos(n2);
        let d = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-9);
        let dir = [d[0] / length, d[1] / length, d[2] / length];
        let roll = match element {
            crate::FemElement::Frame { roll, .. } => *roll,
            crate::FemElement::Bar { .. } => 0.0,
        };
        let rotation = quat_mul(quat_z_to(dir), quat_roll_z(roll));
        let mid = [(p1[0] + p2[0]) / 2.0, (p1[1] + p2[1]) / 2.0, (p1[2] + p2[2]) / 2.0];
        let id = crate::element_id(element);
        instances.push(dsl::json!({
            "id": format!("el-{id}"),
            "meshId": "box",
            "position": mid,
            "rotation": rotation,
            "scale": [MEMBER_THICKNESS_3D, MEMBER_THICKNESS_3D, length],
            "label": id,
        }));
    }
    instances
}

/// 🧱️ Every `FemSolid`'s boundary surface as a custom `meshes_json` entry (flat per-face normals, one
/// duplicated vertex triple per triangle) plus its one identity-transform instance — `nodal_stress`,
/// when present, colors each vertex by `crate::app_surface::von_mises_color` (min/max taken across ALL
/// solids' averaged values), driving the react renderer's vertex-color contour (see
/// `PaintTexturedMesh`). `displacements` deforms vertex positions the same way
/// `fem3d_structural_instances` deforms node/member instances.
#[cfg(test)]
fn fem3d_solid_mesh_entries(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (Vec<Value>, Vec<Value>) {
    use crate::app_surface::{hex_to_rgb01, von_mises_color};

    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    let Ok(solid_meshes) = mesh_preview::fem3d_mesh_preview(doc) else { return (meshes, instances) };
    let (min, max) = match nodal_stress {
        Some(map) if !map.is_empty() => (map.values().cloned().fold(f64::INFINITY, f64::min), map.values().cloned().fold(f64::NEG_INFINITY, f64::max)),
        _ => (0.0, 1.0),
    };

    for solid in &solid_meshes {
        let mut positions: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut normals: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut colors: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut indices: Vec<u32> = Vec::with_capacity(solid.boundary_tris.len() * 3);

        let vertex_pos = |idx: u32| -> [f64; 3] { fem3d_deformed_position(solid.points[idx as usize], &solid.node_ids[idx as usize], displacements, deform_scale) };
        let vertex_color = |idx: u32| -> (f64, f64, f64) {
            let Some(stress_map) = nodal_stress else { return (0.78, 0.78, 0.8) };
            let value = stress_map.get(&solid.node_ids[idx as usize]).copied().unwrap_or(min);
            hex_to_rgb01(von_mises_color(value, min, max))
        };

        for &[a, b, c] in &solid.boundary_tris {
            let (pa, pb, pc) = (vertex_pos(a), vertex_pos(b), vertex_pos(c));
            let e0 = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
            let e1 = [pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]];
            let raw = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            let raw_len = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt().max(1e-12);
            let n = [raw[0] / raw_len, raw[1] / raw_len, raw[2] / raw_len];
            let base = (positions.len() / 3) as u32;
            for (idx, p) in [(a, pa), (b, pb), (c, pc)] {
                positions.extend_from_slice(&p);
                normals.extend_from_slice(&n);
                let (r, g, bl) = vertex_color(idx);
                colors.extend_from_slice(&[r, g, bl]);
            }
            indices.extend_from_slice(&[base, base + 1, base + 2]);
        }

        let mesh_id = format!("solid-{}", solid.solid_id);
        meshes.push(dsl::json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }));
        instances.push(dsl::json!({
            "id": format!("solid-inst-{}", solid.solid_id),
            "meshId": mesh_id,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": solid.solid_id,
        }));
    }
    (meshes, instances)
}

/// 🧊️ Builds the FULL `(meshes_json, instances_json)` pair for a 3D scene: the `"box"` primitive mesh
/// plus every `FemSolid`'s custom surface mesh, and every node/member/solid instance — shared by the
/// model window and every results view (static/modal/buckling).
#[cfg(test)]
pub fn fem3d_scene_parts(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (String, String) {
    let mut meshes = dsl::json::parse(&semio_framework_plugin::world3d_meshes_json_from_kinds(&["box".to_string()]))
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default();
    let mut instances = fem3d_structural_instances(doc, displacements, deform_scale);
    let (solid_meshes, solid_instances) = fem3d_solid_mesh_entries(doc, displacements, deform_scale, nodal_stress);
    meshes.extend(solid_meshes);
    instances.extend(solid_instances);
    (dsl::json::to_string(&Value::Array(meshes)), dsl::json::to_string(&Value::Array(instances)))
}

/// 🎥️ Resolves a `FemCamera` to its JSON string, falling back to the framework's default 3D camera when
/// the document/config still carries the sentinel empty-object placeholder.
pub fn fem3d_camera_json(camera: &crate::FemCamera) -> String {
    if camera.json == "{}" {
        semio_framework_plugin::world3d_default_camera()
    } else {
        camera.json.clone()
    }
}
//#endregion 🎬️SceneRender

//#region 🔖️Fem3dPlayApp
/// 🧮️ v0 design: results are recomputed fresh inside `render()`, no cache, no `RunAnalysis` operation.
/// Unit struct — every former `RefCell` field lives in `Fem3dConfig`, written through
/// `Fem3dConfigMutation`s.
#[derive(Default)]
pub struct Fem3dPlayApp;

impl ArtifactEditor for Fem3dPlayApp {
    type Snapshot = Fem3dSnapshot;
    type Mutation = Fem3dMutation;
    type Config = Fem3dConfig;
    type ConfigMutation = Fem3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::fem3d::presence::Fem3dPresence;
    type PresenceMutation = crate::editor::fem3d::presence::Fem3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Fem3dCommand;

    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::fem3d::config::schema::app_schema_descriptor())
    }

    /// 🪪️ W2 packet P7: the canonical `ArtifactEditor::DIALECT`, derived from the artifact-level
    /// `FEM3D_DIALECT` constant (`🗿️artifacts/🧊️3d/🦀️.rs`) so the sibling `👁️viewer` surface
    /// can read the very same value without ever importing through this `editor` module.
    const DIALECT: Dialect = crate::FEM3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::FEM_3D_SCHEMA;

    /// 📬️ Required by the Artifact publication lane — see `Fem3dArtifactPreparationFactory`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Fem3dArtifactPreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Fem3dConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Fem3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs",
        controller: "s.fem.fem3d@1/*#editor",
        document_schema: "fem.3d",
        factory: "Fem3dRetainedCommandJobFactory",
        factory_type: Fem3dRetainedCommandJobFactory,
        contract: fem3d_retained_contract(),
        tools: [
            "addNode",
            "addBar",
            "addFrame",
            "addMaterial",
            "addSection",
            "addSupport",
            "addNodalLoad",
            "addMemberUdl",
            "addAreaLoad",
            "addSolid",
            "addLoadCase",
            "addCombination",
            "setSelfWeight",
            "setAnalysisSettings",
            "removeSelection",
            "setActiveExample",
            "setCamera",
            "setResultDisplay"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Fem3dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !FEM3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("fem3d-command-tool-mismatch"));
        }
        if fem3d_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("fem3d-command-payload-too-large"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, fem3d_retained_reduce, fem3d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: None, operation: operation_context, completion: request.completion },
            Fem3dCommand::command_id,
            FEM3D_RETAINED_RAW_BYTES,
            FEM3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🚀️ Boots on the bundled `default` example instead of the empty document — the Model window is a
    /// `World3d` surface fed by `live_visual`, and an empty boot document meshes to nothing, so the very
    /// first paint was a blank scene until a client dispatched `setActiveExample`. Mirrors the sibling
    /// `Fem3dViewer::initial_snapshot` (and block3d's `block3d_boot_snapshot`) so editor and viewer boot
    /// the same geometry. See `crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot`.
    fn initial_snapshot() -> Fem3dSnapshot {
        let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot();
        eprintln!(
            "[DEBUG] fem3d editor boot snapshot: nodes={} elements={} solids={} materials={} loadCases={}",
            snapshot.nodes.len(),
            snapshot.elements.len(),
            snapshot.solids.len(),
            snapshot.materials.len(),
            snapshot.load_cases.len()
        );
        snapshot
    }

    fn io() -> Option<AppIo> {
        Some(fem3d_io())
    }

    fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::live_visual::maintenance_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_job_close_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::live_visual::close_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        crate::live_visual::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        crate::live_visual::prepare_snapshot_read(operation, snapshot)
    }

    /// 🎞️ `"document:out"` reproduces the trait's default whole-document pack (overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one). `"results:out"` runs every load case/combination's analysis fresh and returns them as plain
    /// JSON text in a `Structured` payload. A document with no load cases, or a solve failure, is
    /// reported as `MediaError::Payload` rather than an empty/panicking export.
    fn export_media(port: &str, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let media_type = fem3d_io().document_media_type;
                let bytes = <Fem3dSnapshot as store::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = crate::fem3d_engine::fem3d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = fem3d_results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem3d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ No `whole_document_operation` override on this impl — per `📓️taxonomy.md`, whole-document
    /// replace (`SetSnapshot`) is banned outright with NO replacement mutation, so this falls back to
    /// the trait's own default (`None`).
    ///
    /// 🎞️ `"document:in"` swaps the whole live document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, the sanctioned non-history whole-doc-replace path — see
    /// `reset_document_effect`'s own doc comment) instead of routing through `whole_document_operation`.
    /// `"geometry:in"` decodes a minimal, app-owned `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...],
    /// "baseZ"?: f64, "height"?: f64, "layers"?: usize}` extruded-footprint contract into a new
    /// `FemSolid`, defaulted to the document's first existing material if any, else an `"unassigned"`
    /// placeholder id — the solid simply won't solve until a real material is assigned.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "geometry:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured JSON payload".into()));
                };
                let value = dsl::json::parse(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let outline: Vec<[f64; 2]> = dsl::FromValue::from_value(dsl::json::to_dsl_value(&value.get("outline").cloned().unwrap_or(Value::Null))).map_err(|error| MediaError::Payload(port.to_string(), format!("outline: {error}")))?;
                let holes: Vec<Vec<[f64; 2]>> = match value.get("holes").cloned() {
                    Some(holes_value) => dsl::FromValue::from_value(dsl::json::to_dsl_value(&holes_value)).map_err(|error| MediaError::Payload(port.to_string(), format!("holes: {error}")))?,
                    None => Vec::new(),
                };
                let base_z = value.get("baseZ").and_then(Value::as_f64).unwrap_or(0.0);
                let height = value.get("height").and_then(Value::as_f64).unwrap_or(1.0);
                let layers = value.get("layers").and_then(Value::as_u64).map_or(1, |v| v as usize);
                let material_id = doc.snapshot.materials.first().map_or_else(|| "unassigned".into(), |material| material.id.clone());
                let id = crate::app_surface::next_id(doc.snapshot.solids.iter().map(|s| s.id.clone()), "sol");
                let solid = crate::FemSolid { id, name: "Imported Geometry".into(), outline, holes, base_z, height, layers, mesh_size: 0.5, material_id };
                Ok(Emit::mutations(vec![Fem3dMutation::CreateSolid(crate::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid { solid })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧮️ No sticky `ActionArgDef` defaults are mirrored here (all of `addSolid`'s
    /// `baseZ`/`layers`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings).
    fn config_spec() -> ConfigSpec {
        ConfigSpec::default()
    }

    fn command_id(command: &Fem3dCommand) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &Fem3dCommand,
        doc: &ArtifactView<'_, Fem3dSnapshot>,
        cfg: &ConfigView<'_, Fem3dConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn pending_effects(doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Vec<semio_framework::kernel::Effect> {
        crate::live_visual::reconcile(doc)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, Fem3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let camera = &cfg.snapshot.camera;
        match body_key {
            window_model::FEM3D_BODY_MODEL => crate::live_visual::with_live_visual(doc.render_operation(), |visual| window_model::render_with_progress(camera, visual)),
            window_results::FEM3D_BODY_RESULTS => crate::live_visual::with_live_visual(doc.render_operation(), |visual| window_results::render_with_progress(camera, visual)),
            _ => built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fem3d unknown-body label admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️Fem3dPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example). Per `📓️taxonomy.md`, `SetSnapshot` is banned outright with NO replacement
/// mutation: whole-document replace is not expressible as an in-history `Mutation` at all. Every
/// former "replace the whole document" gesture in this package (`import_media`'s `"document:in"`,
/// `commands::set_active_example`) builds this effect instead of an `Emit::mutations([...])`.
/// The spr is a fresh, edit-free op-log for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &Fem3dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Fem3dSnapshot as store::ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<Fem3dSnapshot, Fem3dMutation>(crate::FEM_3D_SCHEMA, "fem3d", scene.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("fem3d document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node. fem3d's mode/windows are all scalar
/// (`.mode(..)`/`.window_kind(..)`) declarations — no `_def` passthrough exists for them since no
/// `ModeDefinition`/`WindowKindDefinition` object is built anywhere (see `modes::edit`'s doc comment).
///
/// 🚧️ SDK GAP (contract §2.4, `App { definition, examples }` split): `EditorBuilder` has no
/// `.example(...)`/`.workflow(...)` methods — the pre-migration chain's trailing
/// `.example(examples::demo::ID, LocalizedLabel::native("Family House", "Einfamilienhaus"),
/// crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT, "file")` and `.workflow("fem3d", "FEM 3D",
/// "structure")` calls are dropped here, not ported. `setActiveExample`'s handler loads the same
/// `FEM3D_EXAMPLE_TEXT` fixture directly.
pub fn create_fem3d_app() -> AppDefinition {
    Editor::builder(crate::FEM3D_DIALECT)
            .document(["semio", "fem", "fem3d"])
            .artifact_kind(crate::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(window_model::FEM3D_WINDOW_MODEL, LocalizedLabel::native("Model", "Modell"), window_model::FEM3D_BODY_MODEL, semio_framework_ui_contract::SurfaceKind::World3d, "fem-model")
            .window_kind(window_results::FEM3D_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), window_results::FEM3D_BODY_RESULTS, semio_framework_ui_contract::SurfaceKind::World3d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[window_model::FEM3D_WINDOW_MODEL.into(), window_results::FEM3D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("z", LocalizedLabel::data("Z")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .mutation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("solidId", LocalizedLabel::native("Solid", "Volumenkörper")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addSolid", LocalizedLabel::native("Add Solid", "Volumenkörper hinzufügen"))
            .action_args("addSolid", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("depth", LocalizedLabel::native("Depth", "Tiefe")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::data("Material")).required(),
                ActionArgDef::number("baseZ", LocalizedLabel::native("Base Z", "Basis Z")).default_value(&0.0),
                ActionArgDef::number("layers", LocalizedLabel::native("Layers", "Schichten")).default_value(&1),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(&0.5),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(&false),
            ])
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::text("terms", LocalizedLabel::native("Terms", "Terme")).required(),
            ])
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Moden")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Beulmoden")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, LocalizedLabel::native("Default", "Standard"))]).default_value(&crate::examples::demo::ID),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", crate::app_surface::result_display_action_args())
            // 🧵️ All 18 rows are owned by `Fem3dRetainedCommandJobFactory` — this set must stay exactly
            // equal to `FEM3D_RETAINED_TOOL_IDS`, or `AppActionRegistry::tool_job_registration` faults the
            // whole app with `interactive-job.catalog-incomplete`.
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBar", InteractiveJobClassification::Migrated)
            .action_interactive_job("addFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNodalLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMemberUdl", InteractiveJobClassification::Migrated)
            .action_interactive_job("addAreaLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSolid", InteractiveJobClassification::Migrated)
            .action_interactive_job("addLoadCase", InteractiveJobClassification::Migrated)
            .action_interactive_job("addCombination", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelfWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAnalysisSettings", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setResultDisplay", InteractiveJobClassification::Migrated)
            // 🎯️ Typed channel surface — `config_spec()`/`fem3d_io()` are this same information's single
            // source of truth, reused here rather than duplicated.
            .config(Fem3dPlayApp::config_spec())
            .io(fem3d_io())
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
