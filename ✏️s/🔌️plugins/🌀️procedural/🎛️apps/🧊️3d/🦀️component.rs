//! 🧱️ Procedural3d play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.

use crate::apps::procedural3d::commands::{eval, example, generation, graph, gumball, locale, selection, sun, view, widget};
use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::apps::procedural3d::modes::edit::windows::{flow, preview as edit_preview};
use crate::apps::procedural3d::modes::generate::windows::{form, generations, preview as generate_preview};
use crate::apps::procedural3d::modes::{edit, generate};
use crate::apps::procedural3d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::procedural3d::terminology::{procedural3d_labels, Procedural3dLabels};
use crate::artifacts::procedural3d::engine::procedural3d_io;
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use crate::artifacts::procedural3d::{artifact_kind, Procedural3dDocument, PROCEDURAL_3D_SCHEMA};
use flow_core::FlowEvalSession;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, HostEffect, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaType, PanelGroup, UiNode,
    UtilityDefinition, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;

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
    pub enum Procedural3dCommand for Procedural3dDocument, Procedural3dOperation, Procedural3dConfig, Procedural3dConfigOperation, ctx = FlowEvalSession {
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
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use eval::{flow_eval_resolve, flow_eval_tick};
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
/// written through [`Procedural3dConfigOperation`]s.
#[derive(Default)]
pub struct Procedural3dPlayApp {
    eval_session: Mutex<FlowEvalSession>,
}

fn eval_session_lock(app: &Procedural3dPlayApp) -> std::sync::MutexGuard<'_, FlowEvalSession> {
    app.eval_session.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl DocumentApp for Procedural3dPlayApp {
    type Projection = Procedural3dDocument;
    type Operation = Procedural3dOperation;
    type Config = Procedural3dConfig;
    type ConfigOperation = Procedural3dConfigOperation;
    type Command = Procedural3dCommand;

    fn app_id(&self) -> &str {
        PROCEDURAL_3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PROCEDURAL_3D_SCHEMA
    }

    fn initial_projection(&self) -> Procedural3dDocument {
        crate::artifacts::procedural3d::engine::default_projection()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(procedural3d_io())
    }

    /// 🎞️ `geometry:out` plus the inherited `document:out` default, replicated inline (overriding
    /// `export_media` shadows the trait's provided body for every port on this app).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Procedural3dDocument>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "geometry:out" => {
                let mesh = crate::artifacts::procedural3d::engine::export_mesh_from_document(doc.projection);
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "3d.mesh".into(), json: serde_json::to_string(&mesh).unwrap_or_default() } })
            }
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = store::DocumentPack::encode_pack(doc.projection);
                Ok(semio_framework_plugin::Media { media_type, payload: semio_framework_plugin::MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"` — patches matching `InputSlider` widgets from a `{widgetId: number}` JSON
    /// object; unmatched keys/non-slider widgets are silently ignored.
    fn import_media(&self, port: &str, media: &semio_framework_plugin::Media, doc: &DocumentView<'_, Procedural3dDocument>) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, MediaError> {
        match port {
            "params:in" => {
                let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "params:in importer only accepts a Structured JSON object payload".into()));
                };
                let object: serde_json::Map<String, Value> = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture = &doc.projection.fixture;
                let mut operations = Vec::new();
                for (target_id, value) in &object {
                    let Some(number) = value.as_f64() else { continue };
                    let Some((index, widget)) = fixture.widgets.iter().enumerate().find(|(_, widget)| crate::artifacts::procedural3d::widget_id(widget) == target_id) else { continue };
                    if let flow_core::Widget::InputSlider { id, min, max, step, .. } = widget {
                        operations.push(Procedural3dOperation::SetWidget { index, widget: flow_core::Widget::InputSlider { id: id.clone(), value: number, min: *min, max: *max, step: *step } });
                    }
                }
                Ok(Emit::operations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(&self, command: &Procedural3dCommand) -> &str {
        command.command_id()
    }

    fn handle(&self, command: &Procedural3dCommand, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let mut session = eval_session_lock(self);
        command.dispatch(doc, cfg, &mut session)
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes.
    fn pending_effects(&self, doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>) -> Vec<HostEffect> {
        let mut session = eval_session_lock(self);
        let host = flow_core::flow_host_with_session(&doc.projection.fixture, &session);
        if session.sync(&host) {
            vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let labels = procedural3d_labels(config);
        let active_utility = config.active_utility_id.as_str();
        let session = eval_session_lock(self);
        match body_key {
            flow::PROCEDURAL_3D_PLAY_BODY_MAIN => flow::render(document, config, &session),
            edit_preview::PROCEDURAL_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, &session, active_utility),
            generations::PROCEDURAL_3D_PLAY_BODY_GENERATIONS => generations::render(&document.generation, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::default()),
            form::PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => form::render(&document.fixture, &document.generation, labels),
            generate_preview::PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(&document.fixture, &document.generation, config, labels, active_utility),
            document_panel::PROCEDURAL_3D_PLAY_BODY_DOCUMENT => document_panel::render(&document.fixture, &config.selected_node_ids, labels),
            catalogue_panel::PROCEDURAL_3D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            inspection_panel::PROCEDURAL_3D_PLAY_BODY_INSPECTION => inspection_panel::render(&document.fixture, &config.selected_node_ids, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let measures = edit_preview::preview_window_measures(config, procedural3d_action);
        HashMap::from([
            (flow::PROCEDURAL_3D_PLAY_WINDOW_MAIN.to_string(), flow::window_measures(&config.lod_mode, procedural3d_action)),
            (edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    /// 🗂️ Grouped disclosure: `reorganize`/`translateSelection`/`rotateSelection`/`scaleSelection` stay
    /// top-level; creation, removal and generation methods fold into taxonomy groups; `delete-selection`
    /// stays a direct destructive item last.
    fn context_menu(&self, request: &semio_framework_plugin::ContextMenuRequest, _doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let config = cfg.projection;
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
            .window_kind_def(flow::definition())
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
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"))
            .operation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_with(ActionDefinition::new_catalog("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"), ActionKind::Operation).with_category("targets"))
            .operation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Operation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("patchFlowWidgets", LocalizedLabel::native("Patch Flow Widgets", "Flow-Elemente aktualisieren"), ActionKind::Operation).with_category("methods"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Operation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::Operation).with_category("targets"))
            .action_with(ActionDefinition::new_catalog("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::Operation).with_category("methods"))
            .action_with(ActionDefinition::new_catalog("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::Operation).with_category("methods"))
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
            .config(Procedural3dPlayApp::default().config_spec())
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
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

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
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }

    /// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
    /// drains in production — a test has to do that draining itself.
    pub fn drain_flow_eval_ticks(app: &mut Procedural3dApp) {
        app.pending_effects();
        for _ in 0..1000 {
            let result = app.dispatch_typed(Procedural3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}), &meta("local")).expect("flowEvalTick");
            if !result.requested_effects.iter().any(|effect| matches!(effect, semio_framework_core::kernel::HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")) {
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

    #[test]
    fn declared_actions_bridge_to_commands() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Procedural3dPlayApp>(create_procedural3d_app);
    }

    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_procedural3d_app().definition).expect("app definition json");
        for id in [flow::PROCEDURAL_3D_PLAY_WINDOW_MAIN, edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, generations::PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS, form::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM, generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW] {
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
            let signature = format!("{:?}", app.projection().expect("projection").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect::<std::collections::BTreeSet<_>>());
            assert!(signatures.insert(signature.clone()), "duplicate fixture signature for {example_id}: {signature}");
        }
    }

    #[test]
    fn refresh_pending_effects_arms_flow_eval_tick_chain() {
        let mut app = app();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample(example::set_active_example::SetActiveExample { example_id: crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }), &semio_framework_plugin::testkit::meta("local"))
            .expect("set example");
        let effects = app.pending_effects();
        assert!(effects.iter().any(|effect| matches!(effect, semio_framework_core::kernel::HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")));
        drain_flow_eval_ticks(&mut app);
    }

    #[test]
    fn undo_redo_round_trips_flow_graph_edits() {
        let mut app = app();
        let before = app.projection().expect("projection").fixture.widgets.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, Procedural3dCommand::AddWidget(widget::add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }), |app| app.projection().expect("projection").fixture.widgets.len(), before, before + 1);
    }

    #[test]
    fn two_instances_converge_disjoint_widget_moves() {
        let widgets: Vec<String> = app().projection().expect("projection").fixture.widgets.iter().map(|widget| crate::artifacts::procedural3d::widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        semio_framework_plugin::testkit::assert_two_instances_converge::<Procedural3dPlayApp, (Option<f64>, Option<f64>)>(
            "mem://procedural3d-convergence",
            Procedural3dCommand::MoveMediaNode(graph::move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
            Procedural3dCommand::MoveMediaNode(graph::move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
            move |app| {
                let layout = &app.projection().expect("projection").fixture.layout;
                (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
            },
        );
    }

    #[test]
    fn procedural3d_labels_translate_catalogue_and_inspector_in_german() {
        let mut app = app();
        app.dispatch_typed(Procedural3dCommand::SetLocale(locale::set_locale::SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set locale");
        let catalogue = crate::apps::procedural3d::testkit::render(&mut app, catalogue_panel::PROCEDURAL_3D_PLAY_BODY_CATALOGUE);
        assert!(catalogue.contains("\"Elemente\""));
        let inspector = crate::apps::procedural3d::testkit::render(&mut app, inspection_panel::PROCEDURAL_3D_PLAY_BODY_INSPECTION);
        assert!(inspector.contains("Elemente:"));
    }

    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        let mut app = app_with_registry();
        let widgets: Vec<String> = app.projection().expect("projection").fixture.widgets.iter().map(|widget| crate::artifacts::procedural3d::widget_id(widget).to_string()).collect();
        assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
        app.dispatch_typed(Procedural3dCommand::SetSelection(selection::set_selection::SetSelection { node_ids: widgets.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("set selection");
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true);
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).map(|child| child.destructive == Some(true)).unwrap_or(false);
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
    }

    #[test]
    fn sun_measures_are_exposed_on_preview_windows() {
        let mut app = app();
        let measures = app.window_measures();
        assert!(measures.contains_key(edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
        assert!(measures.contains_key(generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW));
    }
}
//#endregion 🧪️Tests
