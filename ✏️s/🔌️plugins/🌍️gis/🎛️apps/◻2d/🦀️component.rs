//! 🗺️ GIS 2D play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the map canvas
//! and its chrome in `🎭️modes/✏️edit/🪟️windows/🗺️map` (+ its `🎚️options/*`), panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, the shared `MapHost`
//! projection in `🦀️maphost.rs`, and document-side compute in `crate::artifacts::gismap::engine`.

use crate::apps::gis2d::commands::{example, features, locale, selection, shell, view};
use crate::apps::gis2d::config::{Gis2dConfig, Gis2dConfigOperation};
use crate::apps::gis2d::modes::edit;
use crate::apps::gis2d::modes::edit::windows::map;
use crate::apps::gis2d::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::gis2d::terminology::gis2d_labels;
use crate::artifacts::gismap::engine::{gis2d_features_in_port, gis2d_io, gis2d_map_media, gis2d_map_out_port, gis_map_document_from_descriptor_json, positions_operations, regions_operations, routes_operations};
use crate::artifacts::gismap::op::GisMapOperation;
use crate::artifacts::gismap::{artifact_kind, GisMapDocument, GIS_MAP_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, 
    tree_item_with_action, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass,
    MediaError, MediaForm, MediaPayload, MediaType, Menu, UiNode, UiTreeItemNode, WindowMeasure,
};
use store::EngineHandles;
use serde_json::{json, Value};
use std::collections::HashMap;
use store::DocumentPack;

//#region 🔖️Constants
pub const GIS2D_PLAY_APP_ID: &str = "gis2d-play";

/// 🗂️ The app-wide map layer stack: `(id, native English name, icon id)`. Every chrome node (document
/// and catalogue trees, the layers/weights window options, the inspector summary) enumerates it.
pub const GIS_MAP_LAYER_IDS: &[(&str, &str, &str)] = &[
    ("raster", "Raster", "layers"),
    ("water", "Water", "cloud"),
    ("land", "Land", "trees"),
    ("roads", "Roads", "compass"),
    ("buildings", "Buildings", "building"),
    ("borders", "Borders", "square-dashed"),
    ("labels", "Labels", "type"),
    ("positions", "Positions", "crosshair"),
    ("positionLabels", "Position Labels", "tags"),
    ("routes", "Routes", "git-branch"),
    ("regions", "Regions", "layers"),
];

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub fn gis2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(GIS2D_PLAY_APP_ID).action(action, args)
}

