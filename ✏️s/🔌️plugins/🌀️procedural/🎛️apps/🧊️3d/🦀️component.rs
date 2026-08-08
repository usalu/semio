//! 🧱️ Procedural3d play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.

use crate::apps::procedural3d::commands::{eval, example, generation, graph, gumball, locale, selection, sun, view, widget};
use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::apps::procedural3d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::apps::procedural3d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::apps::procedural3d::modes::{edit, generate};
use crate::apps::procedural3d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::procedural3d::terminology::procedural3d_labels;
use crate::artifacts::procedural3d::engine::procedural3d_io;
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::{artifact_kind, Procedural3dSnapshot, PROCEDURAL_3D_SCHEMA};
use flow::{with_process_flow_eval_session, FlowEvalSession};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, HostEffect, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaType, UiNode, UtilityDefinition, WindowMeasure};
use store::EngineHandles;
use serde_json::Value;
use std::collections::HashMap;

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_APP_ID: &str = "procedural3d-play";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub fn procedural3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PROCEDURAL_3D_PLAY_APP_ID).action(action, args)
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
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "selectNode" as "select-node" => select_node::SelectNode,
        "nodeGraphSelect" as "graph-select" => node_graph_select::NodeGraphSelect,
        "nodeGraphHover" as "graph-hover" => node_graph_hover::NodeGraphHover,
        "setHover" as "set-hover" => set_hover::SetHover,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,
        "graphPointerDown" as "graph-pointer-down" => graph_pointer_down::GraphPointerDown,
        "worldSelect" as "world-select" => world_select::WorldSelect,
        "worldHover" as "world-hover" => world_hover::WorldHover,
        "setSelectionMethod" as "selection-method" => set_selection_method::SetSelectionMethod,
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
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "flowTessellateResolve" as "flow-tessellate-resolve" => flow_tessellate_resolve::FlowTessellateResolve,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use eval::{flow_eval_resolve, flow_eval_tick, flow_tessellate_resolve};
use example::set_active_example;
use generation::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use graph::{graph_pointer_down, move_media_node, node_graph_edit, node_graph_hover, node_graph_select, node_graph_viewport, reorganize};
use gumball::{rotate_selection, scale_selection, translate_selection};
use locale::{set_contributions, set_locale};
use selection::{select_node, set_hover, set_selection, set_selection_method, world_hover, world_pointer_down, world_select};
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use view::{set_active_utility, set_camera, set_lod_mode, set_show_mode};
use widget::{add_widget, delete_selection, patch_flow_widgets, remove_widget};
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
fn parse_preview_camera_json(args: &Value) -> crate::apps::procedural3d::config::Procedural3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = serde_json::from_value::<crate::apps::procedural3d::config::Procedural3dPreviewCamera>(camera.clone()) {
            return parsed;
        }
    }
    crate::apps::procedural3d::config::Procedural3dPreviewCamera::default()
}

impl DocumentApp for Procedural3dPlayApp {
    type Snapshot = Procedural3dSnapshot;
    type Mutation = Procedural3dMutation;
    type Config = Procedural3dConfig;
    type ConfigMutation = Procedural3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;

    type Command = Procedural3dCommand;

    const APP_ID: &'static str = PROCEDURAL_3D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = PROCEDURAL_3D_SCHEMA;

    fn initial_snapshot() -> Procedural3dSnapshot {
        crate::artifacts::procedural3d::engine::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(procedural3d_io())
    }

