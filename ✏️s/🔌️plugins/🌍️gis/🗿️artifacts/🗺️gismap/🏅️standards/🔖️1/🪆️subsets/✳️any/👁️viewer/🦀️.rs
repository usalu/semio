//! 👁️ GIS map viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `GisMapViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<GisMapViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::schema::default_document;
use crate::viewer::gismap::modes::view;
use crate::viewer::gismap::modes::view::windows::map;
use crate::{GisMapSnapshot, GISMAP_DIALECT, GIS_MAP_SCHEMA};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ArtifactViewer, ConfigView, Dialect, Emit, Fault, FaultCode, FaultOrigin, HistoryView,
    InteractiveJobClassification, NoConfig, NoConfigMutation, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, ViewEmit, Viewer, ViewerApp,
};
use store::EngineHandles;

//#region 🏷️ActionIds
/// 🏷️ Every dispatchable verb of this VIEWER, exactly once: the map window's camera.
///
/// 🔒️ The whole list must stay config-only. A viewer's publication contracts may never name
/// `ArtifactToolPublicationLane::Artifact`, and `setCamera` names `WindowConfig` — the lane that writes
/// ONE window instance's own retained state and nothing else.
const GIS_MAP_VIEW_TOOL_IDS: &[&str] = &[map::SET_CAMERA_ACTION_ID];
const GIS_MAP_VIEW_PAYLOAD_SCHEMA: &str = "gis.map.view-command.v1";
/// 🎒️ Real bound for one viewer gesture's wire payload: a camera is three doubles, so 8 KiB is a real
/// ceiling rather than a rubber stamp.
const GIS_MAP_VIEW_RAW_BYTES: usize = 8_192;
/// 🧮️ One window-config write per gesture.
const GIS_MAP_VIEW_WORK_ITEMS: usize = 1;
//#endregion 🏷️ActionIds

//#region 🔖️Command
/// 👁️ The viewer's typed command channel. It has exactly one row: the react `TiledMapHost` dispatches
/// `setCamera` after every pan/zoom of ANY map window, read-only or not, and a window kind that does not
/// declare and reduce it drops every gesture (`dropped action "setCamera" … no window kind declares it`).
///
/// 🔒️ Row order is the binary variant ordinal: appending is safe, reordering is a wire break.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps)]
pub enum GisMapViewCommand {
    /// 🧭️ The camera as the canonical `{x,y,zoom}` JSON the host sends. One text field because a
    /// `dsl::DslOps` variant binds scalars only — and because that string IS what
    /// `TiledMapScene::camera_json` consumes.
    #[dsl(key = "setCamera")]
    SetCamera { camera: String },
}

impl Default for GisMapViewCommand {
    /// 🌱️ An empty camera, which `camera_emit` refuses explicitly rather than applying — inventing a
    /// camera here would let a malformed dispatch write a camera nobody asked for.
    fn default() -> Self {
        Self::SetCamera { camera: String::new() }
    }
}

impl GisMapViewCommand {
    /// 🪪️ The manifest action id this command stands for.
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::SetCamera { .. } => map::SET_CAMERA_ACTION_ID,
        }
    }
}

impl protocol::OpBinary for GisMapViewCommand {
    /// 🧵️ The exact manifest ids this typed command schema owns — the left-hand side of
    /// `validate_tool_job_rows`' set equality against the `bounded_first_step_tool_proofs!` rows.
    const TOOL_JOB_IDS: &'static [&'static str] = GIS_MAP_VIEW_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🧭️Camera
/// 🧭️ Turns one `setCamera` gesture into the addressed window-config write, or into an explicit
/// refusal. This is the WHOLE behavior of the viewer's command channel.
///
/// 🔒️ The emitted `Emit` carries `window_config_mutations` and NOTHING else — no artifact mutation, no
/// config mutation, no effect — the runtime half of the read-only guarantee whose compile-time half
/// is `ViewEmit`.
fn camera_emit(command: &GisMapViewCommand, view_state: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<crate::op::GisMapMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let GisMapViewCommand::SetCamera { camera } = command;
    let view = view_state.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("gis.map.viewer.window-required"), "a camera change requires a concrete map window"))?;
    if camera.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "setCamera carries no {x,y,zoom} camera"));
    }
    let value = dsl::json::from_json_str::<dsl::DslValue>(camera).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera is not a value: {error}")))?;
    let camera = <map::config::GisMapViewerCamera as dsl::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera is malformed: {error}")))?;
    if !camera.is_valid() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the camera is not finite, or its zoom is not positive"));
    }
    let mutation = map::config::GisMapViewerWindowConfigMutation::SetCamera(map::config::SetCamera { camera });
    Ok(Emit {
        window_config_mutations: vec![map::config::addressed(view, mutation)?],
        description: Some("Set camera".into()),
        ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        coalesce_key: Some(format!("gis.map.viewer.camera:{}", view.window_id.as_deref().unwrap_or_default())),
        ..Default::default()
    })
}

