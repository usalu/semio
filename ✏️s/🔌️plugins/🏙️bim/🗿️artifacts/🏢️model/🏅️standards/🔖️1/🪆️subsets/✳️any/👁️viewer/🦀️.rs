//! 👁️ BIM model viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `BimModelViewer` implements `ArtifactViewer`, never
//! `ArtifactEditor`/`ArtifactApp` — `ViewerApp<BimModelViewer>` (framework SDK) is the sole runtime adapter, so this file can never
//! structurally emit an artifact or draft mutation. MUST NOT import anything from the sibling editor module
//! (`policyViewerPurityBreaches`); everything both surfaces draw lives in `crate::render`.
//!
//! 🎥️ The viewer owns exactly one kind of write: the window configuration of the window a gesture came from (camera, projection preset,
//! storey visibility, plan storey). Every verb is a retained bounded job whose publication contract names the window-config lane only.

use crate::render::world::{ELEMENT_DOMAIN, ELEMENT_GRANULARITY};
use crate::viewer::bim::modes::view;
use crate::viewer::bim::modes::view::windows::{plan, world};
use crate::{ModelMutation, ModelSnapshot, BIM_MODEL_DIALECT, BIM_MODEL_DOCUMENT_SCHEMA};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_2d::compute::EngineHandles;
use semio_framework_artifact_reference::Dialect;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionDefinition, ActionDescriptor, ActionKind, AppOperationContext, ArtifactInstanceOperationOwnerHandle, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ArtifactViewer,
    CapabilityAudience, ComponentTree, ConfigView, DomainTopology, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HistoryView, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, MergeMode, NoConfig,
    NoConfigMutation, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, SelectionMethod, SelectionMode, SelectionSpec, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, TopologyNode, UiAssemblyResult, ViewEmit, ViewModel, Viewer, ViewerApp,
    WindowConfigMutation, WindowConfigSnapshot, WindowMeasure,
};
use semio_framework_ui_locale::{Label, LocalizedLabel};
use semio_framework_value::{DslValue, FromValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::{BTreeMap, HashMap};

#[path = "🗣️terminology/🦀️.rs"]
pub mod terminology;

//#region 🔖️Constants
pub const SET_CAMERA: &str = "setCamera";
pub const SET_PROJECTION: &str = "setProjection";
pub const SET_PROJECTION_PARAMETER: &str = "setProjectionParam";
pub const SET_STOREY_VISIBLE: &str = "setStoreyVisible";
pub const SET_PLAN_STOREY: &str = "setPlanStorey";
/// 🏷️ Every dispatchable verb of this viewer, exactly once. All of them write window configuration and nothing else.
const BIM_VIEW_TOOL_IDS: &[&str] = &[SET_CAMERA, SET_PROJECTION, SET_PROJECTION_PARAMETER, SET_STOREY_VISIBLE, SET_PLAN_STOREY];
const BIM_VIEW_PAYLOAD_SCHEMA: &str = "bim.model.view-command.v1";
const BIM_VIEW_RAW_BYTES: usize = 8_192;
const BIM_VIEW_CONTROLLER: &str = "s.bim.model@1/*#viewer";
const HOVER_CHANNEL: &str = "pointer";
//#endregion 🔖️Constants

//#region 🔖️Command
/// 👁️ The viewer's typed command channel: five window-configuration verbs. Row order is the binary variant ordinal: appending is safe,
/// reordering is a wire break.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum)]
pub enum BimViewCommand {
    /// 🎥️ The camera of the addressed window as canonical JSON: `{position,target,zoom,up?}` for a world window, `{x,y,zoom}` for a plan window.
    #[dsl(key = "setCamera")]
    SetCamera { camera: String },
    /// 📐️ One projection preset field (`orthographicView`, `axonometricVariant`, `perspectiveKind`, ...) of the addressed world window.
    #[dsl(key = "setProjection")]
    SetProjection { field: String, value: String },
    /// 📐️ One numeric projection parameter (`fov`, `obliqueAngle`, ...) of the addressed world window.
    #[dsl(key = "setProjectionParam")]
    SetProjectionParameter { param: String, value: f64 },
    /// 👁️ Shows or hides one storey in the addressed world window.
    #[dsl(key = "setStoreyVisible")]
    SetStoreyVisible { storey: String, visible: bool },
    /// 🗺️ Shows another storey in the addressed plan window.
    #[dsl(key = "setPlanStorey")]
    SetPlanStorey { storey: String },
}

impl Default for BimViewCommand {
    fn default() -> Self {
        Self::SetCamera { camera: String::new() }
    }
}

impl BimViewCommand {
    /// 🪪️ The manifest action id this command stands for.
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::SetCamera { .. } => SET_CAMERA,
            Self::SetProjection { .. } => SET_PROJECTION,
            Self::SetProjectionParameter { .. } => SET_PROJECTION_PARAMETER,
            Self::SetStoreyVisible { .. } => SET_STOREY_VISIBLE,
            Self::SetPlanStorey { .. } => SET_PLAN_STOREY,
        }
    }
}