    /// 🎞️ `geometry:out` plus the inherited `document:out` default, replicated inline (overriding
    /// `export_media` shadows the trait's provided body for every port on this app).
    fn export_media(port: &str, doc: &DocumentView<'_, Procedural3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "geometry:out" => {
                let mesh = crate::artifacts::procedural3d::engine::export_mesh_from_document(doc.snapshot);
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "3d.mesh".into(), json: serde_json::to_string(&mesh).unwrap_or_default() } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = store::DocumentPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media { media_type, payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"` — patches matching `InputSlider` widgets from a `{widgetId: number}` JSON
    /// object; unmatched keys/non-slider widgets are silently ignored.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &DocumentView<'_, Procedural3dSnapshot>) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation, Self::DraftMutation>, MediaError> {
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
                    let Some((index, widget)) = fixture.widgets.iter().enumerate().find(|(_, widget)| crate::artifacts::procedural3d::widget_id(widget) == target_id) else { continue };
                    if let flow::Widget::InputSlider { id, min, max, step, .. } = widget {
                        operations.push(Procedural3dMutation::SetWidget { index, widget: flow::Widget::InputSlider { id: id.clone(), value: number, min: *min, max: *max, step: *step } });
                    }
                }
                Ok(Emit::mutations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Procedural3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Procedural3dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
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
            "moveMediaNode" => Ok(Procedural3dCommand::MoveMediaNode(move_media_node::MoveMediaNode {
                node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(),
                x: f64_arg(&["x"]).unwrap_or(0.0),
                y: f64_arg(&["y"]).unwrap_or(0.0),
            })),
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
            "setSelection" => Ok(Procedural3dCommand::SetSelection(set_selection::SetSelection { node_ids: string_list("ids") })),
            "selectNode" => Ok(Procedural3dCommand::SelectNode(select_node::SelectNode { node_ids: string_list("ids").into_iter().chain(string_list("nodeIds")).collect() })),
            "nodeGraphSelect" => Ok(Procedural3dCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: string_list("ids").into_iter().chain(string_list("nodeIds")).collect() })),
            "nodeGraphHover" => Ok(Procedural3dCommand::NodeGraphHover(node_graph_hover::NodeGraphHover { widget_id: str_arg(&["widgetId", "widget_id"]) })),
            "setHover" => Ok(Procedural3dCommand::SetHover(set_hover::SetHover { object_id: str_arg(&["objectId", "object_id", "id"]) })),
            "worldPointerDown" => Ok(Procedural3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {})),
            "graphPointerDown" => Ok(Procedural3dCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {})),
            "worldSelect" => Ok(Procedural3dCommand::WorldSelect(world_select::WorldSelect { ids: string_list("ids"), merge: str_arg(&["merge"]).unwrap_or_else(|| "replace".into()) })),
            "worldHover" => Ok(Procedural3dCommand::WorldHover(world_hover::WorldHover { id: str_arg(&["id", "objectId", "object_id"]) })),
            "setSelectionMethod" => Ok(Procedural3dCommand::SetSelectionMethod(set_selection_method::SetSelectionMethod { method: str_arg(&["value", "method", "selectionMethod"]).unwrap_or_default() })),
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
            "setContributions" => Ok(Procedural3dCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json", "contributionsJson", "contributions_json"]).or_else(|| args.get("contributions").map(|value| value.to_string())).unwrap_or_else(|| "[]".into()),
            })),
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

    fn handle(command: &Procedural3dCommand, doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation, Self::DraftMutation>, Fault> {
        with_process_flow_eval_session(|session| command.dispatch(doc, cfg, session))
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes.
    fn pending_effects(doc: &DocumentView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>) -> Vec<HostEffect> {
        with_process_flow_eval_session(|session| {
            let host = flow::flow_host_with_session(&doc.snapshot.fixture, session);
            if session.sync(&host) {
                vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
            } else {
                Vec::new()
            }
        })
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>) -> UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = procedural3d_labels(config);
        let active_utility = config.active_utility_id.as_str();
        with_process_flow_eval_session(|session| match body_key {
            flow_window::PROCEDURAL_3D_PLAY_BODY_MAIN => flow_window::render(document, config, session),
            edit_preview::PROCEDURAL_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, session, active_utility),
            generations::PROCEDURAL_3D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::default()),
            form::PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => form::render(&document.fixture, &document.generation, labels),
            generate_preview::PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(&document.fixture, &document.generation, config, labels, active_utility),
            document_panel::PROCEDURAL_3D_PLAY_BODY_DOCUMENT => document_panel::render(&document.fixture, &config.selected_node_ids, labels),
            catalogue_panel::PROCEDURAL_3D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            inspection_panel::PROCEDURAL_3D_PLAY_BODY_INSPECTION => inspection_panel::render(&document.fixture, &config.selected_node_ids, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        })
    }

    fn window_measures(_doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
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
    fn context_menu(request: &semio_framework_plugin::ContextMenuRequest, _doc: &DocumentView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let config = cfg.snapshot;
        let labels = procedural3d_labels(config);
        let is_de = config.locale.starts_with("de");
        let selected = config.selected_node_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
        let has_selection = !nodes.is_empty() || !edges.is_empty();
        let mut menu = Menu::of(registry).action("reorganize");
        menu = menu.when(has_selection, |m| m.action("translateSelection").action("rotateSelection").action("scaleSelection"));
        menu = menu.group("create", |m| m.action("addWidget").action("addGeneration"));
        menu = menu.when(has_selection, |m| m.group("targets", |m2| m2.action("removeWidget").action("removeGeneration")));
        menu = menu.group("methods", |m| m.action("renameGeneration").action("updateGenerationValues").action("patchFlowWidgets"));
        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }
}
//#endregion 🔖️Procedural3dPlayApp

