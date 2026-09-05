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

use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
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
const FEM3D_RETAINED_TOOL_IDS: &[&str] = &["setCamera", "setResultDisplay"];
const FEM3D_RETAINED_PAYLOAD_SCHEMA: &str = "fem.3d.tool-command.v1";
const FEM3D_RETAINED_RAW_BYTES: usize = 8_192;
const FEM3D_RETAINED_WORK_ITEMS: usize = 1;
const FEM3D_CONFIG_VALUE_BYTES: usize = 512;
const FEM3D_CONFIG_BASE_BYTES: usize = 512;
const FEM3D_CONFIG_STEP_BYTES: usize = 4_096;
const FEM3D_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setResultDisplay", lanes: &[ArtifactToolPublicationLane::Config] },
];

fn fem3d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(FEM3D_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500, 1, 1)
}

fn fem3d_retained_extent(command: &Fem3dCommand, _snapshot: &Fem3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        Fem3dCommand::SetCamera(payload) if payload.json.len() <= FEM3D_CONFIG_VALUE_BYTES => Some(1),
        Fem3dCommand::SetResultDisplay(payload)
            if payload.mode.len().saturating_add(payload.source_id.as_ref().map_or(0, String::len)) <= FEM3D_CONFIG_VALUE_BYTES =>
        {
            Some(1)
        }
        _ => None,
    }
}

