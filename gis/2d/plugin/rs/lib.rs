//! 🗺️ GIS 2D plugin — GIS map play app bundled as a hot-swappable WASM component.

use gis_2d::{
    empty_gis_map_projection, gis_map_lod_scale_json, projection, GisMapDocument, GisMapEnvelope, GisMapOp,
    GisMapStore, MapHost, GIS_MAP_LOD_MODE_AUTOMATIC, GIS_MAP_SCHEMA,
};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_inspector_groups_to_tree, ui_inspector_mixed_toggle,
    ui_inspector_readonly_field, ui_text, App, Canvas2dScene, CommandDescriptor, PluginApp,
    PluginBundle, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiSelectItem, UiSelectNode,
    UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::LazyLock;
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentVcsCommand};

//#region 🔖Constants
const GIS2D_PLAY_APP_ID: &str = "gis2d-play";
const GIS2D_PLAY_SURFACE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_COMPOSITE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_HIERARCHY: &str = "gis2d.play.hierarchy";
const GIS2D_PLAY_BODY_CATALOGUE: &str = "gis2d.play.catalogue";
const GIS2D_PLAY_BODY_INSPECTION: &str = "gis2d.play.inspection";
const GIS2D_PLAY_WINDOW_MAIN: &str = "gis2d-main";

