//! 🎲️ Procedural2d editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command
//! enum and the manifest stitch (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's own inferences. This file is a routing table: `handle`
//! → `Procedural2dCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls
//! one passthrough per node.

use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::{artifact_kind, Procedural2dSnapshot, PROCEDURAL2D_DIALECT, PROCEDURAL_2D_SCHEMA};
use crate::editor::procedural2d::commands::{
    add_generation, add_widget, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, connect_media_ports, enter_generate, flow_eval_tick, move_media_node, node_graph_edit, node_graph_viewport, remove_generation, remove_widget,
    rename_generation, reorganize, select_generation, set_eval_outputs, set_locale, set_show_mode, update_generation_values,
};
use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::editor::procedural2d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::editor::procedural2d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::editor::procedural2d::modes::{edit, generate};
use crate::editor::procedural2d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::procedural2d::terminology::{procedural2d_labels, Procedural2dLabels};
use flow::FlowEvalSession;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    app::InteractionView, ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect,
    DomainTopology, DraftView, Editor, EditorApp, Effect, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MediaClass,
    MediaForm, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
use store::EngineHandles;

//#region 🔖️Constants
/// 🏷️ Plain string tag (NOT a trait const — `ArtifactEditor::DIALECT`+`ROLE` derive the real surface
/// id now, contract §2.1) reused wherever a window/panel needs a stable controller/action-factory id.
pub const PROCEDURAL2D_PLAY_APP_ID: &str = "procedural2d-play";

fn categorized_action(id: &str, label: LocalizedLabel, kind: ActionKind, category: &str) -> ActionDefinition {
    semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog(id, label, kind).with_category(category))
}

//#endregion 🔖️Constants

//#region 🔖️ArtifactIo
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_procedural2d_app`'s
/// `.artifact_kind(...)` document schema/media type verbatim, plus two workflow ports: `params:in`
/// (generic Data×Value parametric input) and `drawing:out` (TwoD×Vector, tagged with draw's already-
/// registered `2d.drawing` kind id).
pub async fn procedural2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "procedural.2d",
        MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "2d.procedural".into(), name: "2D Procedural".into(), dimension: "2d".into(), component_kind: "procedural2d".into() },
    )
    .await
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::One,
        },
        semio_framework_plugin::MediaPortSpec {
            id: "drawing:out".into(),
            label: "Drawing".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            kind_id: Some("2d.drawing".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
    .await
}
//#endregion 🔖️ArtifactIo

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Procedural2dPlayApp::Command` — the SOLE dispatch surface for procedural2d's own behavior.
    /// Each row states BOTH the manifest action id (`command_id()`) and the `dsl` wire keyword
    /// (`#[dsl(key = ..)]`) — genuinely different vocabularies; `setLocale`/`locale` proves it. **Row
    /// order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Procedural2dCommand for Procedural2dSnapshot, Procedural2dMutation, Procedural2dConfig, Procedural2dConfigMutation, ctx = FlowEvalSession {
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setShowMode" as "set-show-mode" => set_show_mode::SetShowMode,
        "generate" as "generate" => enter_generate::Generate,
        "setEvalOutputs" as "set-eval-outputs" => set_eval_outputs::SetEvalOutputs,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "setLocale" as "locale" => set_locale::SetLocale}
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const PROCEDURAL2D_BOUNDED_TOOL_IDS: &[&str] = &["nodeGraphViewport", "setShowMode", "generate", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasWheel"];
const PROCEDURAL2D_RETAINED_PAYLOAD_SCHEMA: &str = "procedural.2d.tool-command.v1";
const PROCEDURAL2D_RETAINED_RAW_BYTES: usize = 8_192;

fn procedural2d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(PROCEDURAL2D_RETAINED_RAW_BYTES, 64, 1, 16_384, 7_500)
}