/// 🌳️ A layer tree item — `tree_item_with_action` plus the icon that identifies each map layer, since
/// the SDK's `PanelKit` family has no icon-carrying constructor. Shared by the document and catalogue
/// panels, which render the same layer stack under different actions.
pub fn gis2d_layer_tree_item(id: String, label: impl Into<Label>, description: Option<String>, icon_id: &str, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon_id.into()), menu: None, ..tree_item_with_action(id, label, description, action) }
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Gis2dPlayApp::Command` — the SOLE dispatch surface for gis2d's own behavior, covering every
    /// action `create_gis2d_app` declares. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Gis2dCommand for GisMapDocument, GisMapOperation, Gis2dConfig, Gis2dConfigOperation {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "patchPositions" as "patch-positions" => patch_positions::PatchPositions,
        "patchRoutes" as "patch-routes" => patch_routes::PatchRoutes,
        "patchRoute" as "patch-route" => patch_route::PatchRoute,
        "setSelection" as "selection" => set_selection::SetSelection,
        "toggleLayerVisibility" as "toggle-layer-visibility" => toggle_layer_visibility::ToggleLayerVisibility,
        "fitWorld" as "fit-world" => fit_world::FitWorld,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setRenderMode" as "render-mode" => set_render_mode::SetRenderMode,
        "setVectorStyle" as "vector-style" => set_vector_style::SetVectorStyle,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "setFeatureSelection" as "feature-selection" => set_feature_selection::SetFeatureSelection,
        "setHover" as "hover" => set_hover::SetHover,
        "setSelectionMethod" as "selection-method" => set_selection_method::SetSelectionMethod,
        "setSelectionMode" as "selection-mode" => set_selection_mode::SetSelectionMode,
        "clearSelection" as "clear-selection" => clear_selection::ClearSelection,
        "selectAll" as "select-all" => select_all::SelectAll,
        "deselect" as "deselect" => deselect::Deselect,
        "focusFeature" as "focus-feature" => focus_feature::FocusFeature,
        "setLayerStrokeScale" as "layer-stroke-scale" => set_layer_stroke_scale::SetLayerStrokeScale,
        "setLocale" as "locale" => set_locale::SetLocale,
        "openSource" as "open-source" => open_source::OpenSource,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use example::set_active_example;
use features::{patch_positions, patch_route, patch_routes};
use locale::set_locale;
use selection::{clear_selection, deselect, focus_feature, select_all, set_feature_selection, set_selection, set_selection_method, set_selection_mode};
use shell::open_source;
use view::{fit_world, set_camera, set_hover, set_layer_stroke_scale, set_lod_mode, set_render_mode, set_vector_style, toggle_layer_visibility};
//#endregion 🔖️Commands

//#region 🔖️Gis2dPlayApp
/// 🗺️ GIS 2D map play app. The document holds positions/routes/regions; everything else (camera,
/// render mode, style, LOD, selection, hover, layer visibility, stroke weights, locale) is
/// [`Gis2dConfig`] — a session-only but real, undoable config artifact.
#[derive(Default)]
pub struct Gis2dPlayApp;

/// 🖱️ On-demand GIS tiled-map context menu from feature hit-test and selection — grouped
/// disclosure via `Menu::of(registry)`; `organize_context_menu` (run automatically at the
/// `VcsDocumentApp::context_menu` funnel) sorts the declared `.group(...)` rows into
/// `RIBBON_PARENT_CATEGORIES` taxonomy order and inserts the pre-destructive separator itself.
fn gis2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected_ids: &[String]) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    let hits = surface.map_or(&[][..], |s| s.hits.as_slice());
    let feature = hits.iter().find(|h| h.domain == "feature" || h.domain == "position" || h.domain == "route");
    if let Some(feature) = feature {
        let kind = if feature.domain == "route" { "route" } else { "position" };
        let selected = selected_ids.iter().any(|id| id == &feature.id);
        return Menu::of(registry)
            .action_args(
                "setFeatureSelection",
                json!({
                    "positions": if kind == "position" { vec![&feature.id] } else { Vec::<&String>::new() },
                    "routes": if kind == "route" { vec![&feature.id] } else { Vec::<&String>::new() },
                    "mode": "default",
                }),
            )
            .action_args("focusFeature", json!({ "featureId": feature.id, "featureKind": kind }))
            .when(selected, |m| m.group("selection", |m| m.action_args("deselect", json!({ "featureId": feature.id, "featureKind": kind }))))
            .when(kind == "position", |m| m.group("open", |m| m.action_args("openSource", json!({ "featureId": feature.id }))))
            .build();
    }
    let mut items = Menu::of(registry).action("selectAll").action("fitWorld").destructive("clearSelection").build();
    if let Some(clear) = items.iter_mut().find(|entry| entry.id == "clearSelection") {
        clear.disabled = selected_ids.is_empty().then_some(true);
    }
    items
}

impl DocumentApp for Gis2dPlayApp {
    type Projection = GisMapDocument;
    type Operation = GisMapOperation;
    type Config = Gis2dConfig;
    type ConfigOperation = Gis2dConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = Gis2dCommand;

    const APP_ID: &'static str = GIS2D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;

    fn initial_projection() -> GisMapDocument {
        crate::artifacts::gismap::engine::default_document()
    }

    /// 🔌️ `features:in`/`map:out` (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) plus the
    /// implicit document ports.
    fn io() -> Option<AppIo> {
        Some(gis2d_io())
    }

    fn whole_document_operation(projection: GisMapDocument) -> Option<GisMapOperation> {
        Some(GisMapOperation::SetDocument { document: projection })
    }

    /// 🎞️ `map:out` (see `crate::artifacts::gismap::engine::gis2d_map_media`) plus the inherited
    /// `document:out` default (the pack of `doc.projection`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &DocumentView<'_, GisMapDocument>) -> Result<Media, MediaError> {
        match port {
            "map:out" => Ok(gis2d_map_media(doc.projection)),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `features:in` normalizes an incoming `{positions,routes,regions}` descriptor into granular
    /// add/patch/remove operations against every collection (a generic vector-features sink — not
    /// pinned to `2d.map`, so a `draw`/another `gis2d`'s producer both work) plus the inherited
    /// `document:in` default (replicated inline for the same reason as `export_media`).
    fn import_media(port: &str, media: &Media, doc: &DocumentView<'_, GisMapDocument>) -> Result<Emit<GisMapOperation, Gis2dConfigOperation, Self::DraftOperation>, MediaError> {
        match port {
            "features:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "features:in only accepts a Structured JSON payload".into()));
                };
                let incoming = gis_map_document_from_descriptor_json(json);
                let document = doc.projection;
                let mut operations = positions_operations(&document.positions, &incoming.positions);
                operations.extend(routes_operations(&document.routes, &incoming.routes));
                operations.extend(regions_operations(&document.regions, &incoming.regions));
                Ok(Emit::operations(operations))
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <GisMapDocument as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match Self::whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Gis2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Gis2dCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(Value::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "setActiveExample" => Ok(Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "patchPositions" => Ok(Gis2dCommand::PatchPositions(patch_positions::PatchPositions {
                positions_json: str_arg(&["positionsJson", "positions_json"]).or_else(|| args.get("positions").map(ToString::to_string)).unwrap_or_else(|| "[]".into()),
            })),
            "patchRoutes" => Ok(Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes {
                route_ids: {
                    let mut ids = string_list("routeIds");
                    if ids.is_empty() {
                        ids = string_list("route_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: str_arg(&["value"]).unwrap_or_default(),
            })),
            "patchRoute" => Ok(Gis2dCommand::PatchRoute(patch_route::PatchRoute {
                route_id: str_arg(&["routeId", "route_id"]).unwrap_or_default(),
                field: str_arg(&["field"]).unwrap_or_default(),
                value: str_arg(&["value"]).unwrap_or_default(),
            })),
            "setSelection" => Ok(Gis2dCommand::SetSelection(set_selection::SetSelection { ids: string_list("ids") })),
            "toggleLayerVisibility" => Ok(Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default() })),
            "fitWorld" => Ok(Gis2dCommand::FitWorld(fit_world::FitWorld {})),
            "setCamera" => {
                let camera_json = str_arg(&["cameraJson", "camera_json"])
                    .or_else(|| args.get("camera").map(|value| if value.is_string() { value.as_str().unwrap_or("{}").to_string() } else { value.to_string() }))
                    .unwrap_or_else(|| "{}".into());
                Ok(Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json }))
            }
            "setRenderMode" => Ok(Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: str_arg(&["value", "renderMode", "render_mode"]).unwrap_or_default() })),
            "setVectorStyle" => Ok(Gis2dCommand::SetVectorStyle(set_vector_style::SetVectorStyle { value: str_arg(&["value", "vectorStyle", "vector_style"]).unwrap_or_default() })),
            "setLodMode" => Ok(Gis2dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setFeatureSelection" => Ok(Gis2dCommand::SetFeatureSelection(set_feature_selection::SetFeatureSelection {
                positions: string_list("positions"),
                routes: string_list("routes"),
                mode: str_arg(&["mode"]).unwrap_or_else(|| "default".into()),
            })),
            "setHover" => {
                let hover_json = str_arg(&["hoverJson", "hover_json"])
                    .or_else(|| args.get("hover").map(ToString::to_string))
                    .or_else(|| {
                        let object = args.as_object()?;
                        if object.is_empty() || object.keys().all(|key| key == "surfaceId") {
                            Some("null".into())
                        } else {
                            Some(args.to_string())
                        }
                    })
                    .unwrap_or_else(|| "null".into());
                Ok(Gis2dCommand::SetHover(set_hover::SetHover { hover_json }))
            }
            "setSelectionMethod" => Ok(Gis2dCommand::SetSelectionMethod(set_selection_method::SetSelectionMethod { value: str_arg(&["value", "selectionMethod", "selection_method"]).unwrap_or_default() })),
            "setSelectionMode" => Ok(Gis2dCommand::SetSelectionMode(set_selection_mode::SetSelectionMode { value: str_arg(&["value", "selectionMode", "selection_mode"]).unwrap_or_default() })),
            "clearSelection" => Ok(Gis2dCommand::ClearSelection(clear_selection::ClearSelection {})),
            "selectAll" => Ok(Gis2dCommand::SelectAll(select_all::SelectAll {})),
            "deselect" => Ok(Gis2dCommand::Deselect(deselect::Deselect {
                feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(),
                feature_kind: str_arg(&["featureKind", "feature_kind"]).unwrap_or_else(|| "position".into()),
            })),
            "focusFeature" => Ok(Gis2dCommand::FocusFeature(focus_feature::FocusFeature {
                feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(),
                feature_kind: str_arg(&["featureKind", "feature_kind"]).unwrap_or_else(|| "position".into()),
            })),
            "setLayerStrokeScale" => Ok(Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default(), value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setLocale" => Ok(Gis2dCommand::SetLocale(set_locale::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() })),
            "openSource" => Ok(Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default() })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(command: &Gis2dCommand, doc: &DocumentView<'_, GisMapDocument>, cfg: &ConfigView<'_, Gis2dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<GisMapOperation, Gis2dConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🧮️ Empty — gis2d's `Config` is session view state (camera/selection/layer visibility/…), not a
    /// user-facing settings record; `ConfigSpec::empty()` (the trait default) is correct as-is.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty()
    }

    fn render(body_key: &str, doc: &DocumentView<'_, GisMapDocument>, cfg: &ConfigView<'_, Gis2dConfig>) -> UiNode {
        let config = cfg.projection;
        let labels = gis2d_labels(config);
        match body_key {
            map::GIS2D_PLAY_BODY_COMPOSITE => map::render(doc.projection, config),
            document_panel::GIS2D_PLAY_BODY_DOCUMENT => document_panel::render(config, labels),
            catalogue_panel::GIS2D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            inspection_panel::GIS2D_PLAY_BODY_INSPECTION => inspection_panel::render(config, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(_doc: &DocumentView<'_, GisMapDocument>, cfg: &ConfigView<'_, Gis2dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        HashMap::from([(map::GIS2D_PLAY_WINDOW_MAIN.into(), map::window_measures(config, gis2d_labels(config)))])
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, GisMapDocument>,
        cfg: &ConfigView<'_, Gis2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        gis2d_context_menu_items(registry, request.surface.as_ref(), &cfg.projection.selected_ids)
    }
}
//#endregion 🔖️Gis2dPlayApp

//#region 🔖️Manifest
pub fn create_gis2d_app() -> App {
    App::from_builder(
        App::builder(GIS2D_PLAY_APP_ID, LocalizedLabel::native("GIS 2D", "GIS 2D")).document(["semio", "gis", "2d"])
            .artifact_kind(artifact_kind())
            // 🔌️ Typed workflow ports (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) — same
            // constructor fns `gis2d_io()` embeds, so `AppIo.all_ports()` and these declarations can
            // never drift apart. `map:out`'s `2d.map` kind is declared above; `features:in` pins no kind.
            .media_input(gis2d_features_in_port())
            .media_output(gis2d_map_out_port())
            .icon_id("gis2d")
            .mode_def(edit::definition())
            .default_mode_id(edit::GIS2D_PLAY_MODE_EDIT)
            .window_kind_def(map::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Operation actions — flow through the document store with true inverses. `setActiveExample`
            // replaces document content via `SetDocument` operations, so it is an Operation, not a View action.
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("patchPositions", LocalizedLabel::native("Patch Positions", "Positionen aktualisieren"))
            .operation("patchRoutes", LocalizedLabel::native("Patch Routes", "Routen aktualisieren"))
            .operation("patchRoute", LocalizedLabel::native("Patch Route", "Route aktualisieren"))
            // 👁️ View actions — mutate ephemeral config state (selection, camera, render config,
            // hover, layer visibility, stroke weights), never the document.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("toggleLayerVisibility", LocalizedLabel::native("Toggle Layer Visibility", "Ebenensichtbarkeit umschalten"))
            .action_with(ActionDefinition::new_catalog("fitWorld", LocalizedLabel::native("Fit World", "Welt einpassen"), ActionKind::View).with_category("view"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setRenderMode", LocalizedLabel::native("Set Render Mode", "Darstellungsmodus festlegen"))
            .view_action("setVectorStyle", LocalizedLabel::native("Set Vector Style", "Vektorstil festlegen"))
            .view_action("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"))
            .action_with(ActionDefinition::new_catalog("setFeatureSelection", LocalizedLabel::native("Set Feature Selection", "Objektauswahl festlegen"), ActionKind::View).with_category("selection"))
            .view_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"))
            .view_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"))
            .view_action("setSelectionMode", LocalizedLabel::native("Set Selection Mode", "Auswahlmodus festlegen"))
            .action_with(ActionDefinition::new_catalog("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"), ActionKind::View).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::View).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("deselect", LocalizedLabel::native("Deselect", "Abwählen"), ActionKind::View).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("focusFeature", LocalizedLabel::native("Focus Feature", "Objekt fokussieren"), ActionKind::View).with_category("view"))
            .view_action("setLayerStrokeScale", LocalizedLabel::native("Set Layer Stroke Scale", "Ebenenstrichstärke festlegen"))
            // 🌐️ Shell action — opens the picked feature's source URL through the host.
            .action_with(ActionDefinition::new_catalog("openSource", LocalizedLabel::native("Open Source", "Quelle öffnen"), ActionKind::Shell).with_category("open"))
            // 📝️ Argument schemas for the discrete-choice actions so the command palette can stage them
            // and the registry validates the vocabulary. The arg id matches the key each handler reads.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("reuse-map", LocalizedLabel::native("Reuse Map", "Karte wiederverwenden")),
                ]).default_value("reuse-map"),
            ])
            .action_args("setRenderMode", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Render Mode", "Darstellungsmodus"), vec![
                    ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
                    ActionArgOption::new("vector", LocalizedLabel::native("Vector", "Vektor")),
                    ActionArgOption::new("combined", LocalizedLabel::native("Combined", "Kombiniert")),
                ]).default_value("combined"),
            ])
            .action_args("setVectorStyle", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Vector Style", "Vektorstil"), vec![
                    ActionArgOption::new("colored", LocalizedLabel::native("Colored", "Farbig")),
                    ActionArgOption::new("figureGround", LocalizedLabel::native("Figure Ground", "Figur-Grund")),
                    ActionArgOption::new("invertedFigure", LocalizedLabel::native("Inverted Figure", "Invertierte Figur")),
                ]).default_value("colored"),
            ])
            .action_args("setLodMode", vec![
                ActionArgDef::select("value", LocalizedLabel::native("LOD Mode", "LOD-Modus"), map::options::lod_mode::lod_arg_options()).default_value(framework_surface_tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC),
            ])
            .action_args("setSelectionMethod", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Selection Method", "Auswahlmethode"), vec![
                    ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")),
                    ActionArgOption::new("lasso", LocalizedLabel::native("Lasso", "Lasso")),
                ]).default_value("rectangle"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis2dPlayApp::config_spec())
            .io(gis2d_io()),
    )
    .example("reuse-map", LocalizedLabel::native("Reuse Map", "Karte wiederverwenden"), serde_json::to_string(&crate::artifacts::gismap::engine::default_document()).unwrap_or_default(), "file-text")
    .workflow("gis2d", "GIS 2D", "map")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type Gis2dApp = VcsDocumentApp<Gis2dPlayApp>;

    pub fn app() -> Gis2dApp {
        new_app::<Gis2dPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View/Shell-emits-operations rejection) runs.
    pub fn app_with_registry() -> Gis2dApp {
        new_app_with_registry::<Gis2dPlayApp>(create_gis2d_app)
    }

    pub fn dispatch(app: &mut Gis2dApp, command: Gis2dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Gis2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }

    pub fn main_window_measures(app: &mut Gis2dApp) -> Vec<WindowMeasure> {
        app.window_measures().get(map::GIS2D_PLAY_WINDOW_MAIN).cloned().unwrap_or_default()
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::testkit::{app, app_with_registry, render};
    use protocol::CollectionOperation;
    use semio_framework_plugin::{ContextMenuRequest, PluginApp};

    //#region 🔖️CommandSurface
    /// 🎯️ One value per `app_commands!` row, in row order — the wire-law loop below and the id
    /// uniqueness check both run off this list, so a new row that forgets to appear here fails the
    /// coverage assertion.
    fn every_command() -> Vec<Gis2dCommand> {
        vec![
            Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "reuse-map".into() }),
            Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: r#"[{"id":"p1","lon":1.0,"lat":2.0}]"#.into() }),
            Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: vec!["r1".into(), "r2".into()], field: "label".into(), value: "Home".into() }),
            Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: "r1".into(), field: "label".into(), value: "Home".into() }),
            Gis2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["roads".into()] }),
            Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() }),
            Gis2dCommand::FitWorld(fit_world::FitWorld {}),
            Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into() }),
            Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }),
            Gis2dCommand::SetVectorStyle(set_vector_style::SetVectorStyle { value: "colored".into() }),
            Gis2dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "automatic".into() }),
            Gis2dCommand::SetFeatureSelection(set_feature_selection::SetFeatureSelection { positions: vec!["p1".into()], routes: vec!["r1".into()], mode: "default".into() }),
            Gis2dCommand::SetHover(set_hover::SetHover { hover_json: "null".into() }),
            Gis2dCommand::SetSelectionMethod(set_selection_method::SetSelectionMethod { value: "lasso".into() }),
            Gis2dCommand::SetSelectionMode(set_selection_mode::SetSelectionMode { value: "additive".into() }),
            Gis2dCommand::ClearSelection(clear_selection::ClearSelection {}),
            Gis2dCommand::SelectAll(select_all::SelectAll {}),
            Gis2dCommand::Deselect(deselect::Deselect { feature_id: "p1".into(), feature_kind: "position".into() }),
            Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: "p1".into(), feature_kind: "position".into() }),
            Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 1.5 }),
            Gis2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "p1".into() }),
        ]
    }

    /// 🏷️ The wire keyword each row prints under — the kebab `as` literal, independent of the camelCase
    /// manifest action id. Pinned so a reordered/renamed row is caught here, not in production.
    const WIRE_KEYWORDS: &[&str] = &[
        "active-example",
        "patch-positions",
        "patch-routes",
        "patch-route",
        "selection",
        "toggle-layer-visibility",
        "fit-world",
        "camera",
        "render-mode",
        "vector-style",
        "lod-mode",
        "feature-selection",
        "hover",
        "selection-method",
        "selection-mode",
        "clear-selection",
        "select-all",
        "deselect",
        "focus-feature",
        "layer-stroke-scale",
        "locale",
        "open-source",
    ];

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Gis2dCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 22, "every Gis2dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        assert_eq!(every_command().len(), WIRE_KEYWORDS.len());
        for (command, keyword) in every_command().iter().zip(WIRE_KEYWORDS) {
            store::test_support::assert_op_text_binary_equivalence(command);
            let printed = protocol::OpText::print_op(command);
            assert!(printed == *keyword || printed.starts_with(&format!("{keyword} ")), "row {} printed {printed:?}, expected the {keyword:?} wire keyword", command.command_id());
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for the rows the `app_commands!` decomposition could have
    /// silently rewritten: the three fieldless payloads (were unit variants) and the `Vec`-carrying
    /// rows whose empty/non-empty shapes are distinct wire cases. Hex copied verbatim from the
    /// pre-migration baseline dump (ticket
    /// `26/08/05/GIS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`, `🧪️wire-baseline-2d-before.txt`).
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Gis2dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(hex(&Gis2dCommand::FitWorld(fit_world::FitWorld {})), "01060000");
        assert_eq!(hex(&Gis2dCommand::ClearSelection(clear_selection::ClearSelection {})), "010f0000");
        assert_eq!(hex(&Gis2dCommand::SelectAll(select_all::SelectAll {})), "01100000");
        assert_eq!(hex(&Gis2dCommand::SetSelection(set_selection::SetSelection { ids: Vec::new() })), "01040001000c00");
        assert_eq!(hex(&Gis2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["roads".into()] })), "01040105726f61647301000c010600");
        assert_eq!(hex(&Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: Vec::new(), field: "label".into(), value: String::new() })), "01020200056c6162656c03000c00010601020600");
        assert_eq!(
            hex(&Gis2dCommand::SetFeatureSelection(set_feature_selection::SetFeatureSelection { positions: Vec::new(), routes: Vec::new(), mode: "additive".into() })),
            "010b0108616464697469766503000c00010c00020600"
        );
        assert_eq!(hex(&Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 1.5 })), "01130105726f616473020006000105000000000000f83f");
    }

    /// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`. Uses the framework's own harness, which stages each action's declared args and
    /// knows the framework-injected ids to skip (`undo`/`copy`/`recordTutorial`/…).
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Gis2dPlayApp>(create_gis2d_app);
        assert!(Gis2dPlayApp.command_from_action("noSuchAction", None).is_err());
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_gis2d_app().definition;
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        // 🧷️ The framework injects its own panel tabs on top of the app's three, so assert the app's
        // own tabs are stitched in rather than pinning a total.
        for body_key in [document_panel::GIS2D_PLAY_BODY_DOCUMENT, catalogue_panel::GIS2D_PLAY_BODY_CATALOGUE, inspection_panel::GIS2D_PLAY_BODY_INSPECTION] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "2d.map"));
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = app();
        assert!(render(&mut app, "gis2d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Media
    #[test]
    fn export_media_map_out_produces_a_2d_map_structured_payload() {
        let app = app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = Gis2dPlayApp.export_media("map:out", &doc).expect("map:out export");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.map");
        assert!(json.contains("positions"));
    }

    #[test]
    fn import_media_features_in_adds_new_positions_as_operations() {
        let app = app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let incoming = json!({ "positions": [{ "id": "imported-1", "lon": 1.0, "lat": 2.0 }] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.map".into(), json: incoming } };
        let emit = Gis2dPlayApp.import_media("features:in", &media, &doc).expect("features:in import");
        assert!(emit.document_operations.iter().any(|operation| matches!(operation, GisMapOperation::Positions(CollectionOperation::Add { id, .. }) if id == "imported-1")));
    }

    #[test]
    fn media_ports_declare_features_in_and_map_out() {
        let app = Gis2dPlayApp;
        let ports = app.media_ports();
        assert!(ports.iter().any(|port| port.id == "features:in"));
        assert!(ports.iter().any(|port| port.id == "map:out"));
    }
    //#endregion 🔖️Media

    //#region 🔖️ContextMenu
    /// 🖱️ Grouped disclosure: the empty-canvas context menu (no feature under the pointer) stays
    /// within the row budget and keeps the known destructive `clearSelection` last, matching the
    /// canonical migration pattern.
    #[test]
    fn context_menu_stays_within_budget_and_keeps_clear_selection_destructive_last() {
        let mut app = app_with_registry();
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "gis2dMap".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("empty-canvas context menu should not be empty");
        assert_eq!(last.id, "clearSelection", "known destructive clearSelection must be last: {menu:?}");
        assert_eq!(last.destructive, Some(true), "clearSelection must be marked destructive: {menu:?}");
    }
    //#endregion 🔖️ContextMenu
}
//#endregion 🧪️Tests