/// 🪪️ The bridge from a dispatched action's args to the typed command. The host's `camera` object is
/// validated and canonicalized to the exact JSON string `TiledMapScene::camera_json` consumes, so a
/// malformed gesture is refused at `camera_emit` rather than silently ignored.
fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<GisMapViewCommand, Fault> {
    if action != map::SET_CAMERA_ACTION_ID {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the gis map viewer has no command for action '{action}'")));
    }
    let camera = args
        .and_then(|args| args.get("camera"))
        .and_then(|value| {
            let value = match value {
                dsl::DslValue::String(text) => dsl::json::from_json_str::<dsl::DslValue>(text).ok()?,
                other => other.clone(),
            };
            let camera = <map::config::GisMapViewerCamera as dsl::FromValue>::from_value(value).ok()?;
            camera.is_valid().then(|| camera.scene_camera_json())
        })
        .unwrap_or_default();
    Ok(GisMapViewCommand::SetCamera { camera })
}
//#endregion 🧭️Camera

//#region 🧵️RetainedCommands
fn gis_map_view_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GIS_MAP_VIEW_RAW_BYTES, GIS_MAP_VIEW_WORK_ITEMS, 1, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "The retained work contract takes an optional extent callback, and one camera is always one work item.")]
fn gis_map_view_extent(_command: &GisMapViewCommand, _snapshot: &GisMapSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 👁️ The one reducer the viewer's single tool runs. A `Migrated` verb without an exact app-owned
/// reducer is refused with `interactive-job.missing-owned-reducer`, so this function existing is
/// precisely what makes the read-only window's pan survive.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn gis_map_view_reduce(
    command: &GisMapViewCommand,
    _snapshot: &GisMapSnapshot,
    _config: &NoConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<GisMapViewer>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<crate::op::GisMapMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    camera_emit(command, context.and_then(|context| context.view_state.as_ref()))
}

struct GisMapViewCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl GisMapViewCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GIS_MAP_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for GisMapViewCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<GisMapViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<GisMapViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        GIS_MAP_VIEW_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        gis_map_view_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GIS_MAP_VIEW_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("the bounded gis map view command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for GisMapViewCommandJobFactory {
    type Owner = ViewerApp<GisMapViewer>;
    const TOOL_IDS: &'static [&'static str] = GIS_MAP_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;
    /// 🔒️ WINDOW-CONFIG ONLY: naming `ArtifactToolPublicationLane::Artifact` here would be rejected
    /// against a viewer's own emit.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: map::SET_CAMERA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::WindowConfig] }];
}
//#endregion 🧵️RetainedCommands

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct GisMapViewer;

impl ArtifactViewer for GisMapViewer {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = GisMapSnapshot;
    type Mutation = crate::op::GisMapMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = GisMapViewCommand;

    const DIALECT: Dialect = GISMAP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;

    /// 🔐️ The document-store owner catalogue, identical to the sibling editor's: a viewer owns the
    /// same snapshot envelope and must allocate it the same way.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::gis_map_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    /// 🧹️ The four bounded disposers `VcsArtifactApp`'s close ladder drives. The trait default is
    /// `None`, and `None` faults `interactive-job.close-owned-disposer-missing` the first time this
    /// surface is closed, which is what the guest codec resolver does to every app it does not return.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn initial_snapshot() -> GisMapSnapshot {
        default_document()
    }

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::from(format!("gis map child projection failed: {error}")))
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
        crate::genesis_gis_map_child_pack(snapshot, slot, child_id)
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        command_from_action(action, args)
    }

    /// 👁️ Structurally read-only. `ViewEmit` has no window-config lane at all, so the ONE verb this viewer
    /// owns cannot be served here — `setCamera` is `InteractiveJobClassification::Migrated` and travels the
    /// RETAINED route ([`gis_map_view_reduce`]). This arm makes that route's absence a loud refusal
    /// rather than a silently dropped pan.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Err(Fault::new(FaultOrigin::App, FaultCode::new("gis.map.viewer.retained-route-required"), "a viewer camera is a window-config write, which ViewEmit cannot carry — it must reach the retained reducer"))
    }

    /// 🧭️ The map window's own retained camera — one bounded window-config store, keyed by window
    /// INSTANCE, so two open read-only panes never share a camera. Authored under `👁️viewer`, never
    /// imported from the editor's twin.
    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<map::config::GisMapViewerWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(GisMapViewCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        if !GIS_MAP_VIEW_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("gis.map.viewer.retained.tool-mismatch"), "the gis map view command does not match its exact registered tool"));
        }
        let tool_id = request.command.action_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, gis_map_view_reduce, gis_map_view_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
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
            GisMapViewCommand::action_id,
            GIS_MAP_VIEW_RAW_BYTES,
            GIS_MAP_VIEW_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<GisMapViewer>,
        owner_file: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.gis.gismap@1/*#viewer",
        artifact_schema: "gis.map",
        factory: "GisMapViewCommandJobFactory",
        factory_type: GisMapViewCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 1, 1, 16_384, 7_500),
        tools: ["setCamera"]
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            map::BODY_KEY => map::render(doc.snapshot, map::config::current(cfg)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_gismap_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(GISMAP_DIALECT).document(["semio", "gis", "2d"]).icon_id("gis2d").mode_def(view::definition()).default_mode_id(view::GIS_MAP_VIEW_MODE_VIEW).window_kind_def(map::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
