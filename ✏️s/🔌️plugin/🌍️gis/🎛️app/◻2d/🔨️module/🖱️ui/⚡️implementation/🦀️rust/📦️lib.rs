//! 🖥️ GIS 2D app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! migration — `Gis2dPlayApp` is a unit struct; every former `Gis2dPlayRuntime` field (selection,
//! camera, render/vector/LOD mode, feature selection/hover, layer visibility/stroke-weight, …) now
//! lives in `gis2d_engine::Gis2dConfig`, written via `gis2d_op::Gis2dConfigOperation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `gis2d_protocol::Gis2dCommand` channel via `DocumentApp::handle`.

use dsl::DslValue;
use framework_surface_tiled_map::{clamp_map_layer_weight, gis_map_layer_weight_slider_ids_json, gis_map_lod_scale_json, MapHost, GIS_MAP_LOD_MODE_AUTOMATIC};
use gis2d::{MapFeature, MapFeaturePatch, GIS_MAP_SCHEMA};
use gis2d_engine::{default_document, gis2d_features_in_port, gis2d_io, gis2d_map_media, gis2d_map_out_port, gis_map_descriptor_json, gis_map_document_from_descriptor_json, Gis2dConfig};
use gis2d_op::{Gis2dConfigOperation, GisMapOperation};
use gis2d_protocol::Gis2dCommand;
use protocol::CollectionOperation;
use semio_framework_plugin::kernel::HostEffect;
use semio_framework_plugin::{
        app_labels, build_tiled_map_scene, create_default_layout, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor,
    ActionKind, App, AppIo, AppLabels, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, Locale, LocalizedLabel, MeasureSelectItem, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, OsMediaCapability,
    OsMediaFormat, PanelGroup, PanelTreeBuilder, SurfaceKind, Terminology, TiledMapScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemNode, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
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
//#endregion 🔖️Constants

//#region 🔖️Locale

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
    let mut map: HashMap<String, f64> = GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), 1.0)).collect();
    for (id, weight) in &cfg.layer_stroke_scale {
        map.insert(id.clone(), clamp_map_layer_weight(*weight));
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

fn merge_feature_selection(current_json: &str, positions: Vec<String>, routes: Vec<String>, mode: &str) -> Value {
    let current: Value = serde_json::from_str(current_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let current_positions: Vec<String> = current.get("positions").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    let current_routes: Vec<String> = current.get("routes").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
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
    semio_framework_plugin::ActionFactory::new(GIS2D_PLAY_APP_ID).action(action, args)
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
            Some(prev) if prev.data != feature.data => operations.push(wrap(CollectionOperation::Patch { id: feature.id.clone(), patch: MapFeaturePatch { data: Some(feature.data.clone()) } })),
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
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: format!("gis2d-play-inspector.weight.{layer_id}"),
                label: Label::data(format!("{label} {}", labels.weight_suffix.as_str())),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: format!("gis2d-play-inspector.weight.{layer_id}.slider"),
                    value,
                    min: 0.25,
                    max: 3.0,
                    step: 0.05,
                    on_change: gis2d_action("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),
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
        .chain(serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
            let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
            let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            Some((id, name))
        }))
        .collect()
}

fn layer_weight_entries(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> Vec<(String, String, f64)> {
    let ids: Vec<String> = serde_json::from_str(&gis_map_layer_weight_slider_ids_json(&cfg.lod_mode, &cfg.render_mode)).unwrap_or_default();
    ids.into_iter()
        .map(|layer_id| {
            let value = cfg.layer_stroke_scale.get(&layer_id).copied().map(clamp_map_layer_weight).unwrap_or(1.0);
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
            label: Some(format!("{label} {}", labels.weight_suffix.as_str())),
            value,
            min: 0.25,
            max: 3.0,
            step: Some(0.05),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: gis2d_action("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),

            waiting: None,
        })
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
            items: lod_select_entries(labels).into_iter().map(|(value, label)| MeasureSelectItem { id: value.clone(), value, label }).collect(),
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
        window_map: native_en "Map", native_de "Karte", reuse_en "Map", reuse_de "Karte";
        mode_edit: native_en "Edit", native_de "Bearbeiten", reuse_en "Edit", reuse_de "Bearbeiten";
        layer_raster: native_en "Raster", native_de "Raster", reuse_en "Raster", reuse_de "Raster";
        layer_water: native_en "Water", native_de "Wasser", reuse_en "Water", reuse_de "Wasser";
        layer_land: native_en "Land", native_de "Land", reuse_en "Land", reuse_de "Land";
        layer_roads: native_en "Roads", native_de "Straßen", reuse_en "Roads", reuse_de "Straßen";
        layer_buildings: native_en "Buildings", native_de "Gebäude", reuse_en "Buildings", reuse_de "Gebäude";
        layer_borders: native_en "Borders", native_de "Grenzen", reuse_en "Borders", reuse_de "Grenzen";
        layer_map_labels: native_en "Labels", native_de "Beschriftungen", reuse_en "Labels", reuse_de "Beschriftungen";
        layer_positions: native_en "Positions", native_de "Positionen", reuse_en "Positions", reuse_de "Positionen";
        layer_position_labels: native_en "Position Labels", native_de "Positionsbeschriftungen", reuse_en "Position Labels", reuse_de "Positionsbeschriftungen";
        layer_routes: native_en "Routes", native_de "Routen", reuse_en "Routes", reuse_de "Routen";
        layer_regions: native_en "Regions", native_de "Regionen", reuse_en "Regions", reuse_de "Regionen";
        map_view: native_en "Map View", native_de "Kartenansicht", reuse_en "Map View", reuse_de "Kartenansicht";
        render_mode: native_en "Render Mode", native_de "Darstellungsmodus", reuse_en "Render Mode", reuse_de "Darstellungsmodus";
        render_mode_image: native_en "Image", native_de "Bild", reuse_en "Image", reuse_de "Bild";
        render_mode_vector: native_en "Vector", native_de "Vektor", reuse_en "Vector", reuse_de "Vektor";
        render_mode_combined: native_en "Combined", native_de "Kombiniert", reuse_en "Combined", reuse_de "Kombiniert";
        vector_style: native_en "Vector Style", native_de "Vektorstil", reuse_en "Vector Style", reuse_de "Vektorstil";
        vector_style_colored: native_en "Colored", native_de "Farbig", reuse_en "Colored", reuse_de "Farbig";
        vector_style_figure_ground: native_en "Figure Ground", native_de "Figur-Grund", reuse_en "Figure Ground", reuse_de "Figur-Grund";
        vector_style_inverted_figure: native_en "Inverted Figure", native_de "Invertierte Figur", reuse_en "Inverted Figure", reuse_de "Invertierte Figur";
        lod_mode: native_en "LOD Mode", native_de "LOD-Modus", reuse_en "LOD Mode", reuse_de "LOD-Modus";
        lod_automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        selection_method: native_en "Selection Method", native_de "Auswahlmethode", reuse_en "Selection Method", reuse_de "Auswahlmethode";
        selection_method_rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        selection_method_lasso: native_en "Lasso", native_de "Lasso", reuse_en "Lasso", reuse_de "Lasso";
        layers_group: native_en "Layers", native_de "Ebenen", reuse_en "Layers", reuse_de "Ebenen";
        layer_weights_group: native_en "Layer Weights", native_de "Ebenengewichte", reuse_en "Layer Weights", reuse_de "Ebenengewichte";
        weight_suffix: native_en "weight", native_de "Gewicht", reuse_en "weight", reuse_de "Gewicht";
        selected_features: native_en "Selected Features", native_de "Ausgewählte Objekte", reuse_en "Selected Features", reuse_de "Ausgewählte Objekte";
        map_layer: native_en "Map Layer", native_de "Kartenebene", reuse_en "Map Layer", reuse_de "Kartenebene";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        layers_visible: native_en "Layers visible", native_de "Sichtbare Ebenen", reuse_en "Layers visible", reuse_de "Sichtbare Ebenen";
        field_id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        field_label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        field_visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
    }
}

/// 🗣️ Resolves a standard map layer's display label from its stable id; unknown ids fall back to the catalog's native English text.
fn gis2d_layer_label(layer_id: &str, labels: &Gis2dPlayLabels) -> &'static str {
    match layer_id {
        "raster" => labels.layer_raster.as_str(),
        "water" => labels.layer_water.as_str(),
        "land" => labels.layer_land.as_str(),
        "roads" => labels.layer_roads.as_str(),
        "buildings" => labels.layer_buildings.as_str(),
        "borders" => labels.layer_borders.as_str(),
        "labels" => labels.layer_map_labels.as_str(),
        "positions" => labels.layer_positions.as_str(),
        "positionLabels" => labels.layer_position_labels.as_str(),
        "routes" => labels.layer_routes.as_str(),
        "regions" => labels.layer_regions.as_str(),
        // 🗣️ unreachable in practice — the arms above already cover every id in GIS_MAP_LAYER_IDS.
        _ => "",
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
/// 🌳️ A layer tree item — `tree_item_with_action` plus the icon that identifies each map layer, since
/// the SDK's `PanelKit` family has no icon-carrying constructor.
fn gis2d_layer_tree_item(id: String, label: impl Into<Label>, description: Option<String>, icon_id: &str, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon_id.into()), menu: None, ..tree_item_with_action(id, label, description, action) }
}

fn build_document_tree(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("gis2d-play-document");
    let layer_items: Vec<UiTreeItemNode> =
        GIS_MAP_LAYER_IDS.iter().map(|(id, _, icon)| gis2d_layer_tree_item(builder.item_id("layer", id), Label::data(gis2d_layer_label(id, labels)), Some((*id).into()), icon, gis2d_action("setSelection", Some(json!({ "ids": [id] }))))).collect();
    builder
        .section("gis2d-play-document.layers", Some(Label::data(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)), true, layer_items)
        .selected(cfg.selected_ids.iter().map(|id| format!("gis2d-play-document.layer.{id}")).collect())
        .selection_change(gis2d_action("setSelection", None))
        .build()
}

fn build_catalogue_tree(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let _ = cfg;
    let builder = PanelTreeBuilder::new("gis2d-play-catalogue");
    let items: Vec<UiTreeItemNode> =
        GIS_MAP_LAYER_IDS.iter().map(|(id, _, icon)| gis2d_layer_tree_item(builder.item_id("layer", id), Label::data(gis2d_layer_label(id, labels)), None, icon, gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": id }))))).collect();
    builder.section("gis2d-play-catalogue.layers", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, items).build()
}

fn map_view_field_group(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiInspectorFieldGroup {
    let lod_items: Vec<UiSelectItem> = lod_select_entries(labels).into_iter().map(|(value, label)| UiSelectItem { value, label: Label::data(label) }).collect();
    let selection: Value = serde_json::from_str(&cfg.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let selected_count = selection.get("positions").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0) + selection.get("routes").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0);
    let mut fields = vec![
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.render-mode".into(),
            label: labels.render_mode.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.render-mode.select".into(),
                value: cfg.render_mode.clone(),
                items: vec![
                    UiSelectItem { value: "image".into(), label: labels.render_mode_image.into() },
                    UiSelectItem { value: "vector".into(), label: labels.render_mode_vector.into() },
                    UiSelectItem { value: "combined".into(), label: labels.render_mode_combined.into() },
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
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.vector-style".into(),
            label: labels.vector_style.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.vector-style.select".into(),
                value: cfg.vector_style.clone(),
                items: vec![
                    UiSelectItem { value: "colored".into(), label: labels.vector_style_colored.into() },
                    UiSelectItem { value: "figureGround".into(), label: labels.vector_style_figure_ground.into() },
                    UiSelectItem { value: "invertedFigure".into(), label: labels.vector_style_inverted_figure.into() },
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
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.lod-mode".into(),
            label: labels.lod_mode.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
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
        UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "gis2d-play-inspector.selection-method".into(),
            label: labels.selection_method.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.selection-method.select".into(),
                value: cfg.selection_method.clone(),
                items: vec![UiSelectItem { value: "rectangle".into(), label: labels.selection_method_rectangle.into() }, UiSelectItem { value: "lasso".into(), label: labels.selection_method_lasso.into() }],
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
    UiInspectorFieldGroup { presence: UiPresence::default(), id: "gis2d-play-inspector.map-view".into(), label: labels.map_view.into(), default_open: Some(true), fields }
}

fn build_inspector_tree(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let map_view_group = map_view_field_group(cfg, labels);
    if cfg.selected_ids.is_empty() {
        let visible_count = GIS_MAP_LAYER_IDS.iter().filter(|(id, _, _)| layer_visible(cfg, id)).count();
        return ui_inspector_groups_to_tree(&[
            map_view_group,
            UiInspectorFieldGroup {
                presence: UiPresence::default(),
                id: "gis2d-play-inspector.summary".into(),
                label: labels.map_layer.into(),
                default_open: Some(true),
                fields: vec![
                    ui_inspector_readonly_field("gis2d-play-inspector.schema", labels.schema, GIS_MAP_SCHEMA.to_string()),
                    ui_inspector_readonly_field("gis2d-play-inspector.visible-count", labels.layers_visible, format!("{visible_count}/{}", GIS_MAP_LAYER_IDS.len())),
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
                UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
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

/// 🖱️ On-demand GIS tiled-map context menu from feature hit-test and selection — grouped
/// disclosure via `Menu::of(registry)`; `organize_context_menu` (run automatically at the
/// `VcsDocumentApp::context_menu` funnel) sorts the declared `.group(...)` rows into
/// `RIBBON_PARENT_CATEGORIES` taxonomy order and inserts the pre-destructive separator itself.
fn gis2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected_ids: &[String]) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    let hits = surface.map(|s| s.hits.as_slice()).unwrap_or(&[]);
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
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
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

    /// 🎯️ Maps host action id + JSON args onto `Gis2dCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly.
    fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<Self::Command, String> {
        let args = args.cloned().unwrap_or(Value::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "setActiveExample" => Ok(Gis2dCommand::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() }),
            "patchPositions" => Ok(Gis2dCommand::PatchPositions { positions_json: str_arg(&["positionsJson", "positions_json"]).or_else(|| args.get("positions").map(|value| value.to_string())).unwrap_or_else(|| "[]".into()) }),
            "patchRoutes" => Ok(Gis2dCommand::PatchRoutes {
                route_ids: {
                    let mut ids = string_list("routeIds");
                    if ids.is_empty() {
                        ids = string_list("route_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: str_arg(&["value"]).unwrap_or_default(),
            }),
            "patchRoute" => Ok(Gis2dCommand::PatchRoute { route_id: str_arg(&["routeId", "route_id"]).unwrap_or_default(), field: str_arg(&["field"]).unwrap_or_default(), value: str_arg(&["value"]).unwrap_or_default() }),
            "setSelection" => Ok(Gis2dCommand::SetSelection { ids: string_list("ids") }),
            "toggleLayerVisibility" => Ok(Gis2dCommand::ToggleLayerVisibility { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default() }),
            "fitWorld" => Ok(Gis2dCommand::FitWorld),
            "setCamera" => {
                let camera_json = str_arg(&["cameraJson", "camera_json"]).or_else(|| args.get("camera").map(|value| if value.is_string() { value.as_str().unwrap_or("{}").to_string() } else { value.to_string() })).unwrap_or_else(|| "{}".into());
                Ok(Gis2dCommand::SetCamera { camera_json })
            }
            "setRenderMode" => Ok(Gis2dCommand::SetRenderMode { value: str_arg(&["value", "renderMode", "render_mode"]).unwrap_or_default() }),
            "setVectorStyle" => Ok(Gis2dCommand::SetVectorStyle { value: str_arg(&["value", "vectorStyle", "vector_style"]).unwrap_or_default() }),
            "setLodMode" => Ok(Gis2dCommand::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() }),
            "setFeatureSelection" => Ok(Gis2dCommand::SetFeatureSelection { positions: string_list("positions"), routes: string_list("routes"), mode: str_arg(&["mode"]).unwrap_or_else(|| "default".into()) }),
            "setHover" => {
                let hover_json = str_arg(&["hoverJson", "hover_json"])
                    .or_else(|| args.get("hover").map(|value| value.to_string()))
                    .or_else(|| {
                        let object = args.as_object()?;
                        if object.is_empty() || object.keys().all(|key| key == "surfaceId") {
                            Some("null".into())
                        } else {
                            Some(args.to_string())
                        }
                    })
                    .unwrap_or_else(|| "null".into());
                Ok(Gis2dCommand::SetHover { hover_json })
            }
            "setSelectionMethod" => Ok(Gis2dCommand::SetSelectionMethod { value: str_arg(&["value", "selectionMethod", "selection_method"]).unwrap_or_default() }),
            "setSelectionMode" => Ok(Gis2dCommand::SetSelectionMode { value: str_arg(&["value", "selectionMode", "selection_mode"]).unwrap_or_default() }),
            "clearSelection" => Ok(Gis2dCommand::ClearSelection),
            "selectAll" => Ok(Gis2dCommand::SelectAll),
            "deselect" => Ok(Gis2dCommand::Deselect { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(), feature_kind: str_arg(&["featureKind", "feature_kind"]).unwrap_or_else(|| "position".into()) }),
            "focusFeature" => Ok(Gis2dCommand::FocusFeature { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(), feature_kind: str_arg(&["featureKind", "feature_kind"]).unwrap_or_else(|| "position".into()) }),
            "setLayerStrokeScale" => Ok(Gis2dCommand::SetLayerStrokeScale { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default(), value: f64_arg(&["value"]).unwrap_or(1.0) }),
            "setLocale" => Ok(Gis2dCommand::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() }),
            "openSource" => Ok(Gis2dCommand::OpenSource { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default() }),
            other => Err(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            )),
        }
    }

    fn handle(&self, command: &Gis2dCommand, doc: &DocumentView<'_, gis2d::GisMapDocument>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapOperation, Gis2dConfigOperation>, Fault> {
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
            Gis2dCommand::SetSelection { ids } => Ok(Emit::config(vec![Gis2dConfigOperation::SetSelection { ids: ids.clone() }]),
            Gis2dCommand::ToggleLayerVisibility { layer_id } => {
                let visible = !layer_visible(config, layer_id);
                Emit::config(vec![Gis2dConfigOperation::SetLayerVisibility { layer_id: layer_id.clone(), visible }])
            }
            Gis2dCommand::FitWorld => {
                let mut host = map_host_from(document, config);
                host.fit_world_camera();
                Emit::config(vec![Gis2dConfigOperation::SetCamera { camera_json: host.camera_json() }])
            }
            Gis2dCommand::SetCamera { camera_json } => Ok(Emit::config(vec![Gis2dConfigOperation::SetCamera { camera_json: camera_json.clone() }]),
            Gis2dCommand::SetRenderMode { value } => Ok(Emit::config(vec![Gis2dConfigOperation::SetRenderMode { value: value.clone() }]),
            Gis2dCommand::SetVectorStyle { value } => Ok(Emit::config(vec![Gis2dConfigOperation::SetVectorStyle { value: value.clone() }]),
            Gis2dCommand::SetLodMode { value } => Ok(Emit::config(vec![Gis2dConfigOperation::SetLodMode { value: value.clone() }]),
            Gis2dCommand::SetFeatureSelection { positions, routes, mode } => {
                let selection = merge_feature_selection(&config.feature_selection_json, positions.clone(), routes.clone(), mode);
                let mut host = map_host_from(document, config);
                if host.set_selection_json(&selection.to_string()).is_ok() {
                    Emit::config(vec![Gis2dConfigOperation::SetFeatureSelection { value_json: selection.to_string() }])
                } else {
                    Emit::default()
                }
            }
            Gis2dCommand::SetHover { hover_json } => Ok(Emit::config(vec![Gis2dConfigOperation::SetHover { value_json: hover_json.clone() }]),
            Gis2dCommand::SetSelectionMethod { value } => Ok(Emit::config(vec![Gis2dConfigOperation::SetSelectionMethod { value: value.clone() }]),
            Gis2dCommand::SetSelectionMode { value } => Ok(Emit::config(vec![Gis2dConfigOperation::SetSelectionMode { value: value.clone() }]),
            Gis2dCommand::ClearSelection => Ok(Emit::config(vec![Gis2dConfigOperation::SetFeatureSelection { value_json: Gis2dConfig::default().feature_selection_json }]),
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
            Gis2dCommand::SetLayerStrokeScale { layer_id, value } => Ok(Emit::config(vec![Gis2dConfigOperation::SetLayerStrokeScale { layer_id: layer_id.clone(), value: clamp_map_layer_weight(*value) }]),
            Gis2dCommand::SetLocale { value } => Ok(Emit::config(vec![Gis2dConfigOperation::SetLocale { value: value.clone() }]),
            // 🌐️ Shell action — opens the picked feature's source URL through the host.
            Gis2dCommand::OpenSource { feature_id } => {
                let host = map_host_from(document, config);
                match host.positions.get(feature_id).and_then(|row| row.source_url.clone()) {
                    Some(url) => Ok(Emit::effect(HostEffect::OpenExternalUrl { url }),
                    None => Ok(Emit::default(),
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
        let labels = semio_framework_plugin::resolve_labels_for_locale::<Gis2dPlayLabels>(&cfg.projection.locale);
        match body_key {
            GIS2D_PLAY_BODY_COMPOSITE => render_canvas(document, cfg.projection),
            GIS2D_PLAY_BODY_DOCUMENT => build_document_tree(cfg.projection, labels),
            GIS2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(cfg.projection, labels),
            GIS2D_PLAY_BODY_INSPECTION => build_inspector_tree(cfg.projection, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, gis2d::GisMapDocument>, cfg: &ConfigView<'_, Gis2dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<Gis2dPlayLabels>(&cfg.projection.locale);
        HashMap::from([(GIS2D_PLAY_WINDOW_MAIN.into(), gis2d_window_measures(cfg.projection, labels))])
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, gis2d::GisMapDocument>,
        cfg: &ConfigView<'_, Gis2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        gis2d_context_menu_items(registry, request.surface.as_ref(), &cfg.projection.selected_ids)
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
    std::iter::once(ActionArgOption::new(GIS_MAP_LOD_MODE_AUTOMATIC, LocalizedLabel::native("Automatic", "Automatisch")))
        .chain(serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
            let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
            let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            Some(ActionArgOption::new(id, LocalizedLabel::data(name)))
        }))
        .collect()
}

pub fn create_gis2d_app() -> App {
    App::from_builder(
        App::builder(GIS2D_PLAY_APP_ID, LocalizedLabel::native("GIS 2D", "GIS 2D")).document(["semio", "gis", "2d"])
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
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(GIS2D_PLAY_WINDOW_MAIN, LocalizedLabel::native("Map", "Karte"), GIS2D_PLAY_BODY_COMPOSITE, SurfaceKind::TiledMap, "globe")
            .default_layout(create_default_layout(
                &[GIS2D_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Map".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                GIS2D_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                GIS2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                GIS2D_PLAY_BODY_INSPECTION,
            )
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
                ActionArgDef::select("value", LocalizedLabel::native("LOD Mode", "LOD-Modus"), lod_arg_options()).default_value(GIS_MAP_LOD_MODE_AUTOMATIC),
            ])
            .action_args("setSelectionMethod", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Selection Method", "Auswahlmethode"), vec![
                    ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")),
                    ActionArgOption::new("lasso", LocalizedLabel::native("Lasso", "Lasso")),
                ]).default_value("rectangle"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis2dPlayApp::default().config_spec())
            .io(gis2d_io()),
    )
    .example("reuse-map", LocalizedLabel::native("Reuse Map", "Karte wiederverwenden"), serde_json::to_string(&default_document()).unwrap(), "file-text")
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
                    let envelope: GisMapEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    GisMapStore::new(envelope)
                }
                None => GisMapStore::new(store::create_document_envelope(GIS_MAP_SCHEMA, "gis", gis2d_engine::empty_gis_map_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
    use semio_framework_plugin::{testkit, ActionKind, ContextMenuRequest, PluginApp, VcsDocumentApp, ViewState};

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
        let result = app.dispatch_typed(Gis2dCommand::SetActiveExample { example_id: "".into() }, &testkit::meta("local")).expect("operation emits operations without tripping the kind-discipline guard");
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
        let result = app.dispatch_typed(Gis2dCommand::PatchRoute { route_id: route_id.into(), field: "label".into(), value: "Renamed Route".into() }, &testkit::meta("local")).expect("patchRoute");
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
        testkit::assert_two_instances_converge::<Gis2dPlayApp, _>("mem://gis2d-convergence", command_a, command_b, |app| {
            let document = app.projection().expect("projection");
            (label(&document, route_a), label(&document, route_b))
        });
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

    /// 🖱️ Grouped disclosure: the empty-canvas context menu (no feature under the pointer) stays
    /// within the row budget and keeps the known destructive `clearSelection` last, matching the
    /// canonical migration pattern (see also `flow`'s identical budget/destructive-last test).
    #[test]
    fn context_menu_stays_within_budget_and_keeps_clear_selection_destructive_last() {
        let mut app = new_app_with_registry();
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "gis2dMap".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("empty-canvas context menu should not be empty");
        assert_eq!(last.id, "clearSelection", "known destructive clearSelection must be last: {menu:?}");
        assert_eq!(last.destructive, Some(true), "clearSelection must be marked destructive: {menu:?}");
    }
}
//#endregion 🧪️Tests