impl protocol::OpBinary for BimViewCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = BIM_VIEW_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🎥️Configuration
fn refusal(code: &'static str, message: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message.to_string())
}

fn invalid(message: &str) -> Fault {
    refusal("app.command.invalid-payload", message)
}

fn window_kind(view_state: &ViewModel) -> Result<&str, Fault> {
    let id = view_state.window_id.as_deref().ok_or_else(|| refusal("bim.model.viewer.window-required", "a view change requires a concrete window"))?;
    view_state.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| refusal("bim.model.viewer.window-stale", "the addressed window is no longer open"))
}

fn wrong_window(command: &BimViewCommand) -> Fault {
    refusal("bim.model.viewer.window-kind", &format!("{} does not apply to this window", command.action_id()))
}

fn json_arguments(pairs: [(&str, DslValue); 2]) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::from_dsl_value(&DslValue::object(pairs.map(|(name, value)| (name.to_string(), value))))
}

fn parsed<T: FromValue>(text: &str, what: &str) -> Result<T, Fault> {
    let value = semio_framework_pack_json::from_json_str::<DslValue>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| invalid(&format!("the {what} is not a value: {error}")))?;
    T::from_value(value).map_err(|error| invalid(&format!("the {what} is malformed: {error}")))
}

/// 🎥️ Turns one view command into the write of the addressed window's configuration, or into an explicit refusal. This is the whole behaviour of
/// the viewer's command channel and it can only ever produce a window-config mutation.
pub fn window_config_mutation(command: &BimViewCommand, view_state: &ViewModel, window_config: Option<&WindowConfigSnapshot>) -> Result<WindowConfigMutation, Fault> {
    let kind = window_kind(view_state)?;
    let (is_world, is_plan) = (kind == world::WINDOW_KIND_ID, kind == plan::WINDOW_KIND_ID);
    let world_config = || world::config::from_snapshot(window_config);
    let plan_config = || plan::config::from_snapshot(window_config);
    match command {
        BimViewCommand::SetCamera { camera } if is_world => {
            let orbit: store::Viewport3dOrbit = parsed(camera, "camera pose")?;
            orbit.validate().map_err(|error| invalid(&error.to_string()))?;
            world::config::addressed(view_state, world_config().with_orbit(orbit))
        }
        BimViewCommand::SetCamera { camera } if is_plan => {
            let viewport: store::Viewport2d = parsed(camera, "camera pose")?;
            viewport.validate().map_err(|error| invalid(&error.to_string()))?;
            plan::config::addressed(view_state, plan_config().with_viewport(viewport))
        }
        BimViewCommand::SetProjection { field, value } if is_world => {
            let arguments = json_arguments([("field", DslValue::String(field.clone())), ("value", DslValue::String(value.clone()))]);
            let next = world_config().with_projection_action(SET_PROJECTION, &arguments).ok_or_else(|| invalid("the projection field or value is unknown"))?;
            world::config::addressed(view_state, next)
        }
        BimViewCommand::SetProjectionParameter { param, value } if is_world => {
            if !value.is_finite() {
                return Err(invalid("the projection parameter is not finite"));
            }
            let arguments = json_arguments([("param", DslValue::String(param.clone())), ("value", DslValue::float(*value))]);
            let next = world_config().with_projection_action(SET_PROJECTION_PARAMETER, &arguments).ok_or_else(|| invalid("the projection parameter is unknown"))?;
            world::config::addressed(view_state, next)
        }
        BimViewCommand::SetStoreyVisible { storey, visible } if is_world => world::config::addressed(view_state, world_config().with_storey_visible(storey, *visible)),
        BimViewCommand::SetPlanStorey { storey } if is_plan => plan::config::addressed(view_state, plan_config().with_storey(storey)),
        other => Err(wrong_window(other)),
    }
}

