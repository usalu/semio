//! 🧱️ Generation3d editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::{artifact_kind, Generation3dSnapshot, GENERATION_3D_SCHEMA};
use crate::editor::generation3d::commands::{
    add_generation, add_widget, delete_selection, flow_eval_tick, graph_pointer_down, move_media_node, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget,
    rename_generation, reorganize, rotate_selection, scale_selection, select_generation, set_active_example, set_active_utility, set_camera, set_locale, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun,
    translate_selection, update_generation_values, world_pointer_down,
};
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::editor::generation3d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::editor::generation3d::modes::{edit, generate};
use crate::editor::generation3d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::generation3d::terminology::generation3d_labels;
use flow::FlowEvalSession;
// 🚧️ SDK note (ticket 26/08/16 contract §2.1/§2.4): `ArtifactEditor`/`Editor`/`Dialect` are curated at
// `semio_framework_plugin`'s crate root as of W0-F/W2-FIX — imported bare here, no `app::` prefix
// needed (unlike the earlier cad pilot, written before that gap closed). `app::InteractionView` is a
// separate, still-uncurated gap (unrelated to this ticket) — kept qualified.
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    app::InteractionView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, CommandDefinition, ConfigView, Dialect, DomainTopology, DraftView, Editor, EditorApp, Effect, Emit, ExampleSource, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
    TopologyNode, UtilityDefinition, WindowMeasure,
};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_APP_ID: &str = "procedural3d-play";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub fn generation3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: GENERATION_3D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
}

fn categorized_action(id: &str, label: LocalizedLabel, kind: ActionKind, category: &str) -> ActionDefinition {
    semio_framework::io::resolve_ready(ActionDefinition::bounded_catalog(id, label, kind).with_category(category))
}