fn procedural2d_bounded_extent(_command: &Procedural2dCommand, _snapshot: &Procedural2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

fn procedural2d_retained_reduce(
    command: &Procedural2dCommand,
    snapshot: &Procedural2dSnapshot,
    config: &Procedural2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation, NoDraftMutation>, Fault> {
    if !PROCEDURAL2D_BOUNDED_TOOL_IDS.contains(&command.command_id()) { return Err(Fault::from("procedural2d-command-retained-route-rejected")); }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    let mut session = FlowEvalSession::new();
    command.dispatch(&doc, &cfg, &mut session)
}

struct Procedural2dBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Procedural2dBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: PROCEDURAL2D_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Procedural2dBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Procedural2dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Procedural2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PROCEDURAL2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        procedural2d_bounded_contract()
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
        if input.declared_bytes() > PROCEDURAL2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Procedural2d retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Procedural2dBoundedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<Procedural2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PROCEDURAL2D_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PROCEDURAL_2D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShowMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "generate", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasWheel", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
const PROCEDURAL2D_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;
const PROCEDURAL2D_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;

//#region 🎟️Admission
fn procedural2d_config_text_bytes(config: &Procedural2dConfig) -> usize {
    [config.show_mode.len(), config.selected_generation_id.as_ref().map_or(0, String::len), config.generation_preview_text.as_ref().map_or(0, String::len), config.locale.len()].into_iter().fold(0usize, usize::saturating_add)
}

fn procedural2d_config_publication_bytes(mutation: &Procedural2dConfigMutation) -> Result<usize, String> {
    let bytes = match mutation {
        Procedural2dConfigMutation::SetCamera { .. } => 0,
        Procedural2dConfigMutation::SetShowMode { value } => value.len(),
        _ => return Err("procedural2d-config-unsupported-mutation".into()),
    };
    if bytes > PROCEDURAL2D_CONFIG_TEXT_MAXIMUM_BYTES { return Err("procedural2d-config-text-envelope".into()); }
    Ok(PROCEDURAL2D_CONFIG_PUBLICATION_MAXIMUM_BYTES)
}

struct Procedural2dConfigPreparationFactory;

impl store::ArtifactStoreOneItemPreparationFactory<Procedural2dConfig, Procedural2dConfigMutation> for Procedural2dConfigPreparationFactory {
    fn preflight(&self, mutation: &Procedural2dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > 64) {
            return Err("procedural2d-config-lane-or-description-envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: procedural2d_config_publication_bytes(mutation)? })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<Procedural2dConfig, Procedural2dConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Procedural2dConfig, Procedural2dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Procedural2dConfig, Procedural2dConfigMutation>> {
        if request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > 64 || self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err() || procedural2d_config_text_bytes(request.base.get()) > PROCEDURAL2D_CONFIG_TEXT_MAXIMUM_BYTES {
            return Err(request);
        }
        Ok(Box::new(Procedural2dConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}
//#endregion 🎟️Admission

//#region 🧵️Preparation
struct Procedural2dConfigPreparation {
    base: Option<store::SnapshotRead<Procedural2dConfig>>,
    mutation: Option<Procedural2dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Procedural2dConfig, Procedural2dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparation<Procedural2dConfig, Procedural2dConfigMutation> for Procedural2dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < PROCEDURAL2D_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.closing { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.checkpoint.cursor != 0 { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        let base = self.base.as_ref().ok_or_else(|| "procedural2d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.as_ref().ok_or_else(|| "procedural2d-config-mutation-owner-missing".to_string())?;
        let mut next = base.get().clone();
        let inverse = match mutation {
            Procedural2dConfigMutation::SetCamera { camera } => { next.camera = camera.clone(); Procedural2dConfigMutation::SetCamera { camera: base.get().camera.clone() } }
            Procedural2dConfigMutation::SetShowMode { value } => { next.show_mode = value.clone(); Procedural2dConfigMutation::SetShowMode { value: base.get().show_mode.clone() } }
            _ => return Err("procedural2d-config-unsupported-mutation".into()),
        };
        if procedural2d_config_text_bytes(&next) > PROCEDURAL2D_CONFIG_TEXT_MAXIMUM_BYTES { return Err("procedural2d-config-post-text-envelope".into()); }
        let authority = self.authority.as_ref().ok_or_else(|| "procedural2d-config-authority-missing".to_string())?;
        let id = format!("procedural2d-config-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![mutation.clone()], inverse: vec![inverse],
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
            }],
            description: self.description.clone(), coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(next))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: PROCEDURAL2D_CONFIG_PUBLICATION_MAXIMUM_BYTES as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Procedural2dConfig, Procedural2dConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Procedural2dConfig, Procedural2dConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes < PROCEDURAL2D_CONFIG_PUBLICATION_MAXIMUM_BYTES { return Ok(store::SnapshotRetirementStep::Blocked); }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: PROCEDURAL2D_CONFIG_PUBLICATION_MAXIMUM_BYTES });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("procedural2d-config-base-retirement-rejected".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES }); }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 🧵️Preparation
//#region 🧪️PreparationLaws
#[cfg(test)]
mod procedural2d_config_preparation_laws {
    use super::*;
    use store::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory};

    #[test]
    fn admitted_maximum_and_production_grant_make_bounded_progress() {
        let factory = Procedural2dConfigPreparationFactory;
        let maximum = Procedural2dConfigMutation::SetShowMode { value: "x".repeat(PROCEDURAL2D_CONFIG_TEXT_MAXIMUM_BYTES) };
        assert_eq!(factory.preflight(&maximum, None, store::HistoryLane::Document).expect("maximum admission").retained_bytes, 4_096);
        let overflow = Procedural2dConfigMutation::SetShowMode { value: "x".repeat(PROCEDURAL2D_CONFIG_TEXT_MAXIMUM_BYTES + 1) };
        assert!(factory.preflight(&overflow, None, store::HistoryLane::Document).is_err());
        assert!(factory.preflight(&maximum, Some(&"x".repeat(65)), store::HistoryLane::Document).is_err());
        let mut work = Procedural2dConfigPreparation {
            base: None, mutation: Some(maximum), description: None, authority: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        };
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4_096 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_095 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        work.cancel();
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        work.begin_close();
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }), Ok(store::SnapshotRetirementStep::Blocked)));
        assert!(work.mutation.is_some());
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 4_096 })));
        assert!(work.terminal_is_empty());
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::SnapshotRetirementStep::Complete)));
    }
}
//#endregion 🧪️PreparationLaws
//#endregion 📬️ConfigStorePreparation

//#region 🔖️Procedural2dPlayApp
/// 🧪️ Unit struct apart from `eval_session`: every former runtime field lives in [`Procedural2dConfig`],
/// written through [`Procedural2dConfigMutation`]s. The eval session is the one piece of state that is
/// neither document nor view — it is threaded into every command handler as the `app_commands!`
/// dispatch context.
#[derive(Default)]
pub struct Procedural2dPlayApp;

