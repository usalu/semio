//! 🖥️ GIS 2D app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! migration — `Gis2dPlayApp` is a unit struct; every former `Gis2dPlayRuntime` field (selection,
//! camera, render/vector/LOD mode, feature selection/hover, layer visibility/stroke-weight, …) now
//! lives in `gis2d_engine::Gis2dConfig`, written via `gis2d_op::Gis2dConfigOperation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `gis2d_protocol::Gis2dCommand` channel via `DocumentApp::handle`.

use gis2d::{MapFeature, MapFeaturePatch, GIS_MAP_SCHEMA};
use gis2d_engine::{default_document, gis2d_features_in_port, gis2d_io, gis2d_map_media, gis2d_map_out_port, gis_map_descriptor_json, gis_map_document_from_descriptor_json, Gis2dConfig};
use gis2d_op::{Gis2dConfigOperation, GisMapOperation};
use gis2d_protocol::Gis2dCommand;
use framework_surface_tiled_map::{clamp_map_layer_weight, gis_map_layer_weight_slider_ids_json, gis_map_lod_scale_json, MapHost, GIS_MAP_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::{SurfaceKind, PanelGroup,
    app_labels, build_tiled_map_scene, create_default_layout, localized_label_map, tree_item_with_action,
    MeasureSelectItem, ui_inspector_groups_to_tree, ui_inspector_mixed_toggle,
    ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDescriptor, App, AppIo, ArtifactKindSpec, AppLabelsOverlay, AppLabelsOverlayExt,
    ConfigView, DocumentApp, DocumentView, Emit, LocaleLabels, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, OsMediaFormat, PanelTreeBuilder, TiledMapScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode,
    UiToggleNode, UiTreeItemNode, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use dsl::DslValue;
use std::collections::{HashMap, HashSet};
use protocol::CollectionOperation;
use store::DocumentPack;

//#region 🔖️Constants
const GIS2D_PLAY_APP_ID: &str = "gis2d-play";
const GIS2D_PLAY_SURFACE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_COMPOSITE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_DOCUMENT: &str = "gis2d.play.document";
const GIS2D_PLAY_BODY_CATALOGUE: &str = "gis2d.play.catalogue";
const GIS2D_PLAY_BODY_INSPECTION: &str = "gis2d.play.inspection";
const GIS2D_PLAY_WINDOW_MAIN: &str = "gis2d-main";

const GIS_MAP_LAYER_IDS: &[(&str, &str, &str)] = &[
    ("raster", "Raster", "map"),
    ("water", "Water", "droplets"),
    ("land", "Land", "mountain"),
    ("roads", "Roads", "route"),
    ("buildings", "Buildings", "building"),
    ("borders", "Borders", "square-dashed"),
    ("labels", "Labels", "type"),
    ("positions", "Positions", "map-pin"),
    ("positionLabels", "Position Labels", "tag"),
    ("routes", "Routes", "git-branch"),
    ("regions", "Regions", "layers"),
];
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s identical fix.
fn is_de_locale(cfg: &Gis2dConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &Gis2dConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn default_layer_visibility() -> HashMap<String, bool> {
    GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), true)).collect()
}

/// 🗺️ Builds a `MapHost` from the document content (derived descriptor JSON) plus the config's
/// camera/render/style/LOD/selection view state.
fn map_host_from(document: &gis2d::GisMapDocument, cfg: &Gis2dConfig) -> MapHost {
    let mut host = MapHost::new();
    let descriptor = gis_map_descriptor_json(document);
    let _ = host.sync_map_json(&descriptor);
    if let Ok(camera) = serde_json::from_str::<Value>(&cfg.camera_json) {
        let x = camera.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let y = camera.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let zoom = camera.get("zoom").and_then(|value| value.as_f64()).unwrap_or(1.0);
        host.set_camera(x, y, zoom);
    }
    host.set_render_mode(&cfg.render_mode);
    host.set_vector_style(&cfg.vector_style);
    host.set_lod_mode(&cfg.lod_mode);
    let _ = host.set_selection_json(&cfg.feature_selection_json);
    host
}