/// 🧵️ Classifies the internal flow continuation as backed by its bounded first-step factory.
fn migrated_command(mut definition: CommandDefinition) -> CommandDefinition {
    definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    definition
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Generation3dPlayApp::Command` — the SOLE dispatch surface for generation3d's own behavior,
    /// covering EVERY declared action. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Generation3dCommand for Generation3dSnapshot, Generation3dMutation, Generation3dConfig, Generation3dConfigMutation, ctx = FlowEvalSession {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "nodeGraphEdit" as "graph-edit" => node_graph_edit::NodeGraphEdit,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "moveMediaNode" as "move-node" => move_media_node::MoveMediaNode,
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "patchFlowWidgets" as "patch-flow-widgets" => patch_flow_widgets::PatchFlowWidgets,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "nodeGraphViewport" as "viewport" => node_graph_viewport::NodeGraphViewport,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,
        "graphPointerDown" as "graph-pointer-down" => graph_pointer_down::GraphPointerDown,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "setShowMode" as "show-mode" => set_show_mode::SetShowMode,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setCamera" as "camera" => set_camera::SetCamera,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick}
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️Generation3dPlayApp
/// 🧪️ Unit struct apart from `eval_session`: every former runtime field lives in [`Generation3dConfig`],
/// written through [`Generation3dConfigMutation`]s.
#[derive(Default)]
pub struct Generation3dPlayApp;

/// 🎥️ Parses the flow-graph camera out of `command_from_action`'s JSON args — either a nested
/// `{camera: {...}}` object or flat `x`/`y`/`zoom` keys.
fn parse_flow_camera_json(args: &dsl::DslValue) -> flow::CameraJson {
    if let Some(camera) = args.get("camera") {
        // 🌉️ `flow::CameraJson` derives `ToValue`/`FromValue` alongside its `Serialize`/
        // `Deserialize` (see `🌊️flow/🗿️artifact/🦀️.rs`), so this decodes straight off the
        // first-party bridge — no `serde_json` involved.
        if let Ok(parsed) = dsl::from_dsl_value::<flow::CameraJson>(camera.clone()) {
            return parsed;
        }
    }
    flow::CameraJson { x: args.get("x").and_then(dsl::DslValue::as_f64).unwrap_or(0.0), y: args.get("y").and_then(dsl::DslValue::as_f64).unwrap_or(0.0), zoom: args.get("zoom").and_then(dsl::DslValue::as_f64).unwrap_or(1.0) }
}

/// 🎥️ Parses the 3D preview camera out of `command_from_action`'s JSON args; falls back to the default
/// camera on any malformed/missing `camera` object.
fn parse_preview_camera_json(args: &dsl::DslValue) -> crate::editor::generation3d::config::Generation3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = <crate::editor::generation3d::config::Generation3dPreviewCamera as protocol::FromValue>::from_value(camera.clone()) {
            return parsed;
        }
    }
    crate::editor::generation3d::config::Generation3dPreviewCamera::default()
}

/// 🕸️ Every node's visible port ids (`{nodeId}@{portId}`), read from the SAME
/// `fixture_to_workflow` projection the node-graph window paints — so an interaction target and a
/// graph pick can never drift apart.
fn generation3d_port_ids_by_node(fixture: &flow::FlowFixture) -> std::collections::BTreeMap<String, Vec<String>> {
    let host = crate::artifacts::generation3d::schema::host_from_fixture(fixture);
    let (graph_nodes, _) = crate::artifacts::generation3d::schema::fixture_to_workflow(&host.dag.fixture);
    graph_nodes.into_iter().map(|node| (node.id, node.inputs.into_iter().chain(node.outputs).map(|port| port.id).collect())).collect()
}

/// 🧱️ Every window body of the generation3d editor, rendered against one already-resolved set of
/// `graph` marks. Shared by `render` (marks-free) and `render_with_request_context` (live marks) so
/// there is exactly one body-key match in the app.
fn generation3d_render_body(body_key: &str, document: &Generation3dSnapshot, config: &Generation3dConfig, marks: &PreviewInteractionMarks) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = generation3d_labels(config);
    let active_utility = config.active_utility_id.as_str();
    let session = FlowEvalSession::new();
    let node = match body_key {
        flow_window::GENERATION_3D_PLAY_BODY_MAIN => flow_window::render(document, config, &session, marks),
        edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, &session, active_utility, marks),
        generations::GENERATION_3D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::default()),
        form::GENERATION_3D_PLAY_BODY_GENERATE_FORM => form::render(&document.fixture, &document.generation, labels),
        generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(&document.fixture, &document.generation, config, labels, active_utility, marks),
        document_panel::GENERATION_3D_PLAY_BODY_DOCUMENT => document_panel::render(&document.fixture, labels),
        catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
        inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION => inspection_panel::render(&document.fixture, &marks.graph_selection_ids(), labels),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

//#region 🧵️RetainedCommands
/// 🧾️ Every gen3d tool id, in `Generation3dCommand` declaration order — a bijection with
/// `Generation3dCommand`'s 29 rows (asserted by `retained_route_dispositions_are_exact_and_exhaustive`
/// below) and with `Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS`.
const GENERATION3D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "nodeGraphEdit",
    "deleteSelection",
    "removeWidget",
    "moveMediaNode",
    "addWidget",
    "patchFlowWidgets",
    "reorganize",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "addGeneration",
    "removeGeneration",
    "renameGeneration",
    "updateGenerationValues",
    "nodeGraphViewport",
    "worldPointerDown",
    "graphPointerDown",
    "setLodMode",
    "setShowMode",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setCamera",
    "selectGeneration",
    "setActiveUtility",
    "setLocale",
    "flowEvalTick",
];
const GENERATION3D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.3d.tool-command.v1";
const GENERATION3D_RETAINED_RAW_BYTES: usize = 8_192;
/// 🎒️ Real bound for one Artifact-lane edit: the 8 built-in example DSLs top out around 1.6 KB of text
/// (`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`), and `setActiveExample`'s full-fixture replacement is the
/// single largest Artifact mutation any of the 29 tools ever emits — 64 KiB stays a real ceiling, not a
/// rubber stamp, for every example plus ordinary interactive graph edits.
const GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
/// 🎒️ Real bound for one Config-lane edit: `flowEvalTick`'s `SetPreviewEval` carries the largest config
/// payload (`FlowEvalSession::eval_json()`, a per-node scalar/string summary — full mesh geometry leaves
/// through `export_media("geometry:out", ..)` instead, never through config) — 256 KiB comfortably covers
/// every one of the 8 examples' evaluated graphs (a handful to a few dozen nodes) without being unbounded.
const GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 262_144;

fn generation3d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_RETAINED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 🕹️ `nodeGraphEdit`/`deleteSelection`/`{translate,rotate,scale}Selection` read real `graph` selection
/// directly off `protocol::InteractionState` (the raw, crate-public half of what `app::InteractionView`
/// wraps) — plugin code cannot construct an `InteractionView` itself (`state`/`hover`/`peers` are
/// `pub(crate)` to `semio_framework_plugin`), so this is the only way a retained-command-job reducer can
/// preserve the same real-selection behavior `Generation3dPlayApp::handle` gives those five commands above.
fn generation3d_retained_reduce(
    command: &Generation3dCommand,
    snapshot: &Generation3dSnapshot,
    config: &Generation3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>, Fault> {
    if !GENERATION3D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("generation3d-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    let mut session = FlowEvalSession::new();
    let selected = || interaction.selection.get("graph").map(|selection| selection.ids.clone()).unwrap_or_default();
    match command {
        Generation3dCommand::NodeGraphEdit(payload) => Ok(node_graph_edit::apply_selected(payload, &doc, &selected())),
        Generation3dCommand::DeleteSelection(_payload) => Ok(delete_selection::apply_selected(&doc, &selected())),
        Generation3dCommand::TranslateSelection(payload) => Ok(translate_selection::apply_selected(payload, &doc, &selected())),
        Generation3dCommand::RotateSelection(payload) => Ok(rotate_selection::apply_selected(payload, &doc, &selected())),
        Generation3dCommand::ScaleSelection(payload) => Ok(scale_selection::apply_selected(payload, &doc, &selected())),
        _ => command.dispatch(&doc, &cfg, &mut session),
    }
}

struct Generation3dBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Generation3dBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_bounded_contract()
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
        if input.declared_bytes() > GENERATION3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dBoundedCommandJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFlowWidgets", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "graphPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShowMode", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️ArtifactStorePreparation
/// 🧬️ Builds one `protocol::Edit<M>` for either lane's `advance()` — the two lanes differ only in `M`
/// and their id prefix, so this one generic helper replaces two copies of the same ~20-line literal.
fn generation3d_next_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
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

fn generation3d_artifact_mutation_retained_bytes(mutation: &Generation3dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-artifact-mutation-encode-failed".to_string())
}

fn admit_generation3d_artifact_mutation(mutation: &Generation3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation3d-artifact-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_generation3d_artifact(base: &Generation3dSnapshot, mutation: Generation3dMutation) -> Result<(Generation3dSnapshot, Vec<Generation3dMutation>, Generation3dMutation), String> {
    admit_generation3d_artifact_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base);
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let post = protocol::MutationDiff::apply(&diff, base).map_err(|_| "generation3d-artifact-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

struct Generation3dArtifactStorePreparationFactory;

struct Generation3dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Generation3dSnapshot>>,
    mutation: Option<Generation3dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &Generation3dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("generation3d-artifact-lane-or-description-envelope".into());
        }
        admit_generation3d_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation>> {
        let retained_bytes = generation3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation3d-artifact-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation3d_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-artifact-authority-missing".to_string())?;
        let edit = generation3d_next_edit("generation3d-artifact-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
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
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation3d-artifact-base-retirement-rejected".into());
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
fn generation3d_config_mutation_retained_bytes(mutation: &Generation3dConfigMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-config-mutation-encode-failed".to_string())
}

fn admit_generation3d_config_mutation(mutation: &Generation3dConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_config_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("generation3d-config-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_generation3d_config(base: &Generation3dConfig, mutation: Generation3dConfigMutation) -> Result<(Generation3dConfig, Vec<Generation3dConfigMutation>, Generation3dConfigMutation), String> {
    admit_generation3d_config_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base);
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let post = protocol::MutationDiff::apply(&diff, base).map_err(|_| "generation3d-config-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

struct Generation3dConfigPreparationFactory;

struct Generation3dConfigPreparation {
    base: Option<store::SnapshotRead<Generation3dConfig>>,
    mutation: Option<Generation3dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparationFactory {
    fn preflight(&self, mutation: &Generation3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("generation3d-config-lane-or-description-envelope".into());
        }
        admit_generation3d_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Generation3dConfig, Generation3dConfigMutation>> {
        let retained_bytes = generation3d_config_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Generation3dConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Generation3dConfig, Generation3dConfigMutation> for Generation3dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "generation3d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "generation3d-config-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_generation3d_config(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "generation3d-config-authority-missing".to_string())?;
        let edit = generation3d_next_edit("generation3d-config-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Generation3dConfig, Generation3dConfigMutation>> {
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
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("generation3d-config-base-retirement-rejected".into());
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
//#endregion 📬️ConfigStorePreparation

impl ArtifactEditor for Generation3dPlayApp {
    type Snapshot = Generation3dSnapshot;
    type Mutation = Generation3dMutation;
    type Config = Generation3dConfig;
    type ConfigMutation = Generation3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::generation3d::presence::Generation3dPresence;
    type PresenceMutation = crate::editor::generation3d::presence::Generation3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Generation3dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::generation3d::spr::generation3d_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::generation3d::spr::generation3d_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::generation3d::spr::generation3d_document_store_initialization_job(envelope, operation, generation))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        crate::artifacts::generation3d::spr::generation3d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Generation3d atomic publication authority is absent or stale"))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    const DIALECT: Dialect = crate::artifacts::generation3d::GENERATION3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation3dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Generation3dConfigPreparationFactory))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation3dBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("generation3d-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, generation3d_retained_reduce, generation3d_bounded_extent));
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
            Generation3dCommand::command_id,
            GENERATION3D_RETAINED_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        document_schema: "generation.3d",
        factory: "Generation3dBoundedCommandJobFactory",
        factory_type: Generation3dBoundedCommandJobFactory,
        tools: {
            "setActiveExample" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "nodeGraphEdit" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "deleteSelection" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "removeWidget" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "moveMediaNode" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "addWidget" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "patchFlowWidgets" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "reorganize" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "translateSelection" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "rotateSelection" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "scaleSelection" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "addGeneration" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "removeGeneration" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "renameGeneration" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "updateGenerationValues" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "nodeGraphViewport" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "worldPointerDown" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "graphPointerDown" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setLodMode" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setShowMode" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "toggleSun" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setSunAzimuth" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setSunElevation" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setSunIntensity" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setCamera" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "selectGeneration" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setActiveUtility" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setLocale" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "flowEvalTick" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::generation3d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Generation3dSnapshot {
        crate::artifacts::generation3d::schema::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_framework::io::resolve_ready(generation3d_io()))
    }

    /// 🎞️ `geometry:out` plus the inherited `document:out` default, replicated inline (overriding
    /// `export_media` shadows the trait's provided body for every port on this app).
    fn export_media(port: &str, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "geometry:out" => {
                let mesh = export_mesh_from_document(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                    payload: semio_framework_plugin::MediaPayload::Structured { schema: "3d.mesh".into(), json: dsl::json::to_json_string(&mesh) },
                })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media { media_type, payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"` — patches matching `InputSlider` widgets from a `{widgetId: number}` JSON
    /// object; unmatched keys/non-slider widgets are silently ignored.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "params:in" => {
                let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "params:in importer only accepts a Structured JSON object payload".into()));
                };
                let parsed = dsl::json::parse(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let object = parsed.as_object().cloned().ok_or_else(|| MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()))?;
                let fixture = &doc.snapshot.fixture;
                let mut operations = Vec::new();
                for (target_id, value) in object.iter() {
                    let Some(number) = value.as_f64() else { continue };
                    let Some((_index, widget)) = fixture.widgets.iter().enumerate().find(|(_, widget)| crate::artifacts::generation3d::widget_id(widget) == target_id) else { continue };
                    if let flow::Widget::InputSlider { id, label, min, max, step, .. } = widget {
                        operations.push(Generation3dMutation::UpdateWidget(crate::artifacts::generation3d::schema::mutations::update_widget::UpdateWidget {
                            widget: flow::Widget::InputSlider { id: id.clone(), label: label.clone(), value: number, min: *min, max: *max, step: *step },
                        }));
                    }
                }
                Ok(Emit::mutations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Generation3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Generation3dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(dsl::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "setActiveExample" => Ok(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(|value| dsl::json::to_json_string(value))).unwrap_or_else(|| "[]".into()),
            })),
            "deleteSelection" => Ok(Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "removeWidget" => Ok(Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "moveMediaNode" => Ok(Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "addWidget" => Ok(Generation3dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), x: f64_arg(&["x"]), y: f64_arg(&["y"]) })),
            "patchFlowWidgets" => Ok(Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets {
                widget_ids: {
                    let mut ids = string_list("widgetIds");
                    if ids.is_empty() {
                        ids = string_list("widget_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: f64_arg(&["value"]),
            })),
            "reorganize" => Ok(Generation3dCommand::Reorganize(reorganize::Reorganize {})),
            "translateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids, dx: f64_arg(&["dx"]).unwrap_or(0.0), dy: f64_arg(&["dy"]).unwrap_or(0.0), dz: f64_arg(&["dz"]).unwrap_or(0.0) }))
            }
            "rotateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::RotateSelection(rotate_selection::RotateSelection {
                    node_ids,
                    ax: f64_arg(&["ax"]).unwrap_or(0.0),
                    ay: f64_arg(&["ay"]).unwrap_or(0.0),
                    az: f64_arg(&["az"]).unwrap_or(0.0),
                    angle: f64_arg(&["angle"]).unwrap_or(0.0),
                }))
            }
            "scaleSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids, sx: f64_arg(&["sx"]).unwrap_or(1.0), sy: f64_arg(&["sy"]).unwrap_or(1.0), sz: f64_arg(&["sz"]).unwrap_or(1.0) }))
            }
            "addGeneration" => Ok(Generation3dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").cloned().unwrap_or(dsl::DslValue::Null);
                Ok(Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => Ok(Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: parse_flow_camera_json(&args) })),
            "worldPointerDown" => Ok(Generation3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {})),
            "graphPointerDown" => Ok(Generation3dCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {})),
            "setLodMode" => Ok(Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setShowMode" => Ok(Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() })),
            "toggleSun" => Ok(Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setCamera" => Ok(Generation3dCommand::SetCamera(set_camera::SetCamera { camera: parse_preview_camera_json(&args) })),
            "selectGeneration" => Ok(Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "setActiveUtility" => Ok(Generation3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: str_arg(&["utilityId", "utility_id"]).unwrap_or_default() })),
            "setLocale" => Ok(Generation3dCommand::SetLocale(set_locale::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() })),
            "flowEvalTick" => Ok(Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {})),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit`/`{translate,rotate,scale}Selection` read the `graph`
    /// interaction domain directly (bypassing the `app_commands!`-generated `dispatch`, whose
    /// per-row `$module::handle(payload, doc, cfg, ctx)` signature is framework-fixed and has no
    /// `interaction` slot) — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &Generation3dCommand,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, Fault> {
        let mut session = FlowEvalSession::new();
        match command {
            Generation3dCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction, &mut session),
            Generation3dCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction, &mut session),
            Generation3dCommand::TranslateSelection(payload) => translate_selection::apply(payload, doc, cfg, interaction, &mut session),
            Generation3dCommand::RotateSelection(payload) => rotate_selection::apply(payload, doc, cfg, interaction, &mut session),
            Generation3dCommand::ScaleSelection(payload) => scale_selection::apply(payload, doc, cfg, interaction, &mut session),
            _ => command.dispatch(doc, cfg, &mut session),
        }
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every widget's visible ports become `handle`
    /// targets (`{nodeId}@{portId}`, byte-identical to the node-graph's own pick ids) parented to
    /// their widget, which is what makes `HoverSpec { transitive: true }` light up every channel's
    /// preview geometry from a single node hover, and resolve a preview-instance hover back to its
    /// node. Every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    fn interaction_topology(doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>) -> InteractionTopology {
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
        let ports_by_node = generation3d_port_ids_by_node(fixture);
        for widget in &fixture.widgets {
            let id = crate::artifacts::generation3d::widget_id(widget).to_string();
            ordered.push(TopologyNode { id: id.clone(), granularity: "node".into(), parent: None });
            for port in ports_by_node.get(&id).into_iter().flatten() {
                ordered.push(TopologyNode { id: port.clone(), granularity: "handle".into(), parent: Some(id.clone()) });
            }
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

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes.
    fn pending_effects(doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>) -> Vec<Effect> {
        let mut session = FlowEvalSession::new();
        let host = flow::flow_host_with_session(&doc.snapshot.fixture, &session);
        if session.sync(&host) {
            vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(104), action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    /// 🕹️ The marks-free entry point the framework still offers (no owner, no transient, no
    /// interaction) — every live window goes through `render_with_request_context` instead.
    fn render(body_key: &str, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        generation3d_render_body(body_key, doc.snapshot, cfg.snapshot, &PreviewInteractionMarks::default())
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into every
    /// window body — the node graph paints the hovered node/port, the world previews paint the
    /// hovered/selected instances, and the inspection panel finally sees a real selection.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        generation3d_render_body(body_key, doc.snapshot, cfg.snapshot, &PreviewInteractionMarks::from_interaction(interaction))
    }

    fn window_measures(_doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let measures = edit_preview::preview_window_measures(config, generation3d_action);
        HashMap::from([
            (flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.to_string(), flow_window::window_measures(&config.lod_mode, generation3d_action)),
            (edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    /// 🗂️ Grouped disclosure: `reorganize`/`translateSelection`/`rotateSelection`/`scaleSelection` stay
    /// top-level; creation, removal and generation methods fold into taxonomy groups; `delete-selection`
    /// stays a direct destructive item last.
    ///
    /// 🕹️ `context_menu` carries no `InteractionView` either (same gap as `render` — see ticket
    /// 26/08/14's w3b-summary.md), so the selection-dependent rows below always take the "nothing
    /// selected" branch rather than reading a stale/wrong selection.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        semio_framework::io::resolve_ready(async {
            let config = cfg.snapshot;
            let labels = generation3d_labels(config);
            let is_de = config.locale.starts_with("de");
            let selected: Vec<String> = Vec::new();
            let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]).await;
            let has_selection = !nodes.is_empty() || !edges.is_empty();
            let mut menu = Menu::of(registry).await.action("reorganize").await;
            if has_selection {
                menu = menu.action("translateSelection").await.action("rotateSelection").await.action("scaleSelection").await;
            }
            menu = menu.group("create", |m| async { m.action("addWidget").await.action("addGeneration").await }).await;
            if has_selection {
                menu = menu.group("targets", |m| async { m.action("removeWidget").await.action("removeGeneration").await }).await;
            }
            menu = menu.group("methods", |m| async { m.action("renameGeneration").await.action("updateGenerationValues").await.action("patchFlowWidgets").await }).await;
            if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit).await {
                menu = menu.item(spec).await;
            }
            menu.build().await
        })
    }
}
//#endregion 🔖️Generation3dPlayApp

//#region 🔖️Manifest
pub fn create_generation3d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::generation3d::GENERATION3D_DIALECT).document(["semio", "procedural", "3d"])
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) }))
            .artifact_kind(artifact_kind())
            .icon_id("workflow")
            .mode_def(edit::definition())
            .mode_def(generate::definition())
            .default_mode_id(edit::GENERATION_3D_PLAY_MODE_EDIT)
            .mode_layout(generate::GENERATION_3D_PLAY_MODE_GENERATE, generate::GENERATION_3D_PLAY_LAYOUT_GENERATE)
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
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"))
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_with(categorized_action("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"), ActionKind::Mutation, "targets"))
            .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(categorized_action("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation, "create"))
            .action_with(categorized_action("patchFlowWidgets", LocalizedLabel::native("Patch Flow Widgets", "Flow-Elemente aktualisieren"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation, "transform"))
            .action_with(categorized_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation, "create"))
            .action_with(categorized_action("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::Mutation, "targets"))
            .action_with(categorized_action("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::Mutation, "methods"))
            .action_with(categorized_action("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::Mutation, "methods"))
            // 👁️ Ephemeral view actions — world picking, graph camera, sun/LOD/show-mode display toggles, preview camera.
            // Selection/hover are the framework's `graph` interaction domain now (`.interaction(...)`
            // below) — the six framework verbs (`interactionSelect`/`interactionHover`/`clearSelection`/
            // `selectAll`/`setSelectionMode`/`setInteractionGranularity`) auto-inject.
            .view_action("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            .view_action("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"))
            .view_action("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"))
            .view_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("selectGeneration", LocalizedLabel::native("Set Generation", "Generation auswählen"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeWidget", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveMediaNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addWidget", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFlowWidgets", InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("updateGenerationValues", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("graphPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectGeneration", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveUtility", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
            .action_interactive_job("flowEvalTick", InteractiveJobClassification::Migrated)
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("neuron", LocalizedLabel::native("Neuron", "Neuron")),
                    ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                    ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                ]).default_value("inputSlider"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
                    ActionArgOption::new(crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
                ]).required(),
            ])
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one domain over the flow-graph widget DAG, node/edge/handle granularities,
            // `HierarchyProvider::Topology` (see `Generation3dPlayApp::interaction_topology` below) —
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
            .window_kind_interactions(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
            .window_kind_interactions(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph")])
            .window_kind_interactions(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Generation3dPlayApp::config_spec())
            .io(semio_framework::io::resolve_ready(generation3d_io()))
            .build_definition()
}

/// 📚️ The eight bundled `📚️examples/🎬️<slug>` fixtures, in `schema::is_generation3d_example_id`'s
/// order — plugged into `.editor_with_examples::<Generation3dPlayApp>(create_generation3d_app(), …)`
/// at the plugin root so the react shell's example dropdown (`NavbarExampleSelect/🟦️.tsx`, fed by
/// `activePluginManifest.examples`) stops being hidden for `generation3d`.
pub fn examples() -> Vec<ExampleSource> {
    vec![
        crate::examples::art_generation3d_hexagonal_mushroom_column::source(),
        crate::examples::art_generation3d_rectangle_extrude_volume::source(),
        crate::examples::art_generation3d_sphere_cut_with_torus::source(),
        crate::examples::art_generation3d_box_fillet_preview::source(),
        crate::examples::art_generation3d_sphere_box_fuse::source(),
        crate::examples::art_generation3d_face_sweep_extrude::source(),
        crate::examples::art_generation3d_rectangle_wire_preview::source(),
        crate::examples::art_generation3d_box_shell_preview::source(),
    ]
}
//#endregion 🔖️Manifest

//#region 🔖️ArtifactIo
/// 🔌️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// this app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_generation3d_app` declares via `.artifact_kind(...)`; `params:in`/`geometry:out` are the
/// workflow-specific ports beyond the implicit document in/out ports.
pub async fn generation3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "generation.3d",
        MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "3d.generation".into(), name: "3D Generation".into(), dimension: "3d".into(), component_kind: "generation3d".into() },
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
            id: "geometry:out".into(),
            label: "Geometry".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            kind_id: Some("3d.mesh".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
    .await
}
//#endregion 🔖️ArtifactIo