impl ArtifactEditor for Procedural2dPlayApp {
    type Snapshot = Procedural2dSnapshot;
    type Mutation = Procedural2dMutation;
    type Config = Procedural2dConfig;
    type ConfigMutation = Procedural2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::procedural2d::presence::Procedural2dPresence;
    type PresenceMutation = crate::editor::procedural2d::presence::Procedural2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Procedural2dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::procedural2d::spr::procedural2d_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::procedural2d::spr::procedural2d_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::procedural2d::spr::procedural2d_document_store_initialization_job(envelope, operation, generation))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        crate::artifacts::procedural2d::spr::procedural2d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Procedural2d atomic publication authority is absent or stale"))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    const DIALECT: Dialect = PROCEDURAL2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PROCEDURAL_2D_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Procedural2dConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Procedural2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.procedural2d@1/*#editor",
        document_schema: "procedural.2d",
        factory: "Procedural2dBoundedCommandJobFactory",
        factory_type: Procedural2dBoundedCommandJobFactory,
        tools: {
            "nodeGraphViewport" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setShowMode" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "generate" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "canvasPointerDown" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "canvasPointerMove" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "canvasPointerUp" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "canvasWheel" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Procedural2dBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PROCEDURAL2D_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("procedural2d-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, procedural2d_retained_reduce, procedural2d_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            Procedural2dCommand::command_id,
            PROCEDURAL2D_RETAINED_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::procedural2d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Procedural2dSnapshot {
        crate::artifacts::procedural2d::schema::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_framework::io::resolve_ready(procedural2d_io()))
    }

    fn command_id(command: &Procedural2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Procedural2dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(dsl::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "nodeGraphEdit" => Ok(Procedural2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(dsl::json::to_json_string)).unwrap_or_else(|| "[]".into()),
            })),
            "moveMediaNode" => Ok(Procedural2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "addWidget" => Ok(Procedural2dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), neuron_kind: str_arg(&["neuronKind", "neuron_kind"]), x: f64_arg(&["x"]), y: f64_arg(&["y"]) })),
            "removeWidget" => Ok(Procedural2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "connectMediaPorts" => Ok(Procedural2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts {
                source_node_id: str_arg(&["sourceNodeId", "source_node_id"]).unwrap_or_default(),
                source_port_id: str_arg(&["sourcePortId", "source_port_id"]).unwrap_or_default(),
                target_node_id: str_arg(&["targetNodeId", "target_node_id"]).unwrap_or_default(),
                target_port_id: str_arg(&["targetPortId", "target_port_id"]).unwrap_or_default(),
            })),
            "reorganize" => Ok(Procedural2dCommand::Reorganize(reorganize::Reorganize {})),
            "addGeneration" => Ok(Procedural2dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Procedural2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Procedural2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").map_or(dsl::DslValue::Null, |entry| entry.clone());
                Ok(Procedural2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => {
                let viewport_json = str_arg(&["viewportJson", "viewport_json"]).or_else(|| args.get("camera").map(|value| if value.as_str().is_some() { value.as_str().unwrap_or("{}").to_string() } else { dsl::json::to_json_string(value) })).unwrap_or_else(|| "{}".into());
                Ok(Procedural2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json }))
            }
            "setShowMode" => Ok(Procedural2dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode"]).unwrap_or_default() })),
            "generate" => Ok(Procedural2dCommand::Generate(enter_generate::Generate {})),
            "setEvalOutputs" => Ok(Procedural2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: str_arg(&["outputsJson", "outputs_json", "evalJson"]).unwrap_or_else(|| "{}".into()) })),
            "canvasPointerDown" => Ok(Procedural2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {})),
            "canvasPointerMove" => Ok(Procedural2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {})),
            "canvasPointerUp" => Ok(Procedural2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {})),
            "canvasWheel" => Ok(Procedural2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {})),
            "selectGeneration" => Ok(Procedural2dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]) })),
            "flowEvalTick" => Ok(Procedural2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {})),
            "setLocale" => Ok(Procedural2dCommand::SetLocale(set_locale::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 🕹️ `nodeGraphEdit` reads the `graph` interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg, ctx)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &Procedural2dCommand,
        doc: &ArtifactView<'_, Procedural2dSnapshot>,
        cfg: &ConfigView<'_, Procedural2dConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation, Self::DraftMutation>, Fault> {
        let mut session = FlowEvalSession::new();
        match command {
            Procedural2dCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction, &mut session),
            _ => command.dispatch(doc, cfg, &mut session),
        }
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    fn interaction_topology(doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>) -> InteractionTopology {
        fn walk_neuron(neuron: &flow::neural::Neuron, parent: String, ordered: &mut Vec<TopologyNode>) {
            ordered.push(TopologyNode { id: neuron.id.clone(), granularity: "node".into(), parent: Some(parent) });
            if let Some(tree) = &neuron.tree {
                for child in &tree.neurons {
                    walk_neuron(child, neuron.id.clone(), ordered);
                }
            }
        }
        let fixture = &doc.snapshot.fixture;
        let mut ordered = Vec::new();
        for widget in &fixture.widgets {
            let id = crate::artifacts::procedural2d::widget_id(widget).to_string();
            ordered.push(TopologyNode { id: id.clone(), granularity: "node".into(), parent: None });
            if let flow::Widget::Cluster { tree, .. } = widget {
                for child in &tree.neurons {
                    walk_neuron(child, id.clone(), &mut ordered);
                }
            }
        }
        for synapse in &fixture.synapses {
            ordered.push(TopologyNode { id: synapse.id.clone(), granularity: "edge".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("graph".to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, remote operations) in one place instead of each
    /// action re-checking.
    fn pending_effects(doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>) -> Vec<Effect> {
        let mut session = FlowEvalSession::new();
        let host = crate::artifacts::procedural2d::schema::host_from_fixture_with_session(&doc.snapshot.fixture, &session);
        if session.sync(&host) {
            vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(101), action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = procedural2d_labels(config);
        let session = FlowEvalSession::new();
        let node = match body_key {
            flow_window::PROCEDURAL2D_PLAY_BODY_MAIN => flow_window::render(document, config, &session),
            edit_preview::PROCEDURAL2D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, &session),
            generations::PROCEDURAL2D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::Native),
            form::PROCEDURAL2D_PLAY_BODY_GENERATE_FORM => form::render(document, &document.generation, labels),
            generate_preview::PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(config, labels),
            document_panel::PROCEDURAL2D_PLAY_BODY_DOCUMENT => document_panel::render(document, config, labels),
            catalogue_panel::PROCEDURAL2D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            inspection_panel::PROCEDURAL2D_PLAY_BODY_INSPECTION => inspection_panel::render(document, config, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🗂️ Grouped disclosure: `addWidget`/`reorganize`/`generate` stay top-level; the display-mode
    /// toggle, generation authoring, and generation selection each fold into their own taxonomy group;
    /// the delete-selection item stays a direct destructive item last.
    ///
    /// 🕹️ `context_menu` carries no `InteractionView` either (same gap as `render` — see ticket
    /// 26/08/14's w3b-summary.md), so the selection-dependent delete row below always takes the
    /// "nothing selected" branch rather than reading a stale/wrong selection.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, Procedural2dSnapshot>,
        cfg: &ConfigView<'_, Procedural2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        semio_framework::io::resolve_ready(async {
            let config = cfg.snapshot;
            let labels = semio_framework_plugin::resolve_labels_for_locale::<Procedural2dLabels>(&config.locale);
            let is_de = config.locale.starts_with("de");
            let selected: Vec<String> = Vec::new();
            let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]).await;
            let mut menu = Menu::of(registry).await.action("addWidget").await.action("reorganize").await.action("generate").await;
            menu = menu.group("mode", |m| async { m.action("setShowMode").await }).await;
            menu = menu.group("create", |m| async { m.action("addGeneration").await }).await;
            menu = menu.group("methods", |m| async { m.action("selectGeneration").await }).await;
            if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit).await {
                menu = menu.item(spec).await;
            }
            menu.build().await
        })
    }

    /// 🎞️ Declares `export_media`'s default document schema — pack-encodes `doc.snapshot`, wrapped
    /// `Structured{schema: Self::DOCUMENT_SCHEMA, json: base64}` — plus `"drawing:out"`.
    fn export_media(port: &str, doc: &ArtifactView<'_, Procedural2dSnapshot>) -> Result<semio_framework_plugin::Media, semio_framework_plugin::MediaError> {
        match port {
            "drawing:out" => {
                let eval_json = crate::artifacts::procedural2d::schema::evaluate_generation_preview(&doc.snapshot.fixture, &serde_json::Map::new());
                let layers_json = crate::artifacts::procedural2d::schema::generation_preview_layers(&eval_json);
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "2d.drawing".into(), json: layers_json } })
            }
            "document:out" => {
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
                    payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            _ => Err(semio_framework_plugin::MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"`: a generic Data×Value JSON object `{widgetId: number}` — patches matching
    /// `InputSlider` widgets' `value` field, leaving unmatched keys/widget kinds untouched.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Procedural2dSnapshot>) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        if port != "params:in" {
            return Err(semio_framework_plugin::MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in expects a Structured JSON object payload".into()));
        };
        let parsed = dsl::json::parse(json).map_err(|error| semio_framework_plugin::MediaError::Payload(port.to_string(), error.to_string()))?;
        let Some(object) = parsed.as_object() else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()));
        };
        let mut operations = Vec::new();
        for (widget_id_key, value) in object.iter() {
            let Some(number) = value.as_f64() else { continue };
            let Some(widget) = doc.snapshot.fixture.widgets.iter().find(|widget| crate::artifacts::procedural2d::widget_id(widget) == widget_id_key) else { continue };
            if let flow::Widget::InputSlider { id, label, min, max, step, .. } = widget {
                operations.push(crate::artifacts::procedural2d::op::replace_widget(flow::Widget::InputSlider { id: id.clone(), label: label.clone(), value: number, min: *min, max: *max, step: *step }));
            }
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️Procedural2dPlayApp

//#region 🔖️Manifest
pub fn create_procedural2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(PROCEDURAL2D_DIALECT)
        .document(["semio", "procedural", "2d"])
        .artifact_kind(artifact_kind())
        .icon_id("procedural2d")
        .mode_def(edit::definition())
        .mode_def(generate::definition())
        .mode_layout(generate::PROCEDURAL2D_PLAY_MODE_GENERATE, generate::PROCEDURAL2D_PLAY_LAYOUT_GENERATE)
        .default_mode_id(edit::PROCEDURAL2D_PLAY_MODE_EDIT)
        .window_kind_def(flow_window::definition())
        .window_kind_def(edit_preview::definition())
        .window_kind_def(generations::definition())
        .window_kind_def(form::definition())
        .window_kind_def(generate_preview::definition())
        .default_layout(edit::layout())
        .named_layout(generate::layout())
        .panel_tab_def(document_panel::definition())
        .panel_tab_def(catalogue_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
        // 🗂️ Referenced by `Procedural2dPlayApp::context_menu` — categorized for grouped-context-menu disclosure.
        .action_with(categorized_action("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation, "selection"))
        .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
        .action_with(categorized_action("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation, "create"))
        .mutation("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"))
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
        .action_with(categorized_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "transform"))
        .action_with(categorized_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation, "create"))
        .mutation("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"))
        .mutation("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"))
        .mutation("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"))
        // 👁️ Ephemeral view actions — camera, the show-mode display toggle, and evaluation scratch
        // (emit no operations). Selection/hover are the framework's `graph` interaction domain now
        // (`.interaction(...)` below) — the six framework verbs auto-inject.
        .view_action("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"))
        .action_with(categorized_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"), ActionKind::View, "mode"))
        .action_with(categorized_action("generate", LocalizedLabel::native("Generate", "Generieren"), ActionKind::View, "actions"))
        .view_action("setEvalOutputs", LocalizedLabel::native("Set Eval Outputs", "Auswertungsausgaben festlegen"))
        .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Canvas-Zeiger gedrückt"))
        .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Canvas-Zeiger bewegt"))
        .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Canvas-Zeiger losgelassen"))
        .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Canvas-Mausrad"))
        .action_with(categorized_action("selectGeneration", LocalizedLabel::native("Select Generation", "Generation auswählen"), ActionKind::View, "methods"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), ActionKind::View) })
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("moveMediaNode", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("addWidget", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("removeWidget", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("connectMediaPorts", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("reorganize", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("addGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("removeGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("renameGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("updateGenerationValues", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("generate", InteractiveJobClassification::Migrated)
        .action_interactive_job("setEvalOutputs", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasWheel", InteractiveJobClassification::Migrated)
        .action_interactive_job("selectGeneration", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("flowEvalTick", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("setLocale", InteractiveJobClassification::ForbiddenFromUi)
        // 📝️ Staged argument form for the palette-visible add-widget action (default materialized host-side).
        .action_args("addWidget", vec![
            ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                ActionArgOption::new("neuron", LocalizedLabel::native("Component", "Komponente")),
                ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                ActionArgOption::new("outputExport", LocalizedLabel::native("Export", "Export")),
            ]).default_value("inputSlider"),
        ])
        // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
        // one domain over the flow-graph widget DAG, node/edge/handle granularities,
        // `HierarchyProvider::Topology` (see `Procedural2dPlayApp::interaction_topology` above) —
        // transitive hover is the headline feature: hovering a Cluster group node highlights every
        // widget nested in its tree.
        .interaction(InteractionDefinition {
            id: "graph".into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                GranularityDefinition { id: "handle".into(), label: LocalizedLabel::native("Handle", "Griff"), icon_id: "move".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec { transitive: true, ..HoverSpec::default() },
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                transitive: false,
                broadcast: true,
            },
        })
        .window_kind_interactions(flow_window::PROCEDURAL2D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
        .window_kind_interactions(edit_preview::PROCEDURAL2D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph")])
        .window_kind_interactions(generate_preview::PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .config(Procedural2dPlayApp::config_spec())
        .io(semio_framework::io::resolve_ready(procedural2d_io()))
        // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
        // `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
        // `.workflow(...)` on this builder, so the old `"default"` app-level example registration and
        // the no-op `.workflow("procedural2d", …)` call are dropped here (not silently — reported in
        // this packet's migration notes). The subset's own `📚️examples` facet is the modern,
        // role-agnostic replacement surface for this.
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Procedural2dApp = VcsArtifactApp<EditorApp<Procedural2dPlayApp>>;

    pub fn app() -> Procedural2dApp {
        new_app::<EditorApp<Procedural2dPlayApp>>()
    }

    pub fn app_with_registry() -> Procedural2dApp {
        new_app_with_registry::<EditorApp<Procedural2dPlayApp>>(procedural2d_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut Procedural2dApp, command: Procedural2dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Procedural2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// ✏️ Adapts `create_procedural2d_app`'s `AppDefinition` (contract §2.4) into the `App {
    /// definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still
    /// expects — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub fn procedural2d_manifest_for_testkit() -> App {
        App { definition: create_procedural2d_app(), examples: Vec::new() }
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, app_with_registry, dispatch};
    use flow::Widget;
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;
    use semio_framework_plugin::PluginApp;

    fn production_initial_snapshot(label: &str) -> Procedural2dSnapshot {
        let mut snapshot = Procedural2dSnapshot::default();
        snapshot.fixture.schema = label.into();
        for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
            snapshot.fixture.widgets.push(flow::Widget::InputNote { id: id.into(), text: text.into() });
        }
        snapshot.fixture.synapses.push(flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
        snapshot.fixture.synapses.push(flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
        snapshot.fixture.layout.insert("move-target".into(), flow::WidgetLayout { x: 1.0, y: 2.0 });
        snapshot.fixture.layout.insert("clear-target".into(), flow::WidgetLayout { x: 3.0, y: 4.0 });
        for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
            snapshot.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(flow::playbook::FormGeneration { id: id.into(), name: name.into(), values: serde_json::Map::new() });
        }
        snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("rename-generation".into());
        snapshot
    }

    fn production_mutations() -> Vec<Procedural2dMutation> {
        use crate::artifacts::procedural2d::mutations::*;
        let params = flow::neural::Dictionary::new()
            .insert("integer", flow::neural::Value::Atom(flow::neural::Atom::Integer(7)))
            .insert("nested", flow::neural::Value::Dictionary(flow::neural::Dictionary::new().insert("text", flow::neural::Value::Atom(flow::neural::Atom::String("production".into())))));
        vec![
            create_widget(0, flow::Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true }),
            replace_widget(flow::Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() }),
            delete_widget("delete-target".into()),
            connect_synapse(0, flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() }),
            replace_synapse(flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() }),
            disconnect_synapse("disconnect-synapse".into()),
            move_widget("move-target".into(), flow::WidgetLayout { x: 31.0, y: -17.0 }),
            clear_widget_layout("clear-target".into()),
            update_camera(flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 }),
            change_schema("flow.fixture.production-retained".into()),
            create_generation(flow::playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: serde_json::Map::new() }),
            delete_generation("delete-generation".into()),
            rename_generation("rename-generation".into(), "After Rename".into()),
            change_generation_value("change-generation".into(), "deep-answer".into(), serde_json::json!({"object": {"array": [1.0, false, "retained"]}})),
        ]
    }

    fn production_hex(bytes: &[u8]) -> String {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut value = String::new();
        value.try_reserve_exact(bytes.len() * 2).expect("P2 production hex preflight");
        for byte in bytes {
            value.push(char::from(DIGITS[usize::from(byte >> 4)]));
            value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
        }
        value
    }

    fn production_semantic_digest(snapshot: &Procedural2dSnapshot) -> [u8; 32] {
        let mut digest = store::ArtifactStoreInitializationDigest::new(b"procedural2d.production-law.semantic");
        digest.observe(&crate::artifacts::procedural2d::snapshot::binary::encode(snapshot));
        digest.finish()
    }

    fn production_envelope_wire(label: &str) -> (Vec<u8>, Procedural2dSnapshot, [u8; 32]) {
        let snapshot = production_initial_snapshot(label);
        let mutations = production_mutations();
        assert_eq!(mutations.len(), 14, "production ingress carries every P2 mutation variant including clear-widget-layout");
        let mut mutation_hex = Vec::new();
        mutation_hex.try_reserve_exact(mutations.len()).expect("P2 production mutation owner preflight");
        for mutation in &mutations {
            mutation_hex.push(production_hex(&crate::artifacts::procedural2d::spr::encode_op(mutation).expect("P2 production mutation encoding")));
        }
        let mut expected = production_initial_snapshot(label);
        crate::artifacts::procedural2d::spr::procedural2d_apply_retained_mutations_for_test(&mut expected, &mutations);
        let expected_digest = production_semantic_digest(&expected);
        let wire = serde_json::to_vec(&serde_json::json!({
            "schema": crate::artifacts::procedural2d::PROCEDURAL_2D_SCHEMA,
            "id": "procedural2d-production-mounted-law",
            "vcs": {
                "initialSnapshot": production_hex(&crate::artifacts::procedural2d::snapshot::binary::encode(&snapshot)),
                "edits": [{
                    "id": "procedural2d-production-all14-edit",
                    "actor": "procedural2d-production-law",
                    "forwards": mutation_hex,
                    "inverse": [],
                    "sequenceNumber": 1,
                    "startedAt": "1"
                }],
                "changes": [],
                "checkpoints": [],
                "alternatives": []
            },
            "editMessages": [],
            "conflicts": []
        }))
        .expect("schema-first P2 production fixture envelope");
        (wire, expected, expected_digest)
    }

    fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Procedural2dPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P2 production ingress credits");
        crate::artifacts::procedural2d::spr::procedural2d_admit_publication_authority(
            handle.operation,
            handle.generation,
            handle.generation.0,
            handle.generation.0,
            handle.generation.0,
            8_192,
            crate::artifacts::procedural2d::spr::PROCEDURAL2D_MOUNTED_OUTPUT_CHANNELS,
            crate::artifacts::procedural2d::spr::PROCEDURAL2D_MOUNTED_CONTROL_CREDITS,
        )
        .expect("P2 production publication authority");
        for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            bytes[..chunk.len()].copy_from_slice(chunk);
            let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P2 production envelope page");
            app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P2 production envelope page admission failed: {fault}"));
        }
        assert!(app.seal_artifact_envelope_ingress(handle).expect("P2 production envelope seal"));
        handle
    }

    fn drive_production_envelope(
        app: &mut semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Procedural2dPlayApp>>,
        handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle,
    ) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..300_000 {
            crate::artifacts::procedural2d::spr::procedural2d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0).expect("P2 authority refresh immediately before production maintenance");
            PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P2 production maintenance turn");
            let poll = app.advance_artifact_envelope_load(handle).expect("P2 production load advancement");
            if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
                return poll;
            }
            std::thread::yield_now();
        }
        panic!("P2 production envelope load did not reach terminal");
    }

    /// 🔐️ LAW: non-empty P2D2 canonical ingress reaches the real VCS maintenance replacement,
    /// and accepted, stale, ABA, and displaced stores remain owned until explicit terminal ACK/close.
    #[semio_framework_async_macros::async_test]
    async fn vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed() {
        let mut accepted = semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::EditorApp<Procedural2dPlayApp>>::new(semio_framework_plugin::EditorApp::default()).await;
        let base_generation = accepted.artifact_generation_now();
        let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
        let handle = admit_production_envelope(&mut accepted, &wire);
        assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
        let snapshot = accepted.snapshot().await.expect("accepted P2 production snapshot");
        assert_eq!(&*snapshot, &expected, "real maintenance must publish all P2 snapshot and all-14 replay fields");
        assert_eq!(production_semantic_digest(&snapshot), expected_digest);
        assert!(snapshot.fixture.layout.contains_key("move-target"));
        assert!(!snapshot.fixture.layout.contains_key("clear-target"), "2D-only clear-widget-layout must survive retained replay");
        assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P2 terminal ACK"));
        assert!(crate::artifacts::procedural2d::spr::procedural2d_release_publication_authority(handle.operation, handle.generation));

        use crate::artifacts::procedural2d::spr::Procedural2dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
        for (hostile, expected_code) in [
            (Missing, "procedural2d-publication.authority-missing"),
            (WrongOperation, "procedural2d-publication.wrong-operation"),
            (WrongGeneration, "procedural2d-publication.wrong-generation"),
            (WrongBase, "procedural2d-publication.wrong-base"),
            (WrongParent, "procedural2d-publication.wrong-parent"),
        ] {
            let mut app = semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::EditorApp<Procedural2dPlayApp>>::new(semio_framework_plugin::EditorApp::default()).await;
            let last_valid = app.snapshot().await.expect("last-valid P2 snapshot");
            let last_valid_digest = production_semantic_digest(&last_valid);
            let base_generation = app.artifact_generation_now();
            let (wire, _, _) = production_envelope_wire("rejected-production-candidate");
            let handle = admit_production_envelope(&mut app, &wire);
            crate::artifacts::procedural2d::spr::procedural2d_arm_publication_hostile(handle.operation, hostile);
            assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
            assert_eq!(crate::artifacts::procedural2d::spr::procedural2d_take_publication_hostile_observed(handle.operation), Some(expected_code));
            assert_eq!(app.artifact_generation_now(), base_generation);
            let retained = app.snapshot().await.expect("last-valid P2 snapshot after rejected candidate");
            assert_eq!(production_semantic_digest(&retained), last_valid_digest);
            assert_eq!(retained, last_valid);
            assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P2 terminal ACK after candidate retirement"));
            assert!(crate::artifacts::procedural2d::spr::procedural2d_release_publication_authority(handle.operation, handle.generation));
        }
    }

    //#region 🔖️CommandSurface
    #[test]
    fn retained_route_dispositions_are_exact_and_exhaustive() {
        use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
        use semio_framework_plugin::ArtifactOwnedToolJobFactory;
        assert_eq!(PROCEDURAL2D_BOUNDED_TOOL_IDS.len(), 7);
        assert_eq!(<Procedural2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 7);
        assert_eq!(Procedural2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 7);
        assert_eq!(procedural2d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
        assert_eq!(procedural2d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
        assert!(PROCEDURAL2D_BOUNDED_TOOL_IDS.iter().all(|tool_id| Procedural2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
        for blocked in ["nodeGraphEdit", "moveMediaNode", "addWidget", "removeWidget", "connectMediaPorts", "reorganize", "addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "setEvalOutputs", "selectGeneration", "flowEvalTick", "setLocale"] {
            assert!(!PROCEDURAL2D_BOUNDED_TOOL_IDS.contains(&blocked));
        }
    }

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 21, "every Procedural2dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
    /// explicitly per row (not derived from the command id) since `setLocale`/`locale` is the one row
    /// where the two vocabularies genuinely diverge. This is what a missing `#[dsl(keyword = ..)]` on a
    /// payload struct silently breaks (the record prints with no keyword at all and fails to re-parse).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keywords = [
            "node-graph-edit",
            "move-media-node",
            "add-widget",
            "remove-widget",
            "connect-media-ports",
            "reorganize",
            "add-generation",
            "remove-generation",
            "rename-generation",
            "update-generation-values",
            "node-graph-viewport",
            "set-show-mode",
            "generate",
            "set-eval-outputs",
            "canvas-pointer-down",
            "canvas-pointer-move",
            "canvas-pointer-up",
            "canvas-wheel",
            "select-generation",
            "flow-eval-tick",
            "locale",
        ];
        let commands = every_command();
        assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
        for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
            let printed = protocol::OpText::print_op(command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<Procedural2dCommand> {
        vec![
            Procedural2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
            Procedural2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
            Procedural2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None }),
            Procedural2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "n1".into() }),
            Procedural2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
            Procedural2dCommand::Reorganize(reorganize::Reorganize {}),
            Procedural2dCommand::AddGeneration(add_generation::AddGeneration {}),
            Procedural2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "g1".into() }),
            Procedural2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "g1".into(), name: "Copy".into() }),
            Procedural2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
            Procedural2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: "{}".into() }),
            Procedural2dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wire".into() }),
            Procedural2dCommand::Generate(enter_generate::Generate {}),
            Procedural2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: "{}".into() }),
            Procedural2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}),
            Procedural2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}),
            Procedural2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
            Procedural2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {}),
            Procedural2dCommand::SelectGeneration(select_generation::SelectGeneration { id: Some("g1".into()) }),
            Procedural2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
            Procedural2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_procedural2d_app()).expect("app definition json");
        for id in [
            flow_window::PROCEDURAL2D_PLAY_WINDOW_MAIN,
            edit_preview::PROCEDURAL2D_PLAY_WINDOW_PREVIEW,
            generations::PROCEDURAL2D_PLAY_WINDOW_GENERATIONS,
            form::PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM,
            generate_preview::PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW,
        ] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for id in [edit::PROCEDURAL2D_PLAY_MODE_EDIT, generate::PROCEDURAL2D_PLAY_MODE_GENERATE] {
            assert!(json.contains(id), "mode {id} missing from the manifest");
        }
        for body in [document_panel::PROCEDURAL2D_PLAY_BODY_DOCUMENT, catalogue_panel::PROCEDURAL2D_PLAY_BODY_CATALOGUE, inspection_panel::PROCEDURAL2D_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("2d.procedural"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn declared_actions_bridge_to_commands() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<semio_framework_plugin::EditorApp<Procedural2dPlayApp>>(crate::editor::procedural2d::testkit::procedural2d_manifest_for_testkit);
    }

    #[test]
    fn add_widget_materializes_declared_kind_default_into_an_operation() {
        let mut app = app_with_registry();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        app.dispatch_typed(Procedural2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add widget");
        assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before + 1);
    }

    #[test]
    fn add_widget_undo_redo_round_trip() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        assert_undo_redo_round_trip(&mut app, Procedural2dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None }), |app| app.snapshot().expect("snapshot").fixture.widgets.len(), before, before + 1);
    }

    #[test]
    fn two_instances_converge_disjoint_widget_moves() {
        let widgets: Vec<String> = app().snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::procedural2d::widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<Procedural2dPlayApp>, (Option<f64>, Option<f64>)>(
            "mem://procedural2d-convergence",
            Procedural2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
            Procedural2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
            move |app| {
                let layout = &app.snapshot().expect("snapshot").fixture.layout;
                (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
            },
        );
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::procedural2d::testkit::render;
        let mut app = app();
        assert!(render(&mut app, "procedural2d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️ContextMenuTests
    /// 🕹️ `context_menu` no longer has anything to dispatch a selection command WITH (`setSelection`
    /// is deleted — selection is the framework's `graph` interaction domain now) and `context_menu`
    /// itself carries no `InteractionView` to read it back even if it did (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, same discovered gap as `render`), so the
    /// destructive `delete-selection` row — conditioned on a real selection — never appears; this test
    /// now only pins the disclosure budget.
    #[test]
    fn context_menu_stays_within_disclosure_budget() {
        let mut app = app_with_registry();
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let items = app.context_menu(&request);
        assert!(items.len() <= 9, "top-level menu rows (leaves + groups + separator) must stay within disclosure budget, got {}", items.len());
        assert!(items.iter().all(|item| item.id != "delete-selection"), "no interaction data at context_menu time means delete-selection cannot appear");
    }
    //#endregion 🔖️ContextMenuTests

    //#region 🔖️PortTests
    #[test]
    fn export_drawing_out_returns_vector_media() {
        let mut app = app();
        let media = semio_framework_plugin::resolve_ready(app.export_media("drawing:out")).expect("export drawing:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
    }

    #[test]
    fn export_document_out_returns_flow_media() {
        let mut app = app();
        let media = semio_framework_plugin::resolve_ready(app.export_media("document:out")).expect("export document:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Flow });
        assert!(matches!(media.payload, semio_framework_plugin::MediaPayload::Structured { schema, .. } if schema == PROCEDURAL_2D_SCHEMA));
    }

    #[test]
    fn import_params_in_patches_matching_input_slider() {
        let mut app = app();
        app.dispatch_typed(Procedural2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).expect("add slider");
        let slider_id = app
            .snapshot()
            .expect("snapshot")
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::InputSlider { id, .. } => Some(id.clone()),
                _ => None,
            })
            .expect("just-added input slider");
        let media = semio_framework_plugin::Media {
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: serde_json::json!({ slider_id.clone(): 42.0 }).to_string() },
        };
        app.import_media("params:in", &media, &semio_framework_plugin::testkit::meta("local")).expect("import params");
        let value = app.snapshot().expect("snapshot").fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == &slider_id => Some(*value),
            _ => None,
        });
        assert_eq!(value, Some(42.0));
    }

    #[test]
    fn media_ports_declare_params_in_and_drawing_out() {
        let ports = <Procedural2dPlayApp as ArtifactEditor>::media_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let params_in = ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert_eq!(params_in.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let drawing_out = ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
        assert_eq!(drawing_out.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
        assert_eq!(drawing_out.kind_id.as_deref(), Some("2d.drawing"));
    }

    #[test]
    fn procedural2d_io_declares_the_params_and_drawing_ports() {
        let io = semio_framework::io::resolve_ready(procedural2d_io());
        assert_eq!(io.document_schema, "procedural.2d");
        let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert!(!params.required);
        let drawing = io.ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
        assert_eq!(drawing.kind_id.as_deref(), Some("2d.drawing"));
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
