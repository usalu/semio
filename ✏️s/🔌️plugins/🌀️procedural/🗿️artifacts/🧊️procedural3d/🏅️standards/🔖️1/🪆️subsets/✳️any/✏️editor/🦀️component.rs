//! 🧱️ Procedural3d editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::{artifact_kind, Procedural3dSnapshot, PROCEDURAL_3D_SCHEMA};
use crate::editor::procedural3d::commands::{
    add_generation, add_widget, delete_selection, flow_eval_resolve, flow_eval_tick, flow_tessellate_resolve, graph_pointer_down, move_media_node, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_generation, remove_widget,
    rename_generation, reorganize, rotate_selection, scale_selection, select_generation, set_active_example, set_active_utility, set_camera, set_locale, set_lod_mode, set_show_mode, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun,
    translate_selection, update_generation_values, world_pointer_down,
};
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::editor::procedural3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::editor::procedural3d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::editor::procedural3d::modes::{edit, generate};
use crate::editor::procedural3d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::procedural3d::terminology::procedural3d_labels;
use flow::{with_process_flow_eval_session, FlowEvalSession};
// 🚧️ SDK note (ticket 26/08/16 contract §2.1/§2.4): `ArtifactEditor`/`Editor`/`Dialect` are curated at
// `semio_framework_plugin`'s crate root as of W0-F/W2-FIX — imported bare here, no `app::` prefix
// needed (unlike the earlier cad pilot, written before that gap closed). `app::InteractionView` is a
// separate, still-uncurated gap (unrelated to this ticket) — kept qualified.
use semio_framework_plugin::{
    app::InteractionView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, ArtifactEditor, ArtifactView, CommandDefinition, ConfigView, Dialect, DomainTopology, DraftView, Editor, Effect, Emit, Fault, FaultCode,
    FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaType, MergeMode, NoDraft,
    NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UtilityDefinition, WindowMeasure,
};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_APP_ID: &str = "procedural3d-play";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub fn procedural3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PROCEDURAL_3D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
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
    /// 🎯️ `Procedural3dPlayApp::Command` — the SOLE dispatch surface for procedural3d's own behavior,
    /// covering EVERY declared action. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Procedural3dCommand for Procedural3dSnapshot, Procedural3dMutation, Procedural3dConfig, Procedural3dConfigMutation, ctx = FlowEvalSession {
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
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "flowTessellateResolve" as "flow-tessellate-resolve" => flow_tessellate_resolve::FlowTessellateResolve}
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️Procedural3dPlayApp
/// 🧪️ Unit struct apart from `eval_session`: every former runtime field lives in [`Procedural3dConfig`],
/// written through [`Procedural3dConfigMutation`]s.
#[derive(Default)]
pub struct Procedural3dPlayApp;

/// 🎥️ Parses the flow-graph camera out of `command_from_action`'s JSON args — either a nested
/// `{camera: {...}}` object or flat `x`/`y`/`zoom` keys.
fn parse_flow_camera_json(args: &Value) -> flow::CameraJson {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = serde_json::from_value::<flow::CameraJson>(camera.clone()) {
            return parsed;
        }
    }
    flow::CameraJson { x: args.get("x").and_then(Value::as_f64).unwrap_or(0.0), y: args.get("y").and_then(Value::as_f64).unwrap_or(0.0), zoom: args.get("zoom").and_then(Value::as_f64).unwrap_or(1.0) }
}

/// 🎥️ Parses the 3D preview camera out of `command_from_action`'s JSON args; falls back to the default
/// camera on any malformed/missing `camera` object.
fn parse_preview_camera_json(args: &Value) -> crate::editor::procedural3d::config::Procedural3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = serde_json::from_value::<crate::editor::procedural3d::config::Procedural3dPreviewCamera>(camera.clone()) {
            return parsed;
        }
    }
    crate::editor::procedural3d::config::Procedural3dPreviewCamera::default()
}