//#region 🔖️PreviewPipeline
/// 🎥️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// every function here references [`Generation3dConfig`] (directly, or is reachable only from a
/// function that does), which made this app behavior, not artifact-schema-pure document compute; the
/// snapshot-pure fixture/gumball helpers stayed in `crate::artifacts::generation3d::schema` instead.
pub fn preview_tolerance(lod_mode: &str) -> f64 {
    match lod_mode {
        "coarse" => 0.15,
        "fine" => 0.02,
        _ => 0.05,
    }
}

pub fn preview_camera_json(cfg: &Generation3dConfig) -> String {
    ui_wgpu::wgpu::world3d_camera_json(cfg.preview_camera.position, cfg.preview_camera.target, cfg.preview_camera.fov)
}

//#region 🔖️PreviewInteraction
/// 🕹️ The framework interaction domain every generation3d window is bound to (see the
/// `window_kind_interactions` calls in the manifest stitch): the node graph, the edit preview and
/// the generate preview all read and write the same `graph` hover/selection.
pub const GENERATION_3D_INTERACTION_DOMAIN: &str = "graph";

/// 🐁️ The channel a pointer hovers on. `InteractionState.hover` holds exactly one live channel per
/// domain, so reading any other channel reads empty rather than stale.
pub const GENERATION_3D_INTERACTION_CHANNEL: &str = "pointer";

/// 🎯️ The granularity a plain world-3d instance pick/hover reports. A preview instance id is
/// channel-qualified (`{widgetId}@{channel}#{index}`), which is the node graph's own `handle`
/// (port) target shape — so a world hit and a graph port hit land on the same granularity.
pub const GENERATION_3D_INTERACTION_GRANULARITY: &str = "handle";

/// 🕹️ One render's resolved `graph`-domain marks.
///
/// A preview instance id is `{widgetId}@{channel}#{index}`, so an id counts as marked when the
/// domain names the instance itself, its channel (`{widgetId}@{channel}` — byte-identical to the
/// port id the node graph's own picks already use) or its widget (`{widgetId}`). That three-level
/// match is exactly what makes hover bidirectional: hovering a node in the graph lights up every
/// one of its channels' preview geometry, and hovering one preview instance in the world lights up
/// its node — and its port — back in the graph.
///
/// 🎯️ The world window reports the PORT, not the instance: an instance carries
/// `interactionId = {widgetId}@{channel}` (see `preview_payload`) because `validate_state` prunes
/// any hover/selection id absent from `interaction_topology`, and the per-index instance count is
/// evaluation-derived so it cannot be declared there.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PreviewInteractionMarks {
    pub hovered: std::collections::BTreeSet<String>,
    pub selected: std::collections::BTreeSet<String>,
}

impl PreviewInteractionMarks {
    /// 🕹️ Reads the framework-owned domain: hover off the ephemeral pointer channel, selection off
    /// the persisted interaction store. The app stores neither itself.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self {
            hovered: interaction.hover(GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_CHANNEL).ids.iter().cloned().collect(),
            selected: interaction.selection(GENERATION_3D_INTERACTION_DOMAIN).ids.iter().cloned().collect(),
        }
    }

    fn marked(set: &std::collections::BTreeSet<String>, widget_id: &str, channel: &str, index: usize) -> bool {
        set.contains(widget_id) || set.contains(&format!("{widget_id}@{channel}")) || set.contains(&format!("{widget_id}@{channel}#{index}"))
    }

    pub fn hovers(&self, widget_id: &str, channel: &str, index: usize) -> bool {
        Self::marked(&self.hovered, widget_id, channel, index)
    }

    pub fn selects(&self, widget_id: &str, channel: &str, index: usize) -> bool {
        Self::marked(&self.selected, widget_id, channel, index)
    }

    /// 🕸️ The widget id behind any interaction id — `{w}`, `{w}@{c}` and `{w}@{c}#{i}` all resolve
    /// to `{w}`.
    pub fn widget_of(id: &str) -> &str {
        let base = id.split('#').next().unwrap_or(id);
        base.split('@').next().unwrap_or(base)
    }

    /// 🕸️ The channel (port) id behind an interaction id, `None` for a bare widget id.
    pub fn port_of(id: &str) -> Option<&str> {
        let base = id.split('#').next().unwrap_or(id);
        base.split_once('@').map(|(_, port)| port)
    }

    /// 🕸️ `(nodeId, portId)` the node graph paints as hovered — a preview instance hovered in the
    /// world resolves to its node AND its channel, so the graph highlights the exact output port
    /// whose geometry the pointer is over.
    pub fn hovered_graph_target(&self) -> Option<(String, Option<String>)> {
        let id = self.hovered.iter().next()?;
        Some((Self::widget_of(id).to_string(), Self::port_of(id).map(str::to_string)))
    }

    /// 🕸️ Every id the node graph should highlight: each hovered id plus the widget it belongs to,
    /// deduplicated and ordered.
    pub fn graph_highlight_ids(&self) -> Vec<String> {
        let mut ids: std::collections::BTreeSet<String> = self.hovered.clone();
        for id in &self.hovered {
            ids.insert(Self::widget_of(id).to_string());
        }
        ids.into_iter().collect()
    }

    /// 🕸️ The `graph` selection projected onto widget ids — what `NodeGraphScene::selection` paints.
    pub fn graph_selection_ids(&self) -> Vec<String> {
        self.selected.iter().map(|id| Self::widget_of(id).to_string()).collect::<std::collections::BTreeSet<String>>().into_iter().collect()
    }
}
//#endregion 🔖️PreviewInteraction

/// 🧭️ World-3d selection payload with the host-owned gumball utility spliced in, so the transform
/// handles follow `cfg.active_utility_id` instead of any document-stored utility, and with the live
/// `graph` marks `render_with_request_context` resolved — the gumball now shows for a real
/// selection instead of always reporting empty. `"rectangle"` (the pre-migration default
/// `selection_method`) is hardcoded: the framework tracks no persistent "last marquee method"
/// outside a live gesture.
pub fn preview_selection_json(cfg: &Generation3dConfig, active_utility: &str, payload: &PreviewPayload) -> String {
    let mut value = dsl::json::parse(&semio_framework_plugin::world3d_selection_json("rectangle", &payload.selected_ids, payload.hovered_id.as_deref())).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    let show_mode = if cfg.show_mode.is_empty() { "shaded" } else { cfg.show_mode.as_str() };
    let (show_edges, selection_mode) = match show_mode {
        "wireframe" => (true, "mesh"),
        "points" => (false, "mesh"),
        "shaded+edges" => (true, "mesh"),
        _ => (false, "mesh"),
    };
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode", dsl::json::Value::String(active_utility.to_string()));
        object.insert("gumballActive", dsl::json::Value::Bool(!payload.selected_ids.is_empty() && !active_utility.is_empty()));
        object.insert("showEdges", dsl::json::Value::Bool(show_edges));
        object.insert("selectionMode", dsl::json::Value::String(selection_mode.to_string()));
        object.insert("granularity", dsl::json::Value::String(selection_mode.to_string()));
    }
    dsl::json::to_string(&value)
}

fn merge_status_json(computing: Option<String>, preview_status: Option<String>) -> Option<String> {
    match (computing, preview_status) {
        (Some(c), Some(p)) => {
            let mut computing_object = dsl::json::parse(&c).ok().and_then(|value| value.as_object().cloned()).unwrap_or_else(|| {
                let mut fallback = dsl::json::Object::new();
                fallback.insert("computing", dsl::json::Value::Bool(true));
                fallback
            });
            let preview_object = dsl::json::parse(&p).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default();
            for (key, value) in preview_object.iter() {
                computing_object.insert(key, value.clone());
            }
            Some(dsl::json::to_string(&dsl::json::Value::Object(computing_object)))
        }
        (Some(c), None) => Some(c),
        (None, Some(p)) => Some(p),
        (None, None) => None,
    }
}