fn view_emit(command: &BimViewCommand, view_state: Option<&ViewModel>, window_config: Option<&WindowConfigSnapshot>) -> Result<Emit<ModelMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let view_state = view_state.ok_or_else(|| refusal("bim.model.viewer.window-required", "a view change requires a concrete window"))?;
    let mutation = window_config_mutation(command, view_state, window_config)?;
    let quiet = UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false };
    Ok(Emit { window_config_mutations: vec![mutation], ui_scope: if matches!(command, BimViewCommand::SetCamera { .. }) { quiet } else { UiDirtyScope::default() }, ..Default::default() })
}

fn text_argument(args: Option<&DslValue>, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| args.and_then(|args| args.get(name)).and_then(DslValue::as_str)).map(str::to_string)
}

/// 🪪️ The bridge from a dispatched action's args to the typed command. The camera pose is validated as one of the two pose shapes and
/// canonicalised to JSON text; the window kind then decides which shape it must be.
fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<BimViewCommand, Fault> {
    let required = |names: &[&str]| text_argument(args, names).ok_or_else(|| invalid(&format!("{action} lacks its {} argument", names[0])));
    Ok(match action {
        SET_CAMERA => {
            let camera = args.and_then(|args| args.get("camera")).ok_or_else(|| invalid("setCamera carries no camera pose"))?;
            let is_orbit = <store::Viewport3dOrbit as FromValue>::from_value(camera.clone()).is_ok_and(|pose| pose.validate().is_ok());
            let is_plan = <store::Viewport2d as FromValue>::from_value(camera.clone()).is_ok_and(|pose| pose.validate().is_ok());
            if !is_orbit && !is_plan {
                return Err(invalid("the camera pose is neither an orbit nor a plan viewport"));
            }
            BimViewCommand::SetCamera { camera: semio_framework_pack_json::to_json_string(camera) }
        }
        SET_PROJECTION => BimViewCommand::SetProjection { field: required(&["field"])?, value: required(&["value"])? },
        SET_PROJECTION_PARAMETER => {
            let value = args.and_then(|args| args.get("value")).and_then(DslValue::as_f64).ok_or_else(|| invalid("setProjectionParam carries no numeric value"))?;
            BimViewCommand::SetProjectionParameter { param: required(&["param"])?, value }
        }
        SET_STOREY_VISIBLE => {
            let visible = args.and_then(|args| args.get("visible").or_else(|| args.get("value"))).and_then(DslValue::as_bool).ok_or_else(|| invalid("setStoreyVisible carries no visibility"))?;
            BimViewCommand::SetStoreyVisible { storey: required(&["storey"])?, visible }
        }
        SET_PLAN_STOREY => BimViewCommand::SetPlanStorey { storey: required(&["storey", "value"])? },
        other => return Err(refusal("app.command.unsupported", &format!("the BIM viewer has no command for action '{other}'"))),
    })
}
//#endregion 🎥️Configuration

