//! 🎲️ Procedural2d play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `Procedural2dCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! passthrough per node.

use crate::apps::procedural2d::commands::{eval, generation, graph, locale, selection, view, widget};
use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::apps::procedural2d::modes::edit::windows::{flow as flow_window, preview as edit_preview};
use crate::apps::procedural2d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::apps::procedural2d::modes::{edit, generate};
use crate::apps::procedural2d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::procedural2d::terminology::{procedural2d_labels, Procedural2dLabels};
use crate::artifacts::procedural2d::engine::procedural2d_io;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::{artifact_kind, Procedural2dSnapshot, PROCEDURAL_2D_SCHEMA};
use flow::{with_process_flow_eval_session, FlowEvalSession};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, HostEffect, Label, LocalizedLabel, MediaClass, MediaForm, MediaType, UiNode};
use store::EngineHandles;
use serde_json::Value;

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_APP_ID: &str = "procedural2d-play";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn procedural2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PROCEDURAL2D_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

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
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "selectNode" as "select-node" => select_node::SelectNode,
        "nodeGraphSelect" as "node-graph-select" => node_graph_select::NodeGraphSelect,
        "nodeGraphHover" as "node-graph-hover" => node_graph_hover::NodeGraphHover,
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
// payload module is imported here under its own flat name.
use eval::{flow_eval_tick, set_eval_outputs};
use generation::{add_generation, enter_generate, remove_generation, rename_generation, select_generation, update_generation_values};
use graph::{connect_media_ports, move_media_node, node_graph_edit, node_graph_hover, node_graph_select, node_graph_viewport, reorganize};
use locale::set_locale;
use selection::{select_node, set_selection};
use view::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, set_show_mode};
use widget::{add_widget, remove_widget};
//#endregion 🔖️Commands

//#region 🔖️Procedural2dPlayApp
/// 🧪️ Unit struct apart from `eval_session`: every former runtime field lives in [`Procedural2dConfig`],
/// written through [`Procedural2dConfigMutation`]s. The eval session is the one piece of state that is
/// neither document nor view — it is threaded into every command handler as the `app_commands!`
/// dispatch context.
#[derive(Default)]
pub struct Procedural2dPlayApp;

impl DocumentApp for Procedural2dPlayApp {
    type Snapshot = Procedural2dSnapshot;
    type Mutation = Procedural2dMutation;
    type Config = Procedural2dConfig;
    type ConfigMutation = Procedural2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;

    type Command = Procedural2dCommand;

    const APP_ID: &'static str = PROCEDURAL2D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = PROCEDURAL_2D_SCHEMA;

    fn initial_snapshot() -> Procedural2dSnapshot {
        crate::artifacts::procedural2d::engine::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(procedural2d_io())
    }