/// 👁️ Merges the session's live "still computing" flag with a fresh `preview_status_json` result.
pub fn preview_scene_status_json(session: &FlowEvalSession, preview_status: Option<String>) -> Option<String> {
    let computing = session.pending().then(|| r#"{"computing":true}"#.to_string());
    merge_status_json(computing, preview_status)
}

pub fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() {
        return false;
    }
    if handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
    {
        return true;
    }
    // Blake3 hex digests minted by `BrepKernel::mint` (no kind prefix).
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

/// 🔌️ Point/vector geometry synthesized without a kernel round-trip, for a math-style output
/// channel that carries `x`/`y`/`z` coordinates instead of a brep handle.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewInlineGeometry {
    Point { x: f64, y: f64, z: f64 },
    Vector { x: f64, y: f64, z: f64 },
}

/// 🔌️ One previewable value found on one output channel of one widget — the channel-aware
/// replacement for the old unordered handle flattening, so preview instance ids can be
/// channel-qualified (`{widgetId}@{channel}#{index}`, see `preview_payload`).
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewChannelItem {
    pub channel: String,
    pub index: usize,
    pub handle: String,
    pub inline: Option<PreviewInlineGeometry>,
}

/// 🔎️ A `$schema: "list"` dictionary's entries in index order (`"0"`, `"1"`, …) — the wire form
/// `flow::neural::Dictionary` lists actually take (an object with numeric-string keys, not a JSON
/// array), so ordering has to be recovered by parsing the keys rather than trusting map iteration.
fn preview_channel_list_entries(map: &dsl::json::Object) -> Vec<&dsl::json::Value> {
    let mut entries: Vec<(usize, &dsl::json::Value)> = map.iter().filter_map(|(key, value)| key.parse::<usize>().ok().map(|index| (index, value))).collect();
    entries.sort_by_key(|(index, _)| *index);
    entries.into_iter().map(|(_, value)| value).collect()
}

/// 🔎️ Depth-first walk of one channel's evaluated value, emitting one [`PreviewChannelItem`] per
/// geometry-bearing leaf in encounter order. Arrays and `$schema: "list"` dictionaries recurse;
/// a handle passing `is_brep_geometry_handle` or an `x`/`y`/`z` point/vector is a leaf; everything
/// else (numbers, strings, booleans, plain dictionaries) is pure data and yields nothing.
fn collect_preview_channel_items(channel: &str, value: &dsl::json::Value, index: &mut usize, items: &mut Vec<PreviewChannelItem>) {
    match value {
        dsl::json::Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(dsl::json::Value::as_str) {
                if is_brep_geometry_handle(handle) {
                    items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: handle.into(), inline: None });
                    *index += 1;
                    return;
                }
            }
            if map.get("$schema").and_then(dsl::json::Value::as_str) == Some("list") {
                for entry in preview_channel_list_entries(map) {
                    collect_preview_channel_items(channel, entry, index, items);
                }
                return;
            }
            let coords = ["x", "y", "z"].into_iter().map(|key| map.get(key).and_then(dsl::json::Value::as_f64)).collect::<Option<Vec<_>>>();
            if let Some(coords) = coords {
                let (x, y, z) = (coords[0], coords[1], coords[2]);
                let inline = if map.get("$schema").and_then(dsl::json::Value::as_str) == Some("vector") { PreviewInlineGeometry::Vector { x, y, z } } else { PreviewInlineGeometry::Point { x, y, z } };
                items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle: String::new(), inline: Some(inline) });
                *index += 1;
            }
        }
        dsl::json::Value::Array(list) => {
            for entry in list {
                collect_preview_channel_items(channel, entry, index, items);
            }
        }
        _ => {}
    }
}

/// 🔌️ Channel-by-channel enumeration of one widget's preview-bearing values: sorted `"out"`
/// channel keys (falling back to `"in"` only when the widget has no `"out"` at all), each walked
/// depth-first into its geometry-bearing leaves. Replaced the old flat,
/// unordered handle collection — every call site that needs handles/points/vectors for preview routes
/// through this one function now.
pub fn preview_channel_items_for_widget(eval: &dsl::json::Value, widget_id: &str) -> Vec<PreviewChannelItem> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let Some(channels) = widget_eval.get("out").or_else(|| widget_eval.get("in")) else {
        return Vec::new();
    };
    let Some(map) = channels.as_object() else {
        return Vec::new();
    };
    let mut keys: Vec<&str> = map.iter().map(|(key, _)| key).collect();
    keys.sort();
    let mut items = Vec::new();
    for key in keys {
        let mut index = 0usize;
        if let Some(value) = map.get(key) {
            collect_preview_channel_items(key, value, &mut index, &mut items);
        }
    }
    items
}

/// 👁️ Whether a widget contributes preview geometry at all. A `Neuron` carries its own author-set
/// `preview` toggle; an `OutputPreview` is a preview by construction; a `Cluster` has no toggle of
/// its own (`flow::neural::Neuron` — its inner neurons — carries none either), so its contract
/// output channels always preview, which is the only way a grouped sub-graph's geometry reaches
/// the 3D world at all.
pub fn widget_previews(widget: &flow::Widget) -> bool {
    matches!(widget, flow::Widget::Neuron { preview: true, .. } | flow::Widget::OutputPreview { .. } | flow::Widget::Cluster { .. })
}

fn mesh_has_preview_geometry(data: &semio_framework_plugin::MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

/// 🔌️ Half-extent (world units) of the axis cross drawn for a `PreviewInlineGeometry::Point`.
const PREVIEW_POINT_MARKER_HALF_EXTENT: f64 = 0.05;

/// 🔌️ A small axis cross at `(x, y, z)` — the point-channel preview marker, built without a
/// kernel round-trip. Carries both `positions` (so `"points"` show mode still has something to
/// draw once `apply_show_mode_mesh` strips `edge_positions`) and the cross itself as edges.
fn point_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    let e = PREVIEW_POINT_MARKER_HALF_EXTENT as f32;
    semio_framework_plugin::MeshData {
        positions: vec![x, y, z],
        edge_positions: vec![x - e, y, z, x + e, y, z, x, y - e, z, x, y + e, z, x, y, z - e, x, y, z + e],
        ..Default::default()
    }
}

/// 🔌️ A line segment from the world origin to `(x, y, z)` — the vector-channel preview marker,
/// built without a kernel round-trip.
fn vector_marker_mesh(x: f64, y: f64, z: f64) -> semio_framework_plugin::MeshData {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    semio_framework_plugin::MeshData { positions: vec![0.0, 0.0, 0.0, x, y, z], edge_positions: vec![0.0, 0.0, 0.0, x, y, z], ..Default::default() }
}

fn apply_show_mode_mesh(mut data: semio_framework_plugin::MeshData, show_mode: &str) -> semio_framework_plugin::MeshData {
    let show_mode = match show_mode {
        "solid" | "shaded" | "shaded+edges" | "wireframe" | "points" => show_mode,
        _ => "shaded",
    };
    match show_mode {
        "wireframe" => {
            data.positions.clear();
            data.normals.clear();
            data.indices.clear();
            data.face_ids.clear();
            data
        }
        "points" => {
            data.indices.clear();
            data.normals.clear();
            data.edge_positions.clear();
            data
        }
        _ => data,
    }
}

pub fn preview_status_json(eval_json: &str, fixture: &flow::FlowFixture) -> Option<String> {
    let eval = dsl::json::parse(eval_json).ok()?;
    if eval.get("error").and_then(dsl::json::Value::as_str).is_some() {
        let mut error_object = dsl::json::Object::new();
        error_object.insert("error", eval.get("error").cloned().unwrap_or(dsl::json::Value::Null));
        return Some(dsl::json::to_string(&dsl::json::Value::Object(error_object)));
    }
    let mut errors = dsl::json::Object::new();
    for widget in &fixture.widgets {
        let id = crate::artifacts::generation3d::widget_id(widget).to_string();
        let Some(entry) = eval.get(&id) else { continue };
        if let Some(error) = entry.get("error").and_then(dsl::json::Value::as_str) {
            errors.insert(id, dsl::json::Value::String(error.to_string()));
        }
    }
    if errors.is_empty() {
        None
    } else {
        let mut wrapper = dsl::json::Object::new();
        wrapper.insert("widgetErrors", dsl::json::Value::Object(errors));
        Some(dsl::json::to_string(&dsl::json::Value::Object(wrapper)))
    }
}

/// 🧵️ Pure per-render tessellation: bounded-cost, safe to call fresh on every render call instead of
/// behind an outer memoization layer.
fn mesh_data_for_preview_handle(handle: &str, tolerance: f64, session: Option<&FlowEvalSession>) -> Option<semio_framework_plugin::MeshData> {
    if let Some(session) = session {
        if let Some(json) = session.preview_mesh_json(handle) {
            let has_error = dsl::json::parse(json).ok().is_some_and(|value| value.get("error").is_some());
            if !has_error {
                // 🌉️ `MeshData` has its own first-party `FromValue` (`🏗️mesh-engine/🦀️.rs`) — decode
                // through the `pack::json`/`DslValue` bridge, no `serde_json` involved.
                if let Ok(data) = dsl::json::from_json_str::<semio_framework_plugin::MeshData>(json) {
                    if mesh_has_preview_geometry(&data) {
                        return Some(data);
                    }
                }
            }
        }
    }
    let data = flow::tessellate_geometry(handle, tolerance).ok()?;
    mesh_has_preview_geometry(&data).then_some(data)
}