const REUSE_MAP_EXAMPLE_JSON: &str = include_str!("../../example/reuse.map.gis.json");

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GisMapCanvasLayer {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
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

fn parse_feature_hit(hit_json: &str) -> Value {
    let hit: Value = serde_json::from_str(hit_json).unwrap_or(Value::Null);
    let (Some(kind), Some(id)) = (
        hit.get("kind").and_then(|value| value.as_str()),
        hit.get("id").and_then(|value| value.as_str()),
    ) else {
        return json!({ "positions": [], "routes": [] });
    };
    match kind {
        "position" => json!({ "positions": [id], "routes": [] }),
        "route" => json!({ "positions": [], "routes": [id] }),
        _ => json!({ "positions": [], "routes": [] }),
    }
}

fn map_canvas_layers(play: &Gis2dPlayEnvelope) -> String {
    let mut host = map_host_from_play(play);
    host.prepare_visible_tiles();
    let mut layers: Vec<GisMapCanvasLayer> = Vec::new();
    if layer_visible(&play.runtime, "raster") {
        let tiles: Vec<Value> = serde_json::from_str(&host.visible_tiles_json()).unwrap_or_default();
        for tile in tiles {
            let (Some(z), Some(x), Some(y)) = (
                tile.get("z").and_then(|value| value.as_u64()).map(|value| value as u32),
                tile.get("x").and_then(|value| value.as_u64()).map(|value| value as u32),
                tile.get("y").and_then(|value| value.as_u64()).map(|value| value as u32),
            ) else {
                continue;
            };
            let rect = projection::tile_world_rect(z, x, y);
            layers.push(GisMapCanvasLayer {
                id: format!("tile-{z}-{x}-{y}"),
                kind: "tile".into(),
                name: format!("{z}/{x}/{y}"),
                x: rect.x0(),
                y: rect.y0(),
                width: rect.width(),
                height: rect.height(),
            });
        }
    }
    for position in host.positions.values() {
        if !layer_visible(&play.runtime, "positions") {
            continue;
        }
        let world = projection::lonlat_to_world(position.lon, position.lat);
        layers.push(GisMapCanvasLayer {
            id: position.id.clone(),
            kind: "position".into(),
            name: position.label.clone().or_else(|| position.name.clone()).unwrap_or_else(|| position.id.clone()),
            x: world.x,
            y: world.y,
            width: 16.0,
            height: 16.0,
        });
    }
    if layer_visible(&play.runtime, "routes") {
        for route in host.routes.values() {
            for (index, point) in route.points.iter().enumerate() {
                if point.len() < 2 {
                    continue;
                }
                let world = projection::lonlat_to_world(point[0], point[1]);
                layers.push(GisMapCanvasLayer {
                    id: format!("{}-{}", route.id, index),
                    kind: "route".into(),
                    name: route.id.clone(),
                    x: world.x,
                    y: world.y,
                    width: 8.0,
                    height: 8.0,
                });
            }
        }
    }
    if layers.is_empty() {
        return canvas_layers(play);
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
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

fn default_map_layers() -> Vec<Value> {
    GIS_MAP_LAYER_IDS
        .iter()
        .enumerate()
        .map(|(index, (id, label, _))| {
            json!({
                "id": id,
                "kind": "map-layer",
                "name": label,
                "x": (index % 4) as f64 * 140.0,
                "y": (index / 4) as f64 * 90.0,
                "width": 120.0,
                "height": 72.0,
            })
        })
        .collect()
}

fn canvas_layers(play: &Gis2dPlayEnvelope) -> String {
    let projection = materialized_projection(play);
    let source = if projection.layers.is_empty() {
        default_map_layers()
    } else {
        projection.layers.clone()
    };
    let layers: Vec<GisMapCanvasLayer> = source
        .iter()
        .filter_map(|layer| {
            let id = layer.get("id").and_then(|value| value.as_str())?;
            if !layer_visible(&play.runtime, id) {
                return None;
            }
            Some(GisMapCanvasLayer {
                id: id.into(),
                kind: layer.get("kind").and_then(|value| value.as_str()).unwrap_or("map-layer").into(),
                name: layer.get("name").and_then(|value| value.as_str()).unwrap_or(id).into(),
                x: layer.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0),
                y: layer.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0),
                width: layer.get("width").and_then(|value| value.as_f64()).unwrap_or(120.0),
                height: layer.get("height").and_then(|value| value.as_f64()).unwrap_or(72.0),
            })
        })
        .collect();
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
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

fn build_hierarchy_tree(play: &Gis2dPlayEnvelope) -> UiNode {
    let projection = materialized_projection(play);
    let layer_items: Vec<UiTreeItemNode> = if projection.layers.is_empty() {
        GIS_MAP_LAYER_IDS
            .iter()
            .map(|(id, label, icon)| {
                tree_item(
                    format!("gis2d-play-hierarchy.layer.{id}"),
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
                    format!("gis2d-play-hierarchy.layer.{id}"),
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
            id: "gis2d-play-hierarchy.layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
            default_open: Some(true),
            items: layer_items,
        }],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .map(|id| format!("gis2d-play-hierarchy.layer.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(gis2d_cmd("setSelection", None)),
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
    })
}

fn map_view_field_group(play: &Gis2dPlayEnvelope) -> UiInspectorFieldGroup {
    let lod_items: Vec<UiSelectItem> = std::iter::once(UiSelectItem {
        value: GIS_MAP_LOD_MODE_AUTOMATIC.into(),
        label: "Automatic".into(),
    })
    .chain(
        serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|lod| {
                let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
                let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
                Some(UiSelectItem { value: id, label: name })
            }),
    )
    .collect();
    let selection: Value = serde_json::from_str(&play.runtime.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let selected_count = selection.get("positions").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
        + selection.get("routes").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0);
    UiInspectorFieldGroup {
        id: "gis2d-play-inspector.map-view".into(),
        label: "Map View".into(),
        default_open: Some(true),
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.render-mode".into(),
                label: "Render Mode".into(),
                child: UiControlNode::Select(UiSelectNode {
                    id: "gis2d-play-inspector.render-mode.select".into(),
                    value: play.runtime.render_mode.clone(),
                    items: vec![
                        UiSelectItem { value: "image".into(), label: "Image".into() },
                        UiSelectItem { value: "vector".into(), label: "Vector".into() },
                        UiSelectItem { value: "combined".into(), label: "Combined".into() },
                    ],
                    placeholder: None,
                    on_change: gis2d_cmd("setRenderMode", None),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.vector-style".into(),
                label: "Vector Style".into(),
                child: UiControlNode::Select(UiSelectNode {
                    id: "gis2d-play-inspector.vector-style.select".into(),
                    value: play.runtime.vector_style.clone(),
                    items: vec![
                        UiSelectItem { value: "colored".into(), label: "Colored".into() },
                        UiSelectItem { value: "figureGround".into(), label: "Figure Ground".into() },
                        UiSelectItem { value: "invertedFigure".into(), label: "Inverted Figure".into() },
                    ],
                    placeholder: None,
                    on_change: gis2d_cmd("setVectorStyle", None),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.lod-mode".into(),
                label: "LOD Mode".into(),
                child: UiControlNode::Select(UiSelectNode {
                    id: "gis2d-play-inspector.lod-mode.select".into(),
                    value: play.runtime.lod_mode.clone(),
                    items: lod_items,
                    placeholder: None,
                    on_change: gis2d_cmd("setLodMode", None),
                }),
            }),
            ui_inspector_readonly_field("gis2d-play-inspector.feature-selection", "Selected Features", selected_count.to_string()),
        ],
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
                    child: UiControlNode::Toggle(UiToggleNode {
                        id: "gis2d-play-inspector.visible.toggle".into(),
                        icon_id: "eye".into(),
                        pressed: mixed.uniform && mixed.pressed,
                        text: None,
                        on_change: gis2d_cmd("toggleLayerVisibility", Some(json!({ "layerId": layer_id }))),
                    }),
                }),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_canvas(play: &Gis2dPlayEnvelope) -> UiNode {
    let host = map_host_from_play(play);
    let camera: Value = serde_json::from_str(&play.runtime.camera_json).unwrap_or(json!({"x":0,"y":0,"zoom":1}));
    build_canvas_2d_scene(
        GIS2D_PLAY_SURFACE,
        GIS2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.get("x").and_then(|value| value.as_f64()).unwrap_or(host.camera.x),
            camera_y: camera.get("y").and_then(|value| value.as_f64()).unwrap_or(host.camera.y),
            zoom: camera.get("zoom").and_then(|value| value.as_f64()).unwrap_or(host.camera.zoom),
            layers_json: map_canvas_layers(play),
        },
    )
}
//#endregion 🔖Render

//#region 🔖Gis2dPlayApp
struct Gis2dPlayApp;

impl PluginApp for Gis2dPlayApp {
    fn app_id(&self) -> &str {
        GIS2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("gis2d envelope json")
    }

    fn handle_command(
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
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    play.runtime.camera_json = camera.to_string();
                    return vec![set_document_op(&play)];
                }
            }
            "canvasWheel" => {
                let delta = args.and_then(|value| value.get("deltaY")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let sx = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(400.0);
                let sy = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(300.0);
                let mut host = map_host_from_play(&play);
                host.wheel_screen(sx, sy, delta);
                play.runtime.camera_json = host.camera_json();
                return vec![set_document_op(&play)];
            }
            "canvasPointerDown" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let button = args.and_then(|value| value.get("button")).and_then(|value| value.as_u64()).unwrap_or(0) as u8;
                let mut host = map_host_from_play(&play);
                host.pointer_down_screen(x, y, button);
                play.runtime.camera_json = host.camera_json();
                return vec![set_document_op(&play)];
            }
            "canvasPointerMove" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let mut host = map_host_from_play(&play);
                host.pointer_move_screen(x, y);
                play.runtime.camera_json = host.camera_json();
                return vec![set_document_op(&play)];
            }
            "canvasPointerUp" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let mut host = map_host_from_play(&play);
                host.pointer_up_screen(x, y);
                play.runtime.camera_json = host.camera_json();
                return vec![set_document_op(&play)];
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
            "hitTestFeature" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let host = map_host_from_play(&play);
                play.runtime.feature_selection_json = parse_feature_hit(&host.hit_test_feature_json(x, y)).to_string();
                return vec![set_document_op(&play)];
            }
            "setFeatureSelection" => {
                let selection = json!({
                    "positions": args.and_then(|value| value.get("positions")).cloned().unwrap_or_else(|| json!([])),
                    "routes": args.and_then(|value| value.get("routes")).cloned().unwrap_or_else(|| json!([])),
                });
                let mut host = map_host_from_play(&play);
                if host.set_selection_json(&selection.to_string()).is_ok() {
                    play.runtime.feature_selection_json = selection.to_string();
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
            GIS2D_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&play),
            GIS2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(&play),
            GIS2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Gis2dPlayApp

//#region 🔖AppFactory
fn create_gis2d_app() -> App {
    App::from_builder(
        App::builder(GIS2D_PLAY_APP_ID, "GIS 2D").hierarchy(["semio", "gis", "2d"])
            .icon_id("gis2d")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(GIS2D_PLAY_WINDOW_MAIN, "Map", GIS2D_PLAY_BODY_COMPOSITE)
            .default_layout(create_default_layout(
                &[GIS2D_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Map".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                GIS2D_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                GIS2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                GIS2D_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("reuse-map", "Reuse Map", serde_json::to_string(&default_envelope()).unwrap())
    .program("gis2d", "GIS 2D", "map")
}

fn gis2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::map_points_svg(value, "GIS 2D")
}

fn register_gis2d_exports() {
    semio_framework_os::register_2d_svg_png_export_handlers("2d.map", "gis2d", gis2d_document_json_to_svg);
}

fn bundle() -> PluginBundle {
    register_gis2d_exports();
    PluginBundle::new("gis2d", "GIS 2D", "0.1.0").register_app(create_gis2d_app(), || Box::new(Gis2dPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_canvas_scene() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn hierarchy_lists_map_layers() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("gis2d-play-hierarchy.layer.raster"));
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
        let ops = app.handle_command(
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
        let ops = app.handle_command("setLayers", Some(&json!({ "layers": layers })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&next).layers.len(), 1);
    }

    #[test]
    fn toggle_layer_visibility_hides_layer() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
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
        let ops = app.handle_command("setRenderMode", Some(&json!({ "mode": "vector" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.render_mode, "vector");
        let document = serde_json::to_string(&next).unwrap();

        let ops = app.handle_command("setVectorStyle", Some(&json!({ "style": "figureGround" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.vector_style, "figureGround");
        let document = serde_json::to_string(&next).unwrap();

        let ops = app.handle_command("setLodMode", Some(&json!({ "mode": "city" })), &document, &ViewState::default());
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
        let ops = app.handle_command(
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
    fn hit_test_feature_with_no_match_clears_selection() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "hitTestFeature",
            Some(&json!({ "x": -99999.0, "y": -99999.0 })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let selection: Value = serde_json::from_str(&next.runtime.feature_selection_json).unwrap();
        assert_eq!(selection, json!({ "positions": [], "routes": [] }));
    }

    #[test]
    fn patch_route_updates_matching_route_field() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
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