//#region 🔖️Manifest
pub fn create_procedural3d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL_3D_PLAY_APP_ID, LocalizedLabel::native("Procedural 3D", "Procedural 3D")).document(["semio", "procedural", "3d"])
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
            .action_with(ActionDefinition::new_catalog("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"), ActionKind::Mutation).with_category("targets"))
            .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("patchFlowWidgets", LocalizedLabel::native("Patch Flow Widgets", "Flow-Elemente aktualisieren"), ActionKind::Mutation).with_category("methods"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::Mutation).with_category("targets"))
            .action_with(ActionDefinition::new_catalog("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::Mutation).with_category("methods"))
            .action_with(ActionDefinition::new_catalog("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::Mutation).with_category("methods"))
            // 👁️ Ephemeral view actions — selection, hover, world picking, graph camera, sun/LOD/show-mode display toggles, preview camera.
            .view_action("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("selectNode", LocalizedLabel::native("Select Node", "Knoten auswählen"))
            .view_action("nodeGraphSelect", LocalizedLabel::native("Node Graph Select", "Graph-Auswahl"))
            .view_action("nodeGraphHover", LocalizedLabel::native("Node Graph Hover", "Graph-Hover"))
            .view_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            .view_action("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"))
            .view_action("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen"))
            .view_action("worldHover", LocalizedLabel::native("World Hover", "Überfahren (Welt)"))
            .view_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"))
            .view_action("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"))
            .view_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("selectGeneration", LocalizedLabel::native("Set Generation", "Generation auswählen"))
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
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
                    ActionArgOption::new(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
                ]).required(),
            ])
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Procedural3dPlayApp::config_spec())
            .io(procedural3d_io()),
    )
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_HEX_COLUMN), "hexagon")
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_RECT_EXTRUDE), "box")
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_TORUS), "circle")
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_BOX_FILLET), "box")
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE), "combine")
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE), "layers")
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE), "square")
    .example(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau"), crate::artifacts::procedural3d::engine::example_document_json(crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_BOX_SHELL), "box")
    .workflow("procedural3d", "Procedural 3D", "brep")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type Procedural3dApp = VcsDocumentApp<Procedural3dPlayApp>;

    pub fn app() -> Procedural3dApp {
        new_app::<Procedural3dPlayApp>()
    }

    pub fn app_with_registry() -> Procedural3dApp {
        new_app_with_registry::<Procedural3dPlayApp>(create_procedural3d_app)
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
            if !result.requested_effects.iter().any(|effect| matches!(effect, HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")) {
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
    use crate::apps::procedural3d::testkit::{app, app_with_registry, drain_flow_eval_ticks};
    use semio_framework_plugin::PluginApp;

    //#region 🔖️CommandSurface
    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 39, "every Procedural3dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
    /// explicitly per row since procedural3d's wire keys frequently diverge from a mechanical
    /// kebab-case of the command id (e.g. `nodeGraphViewport` → `viewport`, `setLocale` → `locale`).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
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
            "set-selection",
            "select-node",
            "graph-select",
            "graph-hover",
            "set-hover",
            "world-pointer-down",
            "graph-pointer-down",
            "world-select",
            "world-hover",
            "selection-method",
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
            "contributions",
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
            Procedural3dCommand::SetSelection(set_selection::SetSelection { node_ids: vec!["a".into()] }),
            Procedural3dCommand::SelectNode(select_node::SelectNode { node_ids: vec!["a".into()] }),
            Procedural3dCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: vec!["a".into()] }),
            Procedural3dCommand::NodeGraphHover(node_graph_hover::NodeGraphHover { widget_id: Some("extrude".into()) }),
            Procedural3dCommand::SetHover(set_hover::SetHover { object_id: None }),
            Procedural3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {}),
            Procedural3dCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
            Procedural3dCommand::WorldSelect(world_select::WorldSelect { ids: vec!["a".into()], merge: "replace".into() }),
            Procedural3dCommand::WorldHover(world_hover::WorldHover { id: Some("a".into()) }),
            Procedural3dCommand::SetSelectionMethod(set_selection_method::SetSelectionMethod { method: "lasso".into() }),
            Procedural3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
            Procedural3dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
            Procedural3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
            Procedural3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
            Procedural3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
            Procedural3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
            Procedural3dCommand::SetCamera(set_camera::SetCamera { camera: crate::apps::procedural3d::config::Procedural3dPreviewCamera::default() }),
            Procedural3dCommand::SelectGeneration(select_generation::SelectGeneration { id: "generation-1".into() }),
            Procedural3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }),
            Procedural3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            Procedural3dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
            Procedural3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
            Procedural3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    #[test]
    fn declared_actions_bridge_to_commands() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Procedural3dPlayApp>(create_procedural3d_app);
    }

    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let json = serde_json::to_string(&create_procedural3d_app().definition).expect("app definition json");
        for id in [flow_window::PROCEDURAL_3D_PLAY_WINDOW_MAIN, edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, generations::PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS, form::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM, generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for id in [edit::PROCEDURAL_3D_PLAY_MODE_EDIT, generate::PROCEDURAL_3D_PLAY_MODE_GENERATE] {
            assert!(json.contains(id), "mode {id} missing from the manifest");
        }
        assert!(json.contains("3d.procedural"), "artifact kind missing from the manifest");
    }

    #[test]
    fn each_example_loads_distinct_fixture_and_preview_geometry() {
        use crate::artifacts::procedural3d::engine::*;
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
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("set example");
        let effects = app.pending_effects();
        assert!(effects.iter().any(|effect| matches!(effect, HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")));
        drain_flow_eval_ticks(&mut app);
    }

    #[test]
    fn undo_redo_round_trips_flow_graph_edits() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, Procedural3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }), |app| app.snapshot().expect("snapshot").fixture.widgets.len(), before, before + 1);
    }

    #[test]
    fn two_instances_converge_disjoint_widget_moves() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let widgets: Vec<String> = app().snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::procedural3d::widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        semio_framework_plugin::testkit::assert_two_instances_converge::<Procedural3dPlayApp, (Option<f64>, Option<f64>)>(
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
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        app.dispatch_typed(Procedural3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set locale");
        let catalogue = testkit::render(&mut app, catalogue_panel::PROCEDURAL_3D_PLAY_BODY_CATALOGUE);
        assert!(catalogue.contains("\"Elemente\""));
        let inspector = testkit::render(&mut app, inspection_panel::PROCEDURAL_3D_PLAY_BODY_INSPECTION);
        assert!(inspector.contains("Elemente:"));
    }

    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app_with_registry();
        let widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::procedural3d::widget_id(widget).to_string()).collect();
        assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
        app.dispatch_typed(Procedural3dCommand::SetSelection(set_selection::SetSelection { node_ids: widgets }), &semio_framework_plugin::testkit::meta("local")).expect("set selection");
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true);
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
    }

    #[test]
    fn sun_measures_are_exposed_on_preview_windows() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        let measures = app.window_measures();
        assert!(measures.contains_key(edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
        assert!(measures.contains_key(generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW));
    }
}
//#endregion 🧪️Tests