//#region 🧵️RetainedCommands
fn bim_view_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BIM_VIEW_RAW_BYTES, 1, 1, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "The retained work contract takes an optional extent callback, and one window configuration write is always one work item.")]
fn bim_view_extent(_command: &BimViewCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
#[allow(clippy::needless_pass_by_value)]
fn bim_view_reduce(
    command: &BimViewCommand,
    _snapshot: &ModelSnapshot,
    _config: &NoConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<BimModelViewer>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<ModelMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    view_emit(command, context.and_then(|context| context.view_state.as_ref()), context.and_then(|context| context.window_config.as_ref()))
}

struct BimViewCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BimViewCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BIM_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for BimViewCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<BimModelViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<BimModelViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        BIM_VIEW_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        bim_view_contract()
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
        if input.declared_bytes() > BIM_VIEW_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("the bounded BIM view command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BimViewCommandJobFactory {
    type Owner = ViewerApp<BimModelViewer>;
    const TOOL_IDS: &'static [&'static str] = BIM_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;
    /// 🔒️ WINDOW-CONFIG ONLY: naming the artifact lane here would be rejected against a viewer's own emit.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: SET_CAMERA, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: SET_PROJECTION, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: SET_PROJECTION_PARAMETER, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: SET_STOREY_VISIBLE, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: SET_PLAN_STOREY, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 🔖️Viewer
/// 👁️ Reads the framework-owned selection and hover of the element domain.
fn marks_of(interaction: &semio_framework_plugin::app::InteractionView<'_>) -> world::Marks {
    world::Marks { selected: interaction.selection(ELEMENT_DOMAIN).ids.clone(), hovered: interaction.hover(ELEMENT_DOMAIN, HOVER_CHANNEL).ids.clone() }
}

fn render_body(body_key: &str, snapshot: &ModelSnapshot, cfg: &ConfigView<'_, NoConfig>, marks: &world::Marks) -> UiAssemblyResult<ComponentTree> {
    let node = match body_key {
        world::BODY_KEY => world::render(snapshot, &world::config::current(cfg), marks),
        plan::BODY_KEY => plan::render(snapshot, &plan::config::current(cfg), &marks.selected),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("bim.viewer.ui.capacity", "viewer label admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

#[derive(Default, Clone, Copy)]
pub struct BimModelViewer;

impl ArtifactViewer for BimModelViewer {
    type Snapshot = ModelSnapshot;
    type Mutation = ModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = BimViewCommand;

    const DIALECT: Dialect = BIM_MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own composition fields; without it a live envelope load faults before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| refusal("bim.model.viewer.child-projection", &error.to_string()))
    }

    fn initial_snapshot() -> ModelSnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
    }

    /// 🛂️ A viewer declares every nontrivial store owner explicitly (the trait defaults fail closed): the document store is the bounded pair
    /// the editor also uses, every other lane is `No*`. Without them the retained window-config sessions never retire at close.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
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

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        command_from_action(action, args)
    }

    /// 👁️ Structurally read-only. `ViewEmit` has no window-config lane at all, so every verb of this viewer travels the retained route
    /// ([`bim_view_reduce`]); this arm exists so that route's absence is a loud refusal rather than a silently dropped gesture.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Err(refusal("bim.model.viewer.retained-route-required", "a viewer window configuration write must reach the retained reducer"))
    }

    /// 🕹️ Every drawn element is a pickable target of the `elements` domain; without it the framework prunes every world pick.
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
        let ordered = crate::render::element_ids(doc.snapshot).into_iter().map(|id| TopologyNode { id, granularity: ELEMENT_GRANULARITY.into(), parent: None }).collect();
        Ok(InteractionTopology { domains: BTreeMap::from([(ELEMENT_DOMAIN.to_string(), DomainTopology { ordered })]) })
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<world::config::BimViewerWorldWindowConfigOwner>()?;
        registry.register::<plan::config::BimViewerPlanWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(BimViewCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        if !BIM_VIEW_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(refusal("app.command.tool-mismatch", "the BIM view command does not match its exact registered tool"));
        }
        let tool_id = request.command.action_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, bim_view_reduce, bim_view_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
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
            BimViewCommand::action_id,
            BIM_VIEW_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<BimModelViewer>,
        owner_file: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.bim.model@1/*#viewer",
        artifact_schema: "s.bim.model@1",
        factory: "BimViewCommandJobFactory",
        factory_type: BimViewCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 1, 1, 16_384, 7_500),
        tools: ["setCamera", "setProjection", "setProjectionParam", "setStoreyVisible", "setPlanStorey"]
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, doc.snapshot, cfg, &world::Marks::default())
    }

    fn render_with_request_context(
        _owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, doc.snapshot, cfg, &marks_of(interaction))
    }

    /// 🎚️ The chrome of the addressed window: the projection tree and storey toggles of a world window, the storey picker of a plan window.
    fn window_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Ok(kind) = window_kind(view_state) else { return HashMap::new() };
        let labels = terminology::bim_viewer_labels(view_state);
        let measures = match kind {
            world::WINDOW_KIND_ID => world::measures(doc.snapshot, &world::config::current(cfg), labels, viewer_action),
            plan::WINDOW_KIND_ID => plan::measures(doc.snapshot, &plan::config::current(cfg), labels, viewer_action),
            _ => return HashMap::new(),
        };
        HashMap::from([(id.to_string(), measures)])
    }
}

/// 🎚️ A window chrome control bound to this viewer's own controller.
pub fn viewer_action(action: &str, args: Option<DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: BIM_VIEW_CONTROLLER.into(), action: action.into(), args }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
fn view_action(id: &str, label: LocalizedLabel, args: Vec<ActionArgDef>) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, ActionKind::View).with_args(args).in_palette(false)
}

/// 🧱️ The viewer manifest: the `view` mode with its world and plan windows, the five window-configuration verbs (chrome audience), and the
/// read-only `elements` interaction domain bound to the world window.
pub fn create_bim_viewer() -> semio_framework_plugin::AppDefinition {
    let mut builder = Viewer::builder(BIM_MODEL_DIALECT)
        .document(["semio", "bim"])
        .icon_id("building")
        .mode_def(view::definition())
        .default_mode_id(view::BIM_VIEW_MODE_VIEW)
        .window_kind_def(world::definition())
        .window_kind_def(plan::definition())
        .default_layout(view::layout())
        .action_with(view_action(SET_CAMERA, LocalizedLabel::native("Set Camera", "Kamera festlegen"), vec![ActionArgDef::text("camera", LocalizedLabel::native("Camera pose", "Kamerapose")).required()]))
        .action_with(view_action(SET_PROJECTION, LocalizedLabel::native("Set Projection", "Projektion festlegen"), Vec::new()))
        .action_with(view_action(SET_PROJECTION_PARAMETER, LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), Vec::new()))
        .action_with(view_action(SET_STOREY_VISIBLE, LocalizedLabel::native("Show or Hide Storey", "Geschoss ein- oder ausblenden"), vec![ActionArgDef::text("storey", LocalizedLabel::native("Storey", "Geschoss")).required()]))
        .action_with(view_action(SET_PLAN_STOREY, LocalizedLabel::native("Show Storey in Plan", "Geschoss im Grundriss zeigen"), vec![ActionArgDef::text("storey", LocalizedLabel::native("Storey", "Geschoss")).required()]));
    for tool_id in BIM_VIEW_TOOL_IDS {
        builder = builder.action_audience(tool_id, CapabilityAudience::Chrome).action_interactive_job(tool_id, InteractiveJobClassification::Migrated);
    }
    builder
        .interaction(InteractionDefinition {
            id: ELEMENT_DOMAIN.into(),
            label: LocalizedLabel::native("Elements", "Bauteile"),
            granularities: vec![GranularityDefinition { id: ELEMENT_GRANULARITY.into(), label: LocalizedLabel::native("Element", "Bauteil"), icon_id: "box".into() }],
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                transitive: false,
                broadcast: true,
            },
        })
        .window_kind_interactions(world::WINDOW_KIND_ID, vec![InteractionRef::new(ELEMENT_DOMAIN)])
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
