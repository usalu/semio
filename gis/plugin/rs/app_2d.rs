//! 🗺️ GIS 2D plugin — GIS map play app bundled as a hot-swappable WASM component.

use gis_2d::{
    clamp_map_layer_weight, empty_gis_map_projection, gis_map_layer_weight_slider_ids_json, gis_map_lod_scale_json,
    open_url, GisMapDocument, GisMapEnvelope, GisMapOp, GisMapStore, MapHost, GIS_MAP_LOD_MODE_AUTOMATIC,
    GIS_MAP_SCHEMA,
};
use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_gis_map_scene, create_default_layout, layout::MeasureSelectItem, ui_inspector_groups_to_tree, ui_inspector_mixed_toggle,
    ui_inspector_readonly_field, ui_text, App, CommandDescriptor, DwgDrawing, DwgGeometry, GisMapScene, PluginApp, PluginBundle,
    UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiSelectItem, UiSelectNode, UiSliderNode,
    UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentVcsCommand};

//#region 🔖Constants
const GIS2D_PLAY_APP_ID: &str = "gis2d-play";
const GIS2D_PLAY_SURFACE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_COMPOSITE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_DOCUMENT: &str = "gis2d.play.document";
const GIS2D_PLAY_BODY_CATALOGUE: &str = "gis2d.play.catalogue";
const GIS2D_PLAY_BODY_INSPECTION: &str = "gis2d.play.inspection";
const GIS2D_PLAY_WINDOW_MAIN: &str = "gis2d-main";

const REUSE_MAP_EXAMPLE_JSON: &str = include_str!("../../2d/example/reuse.map.gis.json");

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
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Gis2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    layer_visibility: HashMap<String, bool>,
    #[serde(default)]
    map_fixture_json: String,
    #[serde(default = "default_map_camera_json")]
    camera_json: String,
    #[serde(default = "default_render_mode")]
    render_mode: String,
    #[serde(default = "default_vector_style")]
    vector_style: String,
    #[serde(default = "default_lod_mode")]
    lod_mode: String,
    #[serde(default = "default_feature_selection_json")]
    feature_selection_json: String,
    #[serde(default = "default_hover_json")]
    hover_json: String,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default = "default_selection_mode")]
    selection_mode: String,
    #[serde(default)]
    layer_stroke_scale: HashMap<String, f64>,
}

fn default_hover_json() -> String {
    "null".into()
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_selection_mode() -> String {
    "default".into()
}

fn default_map_camera_json() -> String {
    r#"{"x":0,"y":0,"zoom":1}"#.into()
}

fn default_render_mode() -> String {
    "combined".into()
}

fn default_vector_style() -> String {
    "colored".into()
}

fn default_lod_mode() -> String {
    GIS_MAP_LOD_MODE_AUTOMATIC.into()
}

fn default_feature_selection_json() -> String {
    r#"{"positions":[],"routes":[]}"#.into()
}

impl Default for Gis2dPlayRuntime {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            layer_visibility: HashMap::new(),
            map_fixture_json: String::new(),
            camera_json: default_map_camera_json(),
            render_mode: default_render_mode(),
            vector_style: default_vector_style(),
            lod_mode: default_lod_mode(),
            feature_selection_json: default_feature_selection_json(),
            hover_json: default_hover_json(),
            selection_method: default_selection_method(),
            selection_mode: default_selection_mode(),
            layer_stroke_scale: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Gis2dPlayEnvelope {
    envelope: GisMapEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    redo_edit_ids: Vec<String>,
    #[serde(default)]
    runtime: Gis2dPlayRuntime,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_layer_visibility() -> HashMap<String, bool> {
    GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), true)).collect()
}

fn default_envelope() -> Gis2dPlayEnvelope {
    let mut runtime = Gis2dPlayRuntime::default();
    runtime.layer_visibility = default_layer_visibility();
    runtime.map_fixture_json = REUSE_MAP_EXAMPLE_JSON.into();
    let mut play = Gis2dPlayEnvelope {
        envelope: create_document_vcs_envelope(GIS_MAP_SCHEMA, "gis", empty_gis_map_projection(), None),
        applied_edit_ids: Vec::new(),
        redo_edit_ids: Vec::new(),
        runtime,
    };
    let mut host = map_host_from_play(&play);
    host.fit_world_camera();
    play.runtime.camera_json = host.camera_json();
    play
}

fn map_host_from_play(play: &Gis2dPlayEnvelope) -> MapHost {
    let mut host = MapHost::new();
    if !play.runtime.map_fixture_json.is_empty() {
        let _ = host.sync_map_json(&play.runtime.map_fixture_json);
    }
    if let Ok(camera) = serde_json::from_str::<Value>(&play.runtime.camera_json) {
        let x = camera.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let y = camera.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let zoom = camera.get("zoom").and_then(|value| value.as_f64()).unwrap_or(1.0);
        host.set_camera(x, y, zoom);
    }
    host.set_render_mode(&play.runtime.render_mode);
    host.set_vector_style(&play.runtime.vector_style);
    host.set_lod_mode(&play.runtime.lod_mode);
    let _ = host.set_selection_json(&play.runtime.feature_selection_json);
    host
}