/// 🧊 Geometry handles on preview widgets that still need an extension tessellate.
pub fn pending_preview_tessellate_handles(eval_json: &str, fixture: &flow::FlowFixture, session: &FlowEvalSession) -> Vec<String> {
    if eval_json.is_empty() {
        return Vec::new();
    }
    let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    let mut handles = Vec::new();
    for widget in &fixture.widgets {
        let preview = widget_previews(widget);
        if !preview {
            continue;
        }
        let id = crate::artifacts::generation3d::widget_id(widget).to_string();
        for handle in preview_channel_items_for_widget(&eval, &id).into_iter().filter_map(|item| (!item.handle.is_empty()).then_some(item.handle)) {
            let ready = session.preview_mesh_json(&handle).and_then(|json| {
                if dsl::json::parse(json).ok()?.get("error").is_some() {
                    return None;
                }
                let data = dsl::json::from_json_str::<semio_framework_plugin::MeshData>(json).ok()?;
                mesh_has_preview_geometry(&data).then_some(())
            });
            if ready.is_none() {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 📨 Host effects that tessellate preview handles inside the owning brep extension kernel.
pub fn preview_tessellate_effects(session: &mut FlowEvalSession, eval_json: &str, fixture: &flow::FlowFixture, cfg: &Generation3dConfig) -> Vec<Effect> {
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let tolerance_bits = tolerance.to_bits();
    let mut live = std::collections::HashSet::new();
    let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    for widget in &fixture.widgets {
        let id = crate::artifacts::generation3d::widget_id(widget).to_string();
        for item in preview_channel_items_for_widget(&eval, &id) {
            if !item.handle.is_empty() {
                live.insert(item.handle);
            }
        }
    }
    session.retain_preview_meshes(&live);
    let mut effects = Vec::new();
    for handle in pending_preview_tessellate_handles(eval_json, fixture, session) {
        let node_hash = flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone()) {
            let mut request_object = dsl::json::Object::new();
            request_object.insert("handle", dsl::json::Value::String(handle));
            request_object.insert("tolerance", dsl::json::Value::from(tolerance));
            request_object.insert("nodeHash", dsl::json::Value::from(node_hash));
            effects.push(Effect::InvokeExtension {
                req: semio_framework_plugin::RequestId(105),
                extension_id: "brep".into(),
                capability: "tessellate".into(),
                request_json: dsl::json::to_string(&dsl::json::Value::Object(request_object)),
            });
        }
    }
    effects
}

/// 👁️ One preview render's world-3d payload: the handle-deduplicated mesh table, the per-channel
/// instance table, and the instance ids the live `graph` marks resolved to — so the scene's
/// `selection_json` paints exactly the same hover/selection the instances themselves carry.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewPayload {
    pub meshes_json: String,
    pub instances_json: String,
    pub selected_ids: Vec<String>,
    pub hovered_id: Option<String>,
}

/// 👁️ An empty payload is the empty JSON ARRAY, not the empty string — every consumer feeds these
/// straight into a `World3dScene` and compares them against `"[]"`.
impl Default for PreviewPayload {
    fn default() -> Self {
        Self { meshes_json: "[]".into(), instances_json: "[]".into(), selected_ids: Vec::new(), hovered_id: None }
    }
}

/// 🧊️ The interaction-free, session-free entry point: the mesh-export bridge and the schema tests
/// evaluate geometry without any live window, so they carry no marks and no tessellation cache.
pub fn preview_payload_from_eval(eval_json: &str, fixture: &flow::FlowFixture, cfg: &Generation3dConfig) -> (String, String) {
    let payload = preview_payload(eval_json, fixture, cfg, None, &PreviewInteractionMarks::default());
    (payload.meshes_json, payload.instances_json)
}

/// 👁️ One preview instance per geometry-bearing value per OUTPUT CHANNEL — the whole point of the
/// channel-qualified ids: a widget with several outputs previews every one of them, not just the
/// first handle its evaluation happened to expose.
/// 🧮️ `[f64; 3]` -> a `pack::json` array, for the position/scale fields below.
fn vec3_json(v: [f64; 3]) -> dsl::json::Value {
    dsl::json::Value::Array(v.into_iter().map(dsl::json::Value::from).collect())
}

pub fn preview_payload(eval_json: &str, fixture: &flow::FlowFixture, cfg: &Generation3dConfig, session: Option<&FlowEvalSession>, marks: &PreviewInteractionMarks) -> PreviewPayload {
    if eval_json.is_empty() {
        return PreviewPayload::default();
    }
    if let Ok(parsed) = dsl::json::parse(eval_json) {
        if parsed.get("error").and_then(dsl::json::Value::as_str).is_some() {
            return PreviewPayload::default();
        }
    }
    let eval = dsl::json::parse(eval_json).unwrap_or_else(|_| dsl::json::Value::Object(dsl::json::Object::new()));
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let mut meshes: Vec<dsl::json::Value> = Vec::new();
    let mut instances: Vec<dsl::json::Value> = Vec::new();
    // 🔁️ Dedup key is the brep HANDLE, not the widget/channel that emitted it: two channels (even
    // on different widgets) that resolve to the same handle share one tessellated mesh entry and
    // still each get their own instance — see the mesh-id lookup below.
    let mut mesh_id_by_handle: HashMap<String, String> = HashMap::new();
    let mut selected_ids: Vec<String> = Vec::new();
    let mut hovered_id: Option<String> = None;
    for widget in &fixture.widgets {
        let id = crate::artifacts::generation3d::widget_id(widget).to_string();
        let preview = widget_previews(widget);
        if !preview {
            continue;
        }
        let items = preview_channel_items_for_widget(&eval, &id);
        if items.is_empty() {
            continue;
        }
        for item in &items {
            let PreviewChannelItem { channel, index, handle, inline } = item;
            let instance_id = format!("{id}@{channel}#{index}");
            let own_mesh_id = format!("eval-{id}@{channel}#{index}");
            let mesh_id = if handle.is_empty() { own_mesh_id } else { mesh_id_by_handle.get(handle).cloned().unwrap_or(own_mesh_id) };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                let data = match inline {
                    Some(PreviewInlineGeometry::Point { x, y, z }) => Some(point_marker_mesh(*x, *y, *z)),
                    Some(PreviewInlineGeometry::Vector { x, y, z }) => Some(vector_marker_mesh(*x, *y, *z)),
                    None => mesh_data_for_preview_handle(handle, tolerance, session),
                };
                if let Some(data) = data {
                    let data = apply_show_mode_mesh(data, show_mode);
                    if mesh_has_preview_geometry(&data) {
                        let mut mesh_object = dsl::json::Object::new();
                        mesh_object.insert("id", dsl::json::Value::String(mesh_id.clone()));
                        mesh_object.insert("data", dsl::json::Value::from(data));
                        meshes.push(dsl::json::Value::Object(mesh_object));
                        if !handle.is_empty() {
                            mesh_id_by_handle.insert(handle.clone(), mesh_id.clone());
                        }
                    }
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                let selected = marks.selects(&id, channel, *index);
                let hovered = marks.hovers(&id, channel, *index);
                if selected {
                    selected_ids.push(instance_id.clone());
                }
                if hovered && hovered_id.is_none() {
                    hovered_id = Some(instance_id.clone());
                }
                let mut instance_object = dsl::json::Object::new();
                instance_object.insert("id", dsl::json::Value::String(instance_id));
                instance_object.insert("meshId", dsl::json::Value::String(mesh_id));
                instance_object.insert("position", vec3_json([0.0, 0.0, 0.0]));
                instance_object.insert("rotation", dsl::json::Value::Array(vec![dsl::json::Value::from(0.0), dsl::json::Value::from(0.0), dsl::json::Value::from(0.0), dsl::json::Value::from(1.0)]));
                instance_object.insert("scale", vec3_json([1.0, 1.0, 1.0]));
                instance_object.insert("label", dsl::json::Value::String(format!("{id}@{channel}")));
                instance_object.insert("interactionId", dsl::json::Value::String(format!("{id}@{channel}")));
                instance_object.insert("selected", dsl::json::Value::Bool(selected));
                instance_object.insert("hovered", dsl::json::Value::Bool(hovered));
                instances.push(dsl::json::Value::Object(instance_object));
            }
        }
    }
    PreviewPayload {
        meshes_json: dsl::json::to_string(&dsl::json::Value::Array(meshes)),
        instances_json: dsl::json::to_string(&dsl::json::Value::Array(instances)),
        selected_ids,
        hovered_id,
    }
}
//#endregion 🔖️PreviewPipeline

//#region 🔖️MeshBridge
/// 🧊️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the DWG-import mesh bridge: `export_mesh_from_document` builds a default [`Generation3dConfig`] to
/// run the same preview pipeline the app's own render path uses, which is why this cluster is app
/// behavior rather than artifact-schema-pure compute.
pub fn merge_preview_meshes(meshes: &[semio_framework_plugin::MeshData]) -> semio_framework_plugin::MeshData {
    let mut merged = semio_framework_plugin::MeshData::default();
    for mesh in meshes {
        let vertex_offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(&mesh.positions);
        merged.normals.extend(&mesh.normals);
        merged.colors.extend(&mesh.colors);
        merged.indices.extend(mesh.indices.iter().map(|index| index + vertex_offset));
        merged.edge_positions.extend(&mesh.edge_positions);
        if !mesh.edge_ids.is_empty() {
            let edge_base = merged.edge_ids.len() as u32;
            merged.edge_ids.extend(mesh.edge_ids.iter().map(|id| id + edge_base));
        }
    }
    merged
}

pub fn export_mesh_from_document(projection: &Generation3dSnapshot) -> semio_framework_plugin::MeshData {
    let config = Generation3dConfig::default();
    let mut host = crate::artifacts::generation3d::schema::host_from_fixture(&projection.fixture);
    let eval_json = host.evaluate().unwrap_or_default();
    let (meshes_json, _) = preview_payload_from_eval(&eval_json, &projection.fixture, &config);
    // 🌉️ `MeshData` has its own first-party `FromValue` (see `mesh_data_for_preview_handle`'s
    // note) — decode the per-mesh `data` field straight through the `pack::json`/`DslValue` bridge.
    let meshes: Vec<semio_framework_plugin::MeshData> = dsl::json::parse(&meshes_json)
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entry| entry.get("data").cloned())
        .filter_map(|data| dsl::FromValue::from_value(dsl::json::to_dsl_value(&data)).ok())
        .collect();
    merge_preview_meshes(&meshes)
}

pub fn generation3d_mesh_from_document(doc: &dsl::DslValue) -> Result<semio_framework_plugin::MeshData, String> {
    let projection = <Generation3dSnapshot as protocol::FromValue>::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_document(&projection))
}

pub fn generation3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<protocol::json::Value, String> {
    Ok(protocol::json::from_dsl_value(&protocol::ToValue::to_value(&crate::artifacts::generation3d::schema::default_snapshot())))
}

//#endregion 🔖️MeshBridge

//#region 🧪️TestSupport
/// 🧵️ `tessellate_geometry` (flow core brep geometry session) (and the flow-eval neuron kernel cache it sits behind)
/// is a process-wide cache shared by every test in this ONE merged crate — before the crate
/// consolidation, the artifact/app constitutional crates each ran in their own `cargo test` process, so
/// a `TEST_SERIAL` local to one of them never had to coordinate with the other's. Now that every
/// taxonomy node's tests share one test binary, ANY test that evaluates a flow fixture and/or tessellates
/// BRep geometry (directly here, or indirectly via the app's preview-window `render()`) must acquire
/// THIS single crate-wide lock — see `crate::editor::generation3d::modes::edit::windows::preview`'s test
/// for the app-side half of this. Rehomed from the deleted `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::{Mutex, MutexGuard};

    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    pub fn lock() -> MutexGuard<'static, ()> {
        TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