impl ArtifactEditor for Procedural3dPlayApp {
    type Snapshot = Procedural3dSnapshot;
    type Mutation = Procedural3dMutation;
    type Config = Procedural3dConfig;
    type ConfigMutation = Procedural3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::procedural3d::presence::Procedural3dPresence;
    type PresenceMutation = crate::editor::procedural3d::presence::Procedural3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Procedural3dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::procedural3d::spr::procedural3d_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::procedural3d::spr::procedural3d_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::procedural3d::spr::procedural3d_document_store_initialization_job(envelope, operation, generation))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        crate::artifacts::procedural3d::spr::procedural3d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Procedural3d atomic publication authority is absent or stale"))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    const DIALECT: Dialect = crate::artifacts::procedural3d::PROCEDURAL3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PROCEDURAL_3D_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Procedural3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.procedural.procedural3d@1/*#editor",
        document_schema: "procedural.3d",
        factory: "BoundedFirstStepCommandJobFactory",
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
            "flowEvalTick" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::procedural3d::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> Procedural3dSnapshot {
        crate::artifacts::procedural3d::schema::default_snapshot()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(procedural3d_io().await)
    }

    /// 🎞️ `geometry:out` plus the inherited `document:out` default, replicated inline (overriding
    /// `export_media` shadows the trait's provided body for every port on this app).
    async fn export_media(port: &str, doc: &ArtifactView<'_, Procedural3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "geometry:out" => {
                let mesh = export_mesh_from_document(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                    payload: semio_framework_plugin::MediaPayload::Structured { schema: "3d.mesh".into(), json: serde_json::to_string(&mesh).unwrap_or_default() },
                })
            }
            "document:out" => {
                let media_type = Self::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media { media_type, payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"` — patches matching `InputSlider` widgets from a `{widgetId: number}` JSON
    /// object; unmatched keys/non-slider widgets are silently ignored.
    async fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Procedural3dSnapshot>) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "params:in" => {
                let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "params:in importer only accepts a Structured JSON object payload".into()));
                };
                let object: serde_json::Map<String, Value> = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture = &doc.snapshot.fixture;
                let mut operations = Vec::new();
                for (target_id, value) in &object {
                    let Some(number) = value.as_f64() else { continue };
                    let Some((_index, widget)) = fixture.widgets.iter().enumerate().find(|(_, widget)| crate::artifacts::procedural3d::widget_id(widget) == target_id) else { continue };
                    if let flow::Widget::InputSlider { id, min, max, step, .. } = widget {
                        operations.push(Procedural3dMutation::UpdateWidget(crate::artifacts::procedural3d::schema::mutations::update_widget::mutation::UpdateWidget {
                            widget: flow::Widget::InputSlider { id: id.clone(), value: number, min: *min, max: *max, step: *step },
                        }));
                    }
                }
                Ok(Emit::mutations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    async fn command_id(command: &Procedural3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Procedural3dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    async fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(Value::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "setActiveExample" => Ok(Procedural3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(Procedural3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(|value| value.to_string())).unwrap_or_else(|| "[]".into()),
            })),
            "deleteSelection" => Ok(Procedural3dCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "removeWidget" => Ok(Procedural3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "moveMediaNode" => Ok(Procedural3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "addWidget" => Ok(Procedural3dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), x: f64_arg(&["x"]), y: f64_arg(&["y"]) })),
            "patchFlowWidgets" => Ok(Procedural3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets {
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
            "reorganize" => Ok(Procedural3dCommand::Reorganize(reorganize::Reorganize {})),
            "translateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Procedural3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids, dx: f64_arg(&["dx"]).unwrap_or(0.0), dy: f64_arg(&["dy"]).unwrap_or(0.0), dz: f64_arg(&["dz"]).unwrap_or(0.0) }))
            }
            "rotateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Procedural3dCommand::RotateSelection(rotate_selection::RotateSelection {
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
                Ok(Procedural3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids, sx: f64_arg(&["sx"]).unwrap_or(1.0), sy: f64_arg(&["sy"]).unwrap_or(1.0), sz: f64_arg(&["sz"]).unwrap_or(1.0) }))
            }
            "addGeneration" => Ok(Procedural3dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Procedural3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Procedural3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").map_or(dsl::DslValue::Null, |entry| dsl::to_dsl_value(entry).unwrap_or(dsl::DslValue::Null));
                Ok(Procedural3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => Ok(Procedural3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: parse_flow_camera_json(&args) })),
            "worldPointerDown" => Ok(Procedural3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {})),
            "graphPointerDown" => Ok(Procedural3dCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {})),
            "setLodMode" => Ok(Procedural3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setShowMode" => Ok(Procedural3dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() })),
            "toggleSun" => Ok(Procedural3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Procedural3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Procedural3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Procedural3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setCamera" => Ok(Procedural3dCommand::SetCamera(set_camera::SetCamera { camera: parse_preview_camera_json(&args) })),
            "selectGeneration" => Ok(Procedural3dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "setActiveUtility" => Ok(Procedural3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: str_arg(&["utilityId", "utility_id"]).unwrap_or_default() })),
            "setLocale" => Ok(Procedural3dCommand::SetLocale(set_locale::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() })),
            "flowEvalTick" => Ok(Procedural3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {})),
            "flowEvalResolve" => Ok(Procedural3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve {
                node_hash: args.get("nodeHash").or_else(|| args.get("node_hash")).and_then(Value::as_u64).unwrap_or(0),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_else(|| "{}".into()),
            })),
            "flowTessellateResolve" => Ok(Procedural3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve {
                node_hash: args.get("nodeHash").or_else(|| args.get("node_hash")).and_then(Value::as_u64).unwrap_or(0),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_else(|| "{}".into()),
            })),
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
    async fn handle(
        command: &Procedural3dCommand,
        doc: &ArtifactView<'_, Procedural3dSnapshot>,
        cfg: &ConfigView<'_, Procedural3dConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation, Self::DraftMutation>, Fault> {
        with_process_flow_eval_session(|session| match command {
            Procedural3dCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction, session),
            Procedural3dCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction, session),
            Procedural3dCommand::TranslateSelection(payload) => translate_selection::apply(payload, doc, cfg, interaction, session),
            Procedural3dCommand::RotateSelection(payload) => rotate_selection::apply(payload, doc, cfg, interaction, session),
            Procedural3dCommand::ScaleSelection(payload) => scale_selection::apply(payload, doc, cfg, interaction, session),
            _ => command.dispatch(doc, cfg, session),
        })
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    async fn interaction_topology(doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>) -> InteractionTopology {
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
            let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
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

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes.
    async fn pending_effects(doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>) -> Vec<Effect> {
        with_process_flow_eval_session(|session| {
            let host = flow::flow_host_with_session(&doc.snapshot.fixture, session);
            if session.sync(&host) {
                vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(104), action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
            } else {
                Vec::new()
            }
        })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = procedural3d_labels(config);
        let active_utility = config.active_utility_id.as_str();
        let node = with_process_flow_eval_session(|session| match body_key {
            flow_window::PROCEDURAL_3D_PLAY_BODY_MAIN => flow_window::render(document, config, session),
            edit_preview::PROCEDURAL_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, session, active_utility),
            generations::PROCEDURAL_3D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::default()),
            form::PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => form::render(&document.fixture, &document.generation, labels),
            generate_preview::PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(&document.fixture, &document.generation, config, labels, active_utility),
            document_panel::PROCEDURAL_3D_PLAY_BODY_DOCUMENT => document_panel::render(&document.fixture, labels),
            catalogue_panel::PROCEDURAL_3D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
            // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the
            // widget-details view degrades to its "no selection" default until a future wave threads
            // interaction into render. Flagged as a discovered framework gap, not worked around here.
            inspection_panel::PROCEDURAL_3D_PLAY_BODY_INSPECTION => inspection_panel::render(&document.fixture, &[], labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
        })?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    async fn window_measures(_doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let measures = edit_preview::preview_window_measures(config, procedural3d_action);
        HashMap::from([
            (flow_window::PROCEDURAL_3D_PLAY_WINDOW_MAIN.to_string(), flow_window::window_measures(&config.lod_mode, procedural3d_action)),
            (edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    /// 🗂️ Grouped disclosure: `reorganize`/`translateSelection`/`rotateSelection`/`scaleSelection` stay
    /// top-level; creation, removal and generation methods fold into taxonomy groups; `delete-selection`
    /// stays a direct destructive item last.
    ///
    /// 🕹️ `context_menu` carries no `InteractionView` either (same gap as `render` — see ticket
    /// 26/08/14's w3b-summary.md), so the selection-dependent rows below always take the "nothing
    /// selected" branch rather than reading a stale/wrong selection.
    async fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, Procedural3dSnapshot>,
        cfg: &ConfigView<'_, Procedural3dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let config = cfg.snapshot;
        let labels = procedural3d_labels(config);
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
    }
}
//#endregion 🔖️Procedural3dPlayApp

//#region 🔖️Manifest
pub fn create_procedural3d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::procedural3d::PROCEDURAL3D_DIALECT).document(["semio", "procedural", "3d"])
            .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) }))
            .artifact_kind(artifact_kind())
            .icon_id("workflow")
            .mode_def(edit::definition())
            .mode_def(generate::definition())
            .default_mode_id(edit::PROCEDURAL_3D_PLAY_MODE_EDIT)
            .mode_layout(generate::PROCEDURAL_3D_PLAY_MODE_GENERATE, generate::PROCEDURAL_3D_PLAY_LAYOUT_GENERATE)
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
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
                    ActionArgOption::new(crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
                ]).required(),
            ])
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one domain over the flow-graph widget DAG, node/edge/handle granularities,
            // `HierarchyProvider::Topology` (see `Procedural3dPlayApp::interaction_topology` below) —
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
            .window_kind_interactions(flow_window::PROCEDURAL_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
            .window_kind_interactions(edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph")])
            .window_kind_interactions(generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(semio_framework::io::resolve_ready(Procedural3dPlayApp::config_spec()))
            .io(semio_framework::io::resolve_ready(procedural3d_io()))
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the eight
            // `PROCEDURAL_EXAMPLE_*` app-level example registrations and the no-op
            // `.workflow("procedural3d", …)` call are dropped here (not silently: reported in this
            // packet's migration report). The subset's own `📚️examples/🎬️<slug>` facets (eight real
            // examples, pre-existing) are the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🔖️ArtifactIo
/// 🔌️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// this app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_procedural3d_app` declares via `.artifact_kind(...)`; `params:in`/`geometry:out` are the
/// workflow-specific ports beyond the implicit document in/out ports.
pub async fn procedural3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "procedural.3d",
        MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "3d.procedural".into(), name: "3D Procedural".into(), dimension: "3d".into(), component_kind: "procedural3d".into() },
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
/// every function here references [`Procedural3dConfig`] (directly, or is reachable only from a
/// function that does), which made this app behavior, not artifact-schema-pure document compute; the
/// snapshot-pure fixture/gumball helpers stayed in `crate::artifacts::procedural3d::schema` instead.
pub fn preview_tolerance(lod_mode: &str) -> f64 {
    match lod_mode {
        "coarse" => 0.15,
        "fine" => 0.02,
        _ => 0.05,
    }
}

pub fn preview_camera_json(cfg: &Procedural3dConfig) -> String {
    ui_wgpu::wgpu::world3d_camera_json(cfg.preview_camera.position, cfg.preview_camera.target, cfg.preview_camera.fov)
}

/// 🧭️ World-3d selection payload with the host-owned gumball utility spliced in, so the transform
/// handles follow `cfg.active_utility_id` instead of any document-stored utility.
///
/// 🕹️ `render` carries no `InteractionView` (same discovered framework gap as `context_menu` —
/// see ticket 26/08/14's w3b-summary.md), so this always reports an empty `graph` selection/hover
/// rather than a stale one; the gumball never shows until a future wave threads interaction into
/// `render`. `"rectangle"` (the pre-migration default `selection_method`) is hardcoded — the
/// framework no longer tracks a persistent "last marquee method" outside a live gesture.
pub fn preview_selection_json(cfg: &Procedural3dConfig, active_utility: &str) -> String {
    let mut value: Value = serde_json::from_str(&semio_framework_plugin::world3d_selection_json("rectangle", &[], None)).unwrap_or_else(|_| json!({}));
    let show_mode = if cfg.show_mode.is_empty() { "shaded" } else { cfg.show_mode.as_str() };
    let (show_edges, selection_mode) = match show_mode {
        "wireframe" => (true, "mesh"),
        "points" => (false, "mesh"),
        "shaded+edges" => (true, "mesh"),
        _ => (false, "mesh"),
    };
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode".into(), json!(active_utility));
        object.insert("gumballActive".into(), json!(false));
        object.insert("showEdges".into(), json!(show_edges));
        object.insert("selectionMode".into(), json!(selection_mode));
        object.insert("granularity".into(), json!(selection_mode));
    }
    value.to_string()
}

fn merge_status_json(computing: Option<String>, preview_status: Option<String>) -> Option<String> {
    match (computing, preview_status) {
        (Some(c), Some(p)) => {
            let mut computing_val: Value = serde_json::from_str(&c).unwrap_or(json!({ "computing": true }));
            let preview_val: Value = serde_json::from_str(&p).unwrap_or(json!({}));
            if let (Some(c_obj), Some(p_obj)) = (computing_val.as_object_mut(), preview_val.as_object()) {
                for (k, v) in p_obj {
                    c_obj.insert(k.clone(), v.clone());
                }
            }
            Some(computing_val.to_string())
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

pub fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_geometry_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

pub fn geometry_handles_for_widget(eval: &Value, widget_id: &str) -> Vec<String> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"));
    let Some(channels) = channels else {
        return Vec::new();
    };
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles
}

fn mesh_has_preview_geometry(data: &semio_framework_plugin::MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
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
    let eval: Value = serde_json::from_str(eval_json).ok()?;
    if eval.get("error").and_then(Value::as_str).is_some() {
        return Some(json!({ "error": eval.get("error") }).to_string());
    }
    let mut errors = serde_json::Map::new();
    for widget in &fixture.widgets {
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        let Some(entry) = eval.get(&id) else { continue };
        if let Some(error) = entry.get("error").and_then(Value::as_str) {
            errors.insert(id, Value::String(error.to_string()));
        }
    }
    if errors.is_empty() {
        None
    } else {
        Some(json!({ "widgetErrors": errors }).to_string())
    }
}

/// 🧵️ Pure per-render tessellation: bounded-cost, safe to call fresh on every render call instead of
/// behind an outer memoization layer.
fn mesh_data_for_preview_handle(handle: &str, tolerance: f64, session: Option<&FlowEvalSession>) -> Option<semio_framework_plugin::MeshData> {
    if let Some(session) = session {
        if let Some(json) = session.preview_mesh_json(handle) {
            if let Ok(value) = serde_json::from_str::<Value>(json) {
                if value.get("error").is_none() {
                    if let Ok(data) = serde_json::from_value::<semio_framework_plugin::MeshData>(value) {
                        if mesh_has_preview_geometry(&data) {
                            return Some(data);
                        }
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
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    let mut handles = Vec::new();
    for widget in &fixture.widgets {
        let preview = matches!(widget, flow::Widget::Neuron { preview: true, .. } | flow::Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        for handle in geometry_handles_for_widget(&eval, &id) {
            let ready = session.preview_mesh_json(&handle).and_then(|json| {
                let value = serde_json::from_str::<Value>(json).ok()?;
                if value.get("error").is_some() {
                    return None;
                }
                let data = serde_json::from_value::<semio_framework_plugin::MeshData>(value).ok()?;
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
pub fn preview_tessellate_effects(session: &mut FlowEvalSession, eval_json: &str, fixture: &flow::FlowFixture, cfg: &Procedural3dConfig) -> Vec<Effect> {
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let tolerance_bits = tolerance.to_bits();
    let mut live = std::collections::HashSet::new();
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    for widget in &fixture.widgets {
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        for handle in geometry_handles_for_widget(&eval, &id) {
            live.insert(handle);
        }
    }
    session.retain_preview_meshes(&live);
    let mut effects = Vec::new();
    for handle in pending_preview_tessellate_handles(eval_json, fixture, session) {
        let node_hash = flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone()) {
            effects.push(Effect::InvokeExtension {
                req: semio_framework_plugin::RequestId(105),
                extension_id: "brep".into(),
                capability: "tessellate".into(),
                request_json: json!({ "handle": handle, "tolerance": tolerance, "nodeHash": node_hash }).to_string(),
            });
        }
    }
    effects
}

pub fn preview_payload_from_eval(eval_json: &str, fixture: &flow::FlowFixture, cfg: &Procedural3dConfig) -> (String, String) {
    preview_payload_from_eval_with_session(eval_json, fixture, cfg, None)
}

pub fn preview_payload_from_eval_with_session(eval_json: &str, fixture: &flow::FlowFixture, cfg: &Procedural3dConfig, session: Option<&FlowEvalSession>) -> (String, String) {
    if eval_json.is_empty() {
        return ("[]".into(), "[]".into());
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(eval_json) {
        if parsed.get("error").and_then(Value::as_str).is_some() {
            return ("[]".into(), "[]".into());
        }
    }
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        let preview = matches!(widget, flow::Widget::Neuron { preview: true, .. } | flow::Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let handles = geometry_handles_for_widget(&eval, &id);
        if handles.is_empty() {
            continue;
        }
        // 🕹️ `render` carries no `InteractionView` (see `preview_selection_json`'s doc comment) — no
        // preview instance is ever marked selected/hovered until a future wave threads interaction in.
        let selected = false;
        let hovered = false;
        for (index, handle) in handles.iter().enumerate() {
            let mesh_id = if handles.len() == 1 { format!("eval-{id}") } else { format!("eval-{id}#{index}") };
            let instance_id = if handles.len() == 1 { id.clone() } else { format!("{id}#{index}") };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                if let Some(data) = mesh_data_for_preview_handle(handle, tolerance, session) {
                    let data = apply_show_mode_mesh(data, show_mode);
                    if mesh_has_preview_geometry(&data) {
                        meshes.push(json!({ "id": mesh_id, "data": data }));
                    }
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                instances.push(json!({
                    "id": instance_id,
                    "meshId": mesh_id,
                    "position": [0.0, 0.0, 0.0],
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0],
                    "label": id,
                    "selected": selected,
                    "hovered": hovered}));
            }
        }
    }
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}
//#endregion 🔖️PreviewPipeline

//#region 🔖️MeshBridge
/// 🧊️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the DWG-import mesh bridge: `export_mesh_from_document` builds a default [`Procedural3dConfig`] to
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

pub fn export_mesh_from_document(projection: &Procedural3dSnapshot) -> semio_framework_plugin::MeshData {
    let config = Procedural3dConfig::default();
    let mut host = crate::artifacts::procedural3d::schema::host_from_fixture(&projection.fixture);
    let eval_json = host.evaluate().unwrap_or_default();
    let (meshes_json, _) = preview_payload_from_eval(&eval_json, &projection.fixture, &config);
    let meshes: Vec<semio_framework_plugin::MeshData> = serde_json::from_str::<Vec<Value>>(&meshes_json).unwrap_or_default().into_iter().filter_map(|entry| serde_json::from_value(entry.get("data").cloned().unwrap_or(Value::Null)).ok()).collect();
    merge_preview_meshes(&meshes)
}

pub fn procedural3d_mesh_from_document(doc: &Value) -> Result<semio_framework_plugin::MeshData, String> {
    let projection: Procedural3dSnapshot = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_document(&projection))
}

pub fn procedural3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(crate::artifacts::procedural3d::schema::default_snapshot()).map_err(|err| err.to_string())
}

//#endregion 🔖️MeshBridge

//#region 🧪️TestSupport
/// 🧵️ `tessellate_geometry` (flow core brep geometry session) (and the flow-eval neuron kernel cache it sits behind)
/// is a process-wide cache shared by every test in this ONE merged crate — before the crate
/// consolidation, the artifact/app constitutional crates each ran in their own `cargo test` process, so
/// a `TEST_SERIAL` local to one of them never had to coordinate with the other's. Now that every
/// taxonomy node's tests share one test binary, ANY test that evaluates a flow fixture and/or tessellates
/// BRep geometry (directly here, or indirectly via the app's preview-window `render()`) must acquire
/// THIS single crate-wide lock — see `crate::editor::procedural3d::modes::edit::windows::preview`'s test
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

    /// ✏️ `Procedural3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Procedural3dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Procedural3dPlayApp>` builds it.
    pub type Procedural3dApp = VcsArtifactApp<EditorApp<Procedural3dPlayApp>>;

    /// ✏️ Adapts `create_procedural3d_app`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` /
    /// `testkit::new_app_with_registry` still expect — framework testkit gap, not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub fn procedural3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_procedural3d_app(), examples: Vec::new() }
    }

    pub fn app() -> Procedural3dApp {
        new_app::<EditorApp<Procedural3dPlayApp>>()
    }

    pub fn app_with_registry() -> Procedural3dApp {
        new_app_with_registry::<EditorApp<Procedural3dPlayApp>>(procedural3d_app_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut Procedural3dApp, command: Procedural3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Procedural3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
    /// drains in production — a test has to do that draining itself.
    pub fn drain_flow_eval_ticks(app: &mut Procedural3dApp) {
        app.pending_effects();
        for _ in 0..1000 {
            let result = app.dispatch_typed(Procedural3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}), &meta("local")).expect("flowEvalTick");
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
    use crate::editor::procedural3d::testkit::{app, app_with_registry, drain_flow_eval_ticks};
    use semio_framework_plugin::PluginApp;
    fn production_initial_snapshot(label: &str) -> Procedural3dSnapshot {
        let mut snapshot = Procedural3dSnapshot::default();
        snapshot.fixture.schema = label.into();
        for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
            snapshot.fixture.widgets.push(flow::Widget::InputNote { id: id.into(), text: text.into() });
        }
        snapshot.fixture.synapses.push(flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
        snapshot.fixture.synapses.push(flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
        snapshot.fixture.layout.insert("move-target".into(), flow::WidgetLayout { x: 1.0, y: 2.0 });
        snapshot.fixture.layout.insert("clear-target".into(), flow::WidgetLayout { x: 3.0, y: 4.0 });
        for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
            snapshot.generation.generations.push(flow::playbook::FormGeneration { id: id.into(), name: name.into(), values: serde_json::Map::new() });
        }
        snapshot.generation.selected_generation_id = Some("rename-generation".into());
        snapshot
    }

    fn production_mutations() -> Vec<Procedural3dMutation> {
        use crate::artifacts::procedural3d::mutations::*;
        let params = flow::neural::Dictionary::new()
            .insert("integer", flow::neural::Value::Atom(flow::neural::Atom::Integer(7)))
            .insert("nested", flow::neural::Value::Dictionary(flow::neural::Dictionary::new().insert("text", flow::neural::Value::Atom(flow::neural::Atom::String("production".into())))));
        vec![
            create_widget(0, flow::Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true }),
            update_widget(flow::Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() }),
            delete_widget("delete-target".into()),
            connect_synapse(0, flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() }),
            update_synapse(flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() }),
            disconnect_synapse("disconnect-synapse".into()),
            move_widget("move-target".into(), flow::WidgetLayout { x: 31.0, y: -17.0 }),
            delete_widget_position("clear-target".into()),
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
        value.try_reserve_exact(bytes.len() * 2).expect("P3 production hex preflight");
        for byte in bytes {
            value.push(char::from(DIGITS[usize::from(byte >> 4)]));
            value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
        }
        value
    }

    fn production_semantic_digest(snapshot: &Procedural3dSnapshot) -> [u8; 32] {
        let mut digest = store::ArtifactStoreInitializationDigest::new(b"procedural3d.production-law.semantic");
        digest.observe(&crate::artifacts::procedural3d::snapshot::binary::encode(snapshot));
        digest.finish()
    }

    fn production_envelope_wire(label: &str) -> (Vec<u8>, Procedural3dSnapshot, [u8; 32]) {
        let snapshot = production_initial_snapshot(label);
        let mutations = production_mutations();
        assert_eq!(mutations.len(), 14, "production ingress carries every P3 mutation variant including delete-widget-position");
        let mut mutation_hex = Vec::new();
        mutation_hex.try_reserve_exact(mutations.len()).expect("P3 production mutation owner preflight");
        for mutation in &mutations {
            mutation_hex.push(production_hex(&crate::artifacts::procedural3d::spr::encode_op(mutation).expect("P3 production mutation encoding")));
        }
        let mut expected = production_initial_snapshot(label);
        crate::artifacts::procedural3d::spr::procedural3d_apply_retained_mutations_for_test(&mut expected, &mutations);
        let expected_digest = production_semantic_digest(&expected);
        let wire = serde_json::to_vec(&serde_json::json!({
            "schema": crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA,
            "id": "procedural3d-production-mounted-law",
            "vcs": {
                "initialSnapshot": production_hex(&crate::artifacts::procedural3d::snapshot::binary::encode(&snapshot)),
                "edits": [{
                    "id": "procedural3d-production-all14-edit",
                    "actor": "procedural3d-production-law",
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

    fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Procedural3dPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P3 production ingress credits");
        crate::artifacts::procedural3d::spr::procedural3d_admit_publication_authority(
            handle.operation,
            handle.generation,
            handle.generation.0,
            handle.generation.0,
            handle.generation.0,
            8_192,
            crate::artifacts::procedural3d::spr::PROCEDURAL3D_MOUNTED_OUTPUT_CHANNELS,
            crate::artifacts::procedural3d::spr::PROCEDURAL3D_MOUNTED_CONTROL_CREDITS,
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
        app: &mut semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Procedural3dPlayApp>>,
        handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle,
    ) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..300_000 {
            crate::artifacts::procedural3d::spr::procedural3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0).expect("P3 authority refresh immediately before production maintenance");
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
        let mut accepted = semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::EditorApp<Procedural3dPlayApp>>::new(semio_framework_plugin::EditorApp::default()).await;
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
        assert!(crate::artifacts::procedural3d::spr::procedural3d_release_publication_authority(handle.operation, handle.generation));

        use crate::artifacts::procedural3d::spr::Procedural3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
        for (hostile, expected_code) in [
            (Missing, "procedural3d-publication.authority-missing"),
            (WrongOperation, "procedural3d-publication.wrong-operation"),
            (WrongGeneration, "procedural3d-publication.wrong-generation"),
            (WrongBase, "procedural3d-publication.wrong-base"),
            (WrongParent, "procedural3d-publication.wrong-parent"),
        ] {
            let mut app = semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::EditorApp<Procedural3dPlayApp>>::new(semio_framework_plugin::EditorApp::default()).await;
            let last_valid = app.snapshot().await.expect("last-valid P3 snapshot");
            let last_valid_digest = production_semantic_digest(&last_valid);
            let base_generation = app.artifact_generation_now();
            let (wire, _, _) = production_envelope_wire("rejected-production-candidate");
            let handle = admit_production_envelope(&mut app, &wire);
            crate::artifacts::procedural3d::spr::procedural3d_arm_publication_hostile(handle.operation, hostile);
            assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
            assert_eq!(crate::artifacts::procedural3d::spr::procedural3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
            assert_eq!(app.artifact_generation_now(), base_generation);
            let retained = app.snapshot().await.expect("last-valid P3 snapshot after rejected candidate");
            assert_eq!(production_semantic_digest(&retained), last_valid_digest);
            assert_eq!(retained, last_valid);
            assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P3 terminal ACK after candidate retirement"));
            assert!(crate::artifacts::procedural3d::spr::procedural3d_release_publication_authority(handle.operation, handle.generation));
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
        assert_eq!(ids.len(), 31, "every Procedural3dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        let _serial = test_support::lock();
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
    /// explicitly per row since procedural3d's wire keys frequently diverge from a mechanical
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
            "flow-eval-resolve",
        ];
        let commands = every_command();
        assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
        for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
            let printed = protocol::OpText::print_op(command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<Procedural3dCommand> {
        vec![
            Procedural3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }),
            Procedural3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
            Procedural3dCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            Procedural3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "extrude".into() }),
            Procedural3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "extrude".into(), x: 1.0, y: 2.0 }),
            Procedural3dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: Some(10.0), y: None }),
            Procedural3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }),
            Procedural3dCommand::Reorganize(reorganize::Reorganize {}),
            Procedural3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }),
            Procedural3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }),
            Procedural3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
            Procedural3dCommand::AddGeneration(add_generation::AddGeneration {}),
            Procedural3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "generation-1".into() }),
            Procedural3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }),
            Procedural3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) }),
            Procedural3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: flow::CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }),
            Procedural3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {}),
            Procedural3dCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
            Procedural3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
            Procedural3dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
            Procedural3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
            Procedural3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
            Procedural3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
            Procedural3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
            Procedural3dCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::procedural3d::config::Procedural3dPreviewCamera::default() }),
            Procedural3dCommand::SelectGeneration(select_generation::SelectGeneration { id: "generation-1".into() }),
            Procedural3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }),
            Procedural3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            Procedural3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
            Procedural3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    #[test]
    fn declared_actions_bridge_to_commands() {
        let _serial = test_support::lock();
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<semio_framework_plugin::EditorApp<Procedural3dPlayApp>>(testkit::procedural3d_app_manifest_for_testkit);
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_backed_editor_installs_every_declared_bounded_command_proof() {
        let _serial = test_support::lock();
        let _app = semio_framework_plugin::testkit::new_app_with_registry::<semio_framework_plugin::EditorApp<Procedural3dPlayApp>>(testkit::procedural3d_app_manifest_for_testkit).await;
    }

    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let _serial = test_support::lock();
        let json = serde_json::to_string(&create_procedural3d_app()).expect("app definition json");
        for id in [
            flow_window::PROCEDURAL_3D_PLAY_WINDOW_MAIN,
            edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW,
            generations::PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS,
            form::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM,
            generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW,
        ] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for id in [edit::PROCEDURAL_3D_PLAY_MODE_EDIT, generate::PROCEDURAL_3D_PLAY_MODE_GENERATE] {
            assert!(json.contains(id), "mode {id} missing from the manifest");
        }
        assert!(json.contains("3d.procedural"), "artifact kind missing from the manifest");
    }

    #[test]
    fn each_example_loads_distinct_fixture_and_preview_geometry() {
        use crate::artifacts::procedural3d::schema::*;
        use crate::artifacts::procedural3d::widget_id;
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
            let mut app = app();
            app.dispatch_typed(Procedural3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_framework_plugin::testkit::meta("local")).expect("set example");
            let signature = format!("{:?}", app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect::<std::collections::BTreeSet<_>>());
            assert!(signatures.insert(signature.clone()), "duplicate fixture signature for {example_id}: {signature}");
        }
    }

    #[test]
    fn refresh_pending_effects_arms_flow_eval_tick_chain() {
        let _serial = test_support::lock();
        let mut app = app();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("set example");
        let effects = app.pending_effects();
        assert!(effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")));
        drain_flow_eval_ticks(&mut app);
    }

    #[test]
    fn undo_redo_round_trips_flow_graph_edits() {
        let _serial = test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(
            &mut app,
            Procedural3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }),
            |app| app.snapshot().expect("snapshot").fixture.widgets.len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn two_instances_converge_disjoint_widget_moves() {
        let _serial = test_support::lock();
        let widgets: Vec<String> = app().snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::procedural3d::widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<Procedural3dPlayApp>, (Option<f64>, Option<f64>)>(
            "mem://procedural3d-convergence",
            Procedural3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
            Procedural3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
            move |app| {
                let layout = &app.snapshot().expect("snapshot").fixture.layout;
                (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
            },
        );
    }

    #[test]
    fn procedural3d_labels_translate_catalogue_and_inspector_in_german() {
        let _serial = test_support::lock();
        let mut app = app();
        app.dispatch_typed(Procedural3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set locale");
        let catalogue = testkit::render(&mut app, catalogue_panel::PROCEDURAL_3D_PLAY_BODY_CATALOGUE);
        assert!(catalogue.contains("\"Elemente\""));
        let inspector = testkit::render(&mut app, inspection_panel::PROCEDURAL_3D_PLAY_BODY_INSPECTION);
        assert!(inspector.contains("Elemente:"));
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM,
    /// same discovered gap as `render`), so `has_selection` is always false now and the destructive
    /// `delete-selection` row (conditioned on a real selection) never appears; this test now only pins
    /// the disclosure budget.
    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget() {
        let _serial = test_support::lock();
        let mut app = app_with_registry();
        let widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::procedural3d::widget_id(widget).to_string()).collect();
        assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        assert!(!menu.is_empty(), "grouped disclosure menu should not be empty");
    }

    #[test]
    fn sun_measures_are_exposed_on_preview_windows() {
        let _serial = test_support::lock();
        let mut app = app();
        let measures = app.window_measures();
        assert!(measures.contains_key(edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
        assert!(measures.contains_key(generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW));
    }

    //#region 🔖️EngineComputeTests
    use std::sync::MutexGuard;
    /// 🧬️ Rehomed verbatim from the deleted `⚙️engine` (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — these tests exercise
    /// `PreviewPipeline`/`MeshBridge` functions above, all of which are app
    /// behavior (they construct or take a [`Procedural3dConfig`]), so the tests travel with them.
    use ui_wgpu::wgpu::kernel_3d_scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3};

    fn test_serial() -> MutexGuard<'static, ()> {
        test_support::lock()
    }

    fn preview_payload_from_evaluated_fixture(fixture: &flow::FlowFixture, cfg: &Procedural3dConfig) -> (String, String) {
        let mut host = flow::FlowHost::from_fixture(fixture.clone());
        host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
        let eval_json = host.evaluate().unwrap_or_default();
        preview_payload_from_eval(&eval_json, fixture, cfg)
    }

    #[test]
    fn preview_payload_has_meshes_and_instances() {
        let _serial = test_serial();
        let projection = crate::artifacts::procedural3d::schema::default_snapshot();
        let config = Procedural3dConfig::default();
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
            let data: semio_framework::MeshData = serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
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
            let data: semio_framework::MeshData = serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
            let mesh3d = Mesh3d::from_buffers(data.positions, data.normals, data.indices);
            let position = instance.get("position").and_then(|value| value.as_array()).map_or([0.0, 0.0, 0.0], |items| [items[0].as_f64().unwrap_or(0.0) as f32, items[1].as_f64().unwrap_or(0.0) as f32, items[2].as_f64().unwrap_or(0.0) as f32]);
            assert_eq!(position, [0.0, 0.0, 0.0], "preview instances stay in world space");
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
            let (min, max) = transform_aabb(model, mesh3d.aabb_min, mesh3d.aabb_max);
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
        let document = procedural3d_document_from_mesh(&mesh).expect("dwg mesh import document");
        let projection: Procedural3dSnapshot = serde_json::from_value(document).expect("parseable projection");
        assert_eq!(projection.fixture.schema, "flow.fixture");
    }

    #[test]
    fn procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
        let _serial = test_serial();
        use semio_framework_plugin::{GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter};
        let document_json = serde_json::to_value(crate::artifacts::procedural3d::schema::default_snapshot()).expect("projection json");
        let mesh = procedural3d_mesh_from_document(&document_json).expect("mesh from document");
        assert!(!mesh.positions.is_empty());

        let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
        let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
        let obj_document = procedural3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
        let _: Procedural3dSnapshot = serde_json::from_value(obj_document).expect("parseable obj projection");

        let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
        let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
        let glb_document = procedural3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
        let _: Procedural3dSnapshot = serde_json::from_value(glb_document).expect("parseable glb projection");

        let stl_bytes = StlExporter.export(&mesh).expect("stl export");
        let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
        let stl_document = procedural3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
        let _: Procedural3dSnapshot = serde_json::from_value(stl_document).expect("parseable stl projection");
    }

    #[test]
    fn rectangle_wire_preview_emits_edge_only_mesh() {
        let _serial = test_serial();
        let projection = Procedural3dSnapshot::parse_dsl(crate::artifacts::procedural3d::dsl::PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
        let config = Procedural3dConfig::default();
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty(), "rectangle wire preview should tessellate curve edges");
        let data: semio_framework::MeshData = serde_json::from_value(meshes[0].get("data").cloned().unwrap_or_default()).expect("mesh data");
        assert!(data.indices.is_empty(), "wire preview has no shaded triangles");
        assert!(data.edge_positions.len() >= 6, "curve preview should include edge polylines");
        assert!(!instances_json.is_empty());
    }

    #[test]
    fn all_bundled_examples_emit_preview_meshes() {
        let _serial = test_serial();
        let config = Procedural3dConfig::default();
        let cases = [
            ("hexagonal-mushroom-column", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN),
            ("rectangle-extrude-volume", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE),
            ("sphere-cut-with-torus", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS),
            ("box-fillet-preview", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_BOX_FILLET),
            ("sphere-box-fuse", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE),
            ("face-sweep-extrude", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE),
            ("rectangle-wire-preview", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE),
            ("box-shell-preview", crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_BOX_SHELL),
        ];
        for (label, example_id) in cases {
            let projection = crate::artifacts::procedural3d::schema::example_snapshot(example_id).unwrap_or_else(|| panic!("{label}: missing projection"));
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
        let projection = crate::artifacts::procedural3d::schema::default_snapshot();
        let config = Procedural3dConfig { show_mode: "wireframe".into(), ..Default::default() };
        let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty());
        let data: semio_framework::MeshData = serde_json::from_value(meshes[0].get("data").cloned().unwrap_or_default()).expect("mesh data");
        assert!(data.indices.is_empty());
        assert!(!data.edge_positions.is_empty());
    }

    #[test]
    fn procedural3d_io_declares_the_params_and_geometry_ports() {
        let io = semio_framework::io::resolve_ready(procedural3d_io());
        assert_eq!(io.document_schema, "procedural.3d");
        assert_eq!(io.artifact.id, "3d.procedural");
        let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert_eq!(params.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(!params.required);
        let geometry = io.ports.iter().find(|port| port.id == "geometry:out").expect("geometry:out declared");
        assert_eq!(geometry.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(geometry.kind_id.as_deref(), Some("3d.mesh"));
        assert_eq!(geometry.multiplicity, semio_framework::PortMultiplicity::Many);
    }
    //#endregion 🔖️EngineComputeTests
}
//#endregion 🧪️Tests