fn layer_visibility_json(runtime: &Gis2dPlayRuntime) -> String {
    let mut map = default_layer_visibility();
    for (id, visible) in &runtime.layer_visibility {
        map.insert(id.clone(), *visible);
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

fn layer_stroke_scale_json(runtime: &Gis2dPlayRuntime) -> String {
    let mut map: HashMap<String, f64> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, _, _)| ((*id).into(), 1.0))
        .collect();
    for (id, weight) in &runtime.layer_stroke_scale {
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

fn parse_envelope(document_json: &str) -> Gis2dPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Gis2dPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn gis2d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: GIS2D_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn store_from_envelope(play: &Gis2dPlayEnvelope) -> GisMapStore {
    let mut store = GisMapStore::new(play.envelope.clone());
    store.set_envelope(play.envelope.clone(), play.applied_edit_ids.clone());
    store
}

fn sync_store_to_envelope(store: &GisMapStore, runtime: &Gis2dPlayRuntime, redo_edit_ids: &[String]) -> Gis2dPlayEnvelope {
    Gis2dPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        redo_edit_ids: redo_edit_ids.to_vec(),
        runtime: runtime.clone(),
    }
}

fn materialized_projection(play: &Gis2dPlayEnvelope) -> GisMapDocument {
    materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

fn layer_visible(runtime: &Gis2dPlayRuntime, layer_id: &str) -> bool {
    runtime.layer_visibility.get(layer_id).copied().unwrap_or(true)
}

fn layer_weight_slider_fields(play: &Gis2dPlayEnvelope) -> Vec<UiNode> {
    layer_weight_entries(play)
        .into_iter()
        .map(|(layer_id, label, value)| {
            UiNode::Field(UiFieldNode {
                id: format!("gis2d-play-inspector.weight.{layer_id}"),
                label: format!("{label} weight"),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    id: format!("gis2d-play-inspector.weight.{layer_id}.slider"),
                    value,
                    min: 0.25,
                    max: 3.0,
                    step: 0.05,
                    on_change: gis2d_cmd(
                        "setLayerStrokeScale",
                        Some(json!({ "layerId": layer_id })),
                    ),
                    unit: None,
                })),
                description: None,
                required: None,
                error: None,
            })
        })
        .collect()
}