//#endregion 🧪️TestSupport

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `Generation3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Generation3dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Generation3dPlayApp>` builds it.
    pub type Generation3dApp = VcsArtifactApp<EditorApp<Generation3dPlayApp>>;

    /// ✏️ Adapts `create_generation3d_app`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` /
    /// `testkit::new_app_with_registry` still expect — framework testkit gap, not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub fn generation3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_generation3d_app(), examples: Vec::new() }
    }

    pub async fn app() -> Generation3dApp {
        new_app::<EditorApp<Generation3dPlayApp>>().await
    }

    pub async fn app_with_registry() -> Generation3dApp {
        new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_testkit).await
    }

    pub async fn dispatch(app: &mut Generation3dApp, command: Generation3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }

    pub async fn render(app: &mut Generation3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
    }

    /// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
    /// drains in production — a test has to do that draining itself.
    pub async fn drain_flow_eval_ticks(app: &mut Generation3dApp) {
        app.pending_effects().await;
        for _ in 0..1000 {
            let result = app.dispatch_typed(Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}), &meta("local")).await.expect("flowEvalTick");
            if !result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")) {
                return;
            }
        }
        panic!("flowEvalTick chain did not converge within 1000 ticks");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation3d::testkit::{app, app_with_registry, drain_flow_eval_ticks};
    use semio_framework_plugin::PluginApp;
    fn production_initial_snapshot(label: &str) -> Generation3dSnapshot {
        let mut snapshot = Generation3dSnapshot::default();
        snapshot.fixture.schema = label.into();
        for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
            snapshot.fixture.widgets.push(flow::Widget::InputNote { id: id.into(), text: text.into() });
        }
        snapshot.fixture.synapses.push(flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
        snapshot.fixture.synapses.push(flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
        snapshot.fixture.layout.insert("move-target".into(), flow::WidgetLayout { x: 1.0, y: 2.0 });
        snapshot.fixture.layout.insert("clear-target".into(), flow::WidgetLayout { x: 3.0, y: 4.0 });
        for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
            snapshot.generation.cold_builder_mut().unwrap().generations.push(flow::playbook::FormGeneration { id: id.into(), name: name.into(), values: serde_json::Map::new() });
        }
        snapshot.generation.cold_builder_mut().unwrap().selected_generation_id = Some("rename-generation".into());
        snapshot
    }

    fn production_mutations() -> Vec<Generation3dMutation> {
        use crate::artifacts::generation3d::mutations::*;
        let params = flow::neural::Dictionary::new()
            .insert("integer", flow::neural::Value::Atom(flow::neural::Atom::Integer(7)))
            .insert("nested", flow::neural::Value::Dictionary(flow::neural::Dictionary::new().insert("text", flow::neural::Value::Atom(flow::neural::Atom::String("production".into())))));
        vec![
            Generation3dMutation::CreateWidget(create_widget::CreateWidget { index: 0, widget: flow::Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true } }),
            Generation3dMutation::UpdateWidget(update_widget::UpdateWidget { widget: flow::Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() } }),
            Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: "delete-target".into() }),
            Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse { index: 0, synapse: flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() } }),
            Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse { synapse: flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() } }),
            Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: "disconnect-synapse".into() }),
            Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: "move-target".into(), layout: flow::WidgetLayout { x: 31.0, y: -17.0 } }),
            Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: "clear-target".into() }),
            Generation3dMutation::UpdateCamera(update_camera::UpdateCamera { camera: flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 } }),
            Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: "flow.fixture.production-retained".into() }),
            Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation: flow::playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: serde_json::Map::new() } }),
            Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id: "delete-generation".into() }),
            Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id: "rename-generation".into(), new_name: "After Rename".into() }),
            Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue { id: "change-generation".into(), question_id: "deep-answer".into(), new_value: serde_json::json!({"object": {"array": [1.0, false, "retained"]}}) }),
        ]
    }

    fn production_hex(bytes: &[u8]) -> String {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut value = String::new();
        value.try_reserve_exact(bytes.len() * 2).expect("P3 production hex preflight");
        for byte in bytes {
            value.push(char::from(DIGITS[usize::from(byte >> 4)]));
            value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
        }
        value
    }

    fn production_semantic_digest(snapshot: &Generation3dSnapshot) -> [u8; 32] {
        let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation3d.production-law.semantic");
        digest.observe(&crate::artifacts::generation3d::snapshot::binary::encode(snapshot));
        digest.finish()
    }

    fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation3dSnapshot, [u8; 32]) {
        let snapshot = production_initial_snapshot(label);
        let mutations = production_mutations();
        assert_eq!(mutations.len(), 14, "production ingress carries every P3 mutation variant including delete-widget-position");
        let mut mutation_hex = Vec::new();
        mutation_hex.try_reserve_exact(mutations.len()).expect("P3 production mutation owner preflight");
        for mutation in &mutations {
            mutation_hex.push(production_hex(&crate::artifacts::generation3d::spr::encode_op(mutation).expect("P3 production mutation encoding")));
        }
        let mut expected = production_initial_snapshot(label);
        crate::artifacts::generation3d::spr::generation3d_apply_retained_mutations_for_test(&mut expected, &mutations);
        let expected_digest = production_semantic_digest(&expected);
        let wire = serde_json::to_vec(&serde_json::json!({
            "schema": crate::artifacts::generation3d::GENERATION_3D_SCHEMA,
            "id": "generation3d-production-mounted-law",
            "vcs": {
                "initialSnapshot": production_hex(&crate::artifacts::generation3d::snapshot::binary::encode(&snapshot)),
                "edits": [{
                    "id": "generation3d-production-all14-edit",
                    "actor": "generation3d-production-law",
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
        .expect("schema-first P3 production fixture envelope");
        (wire, expected, expected_digest)
    }

    fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Generation3dPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P3 production ingress credits");
        crate::artifacts::generation3d::spr::generation3d_admit_publication_authority(
            handle.operation,
            handle.generation,
            handle.generation.0,
            handle.generation.0,
            handle.generation.0,
            8_192,
            crate::artifacts::generation3d::spr::GENERATION3D_MOUNTED_OUTPUT_CHANNELS,
            crate::artifacts::generation3d::spr::GENERATION3D_MOUNTED_CONTROL_CREDITS,
        )
        .expect("P3 production publication authority");
        for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            bytes[..chunk.len()].copy_from_slice(chunk);
            let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P3 production envelope page");
            app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P3 production envelope page admission failed: {fault}"));
        }
        assert!(app.seal_artifact_envelope_ingress(handle).expect("P3 production envelope seal"));
        handle
    }

    fn drive_production_envelope(
        app: &mut semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Generation3dPlayApp>>,
        handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle,
    ) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..300_000 {
            crate::artifacts::generation3d::spr::generation3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0).expect("P3 authority refresh immediately before production maintenance");
            PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P3 production maintenance turn");
            let poll = app.advance_artifact_envelope_load(handle).expect("P3 production load advancement");
            if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
                return poll;
            }
            std::thread::yield_now();
        }
        panic!("P3 production envelope load did not reach terminal");
    }

    /// 🔐️ LAW: non-empty P3D3 canonical ingress reaches the real VCS maintenance replacement,
    /// and accepted, stale, ABA, and displaced stores remain owned until explicit terminal ACK/close.
    #[semio_framework_async_macros::async_test]
    async fn vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed() {
        let mut accepted = semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::EditorApp<Generation3dPlayApp>>::new(semio_framework_plugin::EditorApp::default()).await;
        let base_generation = accepted.artifact_generation_now();
        let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
        let handle = admit_production_envelope(&mut accepted, &wire);
        assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
        let snapshot = accepted.snapshot().await.expect("accepted P3 production snapshot");
        assert_eq!(&*snapshot, &expected, "real maintenance must publish all P3 snapshot and all-14 replay fields");
        assert_eq!(production_semantic_digest(&snapshot), expected_digest);
        assert!(snapshot.fixture.layout.contains_key("move-target"));
        assert!(!snapshot.fixture.layout.contains_key("clear-target"), "3D-only delete-widget-position must survive retained replay");
        assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P3 terminal ACK"));
        assert!(crate::artifacts::generation3d::spr::generation3d_release_publication_authority(handle.operation, handle.generation));

        use crate::artifacts::generation3d::spr::Generation3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
        for (hostile, expected_code) in [
            (Missing, "generation3d-publication.authority-missing"),
            (WrongOperation, "generation3d-publication.wrong-operation"),
            (WrongGeneration, "generation3d-publication.wrong-generation"),
            (WrongBase, "generation3d-publication.wrong-base"),
            (WrongParent, "generation3d-publication.wrong-parent"),
        ] {
            let mut app = semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::EditorApp<Generation3dPlayApp>>::new(semio_framework_plugin::EditorApp::default()).await;
            let last_valid = app.snapshot().await.expect("last-valid P3 snapshot");
            let last_valid_digest = production_semantic_digest(&last_valid);
            let base_generation = app.artifact_generation_now();
            let (wire, _, _) = production_envelope_wire("rejected-production-candidate");
            let handle = admit_production_envelope(&mut app, &wire);
            crate::artifacts::generation3d::spr::generation3d_arm_publication_hostile(handle.operation, hostile);
            assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
            assert_eq!(crate::artifacts::generation3d::spr::generation3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
            assert_eq!(app.artifact_generation_now(), base_generation);
            let retained = app.snapshot().await.expect("last-valid P3 snapshot after rejected candidate");
            assert_eq!(production_semantic_digest(&retained), last_valid_digest);
            assert_eq!(retained, last_valid);
            assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P3 terminal ACK after candidate retirement"));
            assert!(crate::artifacts::generation3d::spr::generation3d_release_publication_authority(handle.operation, handle.generation));
        }
    }

    //#region 🔖️CommandSurface
    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let _serial = test_support::lock();
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 31, "every Generation3dCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: every one of the 29 declared `Generation3dCommand` rows is retained-owned by
    /// `Generation3dBoundedCommandJobFactory`, with an exact, nonempty publication-lane contract —
    /// the shape `ArtifactToolFactoryRegistry::register` itself enforces
    /// (`🧰️framework/…/🔌️plugin/🦀️.rs:12736-12748`), asserted here so a future command addition that
    /// forgets its retained-tool-id/publication-contract row fails this test instead of silently
    /// reintroducing `interactive-job.missing-owned-reducer` at dispatch. Mirrors generation2d's own
    /// `retained_route_dispositions_are_exact_and_exhaustive` (`…/generation2d/…/✏️editor/🦀️.rs:1033`).
    #[test]
    fn retained_route_dispositions_are_exact_and_exhaustive() {
        use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
        use semio_framework_plugin::ArtifactOwnedToolJobFactory;
        let _serial = test_support::lock();
        assert_eq!(GENERATION3D_RETAINED_TOOL_IDS.len(), 29);
        assert_eq!(<Generation3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 29);
        assert_eq!(Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 29);
        assert_eq!(generation3d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
        assert_eq!(generation3d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
        assert!(GENERATION3D_RETAINED_TOOL_IDS.iter().all(|tool_id| Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
        let mut sorted_ids = GENERATION3D_RETAINED_TOOL_IDS.to_vec();
        sorted_ids.sort_unstable();
        sorted_ids.dedup();
        assert_eq!(sorted_ids.len(), GENERATION3D_RETAINED_TOOL_IDS.len(), "duplicate retained tool ids in {GENERATION3D_RETAINED_TOOL_IDS:?}");
        for command in every_command() {
            assert!(GENERATION3D_RETAINED_TOOL_IDS.contains(&command.command_id()), "command {} is not owned by Generation3dBoundedCommandJobFactory", command.command_id());
        }
    }

    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        let _serial = test_support::lock();
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
    /// explicitly per row since generation3d's wire keys frequently diverge from a mechanical
    /// kebab-case of the command id (e.g. `nodeGraphViewport` → `viewport`, `setLocale` → `locale`).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let _serial = test_support::lock();
        let expected_keywords = [
            "active-example",
            "graph-edit",
            "delete-selection",
            "remove-widget",
            "move-node",
            "add-widget",
            "patch-flow-widgets",
            "reorganize",
            "translate-selection",
            "rotate-selection",
            "scale-selection",
            "add-generation",
            "remove-generation",
            "rename-generation",
            "update-generation-values",
            "viewport",
            "world-pointer-down",
            "graph-pointer-down",
            "lod-mode",
            "show-mode",
            "toggle-sun",
            "sun-azimuth",
            "sun-elevation",
            "sun-intensity",
            "camera",
            "select-generation",
            "active-utility",
            "locale",
            "flow-eval-tick",
        ];
        let commands = every_command();
        assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
        for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
            let printed = protocol::OpText::print_op(command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<Generation3dCommand> {
        vec![
            Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }),
            Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
            Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "extrude".into() }),
            Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "extrude".into(), x: 1.0, y: 2.0 }),
            Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: Some(10.0), y: None }),
            Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }),
            Generation3dCommand::Reorganize(reorganize::Reorganize {}),
            Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }),
            Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }),
            Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
            Generation3dCommand::AddGeneration(add_generation::AddGeneration {}),
            Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "generation-1".into() }),
            Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }),
            Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
            Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: flow::CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }),
            Generation3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {}),
            Generation3dCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
            Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
            Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
            Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
            Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
            Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
            Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
            Generation3dCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::generation3d::config::Generation3dPreviewCamera::default() }),
            Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: "generation-1".into() }),
            Generation3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }),
            Generation3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
        ]
    }
    //#endregion 🔖️CommandSurface

    #[semio_framework_async_macros::async_test]
    async fn declared_actions_bridge_to_commands() {
        let _serial = test_support::lock();
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<semio_framework_plugin::EditorApp<Generation3dPlayApp>>(testkit::generation3d_app_manifest_for_testkit).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_backed_editor_installs_every_declared_bounded_command_proof() {
        let _serial = test_support::lock();
        let _app = semio_framework_plugin::testkit::new_app_with_registry::<semio_framework_plugin::EditorApp<Generation3dPlayApp>>(testkit::generation3d_app_manifest_for_testkit).await;
    }

    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let _serial = test_support::lock();
        let json = serde_json::to_string(&create_generation3d_app()).expect("app definition json");
        for id in [
            flow_window::GENERATION_3D_PLAY_WINDOW_MAIN,
            edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW,
            generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS,
            form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM,
            generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW,
        ] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for id in [edit::GENERATION_3D_PLAY_MODE_EDIT, generate::GENERATION_3D_PLAY_MODE_GENERATE] {
            assert!(json.contains(id), "mode {id} missing from the manifest");
        }
        assert!(json.contains("3d.generation"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn each_example_loads_distinct_fixture_and_preview_geometry() {
        use crate::artifacts::generation3d::schema::*;
        use crate::artifacts::generation3d::widget_id;
        let _serial = test_support::lock();
        let examples = [
            PROCEDURAL_EXAMPLE_HEX_COLUMN,
            PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
            PROCEDURAL_EXAMPLE_SPHERE_TORUS,
            PROCEDURAL_EXAMPLE_BOX_FILLET,
            PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
            PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
            PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
            PROCEDURAL_EXAMPLE_BOX_SHELL,
        ];
        let mut signatures = std::collections::BTreeSet::new();
        for example_id in examples {
            let mut app = app().await;
            app.dispatch_typed(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("set example");
            let signature = format!("{:?}", app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect::<std::collections::BTreeSet<_>>());
            assert!(signatures.insert(signature.clone()), "duplicate fixture signature for {example_id}: {signature}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn refresh_pending_effects_arms_flow_eval_tick_chain() {
        let _serial = test_support::lock();
        let mut app = app().await;
        app.dispatch_typed(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }), &semio_framework_plugin::testkit::meta("local"))
            .await
            .expect("set example");
        let effects = app.pending_effects().await;
        assert!(effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")));
        drain_flow_eval_ticks(&mut app).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trips_flow_graph_edits() {
        let _serial = test_support::lock();
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(
            &mut app,
            Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }),
            |app| app.snapshot().expect("snapshot").fixture.widgets.len(),
            before,
            before + 1,
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_widget_moves() {
        let _serial = test_support::lock();
        let widgets: Vec<String> = app().await.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::generation3d::widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<Generation3dPlayApp>, (Option<f64>, Option<f64>)>(
            "mem://generation3d-convergence",
            Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
            Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
            move |app| {
                let layout = &app.snapshot().expect("snapshot").fixture.layout;
                (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
            },
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn generation3d_labels_translate_catalogue_and_inspector_in_german() {
        let _serial = test_support::lock();
        let mut app = app().await;
        app.dispatch_typed(Generation3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("set locale");
        let catalogue = testkit::render(&mut app, catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE).await;
        assert!(catalogue.contains("\"Elemente\""));
        let inspector = testkit::render(&mut app, inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION).await;
        assert!(inspector.contains("Elemente:"));
    }

    /// 🕹️ The runtime graph-selection route persists through an exactly owned interaction store.
    #[semio_framework_async_macros::async_test]
    async fn generation3d_interaction_selection_owns_its_persisted_history() {
        let _serial = test_support::lock();
        let mut app = app_with_registry().await;
        let node_id = app
            .snapshot()
            .expect("snapshot")
            .fixture
            .widgets
            .first()
            .map(crate::artifacts::generation3d::widget_id)
            .expect("default fixture node")
            .to_string();
        let targets = serde_json::to_string(&vec![semio_framework_plugin::InteractionTarget { granularity: "node".into(), id: node_id.clone() }]).expect("selection targets");
        let args = serde_json::json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" });
        app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&args), &semio_framework_plugin::testkit::meta("local"))
            .await
            .expect("interaction selection persists");
        assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM,
    /// same discovered gap as `render`), so `has_selection` is always false now and the destructive
    /// `delete-selection` row (conditioned on a real selection) never appears; this test now only pins
    /// the disclosure budget.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_grouped_disclosure_stays_within_budget() {
        let _serial = test_support::lock();
        let mut app = app_with_registry().await;
        let widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::generation3d::widget_id(widget).to_string()).collect();
        assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request).await;
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        assert!(!menu.is_empty(), "grouped disclosure menu should not be empty");
    }

    #[semio_framework_async_macros::async_test]
    async fn sun_measures_are_exposed_on_preview_windows() {
        let _serial = test_support::lock();
        let mut app = app().await;
        let measures = app.window_measures().await;
        assert!(measures.contains_key(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW));
        assert!(measures.contains_key(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW));
    }

    //#region 🔖️EngineComputeTests
    use std::sync::MutexGuard;
    /// 🧬️ Rehomed verbatim from the deleted `⚙️engine` (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — these tests exercise
    /// `PreviewPipeline`/`MeshBridge` functions above, all of which are app
    /// behavior (they construct or take a [`Generation3dConfig`]), so the tests travel with them.
    use ui_wgpu::wgpu::kernel_3d_scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Vec3};

    fn test_serial() -> MutexGuard<'static, ()> {
        test_support::lock()
    }

    /// 🌉️ Decodes one preview mesh record's `data` field (a `pack::json`/`serde_json::Value` fragment
    /// off the wire) into `MeshData` through its first-party `FromValue` codec — `MeshData` no longer
    /// derives `serde::Deserialize` in a non-`#[cfg(test)]` build of its own crate (`🏗️mesh-engine/🦀️.rs`'s
    /// `#[cfg_attr(test, derive(Serialize, Deserialize))]` only activates inside that crate's OWN test
    /// build, never for a downstream consumer like this one), so `serde_json::from_value` cannot reach
    /// it here; round-trip through the wire text and `dsl::json::from_json_str` instead, exactly like
    /// production's `mesh_data_for_preview_handle` (above) does.
    fn mesh_data_from_json(value: &Value) -> semio_framework_plugin::MeshData {
        dsl::json::from_json_str(&serde_json::to_string(value).expect("mesh data json text")).expect("mesh data")
    }

    /// 🌉️ `Mesh3d` (a plain positions/normals/indices struct) no longer exists —
    /// `ui_wgpu::wgpu::kernel_3d_scene`'s mesh API is now a generation/revision-keyed write-token/lease
    /// pair (`mesh3d_begin`/`mesh3d_write_vec3`/`mesh3d_seal`) meant for the shared render-owned mesh
    /// arena, not for a one-off AABB check. This test only ever needed the bounding box of the raw
    /// position buffer, so compute it directly instead of standing up a lease.
    fn aabb_of_positions(positions: &[f32]) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for vertex in positions.chunks_exact(3) {
            for axis in 0..3 {
                min[axis] = min[axis].min(vertex[axis]);
                max[axis] = max[axis].max(vertex[axis]);
            }
        }
        (min, max)
    }

    fn preview_payload_from_evaluated_fixture(fixture: &flow::FlowFixture, cfg: &Generation3dConfig) -> (String, String) {
        let mut host = flow::FlowHost::from_fixture(fixture.clone());
        host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
        let eval_json = host.evaluate().unwrap_or_default();
        preview_payload_from_eval(&eval_json, fixture, cfg)
    }

    #[test]
    fn preview_payload_has_meshes_and_instances() {
        let _serial = test_serial();
        let projection = crate::artifacts::generation3d::schema::default_snapshot();
        let config = Generation3dConfig::default();
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        assert_ne!(meshes_json, "[]", "meshes_json was empty");
        assert_ne!(instances_json, "[]", "instances_json was empty");
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
        let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
        assert!(!meshes.is_empty());
        assert!(!instances.is_empty());
        for mesh in &meshes {
            let id = mesh.get("id").and_then(|value| value.as_str()).unwrap_or("");
            assert!(id.starts_with("eval-"), "mesh id must be tessellated eval handle, got {id}");
            let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
            assert!(data.positions.len() >= 9, "mesh has too few positions");
            assert!(data.indices.len() >= 3, "mesh has too few indices");
            assert!(!data.edge_positions.is_empty(), "brep preview should include edge geometry");
        }
        let camera = Camera3d {
            position: Vec3::from_array([config.preview_camera.position[0] as f32, config.preview_camera.position[1] as f32, config.preview_camera.position[2] as f32]),
            target: Vec3::from_array([config.preview_camera.target[0] as f32, config.preview_camera.target[1] as f32, config.preview_camera.target[2] as f32]),
            up: Vec3::new(0.0, 0.0, 1.0),
            fov_y: config.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
            near: 0.1,
            far: 1000.0,
        };
        let view_proj = camera.view_proj(0.6);
        let planes = frustum_planes(view_proj);
        let mut visible = 0usize;
        for instance in instances {
            let mesh_id = instance.get("meshId").or_else(|| instance.get("mesh_id")).and_then(|value| value.as_str()).unwrap_or("eval-missing");
            let mesh = meshes.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id)).expect("mesh record");
            let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
            let (mesh_aabb_min, mesh_aabb_max) = aabb_of_positions(&data.positions);
            let position = instance.get("position").and_then(|value| value.as_array()).map_or([0.0, 0.0, 0.0], |items| [items[0].as_f64().unwrap_or(0.0) as f32, items[1].as_f64().unwrap_or(0.0) as f32, items[2].as_f64().unwrap_or(0.0) as f32]);
            assert_eq!(position, [0.0, 0.0, 0.0], "preview instances stay in world space");
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
            let (min, max) = transform_aabb(model, mesh_aabb_min, mesh_aabb_max);
            if aabb_intersects_frustum(&planes, min, max) {
                visible += 1;
            }
        }
        assert!(visible > 0, "no preview instances intersect camera frustum");
    }

    #[test]
    fn document_from_mesh_returns_valid_default_snapshot() {
        let _serial = test_serial();
        let mesh = semio_framework_plugin::MeshData::default();
        let document = generation3d_document_from_mesh(&mesh).expect("dwg mesh import document");
        let projection: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&document)).expect("parseable projection");
        assert_eq!(projection.fixture.schema, "flow.fixture");
    }

    #[test]
    fn generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
        let _serial = test_serial();
        use semio_framework_plugin::{GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter};
        let document_json: Value = serde_json::from_str(&dsl::json::to_json_string(&crate::artifacts::generation3d::schema::default_snapshot())).expect("projection json");
        let mesh = generation3d_mesh_from_document(&dsl::DslValue::from(&document_json)).expect("mesh from document");
        assert!(!mesh.positions.is_empty());

        let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
        let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
        let obj_document = generation3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
        let _: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&obj_document)).expect("parseable obj projection");

        let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
        let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
        let glb_document = generation3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
        let _: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&glb_document)).expect("parseable glb projection");

        let stl_bytes = StlExporter.export(&mesh).expect("stl export");
        let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
        let stl_document = generation3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
        let _: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&stl_document)).expect("parseable stl projection");
    }

    #[test]
    fn rectangle_wire_preview_emits_edge_only_mesh() {
        let _serial = test_serial();
        let projection = Generation3dSnapshot::parse_dsl(crate::artifacts::generation3d::dsl::GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
        let config = Generation3dConfig::default();
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty(), "rectangle wire preview should tessellate curve edges");
        let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
        assert!(data.indices.is_empty(), "wire preview has no shaded triangles");
        assert!(data.edge_positions.len() >= 6, "curve preview should include edge polylines");
        assert!(!instances_json.is_empty());
    }

    #[test]
    fn all_bundled_examples_emit_preview_meshes() {
        let _serial = test_serial();
        let config = Generation3dConfig::default();
        let cases = [
            ("hexagonal-mushroom-column", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN),
            ("rectangle-extrude-volume", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE),
            ("sphere-cut-with-torus", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS),
            ("box-fillet-preview", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_BOX_FILLET),
            ("sphere-box-fuse", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE),
            ("face-sweep-extrude", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE),
            ("rectangle-wire-preview", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE),
            ("box-shell-preview", crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_BOX_SHELL),
        ];
        for (label, example_id) in cases {
            let projection = crate::artifacts::generation3d::schema::example_snapshot(example_id).unwrap_or_else(|| panic!("{label}: missing projection"));
            let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
            assert_ne!(meshes_json, "[]", "{label}: meshes empty; eval may have failed");
            assert_ne!(instances_json, "[]", "{label}: instances empty");
            let meshes: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_else(|err| panic!("{label}: meshes json: {err}"));
            assert!(!meshes.is_empty(), "{label}: no mesh entries");
        }
    }

    #[test]
    fn preview_tolerance_follows_lod_mode() {
        assert!((preview_tolerance("coarse") - 0.15).abs() < 1e-9);
        assert!((preview_tolerance("fine") - 0.02).abs() < 1e-9);
        assert!((preview_tolerance("") - 0.05).abs() < 1e-9);
    }

    #[test]
    fn wireframe_show_mode_strips_shaded_triangles() {
        let _serial = test_serial();
        let projection = crate::artifacts::generation3d::schema::default_snapshot();
        let config = Generation3dConfig { show_mode: "wireframe".into(), ..Default::default() };
        let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty());
        let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
        assert!(data.indices.is_empty());
        assert!(!data.edge_positions.is_empty());
    }

    #[test]
    fn generation3d_io_declares_the_params_and_geometry_ports() {
        let io = semio_framework::io::resolve_ready(generation3d_io());
        assert_eq!(io.document_schema, "generation.3d");
        assert_eq!(io.artifact.id, "3d.generation");
        let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert_eq!(params.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(!params.required);
        let geometry = io.ports.iter().find(|port| port.id == "geometry:out").expect("geometry:out declared");
        assert_eq!(geometry.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(geometry.kind_id.as_deref(), Some("3d.mesh"));
        assert_eq!(geometry.multiplicity, semio_framework::PortMultiplicity::Many);
    }

    /// 🔌️ One `preview: true` neuron with two output channels (a point channel and a vector
    /// channel — neither needs a brep kernel) must yield one instance PER CHANNEL, each id
    /// qualified with its own channel, not one flattened instance for the whole widget.
    fn preview_widget_fixture(id: &str, output_ports: Vec<String>) -> flow::FlowFixture {
        let widget = flow::Widget::Neuron { id: id.into(), neuron_kind: "test.multi".into(), params: flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports, preview: true };
        flow::FlowFixture { schema: "flow.fixture".into(), camera: flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets: vec![widget], synapses: Vec::new(), layout: Default::default() }
    }

    #[test]
    fn preview_payload_channel_qualifies_ids_across_two_output_channels() {
        let _serial = test_serial();
        let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
        let eval_json = json!({
            "multi": {
                "out": {
                    "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
                    "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
                }
            }
        })
        .to_string();
        let config = Generation3dConfig::default();
        let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
        let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
        assert_eq!(instances.len(), 2, "two output channels should yield two preview instances, got {instances:?}");
        let ids: std::collections::HashSet<&str> = instances.iter().filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
        assert!(ids.contains("multi@a#0"), "point-channel instance id missing, got {ids:?}");
        assert!(ids.contains("multi@b#0"), "vector-channel instance id missing, got {ids:?}");
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
        assert_eq!(meshes.len(), 2, "each inline channel mints its own mesh, got {meshes:?}");
    }

    /// 🔌️ A single channel whose value is a `$schema: "list"` dictionary (the wire form
    /// `flow::neural::Dictionary` lists actually take) of N geometry-bearing entries must flatten
    /// to N instances, indexed `#0..#{N-1}` in list order — proven here with inline points so the
    /// test needs no brep kernel/session.
    #[test]
    fn preview_payload_flattens_a_list_channel_into_indexed_instances() {
        let _serial = test_serial();
        let fixture = preview_widget_fixture("listy", vec!["points".into()]);
        let eval_json = json!({
            "listy": {
                "out": {
                    "points": {
                        "$schema": "list",
                        "0": { "$schema": "point", "x": 1.0, "y": 0.0, "z": 0.0 },
                        "1": { "$schema": "point", "x": 2.0, "y": 0.0, "z": 0.0 },
                        "2": { "$schema": "point", "x": 3.0, "y": 0.0, "z": 0.0 }
                    }
                }
            }
        })
        .to_string();
        let config = Generation3dConfig::default();
        let (_meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
        let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
        assert_eq!(instances.len(), 3, "a 3-entry list channel should yield 3 instances, got {instances:?}");
        for index in 0..3 {
            let expected_id = format!("listy@points#{index}");
            assert!(instances.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some(expected_id.as_str())), "missing {expected_id} in {instances:?}");
        }
    }

    /// 🔌️ A channel carrying only pure data (a number, no handle, no `x`/`y`/`z`) is not
    /// geometry-bearing and must not fabricate a placeholder preview instance.
    #[test]
    fn preview_payload_emits_no_instance_for_a_pure_data_channel() {
        let _serial = test_serial();
        let fixture = preview_widget_fixture("scalar", vec!["value".into()]);
        let eval_json = json!({
            "scalar": {
                "out": {
                    "value": { "$schema": "number", "value": 42.0 }
                }
            }
        })
        .to_string();
        let config = Generation3dConfig::default();
        let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
        assert_eq!(meshes_json, "[]", "pure-data channel must not fabricate mesh geometry");
        assert_eq!(instances_json, "[]", "pure-data channel must not fabricate a preview instance");
    }
    /// 🕹️ The three id forms one mark can take, and the transitive reach of each: a node-level
    /// mark covers every channel and every instance below it, a channel-level mark covers only its
    /// own channel, and an instance-level mark covers only itself.
    #[test]
    fn preview_marks_resolve_node_channel_and_instance_ids() {
        let node = PreviewInteractionMarks { hovered: ["multi".to_string()].into_iter().collect(), selected: Default::default() };
        assert!(node.hovers("multi", "a", 0) && node.hovers("multi", "b", 3));
        assert!(!node.hovers("other", "a", 0));

        let channel = PreviewInteractionMarks { hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
        assert!(channel.hovers("multi", "b", 0) && channel.hovers("multi", "b", 7));
        assert!(!channel.hovers("multi", "a", 0));

        let instance = PreviewInteractionMarks { hovered: ["multi@b#2".to_string()].into_iter().collect(), selected: Default::default() };
        assert!(instance.hovers("multi", "b", 2));
        assert!(!instance.hovers("multi", "b", 1));
    }

    /// 🕹️ Graph → world: hovering the NODE in the node graph must light up every one of its
    /// channels' preview geometry, not just one.
    #[test]
    fn preview_payload_marks_every_channel_of_a_hovered_node() {
        let _serial = test_serial();
        let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
        let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
        .to_string();
        let marks = PreviewInteractionMarks { hovered: ["multi".to_string()].into_iter().collect(), selected: ["multi@a".to_string()].into_iter().collect() };
        let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
        let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
        assert_eq!(instances.len(), 2);
        assert!(instances.iter().all(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)), "node hover must reach every channel: {instances:?}");
        assert_eq!(payload.selected_ids, vec!["multi@a#0".to_string()], "channel-level selection must not spill onto the sibling channel");
        assert!(payload.hovered_id.is_some(), "the scene needs a concrete hovered instance to paint");
    }

    /// 🕹️ Graph → world, narrowed: hovering one PORT lights up only that channel's geometry.
    #[test]
    fn preview_payload_marks_only_the_hovered_channel() {
        let _serial = test_serial();
        let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
        let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
        .to_string();
        let marks = PreviewInteractionMarks { hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
        let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
        let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
        let hovered: Vec<&str> = instances.iter().filter(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)).filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
        assert_eq!(hovered, vec!["multi@b#0"], "only the hovered channel may light up: {instances:?}");
        assert_eq!(payload.hovered_id.as_deref(), Some("multi@b#0"));
    }

    /// 🕹️ World → graph: hovering one preview INSTANCE in the 3D world resolves back to its node
    /// and its port, which is what the node-graph window paints.
    #[test]
    fn graph_marks_project_instance_hover_back_onto_its_node_and_port() {
        let marks = PreviewInteractionMarks { hovered: ["multi@b#0".to_string()].into_iter().collect(), selected: ["multi@a#1".to_string()].into_iter().collect() };
        assert_eq!(marks.hovered_graph_target(), Some(("multi".to_string(), Some("b".to_string()))));
        assert!(marks.graph_highlight_ids().contains(&"multi".to_string()));
        assert_eq!(marks.graph_selection_ids(), vec!["multi".to_string()]);
        assert_eq!(PreviewInteractionMarks::widget_of("multi@b#0"), "multi");
        assert_eq!(PreviewInteractionMarks::port_of("multi@b#0"), Some("b"));
        assert_eq!(PreviewInteractionMarks::port_of("multi"), None);
    }

    /// 🕸️ Every port the node graph paints is also an interaction target parented to its widget —
    /// the topology link `HoverSpec { transitive: true }` walks.
    #[test]
    fn interaction_topology_ports_match_the_node_graph_port_ids() {
        let _serial = test_serial();
        let projection = crate::artifacts::generation3d::schema::default_snapshot();
        let ports_by_node = generation3d_port_ids_by_node(&projection.fixture);
        assert!(!ports_by_node.is_empty(), "default fixture should project graph nodes");
        assert!(ports_by_node.values().any(|ports| !ports.is_empty()), "default fixture should project at least one port");
        for (node_id, ports) in &ports_by_node {
            for port in ports {
                assert!(port.starts_with(&format!("{node_id}@")), "port {port} must be qualified by its node {node_id}");
                assert_eq!(PreviewInteractionMarks::widget_of(port), node_id.as_str());
            }
        }
    }
    /// 👁️ Which widget kinds contribute preview geometry: a neuron only when its author-set toggle
    /// is on, an output preview always, a cluster always (it has no toggle of its own, and its
    /// inner `flow::neural::Neuron`s have none either), and a pure input widget never.
    #[test]
    fn widget_preview_eligibility_covers_neurons_output_previews_and_clusters() {
        let on = flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: true };
        let off = flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: false };
        let output = flow::Widget::OutputPreview { id: "p".into(), preview: Default::default(), expanded: Default::default() };
        let cluster = flow::Widget::Cluster { id: "c".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() };
        let slider = flow::Widget::InputSlider { id: "s".into(), label: "S".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 };
        assert!(widget_previews(&on));
        assert!(!widget_previews(&off));
        assert!(widget_previews(&output));
        assert!(widget_previews(&cluster));
        assert!(!widget_previews(&slider));
    }
    //#endregion 🔖️EngineComputeTests

    //#region 🔖️ExamplesTests
    /// 📚️ Ticket 26/09/03/PROCEDURAL-3D-END-TO-END — `examples()` (wired at the plugin root via
    /// `.editor_with_examples::<Generation3dPlayApp>(create_generation3d_app(), …examples())`) must
    /// carry the same eight ids, in the same order, as the `setActiveExample` select options this app
    /// declares — otherwise the navbar dropdown and the action's own arg picker disagree.
    #[test]
    fn examples_match_set_active_example_select_options() {
        let definition = create_generation3d_app();
        let select_ids: Vec<String> = definition
            .window_kinds
            .iter()
            .flat_map(|window| window.actions.iter())
            .find(|action| action.id == "setActiveExample")
            .and_then(|action| action.args.first())
            .and_then(|arg| match arg.control() {
                semio_framework::ActionArgControl::Select { options } => Some(options.into_iter().map(|option| option.value).collect::<Vec<_>>()),
                _ => None,
            })
            .expect("setActiveExample must declare a Select arg");
        let example_ids: Vec<String> = examples().into_iter().map(|source| source.id().to_string()).collect();
        assert_eq!(example_ids.len(), 8);
        assert_eq!(example_ids, select_ids);
    }
    //#endregion 🔖️ExamplesTests
}
//#endregion 🧪️Tests