    fn command_id(command: &Procedural2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Procedural2dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(Value::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "nodeGraphEdit" => Ok(Procedural2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(|value| value.to_string())).unwrap_or_else(|| "[]".into())})),
            "moveMediaNode" => Ok(Procedural2dCommand::MoveMediaNode(move_media_node::MoveMediaNode {
                node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(),
                x: f64_arg(&["x"]).unwrap_or(0.0),
                y: f64_arg(&["y"]).unwrap_or(0.0)})),
            "addWidget" => Ok(Procedural2dCommand::AddWidget(add_widget::AddWidget {
                kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()),
                neuron_kind: str_arg(&["neuronKind", "neuron_kind"]),
                x: f64_arg(&["x"]),
                y: f64_arg(&["y"])})),
            "removeWidget" => Ok(Procedural2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "connectMediaPorts" => Ok(Procedural2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts {
                source_node_id: str_arg(&["sourceNodeId", "source_node_id"]).unwrap_or_default(),
                source_port_id: str_arg(&["sourcePortId", "source_port_id"]).unwrap_or_default(),
                target_node_id: str_arg(&["targetNodeId", "target_node_id"]).unwrap_or_default(),
                target_port_id: str_arg(&["targetPortId", "target_port_id"]).unwrap_or_default()})),
            "reorganize" => Ok(Procedural2dCommand::Reorganize(reorganize::Reorganize {})),
            "addGeneration" => Ok(Procedural2dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Procedural2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Procedural2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").map_or(dsl::DslValue::Null, |entry| dsl::to_dsl_value(entry).unwrap_or(dsl::DslValue::Null));
                Ok(Procedural2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value}))
            }
            "nodeGraphViewport" => {
                let viewport_json = str_arg(&["viewportJson", "viewport_json"])
                    .or_else(|| args.get("camera").map(|value| if value.is_string() { value.as_str().unwrap_or("{}").to_string() } else { value.to_string() }))
                    .unwrap_or_else(|| "{}".into());
                Ok(Procedural2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json }))
            }
            "setSelection" => Ok(Procedural2dCommand::SetSelection(set_selection::SetSelection { ids: string_list("ids") })),
            "selectNode" => Ok(Procedural2dCommand::SelectNode(select_node::SelectNode { ids: string_list("ids") })),
            "nodeGraphSelect" => Ok(Procedural2dCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { ids: string_list("ids") })),
            "nodeGraphHover" => Ok(Procedural2dCommand::NodeGraphHover(node_graph_hover::NodeGraphHover {})),
            "setShowMode" => Ok(Procedural2dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode"]).unwrap_or_default() })),
            "generate" => Ok(Procedural2dCommand::Generate(enter_generate::Generate {})),
            "setEvalOutputs" => Ok(Procedural2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs {
                outputs_json: str_arg(&["outputsJson", "outputs_json", "evalJson"]).unwrap_or_else(|| "{}".into())})),
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
            )))}
    }

    fn handle(command: &Procedural2dCommand, doc: &DocumentView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation, Self::DraftMutation>, Fault> {
        with_process_flow_eval_session(|session| command.dispatch(doc, cfg, session))
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, remote operations) in one place instead of each
    /// action re-checking.
    fn pending_effects(doc: &DocumentView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>) -> Vec<HostEffect> {
        with_process_flow_eval_session(|session| {
            let host = crate::artifacts::procedural2d::engine::host_from_fixture_with_session(&doc.snapshot.fixture, session);
            if session.sync(&host) {
                vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
            } else {
                Vec::new()
            }
        })
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>) -> UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = procedural2d_labels(config);
        with_process_flow_eval_session(|session| match body_key {
            flow_window::PROCEDURAL2D_PLAY_BODY_MAIN => flow_window::render(document, config, session),
            edit_preview::PROCEDURAL2D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, session),
            generations::PROCEDURAL2D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::Native),
            form::PROCEDURAL2D_PLAY_BODY_GENERATE_FORM => form::render(document, &document.generation, labels),
            generate_preview::PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(config, labels),
            document_panel::PROCEDURAL2D_PLAY_BODY_DOCUMENT => document_panel::render(document, config, labels),
            catalogue_panel::PROCEDURAL2D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            inspection_panel::PROCEDURAL2D_PLAY_BODY_INSPECTION => inspection_panel::render(document, config, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}")))})
    }

    /// 🗂️ Grouped disclosure: `addWidget`/`reorganize`/`generate` stay top-level; the display-mode
    /// toggle, generation authoring, and generation selection each fold into their own taxonomy group;
    /// the delete-selection item stays a direct destructive item last.
    fn context_menu(request: &semio_framework_plugin::ContextMenuRequest, _doc: &DocumentView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let config = cfg.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<Procedural2dLabels>(&config.locale);
        let is_de = config.locale.starts_with("de");
        let selected = config.selected_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
        let mut menu = Menu::of(registry).action("addWidget").action("reorganize").action("generate").group("mode", |m| m.action("setShowMode")).group("create", |m| m.action("addGeneration")).group("methods", |m| m.action("selectGeneration"));
        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }

    /// 🎞️ Declares `export_media`'s default document schema — pack-encodes `doc.snapshot`, wrapped
    /// `Structured{schema: Self::DOCUMENT_SCHEMA, json: base64}` — plus `"drawing:out"`.
    fn export_media(port: &str, doc: &DocumentView<'_, Procedural2dSnapshot>) -> Result<semio_framework_plugin::Media, semio_framework_plugin::MediaError> {
        match port {
            "drawing:out" => {
                let eval_json = crate::artifacts::procedural2d::engine::evaluate_generation_preview(&doc.snapshot.fixture, &serde_json::Map::new());
                let layers_json = crate::artifacts::procedural2d::engine::generation_preview_layers(&eval_json);
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "2d.drawing".into(), json: layers_json } })
            }
            "document:out" => {
                let bytes = store::DocumentPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
                    payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) }})
            }
            _ => Err(semio_framework_plugin::MediaError::NotImplemented)}
    }

    /// 🎞️ `"params:in"`: a generic Data×Value JSON object `{widgetId: number}` — patches matching
    /// `InputSlider` widgets' `value` field, leaving unmatched keys/widget kinds untouched.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &DocumentView<'_, Procedural2dSnapshot>) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        if port != "params:in" {
            return Err(semio_framework_plugin::MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in expects a Structured JSON object payload".into()));
        };
        let parsed: Value = serde_json::from_str(json).map_err(|error| semio_framework_plugin::MediaError::Payload(port.to_string(), error.to_string()))?;
        let Some(object) = parsed.as_object() else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()));
        };
        let mut operations = Vec::new();
        for (widget_id_key, value) in object {
            let Some(number) = value.as_f64() else { continue };
            let Some((index, widget)) = doc.snapshot.fixture.widgets.iter().enumerate().find(|(_, widget)| crate::artifacts::procedural2d::widget_id(widget) == widget_id_key.as_str()) else { continue };
            if let flow::Widget::InputSlider { id, min, max, step, .. } = widget {
                operations.push(Procedural2dMutation::SetWidget { index, widget: flow::Widget::InputSlider { id: id.clone(), value: number, min: *min, max: *max, step: *step } });
            }
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️Procedural2dPlayApp

//#region 🔖️Manifest
pub fn create_procedural2d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL2D_PLAY_APP_ID, LocalizedLabel::native("Procedural 2D", "Procedural 2D")).document(["semio", "procedural", "2d"])
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
            .action_with(ActionDefinition::new_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation).with_category("selection"))
            .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation).with_category("create"))
            .mutation("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"))
            .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation).with_category("create"))
            .mutation("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"))
            .mutation("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"))
            .mutation("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"))
            // 👁️ Ephemeral view actions — selection, hover, camera, the show-mode display toggle, and evaluation scratch (emit no operations).
            .view_action("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("selectNode", LocalizedLabel::native("Select Node", "Knoten auswählen"))
            .view_action("nodeGraphSelect", LocalizedLabel::native("Node Graph Select", "Graph-Auswahl"))
            .view_action("nodeGraphHover", LocalizedLabel::native("Node Graph Hover", "Graph-Hover"))
            .action_with(ActionDefinition::new_catalog("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"), ActionKind::View).with_category("mode"))
            .action_with(ActionDefinition::new_catalog("generate", LocalizedLabel::native("Generate", "Generieren"), ActionKind::View).with_category("actions"))
            .view_action("setEvalOutputs", LocalizedLabel::native("Set Eval Outputs", "Auswertungsausgaben festlegen"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Canvas-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Canvas-Zeiger bewegt"))
            .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Canvas-Zeiger losgelassen"))
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Canvas-Mausrad"))
            .action_with(ActionDefinition::new_catalog("selectGeneration", LocalizedLabel::native("Select Generation", "Generation auswählen"), ActionKind::View).with_category("methods"))
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
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Procedural2dPlayApp::config_spec())
            .io(procedural2d_io()),
    )
    .example("default", LocalizedLabel::native("Default", "Standard"), serde_json::to_string(&crate::artifacts::procedural2d::engine::default_snapshot()).unwrap(), "file")
    .workflow("procedural2d", "Procedural 2D", "layout")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type Procedural2dApp = VcsDocumentApp<Procedural2dPlayApp>;

    pub fn app() -> Procedural2dApp {
        crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();
        new_app::<Procedural2dPlayApp>()
    }

    pub fn app_with_registry() -> Procedural2dApp {
        crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();
        new_app_with_registry::<Procedural2dPlayApp>(create_procedural2d_app)
    }

    pub fn dispatch(app: &mut Procedural2dApp, command: Procedural2dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Procedural2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, app_with_registry, dispatch};
    use flow::Widget;
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;
    use semio_framework_plugin::PluginApp;

    //#region 🔖️CommandSurface
    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 25, "every Procedural2dCommand row must be covered by every_command()");
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
            "set-selection",
            "select-node",
            "node-graph-select",
            "node-graph-hover",
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
            Procedural2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) }),
            Procedural2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: "{}".into() }),
            Procedural2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["n1".into()] }),
            Procedural2dCommand::SelectNode(select_node::SelectNode { ids: vec!["n1".into()] }),
            Procedural2dCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { ids: vec!["n1".into(), "n2".into()] }),
            Procedural2dCommand::NodeGraphHover(node_graph_hover::NodeGraphHover {}),
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
        let json = serde_json::to_string(&create_procedural2d_app().definition).expect("app definition json");
        for id in [flow_window::PROCEDURAL2D_PLAY_WINDOW_MAIN, edit_preview::PROCEDURAL2D_PLAY_WINDOW_PREVIEW, generations::PROCEDURAL2D_PLAY_WINDOW_GENERATIONS, form::PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM, generate_preview::PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW] {
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
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Procedural2dPlayApp>(create_procedural2d_app);
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
    fn document_from_dwg_returns_valid_default_snapshot() {
        let drawing = semio_framework::DwgDrawing::default();
        let document = crate::artifacts::procedural2d::engine::procedural2d_document_from_dwg(&drawing).expect("dwg import document");
        let projection: Procedural2dSnapshot = serde_json::from_value(document).expect("parseable projection");
        assert_eq!(projection.fixture.schema, "flow.fixture");
    }

    #[test]
    fn two_instances_converge_disjoint_widget_moves() {
        let widgets: Vec<String> = app().snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::artifacts::procedural2d::widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        semio_framework_plugin::testkit::assert_two_instances_converge::<Procedural2dPlayApp, (Option<f64>, Option<f64>)>(
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
        use crate::apps::procedural2d::testkit::render;
        let mut app = app();
        assert!(render(&mut app, "procedural2d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️ContextMenuTests
    #[test]
    fn context_menu_stays_within_disclosure_budget_with_destructive_last() {
        let mut app = app_with_registry();
        dispatch(&mut app, Procedural2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["rect".into()] }));
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let items = app.context_menu(&request);
        assert!(items.len() <= 9, "top-level menu rows (leaves + groups + separator) must stay within disclosure budget, got {}", items.len());
        assert_eq!(items.last().map(|item| item.id.as_str()), Some("delete-selection"), "the destructive delete row must be last");
        assert_eq!(items.last().and_then(|item| item.destructive), Some(true));
    }
    //#endregion 🔖️ContextMenuTests

    //#region 🔖️PortTests
    #[test]
    fn export_drawing_out_returns_vector_media() {
        let mut app = app();
        let media = app.export_media("drawing:out").expect("export drawing:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
    }

    #[test]
    fn export_document_out_returns_flow_media() {
        let mut app = app();
        let media = app.export_media("document:out").expect("export document:out");
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
                _ => None})
            .expect("just-added input slider");
        let media = semio_framework_plugin::Media {
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: serde_json::json!({ slider_id.clone(): 42.0 }).to_string() }};
        app.import_media("params:in", &media, &semio_framework_plugin::testkit::meta("local")).expect("import params");
        let value = app.snapshot().expect("snapshot").fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == &slider_id => Some(*value),
            _ => None});
        assert_eq!(value, Some(42.0));
    }

    #[test]
    fn media_ports_declare_params_in_and_drawing_out() {
        let ports = <Procedural2dPlayApp as DocumentApp>::media_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let params_in = ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert_eq!(params_in.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let drawing_out = ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
        assert_eq!(drawing_out.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
        assert_eq!(drawing_out.kind_id.as_deref(), Some("2d.drawing"));
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