fn lod_select_entries() -> Vec<(String, String)> {
    std::iter::once((GIS_MAP_LOD_MODE_AUTOMATIC.into(), "Automatic".into()))
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

fn layer_weight_entries(play: &Gis2dPlayEnvelope) -> Vec<(String, String, f64)> {
    let ids: Vec<String> = serde_json::from_str(&gis_map_layer_weight_slider_ids_json(
        &play.runtime.lod_mode,
        &play.runtime.render_mode,
    ))
    .unwrap_or_default();
    ids.into_iter()
        .map(|layer_id| {
            let value = play
                .runtime
                .layer_stroke_scale
                .get(&layer_id)
                .copied()
                .map(clamp_map_layer_weight)
                .unwrap_or(1.0);
            let label = GIS_MAP_LAYER_IDS
                .iter()
                .find(|(id, _, _)| *id == layer_id.as_str())
                .map(|(_, label, _)| *label)
                .unwrap_or(layer_id.as_str())
                .to_string();
            (layer_id, label, value)
        })
        .collect()
}

fn gis2d_window_measures(play: &Gis2dPlayEnvelope) -> Vec<WindowMeasure> {
    let layer_toggles: Vec<WindowMeasure> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, label, icon)| WindowMeasure::Toggle {
            id: format!("gis2d-play-window.layer.{id}"),
            icon_id: (*icon).into(),
            label: Some((*label).into()),
            pressed: layer_visible(&play.runtime, id),
            text: None,
            on_change: gis2d_cmd("toggleLayerVisibility", Some(json!({ "layerId": id }))),
        })
        .collect();
    let layer_weight_sliders: Vec<WindowMeasure> = layer_weight_entries(play)
        .into_iter()
        .map(|(layer_id, label, value)| WindowMeasure::Slider {
            id: format!("gis2d-play-window.weight.{layer_id}"),
            label: Some(format!("{label} weight")),
            value,
            min: 0.25,
            max: 3.0,
            step: Some(0.05),
            on_change: gis2d_cmd("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),
        })
        .collect();
    vec![
        WindowMeasure::Select {
            id: "gis2d-play-window.render-mode".into(),
            label: Some("Render Mode".into()),
            value: play.runtime.render_mode.clone(),
            items: vec![
                MeasureSelectItem { id: "image".into(), value: "image".into(), label: "Image".into() },
                MeasureSelectItem { id: "vector".into(), value: "vector".into(), label: "Vector".into() },
                MeasureSelectItem { id: "combined".into(), value: "combined".into(), label: "Combined".into() },
            ],
            on_change: gis2d_cmd("setRenderMode", None),
        },
        WindowMeasure::Select {
            id: "gis2d-play-window.vector-style".into(),
            label: Some("Vector Style".into()),
            value: play.runtime.vector_style.clone(),
            items: vec![
                MeasureSelectItem { id: "colored".into(), value: "colored".into(), label: "Colored".into() },
                MeasureSelectItem { id: "figureGround".into(), value: "figureGround".into(), label: "Figure Ground".into() },
                MeasureSelectItem { id: "invertedFigure".into(), value: "invertedFigure".into(), label: "Inverted Figure".into() },
            ],
            on_change: gis2d_cmd("setVectorStyle", None),
        },
        WindowMeasure::Select {
            id: "gis2d-play-window.lod-mode".into(),
            label: Some("LOD Mode".into()),
            value: play.runtime.lod_mode.clone(),
            items: lod_select_entries()
                .into_iter()
                .map(|(value, label)| MeasureSelectItem { id: value.clone(), value, label })
                .collect(),
            on_change: gis2d_cmd("setLodMode", None),
        },
        WindowMeasure::Select {
            id: "gis2d-play-window.selection-method".into(),
            label: Some("Selection Method".into()),
            value: play.runtime.selection_method.clone(),
            items: vec![
                MeasureSelectItem { id: "rectangle".into(), value: "rectangle".into(), label: "Rectangle".into() },
                MeasureSelectItem { id: "lasso".into(), value: "lasso".into(), label: "Lasso".into() },
            ],
            on_change: gis2d_cmd("setSelectionMethod", None),
        },
        WindowMeasure::Group {
            id: "gis2d-play-window.layers".into(),
            label: "Layers".into(),
            default_open: Some(true),
            children: layer_toggles,
        },
        WindowMeasure::Group {
            id: "gis2d-play-window.layer-weights".into(),
            label: "Layer Weights".into(),
            default_open: Some(false),
            children: layer_weight_sliders,
        },
    ]
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn tree_item(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    icon_id: Option<String>,
    command: Option<CommandDescriptor>,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id,
        selected: None,
        default_open: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        command,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_document_tree(play: &Gis2dPlayEnvelope) -> UiNode {
    let projection = materialized_projection(play);
    let layer_items: Vec<UiTreeItemNode> = if projection.layers.is_empty() {
        GIS_MAP_LAYER_IDS
            .iter()
            .map(|(id, label, icon)| {
                tree_item(
                    format!("gis2d-play-document.layer.{id}"),
                    *label,
                    Some((*id).into()),
                    Some((*icon).into()),
                    Some(gis2d_cmd("setSelection", Some(json!({ "ids": [id] })))),
                )
            })
            .collect()
    } else {
        projection
            .layers
            .iter()
            .filter_map(|layer| {
                let id = layer.get("id").and_then(|value| value.as_str())?;
                let label = layer.get("name").and_then(|value| value.as_str()).unwrap_or(id);
                Some(tree_item(
                    format!("gis2d-play-document.layer.{id}"),
                    label,
                    Some(id.into()),
                    Some("layers".into()),
                    Some(gis2d_cmd("setSelection", Some(json!({ "ids": [id] })))),
                ))
            })
            .collect()
    };
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "gis2d-play-document.layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: layer_items,
        }],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .map(|id| format!("gis2d-play-document.layer.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(gis2d_cmd("setSelection", None)),
        drop_command: None,
    })
}

fn build_catalogue_tree(play: &Gis2dPlayEnvelope) -> UiNode {
    let items: Vec<UiTreeItemNode> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, label, icon)| {
            tree_item(
                format!("gis2d-play-catalogue.layer.{id}"),
                *label,
                None,
                Some((*icon).into()),
                Some(gis2d_cmd("toggleLayerVisibility", Some(json!({ "layerId": id })))),
            )
        })
        .collect();
    let _ = play;
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "gis2d-play-catalogue.layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_command: None,
    })
}