fn layer_visibility_json(cfg: &Gis2dConfig) -> String {
    let mut map = default_layer_visibility();
    for (id, visible) in &cfg.layer_visibility {
        map.insert(id.clone(), *visible);
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

fn layer_stroke_scale_json(cfg: &Gis2dConfig) -> String {
    let mut map: HashMap<String, f64> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, _, _)| ((*id).into(), 1.0))
        .collect();
    for (id, weight) in &cfg.layer_stroke_scale {
        map.insert(id.clone(), clamp_map_layer_weight(*weight));
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

fn merge_feature_selection(
    current_json: &str,
    positions: Vec<String>,
    routes: Vec<String>,
    mode: &str,
) -> Value {
    let current: Value = serde_json::from_str(current_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let current_positions: Vec<String> = current
        .get("positions")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    let current_routes: Vec<String> = current
        .get("routes")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    let mut next_positions: HashSet<String> = current_positions.iter().cloned().collect();
    let mut next_routes: HashSet<String> = current_routes.iter().cloned().collect();
    let incoming_positions: HashSet<String> = positions.into_iter().collect();
    let incoming_routes: HashSet<String> = routes.into_iter().collect();
    match mode {
        "additive" => {
            next_positions.extend(incoming_positions);
            next_routes.extend(incoming_routes);
        }
        "subtractive" => {
            next_positions.retain(|id| !incoming_positions.contains(id));
            next_routes.retain(|id| !incoming_routes.contains(id));
        }
        "invertive" => {
            for id in incoming_positions {
                if !next_positions.insert(id.clone()) {
                    next_positions.remove(&id);
                }
            }
            for id in incoming_routes {
                if !next_routes.insert(id.clone()) {
                    next_routes.remove(&id);
                }
            }
        }
        _ => {
            next_positions = incoming_positions;
            next_routes = incoming_routes;
        }
    }
    json!({
        "positions": next_positions.into_iter().collect::<Vec<_>>(),
        "routes": next_routes.into_iter().collect::<Vec<_>>(),
    })
}

fn gis2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: GIS2D_PLAY_APP_ID.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

/// 🌉️ Diffs one feature collection before/after an in-place edit into granular id-keyed
/// add/remove/patch operations — used by `patchPositions` and the `features:in` import (whole-array
/// replacements still converge per-feature). `wrap` picks which `GisMapOperation` variant
/// (`Positions`/`Routes`/`Regions`) the diff belongs to.
fn feature_collection_operations(before: &[MapFeature], after: &[MapFeature], wrap: impl Fn(CollectionOperation<String, MapFeature, MapFeaturePatch>) -> GisMapOperation) -> Vec<GisMapOperation> {
    let mut operations = Vec::new();
    let after_ids: HashSet<&str> = after.iter().map(|feature| feature.id.as_str()).collect();
    for feature in before {
        if !after_ids.contains(feature.id.as_str()) {
            operations.push(wrap(CollectionOperation::Remove { id: feature.id.clone() }));
        }
    }
    for (index, feature) in after.iter().enumerate() {
        match before.iter().find(|entry| entry.id == feature.id) {
            None => operations.push(wrap(CollectionOperation::Add { id: feature.id.clone(), item: feature.clone(), at: index })),
            Some(prev) if prev.data != feature.data => operations.push(wrap(CollectionOperation::Patch {
                id: feature.id.clone(),
                patch: MapFeaturePatch { data: Some(feature.data.clone()) },
            })),
            Some(_) => {}
        }
    }
    operations
}

fn positions_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapOperation> {
    feature_collection_operations(before, after, GisMapOperation::Positions)
}

fn routes_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapOperation> {
    feature_collection_operations(before, after, GisMapOperation::Routes)
}

fn regions_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapOperation> {
    feature_collection_operations(before, after, GisMapOperation::Regions)
}

fn layer_visible(cfg: &Gis2dConfig, layer_id: &str) -> bool {
    cfg.layer_visibility.get(layer_id).copied().unwrap_or(true)
}

fn layer_weight_slider_fields(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> Vec<UiNode> {
    layer_weight_entries(cfg, labels)
        .into_iter()
        .map(|(layer_id, label, value)| {
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("gis2d-play-inspector.weight.{layer_id}"),
                label: format!("{label} {}", labels.weight_suffix),
                child: Box::new(UiNode::Slider(UiSliderNode {presence: UiPresence::default(),
                    id: format!("gis2d-play-inspector.weight.{layer_id}.slider"),
                    value,
                    min: 0.25,
                    max: 3.0,
                    step: 0.05,
                    on_change: gis2d_action(
                        "setLayerStrokeScale",
                        Some(json!({ "layerId": layer_id })),
                    ),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            })
        })
        .collect()
}

fn lod_select_entries(labels: &Gis2dPlayLabels) -> Vec<(String, String)> {
    std::iter::once((GIS_MAP_LOD_MODE_AUTOMATIC.into(), labels.lod_automatic.into()))
        .chain(
            serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json())
                .unwrap_or_default()
                .into_iter()
                .filter_map(|lod| {
                    let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
                    let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
                    Some((id, name))
                }),
        )
        .collect()
}

fn layer_weight_entries(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> Vec<(String, String, f64)> {
    let ids: Vec<String> = serde_json::from_str(&gis_map_layer_weight_slider_ids_json(
        &cfg.lod_mode,
        &cfg.render_mode,
    ))
    .unwrap_or_default();
    ids.into_iter()
        .map(|layer_id| {
            let value = cfg
                .layer_stroke_scale
                .get(&layer_id)
                .copied()
                .map(clamp_map_layer_weight)
                .unwrap_or(1.0);
            let label = gis2d_layer_label(&layer_id, labels).to_string();
            (layer_id, label, value)
        })
        .collect()
}

fn gis2d_window_measures(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> Vec<WindowMeasure> {
    let layer_toggles: Vec<WindowMeasure> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, _, icon)| WindowMeasure::Toggle {
            id: format!("gis2d-play-window.layer.{id}"),
            icon_id: (*icon).into(),
            label: Some(gis2d_layer_label(id, labels).into()),
            pressed: layer_visible(cfg, id),
            text: None,
            on_change: gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": id }))),
        })
        .collect();
    let layer_weight_sliders: Vec<WindowMeasure> = layer_weight_entries(cfg, labels)
        .into_iter()
        .map(|(layer_id, label, value)| WindowMeasure::Slider {
            id: format!("gis2d-play-window.weight.{layer_id}"),
            label: Some(format!("{label} {}", labels.weight_suffix)),
            value,
            min: 0.25,
            max: 3.0,
            step: Some(0.05),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: gis2d_action("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),

            waiting: None,})
        .collect();
    vec![
        WindowMeasure::Select {
            id: "gis2d-play-window.render-mode".into(),
            label: Some(labels.render_mode.into()),
            value: cfg.render_mode.clone(),
            items: vec![
                MeasureSelectItem { id: "image".into(), value: "image".into(), label: labels.render_mode_image.into() },
                MeasureSelectItem { id: "vector".into(), value: "vector".into(), label: labels.render_mode_vector.into() },
                MeasureSelectItem { id: "combined".into(), value: "combined".into(), label: labels.render_mode_combined.into() },
            ],
            on_change: gis2d_action("setRenderMode", None),
        },
        WindowMeasure::Select {
            id: "gis2d-play-window.vector-style".into(),
            label: Some(labels.vector_style.into()),
            value: cfg.vector_style.clone(),
            items: vec![
                MeasureSelectItem { id: "colored".into(), value: "colored".into(), label: labels.vector_style_colored.into() },
                MeasureSelectItem { id: "figureGround".into(), value: "figureGround".into(), label: labels.vector_style_figure_ground.into() },
                MeasureSelectItem { id: "invertedFigure".into(), value: "invertedFigure".into(), label: labels.vector_style_inverted_figure.into() },
            ],
            on_change: gis2d_action("setVectorStyle", None),
        },
        WindowMeasure::Select {
            id: "gis2d-play-window.lod-mode".into(),
            label: Some(labels.lod_mode.into()),
            value: cfg.lod_mode.clone(),
            items: lod_select_entries(labels)
                .into_iter()
                .map(|(value, label)| MeasureSelectItem { id: value.clone(), value, label })
                .collect(),
            on_change: gis2d_action("setLodMode", None),
        },
        WindowMeasure::Select {
            id: "gis2d-play-window.selection-method".into(),
            label: Some(labels.selection_method.into()),
            value: cfg.selection_method.clone(),
            items: vec![
                MeasureSelectItem { id: "rectangle".into(), value: "rectangle".into(), label: labels.selection_method_rectangle.into() },
                MeasureSelectItem { id: "lasso".into(), value: "lasso".into(), label: labels.selection_method_lasso.into() },
            ],
            on_change: gis2d_action("setSelectionMethod", None),
        },
        WindowMeasure::Group {
            id: "gis2d-play-window.layers".into(),
            label: labels.layers_group.into(),
            default_open: Some(true),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: layer_toggles,
        },
        WindowMeasure::Group {
            id: "gis2d-play-window.layer-weights".into(),
            label: labels.layer_weights_group.into(),
            default_open: Some(false),
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: layer_weight_sliders,
            active_utility_id: None,
        },
    ]
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the GIS 2D app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct Gis2dPlayLabels {
        window_map: &'static str = en: "Map", de: "Karte";
        mode_edit: &'static str = en: "Edit", de: "Bearbeiten";
        layer_raster: &'static str = en: "Raster", de: "Raster";
        layer_water: &'static str = en: "Water", de: "Wasser";
        layer_land: &'static str = en: "Land", de: "Land";
        layer_roads: &'static str = en: "Roads", de: "Straßen";
        layer_buildings: &'static str = en: "Buildings", de: "Gebäude";
        layer_borders: &'static str = en: "Borders", de: "Grenzen";
        layer_map_labels: &'static str = en: "Labels", de: "Beschriftungen";
        layer_positions: &'static str = en: "Positions", de: "Positionen";
        layer_position_labels: &'static str = en: "Position Labels", de: "Positionsbeschriftungen";
        layer_routes: &'static str = en: "Routes", de: "Routen";
        layer_regions: &'static str = en: "Regions", de: "Regionen";
        map_view: &'static str = en: "Map View", de: "Kartenansicht";
        render_mode: &'static str = en: "Render Mode", de: "Darstellungsmodus";
        render_mode_image: &'static str = en: "Image", de: "Bild";
        render_mode_vector: &'static str = en: "Vector", de: "Vektor";
        render_mode_combined: &'static str = en: "Combined", de: "Kombiniert";
        vector_style: &'static str = en: "Vector Style", de: "Vektorstil";
        vector_style_colored: &'static str = en: "Colored", de: "Farbig";
        vector_style_figure_ground: &'static str = en: "Figure Ground", de: "Figur-Grund";
        vector_style_inverted_figure: &'static str = en: "Inverted Figure", de: "Invertierte Figur";
        lod_mode: &'static str = en: "LOD Mode", de: "LOD-Modus";
        lod_automatic: &'static str = en: "Automatic", de: "Automatisch";
        selection_method: &'static str = en: "Selection Method", de: "Auswahlmethode";
        selection_method_rectangle: &'static str = en: "Rectangle", de: "Rechteck";
        selection_method_lasso: &'static str = en: "Lasso", de: "Lasso";
        layers_group: &'static str = en: "Layers", de: "Ebenen";
        layer_weights_group: &'static str = en: "Layer Weights", de: "Ebenengewichte";
        weight_suffix: &'static str = en: "weight", de: "Gewicht";
        selected_features: &'static str = en: "Selected Features", de: "Ausgewählte Objekte";
        map_layer: &'static str = en: "Map Layer", de: "Kartenebene";
        schema: &'static str = en: "Schema", de: "Schema";
        layers_visible: &'static str = en: "Layers visible", de: "Sichtbare Ebenen";
        field_id: &'static str = en: "Id", de: "Id";
        field_label: &'static str = en: "Label", de: "Bezeichnung";
        field_visible: &'static str = en: "Visible", de: "Sichtbar";
    }
}

/// 🗣️ Resolves a standard map layer's display label from its stable id; unknown ids fall back to the catalog's native English text.
fn gis2d_layer_label(layer_id: &str, labels: &Gis2dPlayLabels) -> &'static str {
    match layer_id {
        "raster" => labels.layer_raster,
        "water" => labels.layer_water,
        "land" => labels.layer_land,
        "roads" => labels.layer_roads,
        "buildings" => labels.layer_buildings,
        "borders" => labels.layer_borders,
        "labels" => labels.layer_map_labels,
        "positions" => labels.layer_positions,
        "positionLabels" => labels.layer_position_labels,
        "routes" => labels.layer_routes,
        "regions" => labels.layer_regions,
        // 🗣️ unreachable in practice — the arms above already cover every id in GIS_MAP_LAYER_IDS.
        _ => "",
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in
/// `create_gis2d_app`'s static manifest — the manifest itself has no `cfg`/locale parameter, so
/// this overlay is how the command palette and Actions rail get a translated label without threading
/// locale through the whole builder chain.
fn gis2d_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("patchPositions", "Patch Positions", "Positionen aktualisieren"),
        ("patchRoutes", "Patch Routes", "Routen aktualisieren"),
        ("patchRoute", "Patch Route", "Route aktualisieren"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("toggleLayerVisibility", "Toggle Layer Visibility", "Ebenensichtbarkeit umschalten"),
        ("fitWorld", "Fit World", "Welt einpassen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setRenderMode", "Set Render Mode", "Darstellungsmodus festlegen"),
        ("setVectorStyle", "Set Vector Style", "Vektorstil festlegen"),
        ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
        ("setFeatureSelection", "Set Feature Selection", "Objektauswahl festlegen"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("setSelectionMethod", "Set Selection Method", "Auswahlmethode festlegen"),
        ("setSelectionMode", "Set Selection Mode", "Auswahlmodus festlegen"),
        ("clearSelection", "Clear Selection", "Auswahl aufheben"),
        ("selectAll", "Select All", "Alles auswählen"),
        ("deselect", "Deselect", "Abwählen"),
        ("focusFeature", "Focus Feature", "Objekt fokussieren"),
        ("setLayerStrokeScale", "Set Layer Stroke Scale", "Ebenenstrichstärke festlegen"),
        ("openSource", "Open Source", "Quelle öffnen"),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
/// 🌳️ A layer tree item — `tree_item_with_action` plus the icon that identifies each map layer, since
/// the SDK's `PanelKit` family has no icon-carrying constructor.
fn gis2d_layer_tree_item(id: String, label: &str, description: Option<String>, icon_id: &str, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon_id.into()), menu: None,
    ..tree_item_with_action(id, label, description, action) }
}

fn build_document_tree(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("gis2d-play-document");
    let layer_items: Vec<UiTreeItemNode> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, _, icon)| {
            gis2d_layer_tree_item(
                builder.item_id("layer", id),
                gis2d_layer_label(id, labels),
                Some((*id).into()),
                icon,
                gis2d_action("setSelection", Some(json!({ "ids": [id] }))),
            )
        })
        .collect();
    builder
        .section("gis2d-play-document.layers", Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, layer_items)
        .selected(cfg.selected_ids.iter().map(|id| format!("gis2d-play-document.layer.{id}")).collect())
        .selection_change(gis2d_action("setSelection", None))
        .build()
}

fn build_catalogue_tree(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let _ = cfg;
    let builder = PanelTreeBuilder::new("gis2d-play-catalogue");
    let items: Vec<UiTreeItemNode> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, _, icon)| {
            gis2d_layer_tree_item(
                builder.item_id("layer", id),
                gis2d_layer_label(id, labels),
                None,
                icon,
                gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": id }))),
            )
        })
        .collect();
    builder.section("gis2d-play-catalogue.layers", Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()), true, items).build()
}

fn map_view_field_group(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiInspectorFieldGroup {
    let lod_items: Vec<UiSelectItem> = lod_select_entries(labels)
        .into_iter()
        .map(|(value, label)| UiSelectItem { value, label,
        })
        .collect();
    let selection: Value = serde_json::from_str(&cfg.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let selected_count = selection.get("positions").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
        + selection.get("routes").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0);
    let mut fields = vec![
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "gis2d-play-inspector.render-mode".into(),
                label: labels.render_mode.into(),
                child: Box::new(UiNode::Select(UiSelectNode {presence: UiPresence::default(),
                    id: "gis2d-play-inspector.render-mode.select".into(),
                    value: cfg.render_mode.clone(),
                    items: vec![
                        UiSelectItem { value: "image".into(), label: labels.render_mode_image.into(),
        },
                        UiSelectItem { value: "vector".into(), label: labels.render_mode_vector.into(),
        },
                        UiSelectItem { value: "combined".into(), label: labels.render_mode_combined.into(),
        },
                    ],
                    placeholder: None,
                    on_change: gis2d_action("setRenderMode", None),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "gis2d-play-inspector.vector-style".into(),
                label: labels.vector_style.into(),
                child: Box::new(UiNode::Select(UiSelectNode {presence: UiPresence::default(),
                    id: "gis2d-play-inspector.vector-style.select".into(),
                    value: cfg.vector_style.clone(),
                    items: vec![
                        UiSelectItem { value: "colored".into(), label: labels.vector_style_colored.into(),
        },
                        UiSelectItem { value: "figureGround".into(), label: labels.vector_style_figure_ground.into(),
        },
                        UiSelectItem { value: "invertedFigure".into(), label: labels.vector_style_inverted_figure.into(),
        },
                    ],
                    placeholder: None,
                    on_change: gis2d_action("setVectorStyle", None),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "gis2d-play-inspector.lod-mode".into(),
                label: labels.lod_mode.into(),
                child: Box::new(UiNode::Select(UiSelectNode {presence: UiPresence::default(),
                    id: "gis2d-play-inspector.lod-mode.select".into(),
                    value: cfg.lod_mode.clone(),
                    items: lod_items,
                    placeholder: None,
                    on_change: gis2d_action("setLodMode", None),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "gis2d-play-inspector.selection-method".into(),
                label: labels.selection_method.into(),
                child: Box::new(UiNode::Select(UiSelectNode {presence: UiPresence::default(),
                    id: "gis2d-play-inspector.selection-method.select".into(),
                    value: cfg.selection_method.clone(),
                    items: vec![
                        UiSelectItem { value: "rectangle".into(), label: labels.selection_method_rectangle.into(),
        },
                        UiSelectItem { value: "lasso".into(), label: labels.selection_method_lasso.into(),
        },
                    ],
                    placeholder: None,
                    on_change: gis2d_action("setSelectionMethod", None),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("gis2d-play-inspector.feature-selection", labels.selected_features, selected_count.to_string()),
    ];
    fields.extend(layer_weight_slider_fields(cfg, labels));
    UiInspectorFieldGroup {
        presence: UiPresence::default(),
        id: "gis2d-play-inspector.map-view".into(),
        label: labels.map_view.into(),
        default_open: Some(true),
        fields,
    }
}

fn build_inspector_tree(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let map_view_group = map_view_field_group(cfg, labels);
    if cfg.selected_ids.is_empty() {
        let visible_count = GIS_MAP_LAYER_IDS
            .iter()
            .filter(|(id, _, _)| layer_visible(cfg, id))
            .count();
        return ui_inspector_groups_to_tree(&[
            map_view_group,
            UiInspectorFieldGroup { presence: UiPresence::default(),
                id: "gis2d-play-inspector.summary".into(),
                label: labels.map_layer.into(),
                default_open: Some(true),
                fields: vec![
                    ui_inspector_readonly_field("gis2d-play-inspector.schema", labels.schema, GIS_MAP_SCHEMA.to_string()),
                    ui_inspector_readonly_field(
                        "gis2d-play-inspector.visible-count",
                        labels.layers_visible,
                        format!("{visible_count}/{}", GIS_MAP_LAYER_IDS.len()),
                    ),
                ],
            },
        ]);
    }
    let layer_id = &cfg.selected_ids[0];
    let label = gis2d_layer_label(layer_id, labels);
    let visible = layer_visible(cfg, layer_id);
    let mixed = ui_inspector_mixed_toggle(&[visible]);
    ui_inspector_groups_to_tree(&[
        map_view_group,
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.layer".into(),
            label: labels.map_layer.into(),
            default_open: Some(true),
            fields: vec![
                ui_inspector_readonly_field("gis2d-play-inspector.id", labels.field_id, layer_id.clone()),
                ui_inspector_readonly_field("gis2d-play-inspector.label", labels.field_label, label.to_string()),
                UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                    id: "gis2d-play-inspector.visible".into(),
                    label: labels.field_visible.into(),
                    child: Box::new(UiNode::Toggle(UiToggleNode {
                        id: "gis2d-play-inspector.visible.toggle".into(),
                        icon_id: "eye".into(),
                        presence: UiPresence::selected(mixed.uniform && mixed.pressed),
                        text: None,
                        on_change: gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": layer_id }))),
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }),
            ],
        },
    ])
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn apply_gis_map_tile_base_url(scene: &mut TiledMapScene) {
    let Ok(base) = std::env::var("SEMIO_ASSET_BASE_URL") else {
        return;
    };
    let base = base.trim_end_matches('/');
    scene.tile_url_template = format!("{base}/osm/{{z}}/{{x}}/{{y}}.png");
    scene.vector_tile_url_template = format!("{base}/vt/{{z}}/{{x}}/{{y}}.pbf");
}

fn render_canvas(document: &gis2d::GisMapDocument, cfg: &Gis2dConfig) -> UiNode {
    let mut scene = TiledMapScene::base(gis_map_descriptor_json(document), cfg.camera_json.clone());
    scene.render_mode = cfg.render_mode.clone();
    scene.vector_style = cfg.vector_style.clone();
    scene.lod_mode = cfg.lod_mode.clone();
    scene.layer_visibility_json = layer_visibility_json(cfg);
    scene.layer_stroke_scale_json = layer_stroke_scale_json(cfg);
    scene.selection_json = cfg.feature_selection_json.clone();
    scene.hover_json = cfg.hover_json.clone();
    scene.selection_method = cfg.selection_method.clone();
    scene.selection_mode = cfg.selection_mode.clone();
    apply_gis_map_tile_base_url(&mut scene);
    build_tiled_map_scene(GIS2D_PLAY_SURFACE, GIS2D_PLAY_APP_ID, scene)
}
//#endregion 🔖️Render

//#region 🔖️Gis2dPlayApp
/// 🗺️ GIS 2D map play app. The document holds positions/routes/regions; everything else (camera,
/// render mode, style, LOD, selection, hover, layer visibility, stroke weights, locale) is
/// `gis2d_engine::Gis2dConfig` — a session-only but real, undoable config artifact (B1: unit struct,
/// no more app-owned `RefCell` runtime).
#[derive(Default)]
pub struct Gis2dPlayApp;

/// 🖱️ On-demand GIS tiled-map context menu from feature hit-test and selection.
fn gis2d_context_menu_items(
    surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>,
    selected_ids: &[String],
    is_de: bool,
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::ContextMenuItemSpec;
    let item = |id: &str, label: &str, icon: &str, action: &str, args: Option<serde_json::Value>, disabled: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
        disabled: disabled.then_some(true),
        ..Default::default()
    };
    let hits = surface.map(|s| s.hits.as_slice()).unwrap_or(&[]);
    let feature = hits.iter().find(|h| h.domain == "feature" || h.domain == "position" || h.domain == "route");
    if let Some(feature) = feature {
        let kind = if feature.domain == "route" { "route" } else { "position" };
        let selected = selected_ids.iter().any(|id| id == &feature.id);
        let mut items = vec![item(
            "tiled-map.ctx.select",
            if is_de { "Auswählen" } else { "Select" },
            "mouse-pointer",
            "setFeatureSelection",
            Some(json!({
                "positions": if kind == "position" { vec![&feature.id] } else { Vec::<&String>::new() },
                "routes": if kind == "route" { vec![&feature.id] } else { Vec::<&String>::new() },
                "mode": "default",
            })),
            false,
        )];
        if selected {
            items.push(item(
                "tiled-map.ctx.deselect",
                if is_de { "Abwählen" } else { "Deselect" },
                "square-dashed",
                "deselect",
                Some(json!({ "featureId": feature.id, "featureKind": kind })),
                false,
            ));
        }
        items.push(item(
            "tiled-map.ctx.focus",
            if is_de { "Fokussieren / Zoomen" } else { "Focus / Zoom" },
            "crosshair",
            "focusFeature",
            Some(json!({ "featureId": feature.id, "featureKind": kind })),
            false,
        ));
        if kind == "position" {
            items.push(item(
                "tiled-map.ctx.source",
                if is_de { "Quelle öffnen" } else { "Open source" },
                "external-link",
                "openSource",
                Some(json!({ "featureId": feature.id })),
                false,
            ));
        }
        return items;
    }
    vec![
        item("tiled-map.ctx.select-all", if is_de { "Alles auswählen" } else { "Select All" }, "select-all", "selectAll", None, false),
        item("tiled-map.ctx.clear", if is_de { "Auswahl aufheben" } else { "Clear selection" }, "square-dashed", "clearSelection", None, selected_ids.is_empty()),
        item("tiled-map.ctx.fit-world", if is_de { "Welt einpassen" } else { "Fit world" }, "maximize-2", "fitWorld", None, false),
    ]
}

impl DocumentApp for Gis2dPlayApp {
    type Projection = gis2d::GisMapDocument;
    type Operation = GisMapOperation;
    type Config = Gis2dConfig;
    type ConfigOperation = Gis2dConfigOperation;
    type Command = Gis2dCommand;

    fn app_id(&self) -> &str {
        GIS2D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        GIS_MAP_SCHEMA
    }

    fn initial_projection(&self) -> gis2d::GisMapDocument {
        default_document()
    }

    /// 🔌️ `features:in`/`map:out` (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) plus the
    /// implicit document ports.
    fn io(&self) -> Option<AppIo> {
        Some(gis2d_io())
    }

    fn whole_document_operation(&self, projection: gis2d::GisMapDocument) -> Option<GisMapOperation> {
        Some(GisMapOperation::SetDocument { document: projection })
    }

    /// 🎞️ `map:out` (see `gis2d_engine::gis2d_map_media`) plus the inherited `document:out` default
    /// (the pack of `doc.projection`, replicated inline — overriding `export_media` shadows the
    /// trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, gis2d::GisMapDocument>) -> Result<Media, MediaError> {
        match port {
            "map:out" => Ok(gis2d_map_media(doc.projection)),
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media {
                    media_type,
                    payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `features:in` normalizes an incoming `{positions,routes,regions}` descriptor into granular
    /// add/patch/remove operations against every collection (a generic vector-features sink — not
    /// pinned to `2d.map`, so a `draw`/another `gis2d`'s producer both work) plus the inherited
    /// `document:in` default (replicated inline for the same reason as `export_media`).
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, gis2d::GisMapDocument>) -> Result<Emit<GisMapOperation, Gis2dConfigOperation>, MediaError> {
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
                let projection = <gis2d::GisMapDocument as store::DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match self.whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `Gis2dCommand` variant back to the action id it was declared under in
    /// `create_gis2d_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check. `SetLocale` is not palette-declared (host/test infra
    /// dispatches it directly, mirroring `shooting_ui::ShootingPlayApp::command_id`).
    fn command_id(&self, command: &Gis2dCommand) -> &str {
        match command {
            Gis2dCommand::SetActiveExample { .. } => "setActiveExample",
            Gis2dCommand::PatchPositions { .. } => "patchPositions",
            Gis2dCommand::PatchRoutes { .. } => "patchRoutes",
            Gis2dCommand::PatchRoute { .. } => "patchRoute",
            Gis2dCommand::SetSelection { .. } => "setSelection",
            Gis2dCommand::ToggleLayerVisibility { .. } => "toggleLayerVisibility",
            Gis2dCommand::FitWorld => "fitWorld",
            Gis2dCommand::SetCamera { .. } => "setCamera",
            Gis2dCommand::SetRenderMode { .. } => "setRenderMode",
            Gis2dCommand::SetVectorStyle { .. } => "setVectorStyle",
            Gis2dCommand::SetLodMode { .. } => "setLodMode",
            Gis2dCommand::SetFeatureSelection { .. } => "setFeatureSelection",
            Gis2dCommand::SetHover { .. } => "setHover",
            Gis2dCommand::SetSelectionMethod { .. } => "setSelectionMethod",
            Gis2dCommand::SetSelectionMode { .. } => "setSelectionMode",
            Gis2dCommand::ClearSelection => "clearSelection",
            Gis2dCommand::SelectAll => "selectAll",
            Gis2dCommand::Deselect { .. } => "deselect",
            Gis2dCommand::FocusFeature { .. } => "focusFeature",
            Gis2dCommand::SetLayerStrokeScale { .. } => "setLayerStrokeScale",
            Gis2dCommand::SetLocale { .. } => "setLocale",
            Gis2dCommand::OpenSource { .. } => "openSource",
        }
    }

    fn handle(
        &self,
        command: &Gis2dCommand,
        doc: &DocumentView<'_, gis2d::GisMapDocument>,
        cfg: &ConfigView<'_, Gis2dConfig>,
    ) -> Emit<GisMapOperation, Gis2dConfigOperation> {
        let document = doc.projection;
        let config = cfg.projection;
        match command {
            // ✏️ Operation actions — flow through the document store with true inverses.
            Gis2dCommand::SetActiveExample { example_id } => {
                let next = if example_id.is_empty() { gis2d::GisMapDocument::default() } else { default_document() };
                let mut config_operations = vec![Gis2dConfigOperation::SetSelection { ids: Vec::new() }];
                if !example_id.is_empty() {
                    let mut host = map_host_from(&next, config);
                    host.fit_world_camera();
                    config_operations.push(Gis2dConfigOperation::SetCamera { camera_json: host.camera_json() });
                }
                Emit { document_operations: vec![GisMapOperation::SetDocument { document: next }], config_operations, ..Default::default() }
            }
            Gis2dCommand::PatchPositions { positions_json } => {
                let Ok(positions) = serde_json::from_str::<Value>(positions_json) else {
                    return Emit::default();
                };
                let next = gis_map_document_from_descriptor_json(&json!({ "positions": positions }).to_string()).positions;
                Emit::operations(positions_operations(&document.positions, &next))
            }
            Gis2dCommand::PatchRoutes { route_ids, field, value } => patch_routes_operations(document, route_ids, field, value),
            Gis2dCommand::PatchRoute { route_id, field, value } => patch_routes_operations(document, std::slice::from_ref(route_id), field, value),
            // 👁️ View/config actions — mutate the config, emit no document operations.
            Gis2dCommand::SetSelection { ids } => Emit::config(vec![Gis2dConfigOperation::SetSelection { ids: ids.clone() }]),
            Gis2dCommand::ToggleLayerVisibility { layer_id } => {
                let visible = !layer_visible(config, layer_id);
                Emit::config(vec![Gis2dConfigOperation::SetLayerVisibility { layer_id: layer_id.clone(), visible }])
            }
            Gis2dCommand::FitWorld => {
                let mut host = map_host_from(document, config);
                host.fit_world_camera();
                Emit::config(vec![Gis2dConfigOperation::SetCamera { camera_json: host.camera_json() }])
            }
            Gis2dCommand::SetCamera { camera_json } => Emit::config(vec![Gis2dConfigOperation::SetCamera { camera_json: camera_json.clone() }]),
            Gis2dCommand::SetRenderMode { value } => Emit::config(vec![Gis2dConfigOperation::SetRenderMode { value: value.clone() }]),
            Gis2dCommand::SetVectorStyle { value } => Emit::config(vec![Gis2dConfigOperation::SetVectorStyle { value: value.clone() }]),
            Gis2dCommand::SetLodMode { value } => Emit::config(vec![Gis2dConfigOperation::SetLodMode { value: value.clone() }]),
            Gis2dCommand::SetFeatureSelection { positions, routes, mode } => {
                let selection = merge_feature_selection(&config.feature_selection_json, positions.clone(), routes.clone(), mode);
                let mut host = map_host_from(document, config);
                if host.set_selection_json(&selection.to_string()).is_ok() {
                    Emit::config(vec![Gis2dConfigOperation::SetFeatureSelection { value_json: selection.to_string() }])
                } else {
                    Emit::default()
                }
            }
            Gis2dCommand::SetHover { hover_json } => Emit::config(vec![Gis2dConfigOperation::SetHover { value_json: hover_json.clone() }]),
            Gis2dCommand::SetSelectionMethod { value } => Emit::config(vec![Gis2dConfigOperation::SetSelectionMethod { value: value.clone() }]),
            Gis2dCommand::SetSelectionMode { value } => Emit::config(vec![Gis2dConfigOperation::SetSelectionMode { value: value.clone() }]),
            Gis2dCommand::ClearSelection => Emit::config(vec![Gis2dConfigOperation::SetFeatureSelection { value_json: Gis2dConfig::default().feature_selection_json }]),
            Gis2dCommand::SelectAll => {
                let host = map_host_from(document, config);
                let selection = json!({
                    "positions": host.positions.keys().cloned().collect::<Vec<_>>(),
                    "routes": host.routes.keys().cloned().collect::<Vec<_>>(),
                });
                Emit::config(vec![Gis2dConfigOperation::SetFeatureSelection { value_json: selection.to_string() }])
            }
            Gis2dCommand::Deselect { feature_id, feature_kind } => {
                let mut selection: Value = serde_json::from_str(&config.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
                let bucket = if feature_kind == "position" { "positions" } else { "routes" };
                if let Some(rows) = selection.get_mut(bucket).and_then(|value| value.as_array_mut()) {
                    rows.retain(|row| row.as_str() != Some(feature_id.as_str()));
                }
                Emit::config(vec![Gis2dConfigOperation::SetFeatureSelection { value_json: selection.to_string() }])
            }
            Gis2dCommand::FocusFeature { feature_id, feature_kind } => {
                let mut host = map_host_from(document, config);
                if host.focus_feature(feature_kind, feature_id) {
                    Emit::config(vec![Gis2dConfigOperation::SetCamera { camera_json: host.camera_json() }])
                } else {
                    Emit::default()
                }
            }
            Gis2dCommand::SetLayerStrokeScale { layer_id, value } => {
                Emit::config(vec![Gis2dConfigOperation::SetLayerStrokeScale { layer_id: layer_id.clone(), value: clamp_map_layer_weight(*value) }])
            }
            Gis2dCommand::SetLocale { value } => Emit::config(vec![Gis2dConfigOperation::SetLocale { value: value.clone() }]),
            // 🌐️ Shell action — opens the picked feature's source URL through the host.
            Gis2dCommand::OpenSource { feature_id } => {
                let host = map_host_from(document, config);
                match host.positions.get(feature_id).and_then(|row| row.source_url.clone()) {
                    Some(url) => Emit::effect(HostEffect::OpenExternalUrl { url }),
                    None => Emit::default(),
                }
            }
        }
    }

    /// 🧮️ Empty — gis2d's `Config` is session view state (camera/selection/layer visibility/…), not a
    /// user-facing settings record; `ConfigSpec::empty()` (the trait default) is correct as-is.
    fn config_spec(&self) -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, gis2d::GisMapDocument>, cfg: &ConfigView<'_, Gis2dConfig>) -> UiNode {
        let document = doc.projection;
        let labels = resolve_labels::<Gis2dPlayLabels>(cfg.projection);
        match body_key {
            GIS2D_PLAY_BODY_COMPOSITE => render_canvas(document, cfg.projection),
            GIS2D_PLAY_BODY_DOCUMENT => build_document_tree(cfg.projection, labels),
            GIS2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(cfg.projection, labels),
            GIS2D_PLAY_BODY_INSPECTION => build_inspector_tree(cfg.projection, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(
        &self,
        _doc: &DocumentView<'_, gis2d::GisMapDocument>,
        cfg: &ConfigView<'_, Gis2dConfig>,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = resolve_labels::<Gis2dPlayLabels>(cfg.projection);
        HashMap::from([(GIS2D_PLAY_WINDOW_MAIN.into(), gis2d_window_measures(cfg.projection, labels))])
    }

    fn app_labels(&self, cfg: &ConfigView<'_, Gis2dConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<Gis2dPlayLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(GIS2D_PLAY_WINDOW_MAIN, labels.window_map)
            .mode_label("edit", labels.mode_edit)
            .action_labels(gis2d_action_labels(is_de))
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, gis2d::GisMapDocument>,
        cfg: &ConfigView<'_, Gis2dConfig>,
        _registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let is_de = is_de_locale(cfg.projection);
        gis2d_context_menu_items(request.surface.as_ref(), &cfg.projection.selected_ids, is_de)
    }
}

/// 🌉️ Shared `patchRoutes`/`patchRoute` implementation — a single route id (`patchRoute`) is just a
/// one-element slice of the many-route form (`patchRoutes`).
fn patch_routes_operations(document: &gis2d::GisMapDocument, route_ids: &[String], field: &str, value: &str) -> Emit<GisMapOperation, Gis2dConfigOperation> {
    if route_ids.is_empty() {
        return Emit::default();
    }
    let dsl_value = dsl::to_dsl_value(&json!(value)).unwrap_or(DslValue::Null);
    let operations: Vec<GisMapOperation> = document
        .routes
        .iter()
        .filter(|route| route_ids.iter().any(|id| id == &route.id))
        .filter_map(|route| {
            let mut data = route.data.clone();
            let DslValue::Object(entries) = &mut data else {
                return None;
            };
            if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == field) {
                *slot = dsl_value.clone();
            } else {
                entries.push((field.to_string(), dsl_value.clone()));
            }
            Some(GisMapOperation::Routes(CollectionOperation::Patch { id: route.id.clone(), patch: MapFeaturePatch { data: Some(data) } }))
        })
        .collect();
    Emit::operations(operations)
}
//#endregion 🔖️Gis2dPlayApp

//#region 🔖️Manifest
/// 🔽️ The static LOD-mode choices for the palette arg schema: the automatic mode plus each LOD scale
/// tier from the map descriptor, labelled in the app's base locale (localization is applied by overlay).
fn lod_arg_options() -> Vec<ActionArgOption> {
    std::iter::once(ActionArgOption::new(GIS_MAP_LOD_MODE_AUTOMATIC, Gis2dPlayLabels::EN.lod_automatic))
        .chain(
            serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json())
                .unwrap_or_default()
                .into_iter()
                .filter_map(|lod| {
                    let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
                    let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
                    Some(ActionArgOption::new(id, name))
                }),
        )
        .collect()
}

pub fn create_gis2d_app() -> App {
    App::from_builder(
        App::builder(GIS2D_PLAY_APP_ID, "GIS 2D").document(["semio", "gis", "2d"])
            .artifact_kind(ArtifactKindSpec {
                id: "2d.map".into(),
                name: "2D Map".into(),
                source_format: "gis.map".into(),
                component_kind: "gismap".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                schema: "gis.map".into(),
                export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
            })
            // 🔌️ Typed workflow ports (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) — same
            // constructor fns `gis2d_io()` embeds, so `AppIo.all_ports()` and these declarations can
            // never drift apart. `map:out`'s `2d.map` kind is declared above; `features:in` pins no kind.
            .media_input(gis2d_features_in_port())
            .media_output(gis2d_map_out_port())
            .icon_id("gis2d")
            .mode("edit", "Edit", "square-pen")
            .default_mode_id("edit")
            .window_kind(GIS2D_PLAY_WINDOW_MAIN, "Map", GIS2D_PLAY_BODY_COMPOSITE, SurfaceKind::TiledMap, "globe")
            .default_layout(create_default_layout(
                &[GIS2D_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Map".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                GIS2D_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                GIS2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                GIS2D_PLAY_BODY_INSPECTION,
            )
            // ✏️ Operation actions — flow through the document store with true inverses. `setActiveExample`
            // replaces document content via `SetDocument` operations, so it is an Operation, not a View action.
            .operation("setActiveExample", "Set Active Example")
            .operation("patchPositions", "Patch Positions")
            .operation("patchRoutes", "Patch Routes")
            .operation("patchRoute", "Patch Route")
            // 👁️ View actions — mutate ephemeral config state (selection, camera, render config,
            // hover, layer visibility, stroke weights), never the document.
            .view_action("setSelection", "Set Selection")
            .view_action("toggleLayerVisibility", "Toggle Layer Visibility")
            .view_action("fitWorld", "Fit World")
            .view_action("setCamera", "Set Camera")
            .view_action("setRenderMode", "Set Render Mode")
            .view_action("setVectorStyle", "Set Vector Style")
            .view_action("setLodMode", "Set LOD Mode")
            .view_action("setFeatureSelection", "Set Feature Selection")
            .view_action("setHover", "Set Hover")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("setSelectionMode", "Set Selection Mode")
            .view_action("clearSelection", "Clear Selection")
            .view_action("selectAll", "Select All")
            .view_action("deselect", "Deselect")
            .view_action("focusFeature", "Focus Feature")
            .view_action("setLayerStrokeScale", "Set Layer Stroke Scale")
            // 🌐️ Shell action — opens the picked feature's source URL through the host.
            .shell_action("openSource", "Open Source")
            // 📝️ Argument schemas for the discrete-choice actions so the command palette can stage them
            // and the registry validates the vocabulary. The arg id matches the key each handler reads.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new("reuse-map", "Reuse Map"),
                ]).default_value("reuse-map"),
            ])
            .action_args("setRenderMode", vec![
                ActionArgDef::select("value", "Render Mode", vec![
                    ActionArgOption::new("image", "Image"),
                    ActionArgOption::new("vector", "Vector"),
                    ActionArgOption::new("combined", "Combined"),
                ]).default_value("combined"),
            ])
            .action_args("setVectorStyle", vec![
                ActionArgDef::select("value", "Vector Style", vec![
                    ActionArgOption::new("colored", "Colored"),
                    ActionArgOption::new("figureGround", "Figure Ground"),
                    ActionArgOption::new("invertedFigure", "Inverted Figure"),
                ]).default_value("colored"),
            ])
            .action_args("setLodMode", vec![
                ActionArgDef::select("value", "LOD Mode", lod_arg_options()).default_value(GIS_MAP_LOD_MODE_AUTOMATIC),
            ])
            .action_args("setSelectionMethod", vec![
                ActionArgDef::select("value", "Selection Method", vec![
                    ActionArgOption::new("rectangle", "Rectangle"),
                    ActionArgOption::new("lasso", "Lasso"),
                ]).default_value("rectangle"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis2dPlayApp::default().config_spec())
            .io(gis2d_io()),
    )
    .example("reuse-map", "Reuse Map", serde_json::to_string(&default_document()).unwrap(), "file-text")
    .workflow("gis2d", "GIS 2D", "map")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
/// 🗂️ Raw wasm-bindgen JS binding surface for `GisMapDocument`'s VCS store — independent of the
/// `App`/`DocumentApp` plugin-registry path above (`create_gis2d_app`/`Gis2dPlayApp`), this exposes the
/// document store directly for callers that talk to the compiled wasm module without going through the
/// host's app registry.
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use gis2d_op::{GisMapEnvelope, GisMapStore};
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct GisMapDocumentVcs {
        store: RefCell<GisMapStore>,
    }

    #[wasm_bindgen]
    impl GisMapDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<GisMapDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: GisMapEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    GisMapStore::new(envelope)
                }
                None => GisMapStore::new(store::create_document_envelope(
                    GIS_MAP_SCHEMA,
                    "gis",
                    gis2d_engine::empty_gis_map_projection(),
                    None,
                )),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_text(command_text)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_binary(command_bytes)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, ActionKind, PluginApp, ViewState, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<Gis2dPlayApp> {
        testkit::new_app::<Gis2dPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View/Shell-emits-operations rejection) runs.
    fn new_app_with_registry() -> VcsDocumentApp<Gis2dPlayApp> {
        testkit::new_app_with_registry::<Gis2dPlayApp>(create_gis2d_app)
    }

    fn render(app: &mut VcsDocumentApp<Gis2dPlayApp>, body_key: &str, view_state: &ViewState) -> String {
        serde_json::to_string(&app.render(body_key, None, view_state).expect("render")).unwrap()
    }

    #[test]
    fn renders_gis_map_scene() {
        let mut app = new_app();
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE, &ViewState::default()).contains("tiled-map"));
    }

    #[test]
    fn render_canvas_uses_absolute_tile_urls_when_env_set() {
        unsafe { std::env::set_var("SEMIO_ASSET_BASE_URL", "http://127.0.0.1:6141") };
        let mut app = new_app();
        let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE, &ViewState::default());
        assert!(json.contains("http://127.0.0.1:6141/osm/{z}/{x}/{y}.png"));
        assert!(json.contains("http://127.0.0.1:6141/vt/{z}/{x}/{y}.pbf"));
        unsafe { std::env::remove_var("SEMIO_ASSET_BASE_URL") };
    }

    #[test]
    fn document_lists_map_layers() {
        let mut app = new_app();
        assert!(render(&mut app, GIS2D_PLAY_BODY_DOCUMENT, &ViewState::default()).contains("gis2d-play-document.layer.raster"));
    }

    #[test]
    fn catalogue_lists_layer_toggles() {
        let mut app = new_app();
        assert!(render(&mut app, GIS2D_PLAY_BODY_CATALOGUE, &ViewState::default()).contains("gis2d-play-catalogue.layer.water"));
    }

    #[test]
    fn gis2d_labels_resolve_native_by_default() {
        let mut app = new_app();
        let json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION, &ViewState::default());
        assert!(json.contains("\"Map View\""));
        assert!(json.contains("\"Render Mode\""));
        assert!(json.contains("\"Selected Features\""));
        assert!(json.contains("\"Map Layer\""));
        assert!(!json.contains("Kartenansicht"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// `ViewState`-pushed locale (mirrors `shooting_ui`'s identical test rewrite).
    #[test]
    fn gis2d_labels_translate_inspector_and_layers_in_german() {
        let mut app = new_app();
        app.dispatch_typed(Gis2dCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let inspector_json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION, &ViewState::default());
        assert!(inspector_json.contains("Kartenansicht"));
        assert!(inspector_json.contains("Darstellungsmodus"));
        assert!(inspector_json.contains("Ausgewählte Objekte"));
        assert!(inspector_json.contains("Kartenebene"));
        assert!(!inspector_json.contains("\"Map View\""));

        let document_json = render(&mut app, GIS2D_PLAY_BODY_DOCUMENT, &ViewState::default());
        assert!(document_json.contains("Wasser"));
        assert!(!document_json.contains("\"Water\""));

        let window = app.window_measures();
        let window_json = serde_json::to_string(window.get(GIS2D_PLAY_WINDOW_MAIN).unwrap()).unwrap();
        assert!(window_json.contains("Ebenen"));
        assert!(window_json.contains("Ebenengewichte"));
    }

    #[test]
    fn set_selection_is_view_state_and_emits_no_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(Gis2dCommand::SetSelection { ids: vec!["roads".into()] }, &testkit::meta("local")).expect("setSelection");
        assert!(result.operations.is_empty(), "selection must not produce document operations");
    }

    #[test]
    fn set_render_mode_is_view_state() {
        let mut app = new_app();
        let result = app.dispatch_typed(Gis2dCommand::SetRenderMode { value: "vector".into() }, &testkit::meta("local")).expect("setRenderMode");
        assert!(result.operations.is_empty());
        assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE, &ViewState::default()).contains("\"renderMode\":\"vector\""));
    }

    #[test]
    fn set_active_example_empty_then_reuse_round_trips_document() {
        let mut app = new_app();
        assert!(!app.projection().expect("projection").positions.is_empty());
        app.dispatch_typed(Gis2dCommand::SetActiveExample { example_id: "".into() }, &testkit::meta("local")).expect("empty");
        assert!(app.projection().expect("projection").positions.is_empty());
        app.dispatch_typed(Gis2dCommand::SetActiveExample { example_id: "reuse-map".into() }, &testkit::meta("local")).expect("reuse");
        assert!(!app.projection().expect("projection").positions.is_empty());
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert!(app.projection().expect("projection").positions.is_empty(), "undo returns to the empty document");
    }

    /// 🧬️ `setActiveExample` replaces document content with `SetDocument` operations, so it MUST be declared as
    /// an Operation. Under the real registry the View/Shell → emits-operations guard rejects a mis-declaration;
    /// this proves the corrected declaration lets the document-replacing edit flow through without erroring.
    #[test]
    fn set_active_example_is_operation_under_registry_kind_discipline() {
        let definition = create_gis2d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Operation), "loading an example emits SetDocument operations, so it is an Operation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");

        let mut app = new_app_with_registry();
        let result = app
            .dispatch_typed(Gis2dCommand::SetActiveExample { example_id: "".into() }, &testkit::meta("local"))
            .expect("operation emits operations without tripping the kind-discipline guard");
        assert_eq!(result.operations.len(), 1, "loading an example is one document-replacing edit");
        assert!(app.projection().expect("projection").positions.is_empty(), "the empty example clears every position feature");
    }

    /// 👁️ A representative View action mutates only config state, so under the real registry it
    /// emits no operations and never trips the View → emits-operations guard.
    #[test]
    fn view_actions_emit_no_ops_under_registry_kind_discipline() {
        let mut app = new_app_with_registry();
        let render_mode = app.dispatch_typed(Gis2dCommand::SetRenderMode { value: "vector".into() }, &testkit::meta("local")).expect("setRenderMode");
        assert!(render_mode.operations.is_empty(), "render mode is ephemeral config state");
        let fit = app.dispatch_typed(Gis2dCommand::FitWorld, &testkit::meta("local")).expect("fitWorld");
        assert!(fit.operations.is_empty(), "framing the world only moves the config camera");
    }

    #[test]
    fn patch_routes_emits_route_patch_ops_and_updates_document() {
        let mut app = new_app();
        let route_id = "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0";
        let result = app
            .dispatch_typed(Gis2dCommand::PatchRoute { route_id: route_id.into(), field: "label".into(), value: "Renamed Route".into() }, &testkit::meta("local"))
            .expect("patchRoute");
        assert_eq!(result.operations.len(), 1, "one matching route → one patch operation");
        let document = app.projection().expect("projection");
        let route = document.routes.iter().find(|route| route.id == route_id).expect("route");
        assert_eq!(route.data.get("label").and_then(|value| value.as_str()), Some("Renamed Route"));
    }

    /// 🤝️ Definitional merge proof: two instances on one backbone patch DIFFERENT routes; after
    /// exchanging operations both converge and keep both edits — impossible under whole-map LWW snapshots.
    #[test]
    fn two_instances_converge_on_disjoint_route_edits() {
        let route_a = "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0";
        let route_b = "bg_stahl_mehrere_lycee_profiles_canopy:bw_lycee_block_3000:0";
        let command_a = Gis2dCommand::PatchRoute { route_id: route_a.into(), field: "label".into(), value: "A".into() };
        let command_b = Gis2dCommand::PatchRoute { route_id: route_b.into(), field: "label".into(), value: "B".into() };
        let label = |document: &gis2d::GisMapDocument, id: &str| document.routes.iter().find(|route| route.id == id).and_then(|route| route.data.get("label").and_then(|value| value.as_str().map(str::to_string)));
        testkit::assert_two_instances_converge::<Gis2dPlayApp, _>(
            "mem://gis2d-convergence",
            command_a,
            command_b,
            |app| {
                let document = app.projection().expect("projection");
                (label(&document, route_a), label(&document, route_b))
            },
        );
    }

    #[test]
    fn export_media_map_out_produces_a_2d_map_structured_payload() {
        let app = new_app();
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
        let app = new_app();
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
}
//#endregion 🧪️Tests