fn fem3d_retained_reduce(
    command: &Fem3dCommand,
    snapshot: &Fem3dSnapshot,
    config: &Fem3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, NoDraftMutation>, Fault> {
    let document = ArtifactView::with_operation(snapshot, history, operation.clone());
    let config = ConfigView { snapshot: config };
    match command {
        Fem3dCommand::SetCamera(payload) if payload.json.len() <= FEM3D_CONFIG_VALUE_BYTES => set_camera::handle(payload, &document, &config),
        Fem3dCommand::SetResultDisplay(payload)
            if payload.mode.len().saturating_add(payload.source_id.as_ref().map_or(0, String::len)) <= FEM3D_CONFIG_VALUE_BYTES =>
        {
            set_result_display::handle(payload, &document, &config)
        }
        _ => Err(Fault::from("fem3d-retained-route-mismatch")),
    }
}

struct Fem3dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Fem3dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FEM3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Fem3dRetainedCommandJobFactory {
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

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Fem3dRetainedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<Fem3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FEM3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem3d::FEM_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = FEM3D_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

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
    config.result_source_id.as_ref().map_or(0, String::len).saturating_add(config.result_mode.len()).saturating_add(config.camera.json.len()).saturating_add(std::mem::size_of::<u32>())
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
/// `crate::artifacts::fem3d::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub fn fem3d_io() -> AppIo {
    AppIo {
        document_schema: crate::artifacts::fem3d::FEM_3D_SCHEMA.into(),
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
const NODE_SIZE_3D: f64 = 0.05;
/// 🧊️ Cross-section (x/y) thickness of the oriented box prism drawn for each `Bar`/`Frame` member —
/// a fixed visual thickness, not the member's actual section dimensions (see `fem3d_structural_instances`).
const MEMBER_THICKNESS_3D: f64 = 0.05;

#[cfg(test)]
fn find_node_3d<'a>(nodes: &'a [crate::artifacts::fem3d::FemNode], id: &str) -> Option<&'a crate::artifacts::fem3d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

#[cfg(test)]
fn fem3d_element_endpoints(element: &crate::artifacts::fem3d::FemElement) -> (&str, &str) {
    match element {
        crate::artifacts::fem3d::FemElement::Bar { start, end, .. } | crate::artifacts::fem3d::FemElement::Frame { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🧊️ One small box instance per node, plus one ORIENTED box prism per `Bar`/`Frame` member — position
/// at the (possibly deformed) midpoint, `scale=[t,t,length]` so the mesh's own long (local Z) axis
/// stretches along the member, `rotation` a quaternion aligning that axis to the member's direction
/// (composed with a `Frame`'s own `roll` about its own axis; `Bar`s have no roll).
#[cfg(test)]
fn fem3d_structural_instances(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let node_pos = |node: &crate::artifacts::fem3d::FemNode| fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);

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
            crate::artifacts::fem3d::FemElement::Frame { roll, .. } => *roll,
            crate::artifacts::fem3d::FemElement::Bar { .. } => 0.0,
        };
        let rotation = quat_mul(quat_z_to(dir), quat_roll_z(roll));
        let mid = [(p1[0] + p2[0]) / 2.0, (p1[1] + p2[1]) / 2.0, (p1[2] + p2[2]) / 2.0];
        let id = crate::artifacts::fem3d::element_id(element);
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
    let mut meshes = dsl::json::parse(&semio_framework_plugin::resolve_ready(semio_framework_plugin::world3d_meshes_json_from_kinds(&["box".to_string()])))
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
pub fn fem3d_camera_json(camera: &crate::artifacts::fem3d::FemCamera) -> String {
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

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::fem3d::config::schema::app_schema_descriptor())
    }

    /// 🪪️ W2 packet P7: the canonical `ArtifactEditor::DIALECT`, derived from the artifact-level
    /// `FEM3D_DIALECT` constant (`🗿️artifacts/🧊️3d/🦀️.rs`) so the sibling `👁️viewer` surface
    /// can read the very same value without ever importing through this `editor` module.
    const DIALECT: Dialect = crate::artifacts::fem3d::FEM3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem3d::FEM_3D_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Fem3dConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Fem3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs",
        controller: "s.fem.fem3d@1/*#editor",
        document_schema: "fem.3d",
        factory: "Fem3dRetainedCommandJobFactory",
        factory_type: Fem3dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 65_536, 7_500),
        tools: ["setCamera", "setResultDisplay"]
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
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            operation_context,
            request.completion,
            Fem3dCommand::command_id,
            FEM3D_RETAINED_RAW_BYTES,
            FEM3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> Fem3dSnapshot {
        crate::artifacts::fem3d::schema::empty_fem3d_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(fem3d_io())
    }

    fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::artifacts::fem3d::live_visual::maintenance_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_job_close_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::artifacts::fem3d::live_visual::close_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        crate::artifacts::fem3d::live_visual::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        crate::artifacts::fem3d::live_visual::prepare_snapshot_read(operation, snapshot)
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
                let layers = value.get("layers").and_then(Value::as_u64).map(|v| v as usize).unwrap_or(1);
                let material_id = doc.snapshot.materials.first().map(|material| material.id.clone()).unwrap_or_else(|| "unassigned".into());
                let id = crate::app_surface::next_id(doc.snapshot.solids.iter().map(|s| s.id.clone()), "sol");
                let solid = crate::artifacts::fem3d::FemSolid { id, name: "Imported Geometry".into(), outline, holes, base_z, height, layers, mesh_size: 0.5, material_id };
                Ok(Emit::mutations(vec![Fem3dMutation::CreateSolid(crate::artifacts::fem3d::mutations::create_solid::mutation::CreateSolid { solid })]))
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
        crate::artifacts::fem3d::live_visual::reconcile(doc)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, Fem3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let camera = &cfg.snapshot.camera;
        match body_key {
            window_model::FEM3D_BODY_MODEL => crate::artifacts::fem3d::live_visual::with_live_visual(doc.render_operation(), |visual| window_model::render_with_progress(camera, visual)),
            window_results::FEM3D_BODY_RESULTS => crate::artifacts::fem3d::live_visual::with_live_visual(doc.render_operation(), |visual| window_results::render_with_progress(camera, visual)),
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
    let envelope = store::create_document_envelope::<Fem3dSnapshot, Fem3dMutation>(crate::artifacts::fem3d::FEM_3D_SCHEMA, "fem3d", scene.clone(), None);
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
/// `.example("default", LocalizedLabel::native("Family House", "Einfamilienhaus"),
/// crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT, "file")` and `.workflow("fem3d", "FEM 3D",
/// "structure")` calls are dropped here, not ported. `setActiveExample`'s handler loads the same
/// `FEM3D_EXAMPLE_TEXT` fixture directly.
pub fn create_fem3d_app() -> AppDefinition {
    Editor::builder(crate::artifacts::fem3d::FEM3D_DIALECT)
            .document(["semio", "fem", "fem3d"])
            .artifact_kind(crate::artifacts::fem3d::computation_artifact_kind())
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
                ActionArgDef::number("baseZ", LocalizedLabel::native("Base Z", "Basis Z")).default_value(0.0),
                ActionArgDef::number("layers", LocalizedLabel::native("Layers", "Schichten")).default_value(1),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(0.5),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(false),
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
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard"))]).default_value("default"),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", crate::app_surface::result_display_action_args())
            .action_interactive_job("addNode", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addBar", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addFrame", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addMaterial", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addSection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addSupport", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addNodalLoad", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addMemberUdl", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addAreaLoad", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addSolid", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addLoadCase", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addCombination", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSelfWeight", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setAnalysisSettings", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("removeSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::BatchOnlyPendingRewrite)
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
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{ArtifactApp, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Fem3dApp = VcsArtifactApp<EditorApp<Fem3dPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    /// `EditorApp<Fem3dPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
    /// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<Fem3dPlayApp>` builds it.
    pub fn fem3d_app() -> Fem3dApp {
        semio_framework_plugin::resolve_ready(new_app::<EditorApp<Fem3dPlayApp>>())
    }

    /// 🚧️ SDK GAP: `new_app_with_registry` still expects `fn() -> App` (contract §2.4's
    /// `App { definition, examples }` split was not threaded through this testkit fn) — wrap the now
    /// `AppDefinition`-returning `create_fem3d_app` the same way the cad pilot's own
    /// `cad_app_manifest_for_testkit` does.
    fn fem3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_fem3d_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn fem3d_app_with_registry() -> Fem3dApp {
        semio_framework_plugin::resolve_ready(new_app_with_registry::<EditorApp<Fem3dPlayApp>>(fem3d_app_manifest_for_testkit))
    }

    pub async fn dispatch(app: &mut Fem3dApp, command: Fem3dCommand) -> InvocationResult {
        let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        for effect in &result.requested_effects {
            if let semio_framework_plugin::Effect::LoadDocument { pack, spr } = effect {
                let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                app.load_document_pack(&files).await.expect("test host applies load-document effect");
            }
        }
        result
    }

    pub fn render(app: &mut Fem3dApp, body_key: &str) -> String {
        dsl::json::to_json_string(&semio_framework_plugin::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render"))
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧪️RetainedCommandEnvelope
    #[test]
    fn retained_command_fixture_matches_exact_routes_and_value_codec_boundaries() {
        use store::ArtifactStoreOneItemPreparationFactory as _;
        let fixture: dsl::DslValue = dsl::json::from_json_str(include_str!("🧪️fixtures/🧫️retained-command-limits/🔣️.json")).expect("language-neutral retained fixture");
        let migrated: Vec<&str> = fixture["routes"].as_array().expect("routes").iter().filter(|row| row["disposition"] == "Migrated").map(|row| row["id"].as_str().expect("route id")).collect();
        assert_eq!(migrated, FEM3D_RETAINED_TOOL_IDS);
        assert_eq!(FEM3D_RETAINED_PUBLICATION_CONTRACTS.len(), migrated.len());
        assert_eq!(fixture["limits"]["configValueBytes"].as_u64(), Some(FEM3D_CONFIG_VALUE_BYTES as u64));
        assert_eq!(fixture["limits"]["storeStepBytes"].as_u64(), Some(FEM3D_CONFIG_STEP_BYTES as u64));
        let factory = Fem3dConfigPreparationFactory;
        for case in fixture["boundaryCases"].as_array().expect("boundary cases") {
            let value = "x".repeat(case["bytes"].as_u64().expect("byte count") as usize);
            let mutation = Fem3dConfigMutation::SetCamera { camera: crate::artifacts::fem3d::FemCamera { json: value } };
            let encoded = dsl::json::to_json_string(&mutation);
            let decoded: Fem3dConfigMutation = dsl::json::from_json_str(&encoded).expect("first-party JSON decode");
            assert_eq!(decoded, mutation);
            assert_eq!(factory.preflight(&decoded, None, store::HistoryLane::Document).is_ok(), case["accepted"].as_bool().expect("admission oracle"));
        }
    }

    #[test]
    fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
        use std::io::Write as _;
        use store::ArtifactStoreOneItemPreparation as _;
        let value = "x".repeat(FEM3D_CONFIG_VALUE_BYTES);
        let mut preparation = Fem3dConfigPreparation {
            base: None, mutation: Some(Fem3dConfigMutation::SetCamera { camera: crate::artifacts::fem3d::FemCamera { json: value } }), description: None, authority: None, candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        };
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
        preparation.cancel();
        assert!(matches!(preparation.advance(grant).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
        preparation.begin_close();
        assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).expect("undersized close"), store::SnapshotRetirementStep::Blocked));
        assert!(matches!(preparation.close_step(grant).expect("bounded close"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 4_096 }));
        assert!(matches!(preparation.close_step(grant).expect("terminal close"), store::SnapshotRetirementStep::Complete));
        assert!(preparation.terminal_is_empty());
        let mut counter = Fem3dConfigByteCounter { bytes: 0 };
        assert_eq!(counter.write(&[0; 4_096]).expect("maximum serialized envelope"), 4_096);
        assert!(counter.write(&[0]).is_err());
    }
    //#endregion 🧪️RetainedCommandEnvelope

    use crate::editor::fem3d::testkit::{dispatch, fem3d_app, Fem3dApp};
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

    //#region 🔖️CommandSurface
    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the exact
    /// fixture values the pre-migration `fem3d_protocol` crate's own `Fem3dCommand` test used.
    fn every_command() -> Vec<Fem3dCommand> {
        vec![
            Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }),
            Fem3dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
            Fem3dCommand::AddFrame(add_frame::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }),
            Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.077e10 }),
            Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 }),
            Fem3dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: crate::artifacts::fem3d::FemDof::ALL.to_vec() }),
            Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: Some("live".into()) }),
            Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -500.0, case_id: None }),
            Fem3dCommand::AddAreaLoad(add_area_load::AddAreaLoad { solid_id: "sol1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
            Fem3dCommand::AddSolid(add_solid::AddSolid { x: 0.0, y: 0.0, width: 4.0, depth: 2.0, height: 0.5, material_id: "concrete".into(), base_z: Some(0.0), layers: Some(2), mesh_size: None }),
            Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
            Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "[[\"dead\",1.35],[\"live\",1.5]]".into() }),
            Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
            Fem3dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }),
            Fem3dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }),
            Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
            Fem3dCommand::SetCamera(set_camera::SetCamera { json: "{\"x\":1}".into() }),
            Fem3dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }),
        ]
    }

    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 18, "every Fem3dCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 📌️ LAW: the pre-migration command wire format, row for row — the hex list is positionally aligned
    /// to `every_command()`, which carries exactly the values the old `📡️protocol` crate's baseline dump
    /// used (ticket `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-before-3d.txt`). Row order is the binary variant ordinal, so a reordering — which
    /// no round-trip law can catch — shows up here as a leading-byte mismatch. `addNodalLoad`'s `None`
    /// case is pinned separately below because `every_command()` only carries its `Some` shape.
    #[semio_framework_async_macros::async_test]
    async fn every_command_keeps_its_pre_migration_bytes() {
        use protocol::OpBinary;
        let expected = [
            "010000030005000000000000f03f0105000000000000004002050000000000000840",
            "010104026e31026e3203726f6405737465656c04000600010601020603030602",
            "01020406686561323030026e31026e3205737465656c050006010106020206030306000405000000000000e03f",
            "01030105537465656c030006000105000000da7c72484202050000806444ce3242",
            "0104010648454132303005000600010545f5d6c05609763f020554fc8458a258033f0305210ec81462e4eb3e040576830df4f521a43e",
            "010501026e310200060001160600020406080a",
            "010602046c697665026e3104000601010a020205000000000088b3c0030600",
            "01070102653104000600010500000000000000000205000000000000000003050000000000407fc0",
            "010802046465616404736f6c31030006010105000000000088b340020600",
            "01090108636f6e637265746508000500000000000000000105000000000000000002050000000000001040030500000000000000400405000000000000e03f05060006050000000000000000070402",
            "010a01044c697665020006000101",
            "010b0203554c531c5b5b2264656164222c312e33355d2c5b226c697665222c312e355d5d02000600010601",
            "010c010464656164020006000102",
            "010d000200040502050000000000003e40",
            "010e02026531026e3101000c0206010600",
            "010f010764656661756c7401000600",
            "011001077b2278223a317d01000600",
            "0111020464656164056d6f64616c03000600010601020400",
        ];
        let commands = every_command();
        assert_eq!(commands.len(), expected.len(), "the baseline hex list must cover every command row");
        for (command, expected) in commands.iter().zip(expected) {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected, "wire bytes changed for {}", command.command_id());
        }
        let nodal_load_without_case = Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: None });
        assert_eq!(nodal_load_without_case.encode_op().expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>(), "010601026e3103000600010a020205000000000088b3c0");
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. Three rows
    /// (`setActiveExample`/`setCamera`/`setResultDisplay`) prove the wire keyword is NOT simply the
    /// kebab-cased command id — this is exactly what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keys = [
            "add-node",
            "add-bar",
            "add-frame",
            "add-material",
            "add-section",
            "add-support",
            "add-nodal-load",
            "add-member-udl",
            "add-area-load",
            "add-solid",
            "add-load-case",
            "add-combination",
            "set-self-weight",
            "set-analysis-settings",
            "remove-selection",
            "active-example",
            "camera",
            "result-display",
        ];
        for (command, expected) in every_command().into_iter().zip(expected_keys) {
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {command:?}: {printed:?}");
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = dsl::json::to_json_string(&create_fem3d_app());
        for id in [window_model::FEM3D_WINDOW_MODEL, window_results::FEM3D_WINDOW_RESULTS] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
        assert!(json.contains("computation.fem3d"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_labels_resolve_german_3d() {
        use semio_framework_plugin::{Locale, Terminology};
        let definition = create_fem3d_app();
        let window = definition.window_kinds.iter().find(|w| w.id == window_model::FEM3D_WINDOW_MODEL).expect("model window declared");
        assert_eq!(window.label.resolve(Terminology::Native, Locale::De), "Modell");
        let action = window.actions.iter().find(|action| action.id == "addFrame").expect("addFrame declared");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), "Rahmen hinzufügen");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), "Add Frame");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn undo_restores_document_after_add_node() {
        let mut app = fem3d_app();
        let before = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").nodes.len();
        assert_undo_redo_round_trip(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }), |app| semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").nodes.len(), before, before + 1).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::fem3d::testkit::render;
        let mut app = fem3d_app();
        assert!(render(&mut app, "fem3d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    /// 🎞️ `"results:out"` runs every load case fresh and returns a `Structured` JSON payload — build a
    /// doc with the bundled example (which has load cases), export, assert the JSON round-trips through
    /// the first-party value codec and names a case id.
    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_returns_solved_json_for_every_case_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() })).await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let history = semio_framework_plugin::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let doc = semio_framework_plugin::resolve_ready(ArtifactView::new(&snapshot, &history));
        let media = semio_framework_plugin::resolve_ready(Fem3dPlayApp::export_media("results:out", &doc)).expect("results:out exports");
        assert_eq!(media.media_type.class, MediaClass::Data);
        assert_eq!(media.media_type.form, MediaForm::Value);
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a Structured payload") };
        assert_eq!(schema, "computation.fem3d");
        let value = dsl::json::parse(&json).expect("results:out payload is valid JSON");
        assert!(value.get("dead").is_some(), "expected the example fixture's dead case in the results map: {json}");
        assert!(value["dead"].get("displacements").is_some(), "expected a displacements array: {json}");
    }

    /// 🎞️ `"results:out"` on a document with no load cases errors rather than panicking or returning an
    /// empty payload.
    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_errors_without_load_cases_3d() {
        let snapshot = crate::artifacts::fem3d::schema::empty_fem3d_snapshot();
        let history = semio_framework_plugin::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let doc = semio_framework_plugin::resolve_ready(ArtifactView::new(&snapshot, &history));
        let err = semio_framework_plugin::resolve_ready(Fem3dPlayApp::export_media("results:out", &doc)).expect_err("no load cases should error");
        assert!(matches!(err, MediaError::Payload(..)));
    }

    /// 🎞️ `"geometry:in"` decodes an extruded-footprint JSON contract into a new `FemSolid` operation.
    #[semio_framework_async_macros::async_test]
    async fn import_media_geometry_in_adds_a_new_solid_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Concrete".into(), e: 30e9, g: 12.5e9 })).await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let history = semio_framework_plugin::resolve_ready(semio_framework_plugin::HistoryView::empty());
        let doc = semio_framework_plugin::resolve_ready(ArtifactView::new(&snapshot, &history));
        let json = dsl::json!({
            "outline": [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]],
            "holes": [],
            "baseZ": 0.5,
            "height": 3.0,
            "layers": 2,
        })
        .to_string();
        let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "geometry".into(), json } };
        let emit = semio_framework_plugin::resolve_ready(Fem3dPlayApp::import_media("geometry:in", &media, &doc)).expect("geometry:in imports");
        assert_eq!(emit.artifact_mutations.len(), 1);
        match &emit.artifact_mutations[0] {
            Fem3dMutation::CreateSolid(crate::artifacts::fem3d::mutations::create_solid::mutation::CreateSolid { solid }) => {
                assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
                assert_eq!(solid.base_z, 0.5);
                assert_eq!(solid.height, 3.0);
                assert_eq!(solid.layers, 2);
                assert_eq!(solid.material_id, "m0");
            }
            _ => panic!("expected CreateSolid"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_io_matches_declared_artifact_identity_3d() {
        let io = semio_framework_plugin::resolve_ready(Fem3dPlayApp::io()).expect("fem3d declares typed media I/O");
        assert_eq!(io.artifact.id, "3d.fem");
        assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
        assert!(io.ports.iter().any(|port| port.id == "results:out"));
    }

    /// 🔌️ Wave-1's `required: true` unwired-input enforcement (`validate_edge_kinds`) lives in the run
    /// crate, not here — this test only proves the port DECLARATION is correct; the cross-crate
    /// enforcement is exercised at the run-crate level.
    #[semio_framework_async_macros::async_test]
    async fn fem3d_io_declares_geometry_in_and_results_out_ports() {
        let io = fem3d_io();
        assert_eq!(io.document_schema, crate::artifacts::fem3d::FEM_3D_SCHEMA);
        assert_eq!(io.document_media_type.class, semio_framework_plugin::MediaClass::ThreeD);
        assert_eq!(io.document_media_type.form, semio_framework_plugin::MediaForm::Any);
        assert_eq!(io.artifact.id, "3d.fem");
        assert_eq!(io.artifact.component_kind, "fem3d");

        let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(geometry_in.required, "geometry:in is a required input port");
        assert_eq!(geometry_in.media_type.class, semio_framework_plugin::MediaClass::ThreeD);
        assert_eq!(geometry_in.media_type.form, semio_framework_plugin::MediaForm::Any);
        assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

        let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
        assert_eq!(results_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert!(!results_out.required, "results:out is optional");
        assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem3d"));
        assert_eq!(results_out.media_type.class, semio_framework_plugin::MediaClass::Data);
        assert_eq!(results_out.media_type.form, semio_framework_plugin::MediaForm::Value);
    }
    //#endregion 🔖️MediaPorts

    //#region 🎬️SceneRender
    #[semio_framework_async_macros::async_test]
    async fn quat_z_to_identity_for_parallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, 1.0]), [0.0, 0.0, 0.0, 1.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn quat_z_to_handles_antiparallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, -1.0]), [1.0, 0.0, 0.0, 0.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_camera_json_falls_back_to_world3d_default_for_empty_object() {
        let camera = crate::artifacts::fem3d::FemCamera::default();
        assert_eq!(fem3d_camera_json(&camera), semio_framework_plugin::world3d_default_camera());
        let custom = crate::artifacts::fem3d::FemCamera { json: "{\"x\":1}".into() };
        assert_eq!(fem3d_camera_json(&custom), "{\"x\":1}");
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_scene_parts_include_solid_mesh_and_oriented_member_instances() {
        let doc: Fem3dSnapshot = crate::artifacts::fem3d::dsl::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).expect("example fixture parses");
        let (meshes_json, instances_json) = fem3d_scene_parts(&doc, None, doc.analysis.deformation_scale, None);
        assert!(meshes_json.contains("solid-sol1"), "expected a solid- mesh id for the example fixture's solid: {meshes_json}");
        assert!(instances_json.contains("el-e1"), "expected a single oriented box instance per member (no -{{i}} sphere chain): {instances_json}");
    }
    //#endregion 🎬️SceneRender
}
//#endregion 🧪️Tests