fn map_view_field_group(play: &Gis2dPlayEnvelope) -> UiInspectorFieldGroup {
    let lod_items: Vec<UiSelectItem> = lod_select_entries()
        .into_iter()
        .map(|(value, label)| UiSelectItem { value, label })
        .collect();
    let selection: Value = serde_json::from_str(&play.runtime.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let selected_count = selection.get("positions").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
        + selection.get("routes").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0);
    let mut fields = vec![
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.render-mode".into(),
                label: "Render Mode".into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "gis2d-play-inspector.render-mode.select".into(),
                    value: play.runtime.render_mode.clone(),
                    items: vec![
                        UiSelectItem { value: "image".into(), label: "Image".into() },
                        UiSelectItem { value: "vector".into(), label: "Vector".into() },
                        UiSelectItem { value: "combined".into(), label: "Combined".into() },
                    ],
                    placeholder: None,
                    on_change: gis2d_cmd("setRenderMode", None),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.vector-style".into(),
                label: "Vector Style".into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "gis2d-play-inspector.vector-style.select".into(),
                    value: play.runtime.vector_style.clone(),
                    items: vec![
                        UiSelectItem { value: "colored".into(), label: "Colored".into() },
                        UiSelectItem { value: "figureGround".into(), label: "Figure Ground".into() },
                        UiSelectItem { value: "invertedFigure".into(), label: "Inverted Figure".into() },
                    ],
                    placeholder: None,
                    on_change: gis2d_cmd("setVectorStyle", None),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.lod-mode".into(),
                label: "LOD Mode".into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "gis2d-play-inspector.lod-mode.select".into(),
                    value: play.runtime.lod_mode.clone(),
                    items: lod_items,
                    placeholder: None,
                    on_change: gis2d_cmd("setLodMode", None),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.selection-method".into(),
                label: "Selection Method".into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "gis2d-play-inspector.selection-method.select".into(),
                    value: play.runtime.selection_method.clone(),
                    items: vec![
                        UiSelectItem { value: "rectangle".into(), label: "Rectangle".into() },
                        UiSelectItem { value: "lasso".into(), label: "Lasso".into() },
                    ],
                    placeholder: None,
                    on_change: gis2d_cmd("setSelectionMethod", None),
                })),
                description: None,
                required: None,
                error: None,
            }),
            ui_inspector_readonly_field("gis2d-play-inspector.feature-selection", "Selected Features", selected_count.to_string()),
    ];
    fields.extend(layer_weight_slider_fields(play));
    UiInspectorFieldGroup {
        id: "gis2d-play-inspector.map-view".into(),
        label: "Map View".into(),
        default_open: Some(true),
        fields,
    }
}

fn build_inspector_tree(play: &Gis2dPlayEnvelope) -> UiNode {
    let map_view_group = map_view_field_group(play);
    if play.runtime.selected_ids.is_empty() {
        let visible_count = GIS_MAP_LAYER_IDS
            .iter()
            .filter(|(id, _, _)| layer_visible(&play.runtime, id))
            .count();
        return ui_inspector_groups_to_tree(&[
            map_view_group,
            UiInspectorFieldGroup {
                id: "gis2d-play-inspector.summary".into(),
                label: "Map Layer".into(),
                default_open: Some(true),
                fields: vec![
                    ui_inspector_readonly_field("gis2d-play-inspector.schema", "Schema", GIS_MAP_SCHEMA.to_string()),
                    ui_inspector_readonly_field(
                        "gis2d-play-inspector.visible-count",
                        "Layers visible",
                        format!("{visible_count}/{}", GIS_MAP_LAYER_IDS.len()),
                    ),
                ],
            },
        ]);
    }
    let layer_id = &play.runtime.selected_ids[0];
    let label = GIS_MAP_LAYER_IDS
        .iter()
        .find(|(id, _, _)| *id == layer_id.as_str())
        .map(|(_, label, _)| *label)
        .unwrap_or(layer_id.as_str());
    let visible = layer_visible(&play.runtime, layer_id);
    let mixed = ui_inspector_mixed_toggle(&[visible]);
    ui_inspector_groups_to_tree(&[
        map_view_group,
        UiInspectorFieldGroup {
            id: "gis2d-play-inspector.layer".into(),
            label: "Map Layer".into(),
            default_open: Some(true),
            fields: vec![
                ui_inspector_readonly_field("gis2d-play-inspector.id", "Id", layer_id.clone()),
                ui_inspector_readonly_field("gis2d-play-inspector.label", "Label", label.to_string()),
                UiNode::Field(UiFieldNode {
                    id: "gis2d-play-inspector.visible".into(),
                    label: "Visible".into(),
                    child: Box::new(UiNode::Toggle(UiToggleNode {
                        id: "gis2d-play-inspector.visible.toggle".into(),
                        icon_id: "eye".into(),
                        pressed: mixed.uniform && mixed.pressed,
                        text: None,
                        on_change: gis2d_cmd("toggleLayerVisibility", Some(json!({ "layerId": layer_id }))),
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn apply_gis_map_tile_base_url(scene: &mut GisMapScene) {
    let Ok(base) = std::env::var("SEMIO_GIS_MAP_TILE_BASE_URL") else {
        return;
    };
    let base = base.trim_end_matches('/');
    scene.tile_url_template = format!("{base}/osm/{{z}}/{{x}}/{{y}}.png");
    scene.vector_tile_url_template = format!("{base}/vt/{{z}}/{{x}}/{{y}}.pbf");
}

fn render_canvas(play: &Gis2dPlayEnvelope) -> UiNode {
    let mut scene = GisMapScene::base(
        play.runtime.map_fixture_json.clone(),
        play.runtime.camera_json.clone(),
    );
    scene.render_mode = play.runtime.render_mode.clone();
    scene.vector_style = play.runtime.vector_style.clone();
    scene.lod_mode = play.runtime.lod_mode.clone();
    scene.layer_visibility_json = layer_visibility_json(&play.runtime);
    scene.layer_stroke_scale_json = layer_stroke_scale_json(&play.runtime);
    scene.selection_json = play.runtime.feature_selection_json.clone();
    scene.hover_json = play.runtime.hover_json.clone();
    scene.selection_method = play.runtime.selection_method.clone();
    scene.selection_mode = play.runtime.selection_mode.clone();
    apply_gis_map_tile_base_url(&mut scene);
    build_gis_map_scene(GIS2D_PLAY_SURFACE, GIS2D_PLAY_APP_ID, scene)
}
//#endregion 🔖Render

//#region 🔖Gis2dPlayApp
pub struct Gis2dPlayApp;

impl PluginApp for Gis2dPlayApp {
    fn app_id(&self) -> &str {
        GIS2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("gis2d envelope json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<Gis2dPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                play.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&play)];
            }
            "toggleLayerVisibility" => {
                if let Some(layer_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    let visible = !layer_visible(&play.runtime, layer_id);
                    play.runtime.layer_visibility.insert(layer_id.into(), visible);
                    return vec![set_document_op(&play)];
                }
            }
            "setLayers" => {
                if let Some(layers) = args.and_then(|value| value.get("layers")) {
                    if let Ok(parsed) = serde_json::from_value(layers.clone()) {
                        let _ = store.dispatch(DocumentVcsCommand::Apply {
                            operations: vec![GisMapOp::SetLayers { layers: parsed }],
                            description: None,
                        });
                        play.redo_edit_ids.clear();
                        return vec![set_document_op(&sync_store_to_envelope(&store, &play.runtime, &play.redo_edit_ids))];
                    }
                }
            }
            "undo" => {
                let _ = store.dispatch(DocumentVcsCommand::Undo);
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.runtime, &play.redo_edit_ids))];
            }
            "redo" => {
                let _ = store.dispatch(DocumentVcsCommand::Redo);
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.runtime, &play.redo_edit_ids))];
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                play.runtime.map_fixture_json = if example_id.is_empty() || example_id == "empty" {
                    r#"{"positions":[],"routes":[],"regions":[]}"#.into()
                } else {
                    REUSE_MAP_EXAMPLE_JSON.into()
                };
                play.runtime.selected_ids.clear();
                let mut host = map_host_from_play(&play);
                if !example_id.is_empty() && example_id != "empty" {
                    host.fit_world_camera();
                    play.runtime.camera_json = host.camera_json();
                }
                return vec![set_document_op(&play)];
            }
            "fitWorld" => {
                let mut host = map_host_from_play(&play);
                host.fit_world_camera();
                play.runtime.camera_json = host.camera_json();
                return vec![set_document_op(&play)];
            }
            "patchPositions" => {
                if let Some(positions) = args.and_then(|value| value.get("positions")) {
                    let mut descriptor: Value = serde_json::from_str(&play.runtime.map_fixture_json)
                        .unwrap_or_else(|_| json!({ "positions": [], "routes": [], "regions": [] }));
                    descriptor["positions"] = positions.clone();
                    play.runtime.map_fixture_json = descriptor.to_string();
                    let _ = store.dispatch(DocumentVcsCommand::Apply {
                        operations: vec![GisMapOp::SetLayers {
                            layers: vec![json!({ "id": "positions", "name": "Positions", "kind": "map-layer" })],
                        }],
                        description: None,
                    });
                    play.redo_edit_ids.clear();
                    return vec![set_document_op(&sync_store_to_envelope(&store, &play.runtime, &play.redo_edit_ids))];
                }
            }
            "setCamera" => {
                let camera = args
                    .and_then(|value| value.get("camera"))
                    .or_else(|| args.and_then(|value| value.get("cameraJson")));
                if let Some(camera) = camera {
                    play.runtime.camera_json = camera.to_string();
                    return vec![set_document_op(&play)];
                }
            }
            "setRenderMode" => {
                if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                    play.runtime.render_mode = mode.into();
                    return vec![set_document_op(&play)];
                }
            }
            "setVectorStyle" => {
                if let Some(style) = args.and_then(|value| value.get("style").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                    play.runtime.vector_style = style.into();
                    return vec![set_document_op(&play)];
                }
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                    play.runtime.lod_mode = mode.into();
                    return vec![set_document_op(&play)];
                }
            }
            "setFeatureSelection" => {
                let positions: Vec<String> = args
                    .and_then(|value| value.get("positions"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let routes: Vec<String> = args
                    .and_then(|value| value.get("routes"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let mode = args
                    .and_then(|value| value.get("mode"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("default");
                let selection = merge_feature_selection(&play.runtime.feature_selection_json, positions, routes, mode);
                let mut host = map_host_from_play(&play);
                if host.set_selection_json(&selection.to_string()).is_ok() {
                    play.runtime.feature_selection_json = selection.to_string();
                    return vec![set_document_op(&play)];
                }
            }
            "setHover" => {
                let hover = args.and_then(|value| value.get("hover")).cloned().unwrap_or(Value::Null);
                play.runtime.hover_json = hover.to_string();
                return vec![set_document_op(&play)];
            }
            "setSelectionMethod" => {
                if let Some(method) = args
                    .and_then(|value| value.get("method").or_else(|| value.get("value")))
                    .and_then(|value| value.as_str())
                {
                    play.runtime.selection_method = method.into();
                    return vec![set_document_op(&play)];
                }
            }
            "setSelectionMode" => {
                if let Some(mode) = args
                    .and_then(|value| value.get("mode").or_else(|| value.get("value")))
                    .and_then(|value| value.as_str())
                {
                    play.runtime.selection_mode = mode.into();
                    return vec![set_document_op(&play)];
                }
            }
            "clearSelection" => {
                play.runtime.feature_selection_json = default_feature_selection_json();
                return vec![set_document_op(&play)];
            }
            "selectAll" => {
                let host = map_host_from_play(&play);
                let selection = json!({
                    "positions": host.positions.keys().cloned().collect::<Vec<_>>(),
                    "routes": host.routes.keys().cloned().collect::<Vec<_>>(),
                });
                play.runtime.feature_selection_json = selection.to_string();
                return vec![set_document_op(&play)];
            }
            "deselect" => {
                let (Some(kind), Some(id)) = (
                    args.and_then(|value| value.get("featureKind")).and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("featureId")).and_then(|value| value.as_str()),
                ) else {
                    return Vec::new();
                };
                let mut selection: Value =
                    serde_json::from_str(&play.runtime.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
                if kind == "position" {
                    if let Some(rows) = selection.get_mut("positions").and_then(|value| value.as_array_mut()) {
                        rows.retain(|row| row.as_str() != Some(id));
                    }
                } else if kind == "route" {
                    if let Some(rows) = selection.get_mut("routes").and_then(|value| value.as_array_mut()) {
                        rows.retain(|row| row.as_str() != Some(id));
                    }
                }
                play.runtime.feature_selection_json = selection.to_string();
                return vec![set_document_op(&play)];
            }
            "focusFeature" => {
                let (Some(kind), Some(id)) = (
                    args.and_then(|value| value.get("featureKind")).and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("featureId")).and_then(|value| value.as_str()),
                ) else {
                    return Vec::new();
                };
                let mut host = map_host_from_play(&play);
                if host.focus_feature(kind, id) {
                    play.runtime.camera_json = host.camera_json();
                    return vec![set_document_op(&play)];
                }
            }
            "openSource" => {
                let feature_id = args.and_then(|value| value.get("featureId")).and_then(|value| value.as_str());
                if let Some(feature_id) = feature_id {
                    let host = map_host_from_play(&play);
                    if let Some(url) = host.positions.get(feature_id).and_then(|row| row.source_url.as_deref()) {
                        let _ = open_url(url);
                    }
                }
                return Vec::new();
            }
            "setLayerStrokeScale" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str());
                let value = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_f64())
                    .or_else(|| args.and_then(|value| value.get("weight")).and_then(|value| value.as_f64()));
                if let (Some(layer_id), Some(value)) = (layer_id, value) {
                    play.runtime
                        .layer_stroke_scale
                        .insert(layer_id.into(), clamp_map_layer_weight(value));
                    return vec![set_document_op(&play)];
                }
            }
            "patchRoutes" | "patchRoute" => {
                let route_ids: Vec<String> = if command == "patchRoute" {
                    args.and_then(|value| value.get("routeId"))
                        .and_then(|value| value.as_str())
                        .map(|id| vec![id.to_string()])
                        .unwrap_or_default()
                } else {
                    args.and_then(|value| value.get("routeIds"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_default()
                };
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str());
                let value = args.and_then(|value| value.get("value"));
                if let (false, Some(field), Some(value)) = (route_ids.is_empty(), field, value) {
                    let mut descriptor: Value = serde_json::from_str(&play.runtime.map_fixture_json)
                        .unwrap_or_else(|_| json!({ "positions": [], "routes": [], "regions": [] }));
                    if let Some(routes) = descriptor.get_mut("routes").and_then(|value| value.as_array_mut()) {
                        for route in routes.iter_mut() {
                            let matches = route
                                .get("id")
                                .and_then(|value| value.as_str())
                                .map(|id| route_ids.iter().any(|route_id| route_id == id))
                                .unwrap_or(false);
                            if matches {
                                if let Some(object) = route.as_object_mut() {
                                    object.insert(field.into(), value.clone());
                                }
                            }
                        }
                    }
                    play.runtime.map_fixture_json = descriptor.to_string();
                    return vec![set_document_op(&play)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            GIS2D_PLAY_BODY_COMPOSITE => render_canvas(&play),
            GIS2D_PLAY_BODY_DOCUMENT => build_document_tree(&play),
            GIS2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(&play),
            GIS2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(
        &self,
        document_json: &str,
        _view_state: &ViewState,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        let play = parse_envelope(document_json);
        HashMap::from([(GIS2D_PLAY_WINDOW_MAIN.into(), gis2d_window_measures(&play))])
    }
}
//#endregion 🔖Gis2dPlayApp

//#region 🔖AppFactory
pub fn create_gis2d_app() -> App {
    App::from_builder(
        App::builder(GIS2D_PLAY_APP_ID, "GIS 2D").document(["semio", "gis", "2d"])
            .icon_id("gis2d")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(GIS2D_PLAY_WINDOW_MAIN, "Map", GIS2D_PLAY_BODY_COMPOSITE, SurfaceKind::GisMap)
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
            .operation("setLayers", "Set Layers")
            .operation("patchPositions", "Patch Positions")
            .operation("patchRoutes", "Patch Routes")
            .operation("patchRoute", "Patch Route")
            .view_command("setSelection", "Set Selection")
            .view_command("toggleLayerVisibility", "Toggle Layer Visibility")
            .view_command("setActiveExample", "Set Active Example")
            .view_command("fitWorld", "Fit World")
            .view_command("setCamera", "Set Camera")
            .view_command("setRenderMode", "Set Render Mode")
            .view_command("setVectorStyle", "Set Vector Style")
            .view_command("setLodMode", "Set LOD Mode")
            .view_command("setFeatureSelection", "Set Feature Selection")
            .view_command("setHover", "Set Hover")
            .view_command("setSelectionMethod", "Set Selection Method")
            .view_command("setSelectionMode", "Set Selection Mode")
            .view_command("clearSelection", "Clear Selection")
            .view_command("selectAll", "Select All")
            .view_command("deselect", "Deselect")
            .view_command("focusFeature", "Focus Feature")
            .view_command("setLayerStrokeScale", "Set Layer Stroke Scale")
            .shell_command("setDocument", "Set Document")
            .shell_command("openSource", "Open Source")
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("reuse-map", "Reuse Map", serde_json::to_string(&default_envelope()).unwrap())
    .program("gis2d", "GIS 2D", "map")
}

fn gis2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::map_points_svg(value, "GIS 2D")
}

/// 🗺️ Imports a DWG drawing into a gis map document: entity vertices become the map's points list, framed to the drawing's extents. Falls back to the default reuse-map fixture when the DWG carries no point-like geometry.
fn gis2d_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let positions: Vec<[f64; 2]> = drawing
        .entities
        .iter()
        .flat_map(|entity| match &entity.geometry {
            DwgGeometry::Point { at } => vec![[at[0], at[1]]],
            DwgGeometry::Line { start, end } => vec![[start[0], start[1]], [end[0], end[1]]],
            DwgGeometry::LwPolyline { vertices, .. } => vertices.clone(),
            DwgGeometry::Polyline3d { vertices, .. } => vertices.iter().map(|v| [v[0], v[1]]).collect(),
            _ => Vec::new(),
        })
        .collect();
    if positions.is_empty() {
        return serde_json::to_value(&default_envelope()).map_err(|error| error.to_string());
    }
    let mut envelope = default_envelope();
    envelope.runtime.camera_json = serde_json::json!({
        "x": (drawing.extmin[0] + drawing.extmax[0]) / 2.0,
        "y": (drawing.extmin[1] + drawing.extmax[1]) / 2.0,
        "zoom": 1.0,
    })
    .to_string();
    let mut value = serde_json::to_value(&envelope).map_err(|error| error.to_string())?;
    value["positions"] = serde_json::to_value(&positions).map_err(|error| error.to_string())?;
    Ok(value)
}

pub fn register_gis2d_exports() {
    semio_framework_os::register_2d_export_handlers("2d.map", "gis2d", gis2d_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.map", gis2d_document_json_from_dwg);
}
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn dwg_import_collects_point_and_line_vertices() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
        drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [3.0, 4.0, 0.0] } });
        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn dwg_import_falls_back_to_default_envelope_when_empty() {
        let drawing = DwgDrawing::default();
        let value = gis2d_document_json_from_dwg(&drawing).expect("import empty dwg");
        assert!(serde_json::from_value::<Gis2dPlayEnvelope>(value).is_ok());
    }

    #[test]
    fn renders_gis_map_scene() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("gis2d-map"));
    }

    #[test]
    fn render_canvas_uses_absolute_tile_urls_when_env_set() {
        unsafe { std::env::set_var("SEMIO_GIS_MAP_TILE_BASE_URL", "http://127.0.0.1:6141") };
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("http://127.0.0.1:6141/osm/{z}/{x}/{y}.png"));
        assert!(json.contains("http://127.0.0.1:6141/vt/{z}/{x}/{y}.pbf"));
        unsafe { std::env::remove_var("SEMIO_GIS_MAP_TILE_BASE_URL") };
    }

    #[test]
    fn document_lists_map_layers() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("gis2d-play-document.layer.raster"));
    }

    #[test]
    fn catalogue_lists_layer_toggles() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("gis2d-play-catalogue.layer.water"));
    }

    #[test]
    fn set_selection_updates_runtime() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setSelection",
            Some(&json!({ "ids": ["roads"] })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.selected_ids, vec!["roads".to_string()]);
    }

    #[test]
    fn set_layers_command_persists_projection() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let layers = vec![json!({ "id": "custom", "name": "Custom", "x": 0.0, "y": 0.0, "width": 80.0, "height": 40.0 })];
        let ops = app.handle_command_patch_ops("setLayers", Some(&json!({ "layers": layers })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&next).layers.len(), 1);
    }

    #[test]
    fn toggle_layer_visibility_hides_layer() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "toggleLayerVisibility",
            Some(&json!({ "layerId": "raster" })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert!(!layer_visible(&next.runtime, "raster"));
    }

    #[test]
    fn set_render_mode_vector_style_lod_mode_persist() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setRenderMode", Some(&json!({ "mode": "vector" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.render_mode, "vector");
        let document = serde_json::to_string(&next).unwrap();

        let ops = app.handle_command_patch_ops("setVectorStyle", Some(&json!({ "style": "figureGround" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.vector_style, "figureGround");
        let document = serde_json::to_string(&next).unwrap();

        let ops = app.handle_command_patch_ops("setLodMode", Some(&json!({ "mode": "city" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.lod_mode, "city");

        let host = map_host_from_play(&next);
        assert_eq!(host.render_mode_str(), "vector");
        assert_eq!(host.vector_style_str(), "figureGround");
    }

    #[test]
    fn set_feature_selection_updates_runtime_and_host() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["p_institut_de_botanique_ulg_liege"], "routes": [] })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let selection: Value = serde_json::from_str(&next.runtime.feature_selection_json).unwrap();
        assert_eq!(selection["positions"], json!(["p_institut_de_botanique_ulg_liege"]));
    }

    #[test]
    fn set_hover_updates_runtime() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setHover",
            Some(&json!({ "hover": { "kind": "position", "id": "p_test" } })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let hover: Value = serde_json::from_str(&next.runtime.hover_json).unwrap();
        assert_eq!(hover["kind"], "position");
        assert_eq!(hover["id"], "p_test");
    }

    #[test]
    fn clear_selection_resets_features() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["p_institut_de_botanique_ulg_liege"], "routes": [] })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let document = payload["document"].to_string();
        let ops = app.handle_command_patch_ops("clearSelection", None, &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.feature_selection_json, default_feature_selection_json());
    }

    #[test]
    fn set_feature_selection_additive_merges() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["a"], "routes": [], "mode": "default" })),
            &document,
            &ViewState::default(),
        );
        let document = serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["b"], "routes": [], "mode": "additive" })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let selection: Value = serde_json::from_str(&next.runtime.feature_selection_json).unwrap();
        let positions = selection["positions"].as_array().unwrap();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&json!("a")));
        assert!(positions.contains(&json!("b")));
    }

    #[test]
    fn set_feature_selection_subtractive_removes() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["a", "b"], "routes": [], "mode": "default" })),
            &document,
            &ViewState::default(),
        );
        let document = serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["a"], "routes": [], "mode": "subtractive" })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let selection: Value = serde_json::from_str(&next.runtime.feature_selection_json).unwrap();
        assert_eq!(selection["positions"], json!(["b"]));
    }

    #[test]
    fn set_feature_selection_invertive_toggles() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["a"], "routes": [], "mode": "default" })),
            &document,
            &ViewState::default(),
        );
        let document = serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string();
        let ops = app.handle_command_patch_ops(
            "setFeatureSelection",
            Some(&json!({ "positions": ["b"], "routes": [], "mode": "invertive" })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let selection: Value = serde_json::from_str(&next.runtime.feature_selection_json).unwrap();
        assert_eq!(selection["positions"], json!(["a", "b"]));
    }

    #[test]
    fn set_layer_stroke_scale_persists_weight() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "setLayerStrokeScale",
            Some(&json!({ "layerId": "roads", "value": 1.5 })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.layer_stroke_scale.get("roads").copied(), Some(1.5));
    }

    #[test]
    fn inspector_includes_layer_weight_slider() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_INSPECTION, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("gis2d-play-inspector.weight."));
    }

    #[test]
    fn window_measures_include_render_mode_and_layers_group() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let measures = app.window_measures(&document, &ViewState::default());
        let window = measures.get(GIS2D_PLAY_WINDOW_MAIN).expect("main window measures");
        let json = serde_json::to_string(window).unwrap();
        assert!(json.contains("gis2d-play-window.render-mode"));
        assert!(json.contains("gis2d-play-window.layers"));
        assert!(json.contains("gis2d-play-window.layer-weights"));
    }

    #[test]
    fn patch_route_updates_matching_route_field() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "patchRoute",
            Some(&json!({
                "routeId": "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0",
                "field": "label",
                "value": "Renamed Route",
            })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let descriptor: Value = serde_json::from_str(&next.runtime.map_fixture_json).unwrap();
        let route = descriptor["routes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|route| route["id"] == "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0")
            .unwrap();
        assert_eq!(route["label"], "Renamed Route");
    }
}
//#endregion 🧪Tests
