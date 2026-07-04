//! 🖼️ Raster plugin — declarative raster board bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_canvas_2d_scene, build_raster_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical,
    ui_text, App, Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, RasterScene, UiInspectorFieldGroup,
    UiNode, UiSectionNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    create_default_layout,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const RASTER_PLAY_APP_ID: &str = "raster-play";
const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
const RASTER_PLAY_SURFACE_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_SURFACE_NAVIGATOR: &str = "raster.play.navigator";
const RASTER_PLAY_BODY_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_BODY_NAVIGATOR: &str = "raster.play.navigator";
const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
const RASTER_PLAY_BODY_CATALOGUE: &str = "raster.play.catalogue";
const RASTER_PLAY_BODY_MASKS: &str = "raster.play.masks";
const RASTER_PLAY_BODY_PROPERTIES: &str = "raster.play.properties";
const RASTER_PLAY_WINDOW_COMPOSITE: &str = "raster-composite";
const RASTER_PLAY_WINDOW_NAVIGATOR: &str = "raster-navigator";
const RASTER_PLAY_MASKS_TAB_ID: &str = "raster.panel.masks";
const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";
const RASTER_TREE_PREFIX: &str = "raster-play-layers";

const SEMIO_EXAMPLE_JSON: &str = include_str!("../../example/semio.raster.json");

static RASTER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterCamera {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "default_zoom")]
    zoom: f64,
}

fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct RasterTransform {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "one_f64")]
    scale_x: f64,
    #[serde(default = "one_f64")]
    scale_y: f64,
    #[serde(default)]
    rotation: f64,
}

fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterLayerMask {
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default = "default_true")]
    linked: bool,
    #[serde(default)]
    invert: bool,
    width: Option<u32>,
    height: Option<u32>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum RasterLayerNode {
    #[serde(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[serde(default)]
        transform: RasterTransform,
        mask: Option<RasterLayerMask>,
        width: Option<u32>,
        height: Option<u32>,
        image_key: Option<String>,
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[serde(default)]
        transform: RasterTransform,
        mask: Option<RasterLayerMask>,
        children: Vec<RasterLayerNode>,
    },
    #[serde(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[serde(default)]
        transform: RasterTransform,
        adjustment_kind: String,
    },
}

fn one_f32() -> f32 {
    1.0
}

fn default_blend() -> String {
    "normal".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterDocument {
    schema: String,
    id: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default = "default_camera")]
    camera: RasterCamera,
    #[serde(default)]
    layers: Vec<RasterLayerNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brush_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brush_opacity: Option<f32>,
}

fn default_camera() -> RasterCamera {
    RasterCamera {
        x: 0.0,
        y: 0.0,
        zoom: 1.0,
    }
}

fn create_raster_id(prefix: &str) -> String {
    let next = RASTER_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

fn create_pixel_layer(name: &str, width: u32, height: u32) -> RasterLayerNode {
    RasterLayerNode::Pixel {
        id: create_raster_id("layer"),
        name: name.into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        mask: None,
        width: Some(width),
        height: Some(height),
        image_key: None,
    }
}

fn empty_raster_document() -> RasterDocument {
    RasterDocument {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        title: Some("Untitled".into()),
        camera: default_camera(),
        layers: vec![create_pixel_layer("Background", 512, 512)],
        active_tool: Some("selectMarquee".into()),
        brush_size: Some(24.0),
        brush_opacity: Some(1.0),
    }
}

fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    let id = layer_node_id(layer);
    format!("{RASTER_TREE_PREFIX}.{segment}.{id}")
}

fn mask_row_id(target_id: &str) -> String {
    format!("{RASTER_TREE_PREFIX}.mask.{target_id}")
}

fn layer_node_id(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { id, .. }
        | RasterLayerNode::Group { id, .. }
        | RasterLayerNode::Adjustment { id, .. } => id,
    }
}

fn layer_name(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { name, .. }
        | RasterLayerNode::Group { name, .. }
        | RasterLayerNode::Adjustment { name, .. } => name,
    }
}

fn layer_visible(layer: &RasterLayerNode) -> bool {
    match layer {
        RasterLayerNode::Pixel { visible, .. }
        | RasterLayerNode::Group { visible, .. }
        | RasterLayerNode::Adjustment { visible, .. } => *visible,
    }
}

fn find_layer<'a>(layers: &'a [RasterLayerNode], target_id: &str) -> Option<&'a RasterLayerNode> {
    for layer in layers {
        if layer_node_id(layer) == target_id {
            return Some(layer);
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            if let Some(found) = find_layer(children, target_id) {
                return Some(found);
            }
        }
    }
    None
}

fn update_layer_in_tree(
    layers: &mut [RasterLayerNode],
    target_id: &str,
    mutator: &mut impl FnMut(&mut RasterLayerNode),
) -> bool {
    for layer in layers.iter_mut() {
        if layer_node_id(layer) == target_id {
            mutator(layer);
            return true;
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            if update_layer_in_tree(children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

fn remove_layer_from_tree(layers: &mut Vec<RasterLayerNode>, target_id: &str) -> bool {
    if let Some(index) = layers.iter().position(|layer| layer_node_id(layer) == target_id) {
        layers.remove(index);
        return true;
    }
    for layer in layers.iter_mut() {
        if let RasterLayerNode::Group { children, .. } = layer {
            if remove_layer_from_tree(children, target_id) {
                return true;
            }
        }
    }
    false
}

fn insert_layer(
    layers: &mut Vec<RasterLayerNode>,
    parent_id: Option<&str>,
    index: usize,
    layer: RasterLayerNode,
) {
    if let Some(parent_id) = parent_id {
        for node in layers.iter_mut() {
            if let RasterLayerNode::Group { id, children, .. } = node {
                if id == parent_id {
                    let index = index.min(children.len());
                    children.insert(index, layer);
                    return;
                }
                insert_layer(children, Some(parent_id), index, layer.clone());
            }
        }
        return;
    }
    let index = index.min(layers.len());
    layers.insert(index, layer);
}

fn clone_layer(layer: &RasterLayerNode) -> RasterLayerNode {
    match layer {
        RasterLayerNode::Pixel {
            name,
            visible,
            opacity,
            blend_mode,
            transform,
            mask,
            width,
            height,
            image_key,
            ..
        } => RasterLayerNode::Pixel {
            id: create_raster_id("layer"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            width: *width,
            height: *height,
            image_key: image_key.clone(),
        },
        RasterLayerNode::Group {
            name,
            visible,
            opacity,
            blend_mode,
            transform,
            mask,
            children,
            ..
        } => RasterLayerNode::Group {
            id: create_raster_id("group"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            children: children.iter().map(clone_layer).collect(),
        },
        RasterLayerNode::Adjustment {
            name,
            visible,
            opacity,
            blend_mode,
            transform,
            adjustment_kind,
            ..
        } => RasterLayerNode::Adjustment {
            id: create_raster_id("adjust"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            adjustment_kind: adjustment_kind.clone(),
        },
    }
}

fn raster_document_bounds(document: &RasterDocument) -> (u32, u32) {
    let mut max_x = 512u32;
    let mut max_y = 512u32;
    fn visit(layer: &RasterLayerNode, max_x: &mut u32, max_y: &mut u32) {
        if !layer_visible(layer) {
            return;
        }
        let (width, height, x, y) = match layer {
            RasterLayerNode::Pixel {
                width,
                height,
                transform,
                ..
            } => (
                width.unwrap_or(512),
                height.unwrap_or(512),
                transform.x,
                transform.y,
            ),
            RasterLayerNode::Group { transform, .. } => (512, 512, transform.x, transform.y),
            RasterLayerNode::Adjustment { transform, .. } => (256, 256, transform.x, transform.y),
        };
        *max_x = (*max_x).max((x + width as f64) as u32);
        *max_y = (*max_y).max((y + height as f64) as u32);
        if let RasterLayerNode::Group { children, .. } = layer {
            for child in children {
                visit(child, max_x, max_y);
            }
        }
    }
    for layer in &document.layers {
        visit(layer, &mut max_x, &mut max_y);
    }
    (max_x.max(1), max_y.max(1))
}
//#endregion 🔖Document

//#region 🔖Panels
fn play_cmd(controller_id: &str, command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: controller_id.into(),
        command: command.into(),
        args,
    }
}

fn selection_from_view(view_state: &ViewState) -> Vec<String> {
    view_state
        .selection_json
        .as_ref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| {
            value.as_array().map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect()
            })
        })
        .unwrap_or_default()
}

fn layer_tree_item(layer: &RasterLayerNode) -> UiTreeItemNode {
    let nested = match layer {
        RasterLayerNode::Group { children, .. } => {
            if children.is_empty() {
                None
            } else {
                Some(children.iter().map(layer_tree_item).collect())
            }
        }
        _ => None,
    };
    UiTreeItemNode {
        id: layer_row_id(layer),
        label: layer_name(layer).into(),
        description: Some(match layer {
            RasterLayerNode::Pixel { .. } => "pixel",
            RasterLayerNode::Group { .. } => "group",
            RasterLayerNode::Adjustment { .. } => "adjustment",
        }.into()),
        icon_id: Some(match layer {
            RasterLayerNode::Pixel { .. } => "image",
            RasterLayerNode::Group { .. } => "folder",
            RasterLayerNode::Adjustment { .. } => "sliders-horizontal",
        }.into()),
        selected: None,
        default_open: Some(matches!(layer, RasterLayerNode::Group { .. })),
        command: Some(play_cmd(
            RASTER_PLAY_CONTROLLER_ID,
            "setSelection",
            Some(json!({ "ids": [layer_node_id(layer)] })),
        )),
        draggable: Some(true),
        drag_data: None,
        items: nested,
        control: None,
        is_hidden: if layer_visible(layer) { None } else { Some(true) },
    }
}

fn render_layers_panel(document: &RasterDocument, view_state: &ViewState) -> UiNode {
    let toolbar = vec![
        UiTreeItemNode {
            id: "raster-play-layers.add.pixel".into(),
            label: "Add Pixel".into(),
            description: None,
            icon_id: Some("image".into()),
            selected: None,
            default_open: None,
            command: Some(play_cmd(
                RASTER_PLAY_CONTROLLER_ID,
                "addLayer",
                Some(json!({ "kind": "pixel" })),
            )),
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        UiTreeItemNode {
            id: "raster-play-layers.add.group".into(),
            label: "Add Group".into(),
            description: None,
            icon_id: Some("folder-plus".into()),
            selected: None,
            default_open: None,
            command: Some(play_cmd(
                RASTER_PLAY_CONTROLLER_ID,
                "addLayer",
                Some(json!({ "kind": "group" })),
            )),
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
    ];
    let layer_items: Vec<UiTreeItemNode> = document.layers.iter().map(layer_tree_item).collect();
    let selected_ids: Vec<String> = selection_from_view(view_state)
        .iter()
        .filter_map(|id| find_layer(&document.layers, id).map(layer_row_id))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "raster-play-layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
            default_open: Some(true),
            items: [toolbar, layer_items].concat(),
        }],
        selected_ids: Some(selected_ids),
        highlighted_ids: None,
        selection_change: Some(play_cmd(
            RASTER_PLAY_CONTROLLER_ID,
            "setSelection",
            None,
        )),
    })
}

fn render_masks_panel(document: &RasterDocument, view_state: &ViewState) -> UiNode {
    let mut items = Vec::new();
    fn collect_masks(layer: &RasterLayerNode, items: &mut Vec<UiTreeItemNode>) {
        if let RasterLayerNode::Pixel { id, name, mask, .. }
        | RasterLayerNode::Group { id, name, mask, .. } = layer
        {
            if mask.as_ref().is_some_and(|mask| mask.enabled) {
                items.push(UiTreeItemNode {
                    id: mask_row_id(id),
                    label: format!("{name} mask"),
                    description: Some("mask".into()),
                    icon_id: Some("scan".into()),
                    selected: None,
                    default_open: None,
                    command: Some(play_cmd(
                        RASTER_PLAY_CONTROLLER_ID,
                        "setSelection",
                        Some(json!({ "ids": [id] })),
                    )),
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                });
            }
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            for child in children {
                collect_masks(child, items);
            }
        }
    }
    for layer in &document.layers {
        collect_masks(layer, &mut items);
    }
    if items.is_empty() {
        items.push(UiTreeItemNode {
            id: "raster-play-masks.empty".into(),
            label: "No masks".into(),
            description: None,
            icon_id: Some("scan".into()),
            selected: None,
            default_open: None,
            command: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        });
    }
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "raster-play-masks".into(),
            label: Some("Masks".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: Some(
            selection_from_view(view_state)
                .iter()
                .map(|id| mask_row_id(id))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn render_catalogue_panel() -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "raster-catalogue".into(),
        label: Some("Layer kinds".into()),
        default_open: Some(true),
        children: vec![
            ui_text("pixel — paintable bitmap layer"),
            ui_text("group — nested layer stack"),
            ui_text("adjustment — non-destructive filter"),
        ],
    }])
}

fn render_properties_panel(document: &RasterDocument, view_state: &ViewState) -> UiNode {
    let selected = selection_from_view(view_state);
    let layers: Vec<&RasterLayerNode> = selected
        .iter()
        .filter_map(|id| find_layer(&document.layers, id))
        .collect();
    if layers.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", document.schema)),
            ui_text(format!(
                "Brush: {} @ {}",
                document.brush_size.unwrap_or(24.0),
                document.brush_opacity.unwrap_or(1.0)
            )),
        ]);
    }
    let names: Vec<String> = layers.iter().map(|layer| layer_name(*layer).into()).collect();
    let opacities: Vec<f64> = layers
        .iter()
        .map(|layer| match layer {
            RasterLayerNode::Pixel { opacity, .. }
            | RasterLayerNode::Group { opacity, .. }
            | RasterLayerNode::Adjustment { opacity, .. } => *opacity as f64,
        })
        .collect();
    let mixed_name = ui_inspector_mixed_text(&names);
    let mixed_opacity = ui_inspector_mixed_number(&opacities);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "raster-properties.layer".into(),
        label: "Layer".into(),
        default_open: Some(true),
        fields: vec![
            ui_inspector_readonly_field(
                "raster-properties.name",
                "Name",
                mixed_name.placeholder.unwrap_or(mixed_name.value),
            ),
            ui_inspector_readonly_field(
                "raster-properties.opacity",
                "Opacity",
                if mixed_opacity.uniform {
                    mixed_opacity.value.to_string()
                } else {
                    "Mixed".into()
                },
            ),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Scenes
fn render_composite_scene(document: &RasterDocument) -> UiNode {
    let (width, height) = raster_document_bounds(document);
    build_raster_scene(
        RASTER_PLAY_SURFACE_COMPOSITE,
        RASTER_PLAY_CONTROLLER_ID,
        RasterScene {
            width,
            height,
            pixels_base64: String::new(),
        },
    )
}

fn render_navigator_scene(document: &RasterDocument) -> UiNode {
    build_canvas_2d_scene(
        RASTER_PLAY_SURFACE_NAVIGATOR,
        RASTER_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x: document.camera.x,
            camera_y: document.camera.y,
            zoom: document.camera.zoom,
            layers_json: serde_json::to_string(&document.layers).unwrap_or_else(|_| "[]".into()),
        },
    )
}
//#endregion 🔖Scenes

//#region 🔖RasterApp
struct RasterApp;

impl PluginApp for RasterApp {
    fn app_id(&self) -> &str {
        RASTER_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&empty_raster_document()).expect("raster document json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut document: RasterDocument =
            serde_json::from_str(document_json).unwrap_or_else(|_| empty_raster_document());
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        document = parsed;
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            "setBrushSize" => {
                if let Some(size) = args.and_then(|value| value.get("brushSize")).and_then(|value| value.as_f64()) {
                    document.brush_size = Some(size as f32);
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setBrushOpacity" => {
                if let Some(opacity) = args.and_then(|value| value.get("opacity")).and_then(|value| value.as_f64()) {
                    document.brush_opacity = Some(opacity as f32);
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    document.active_tool = Some(tool.into());
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        document.camera = parsed;
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
                if let Some(zoom) = args.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()) {
                    document.camera.zoom = zoom;
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setLayerVisible" | "toggleLayerVisible" => {
                if let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    let visible = args
                        .and_then(|value| value.get("visible"))
                        .and_then(|value| value.as_bool())
                        .unwrap_or_else(|| {
                            find_layer(&document.layers, target_id)
                                .map(|layer| !layer_visible(layer))
                                .unwrap_or(true)
                        });
                    update_layer_in_tree(&mut document.layers, target_id, &mut |layer| match layer {
                        RasterLayerNode::Pixel { visible: v, .. }
                        | RasterLayerNode::Group { visible: v, .. }
                        | RasterLayerNode::Adjustment { visible: v, .. } => *v = visible,
                    });
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "addLayer" => {
                let kind = args
                    .and_then(|value| value.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("pixel");
                let layer = match kind {
                    "group" => RasterLayerNode::Group {
                        id: create_raster_id("group"),
                        name: "Group".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        children: Vec::new(),
                    },
                    "adjustment" => RasterLayerNode::Adjustment {
                        id: create_raster_id("adjust"),
                        name: "Adjustment".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        adjustment_kind: "brightnessContrast".into(),
                    },
                    _ => create_pixel_layer("Layer", 512, 512),
                };
                document.layers.push(layer);
                return vec![json!({ "op": "setDocument", "document": document }).to_string()];
            }
            "deleteLayer" => {
                if let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    remove_layer_from_tree(&mut document.layers, target_id);
                    return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "duplicateLayer" => {
                if let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    if let Some(layer) = find_layer(&document.layers, target_id).cloned() {
                        document.layers.push(clone_layer(&layer));
                        return vec![json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            "setSelection" | "setHover" | "selectAll" => {}
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let document: RasterDocument =
            serde_json::from_str(document_json).unwrap_or_else(|_| empty_raster_document());
        match body_key {
            RASTER_PLAY_BODY_COMPOSITE => render_composite_scene(&document),
            RASTER_PLAY_BODY_NAVIGATOR => render_navigator_scene(&document),
            RASTER_PLAY_BODY_LAYERS => render_layers_panel(&document, view_state),
            RASTER_PLAY_BODY_MASKS => render_masks_panel(&document, view_state),
            RASTER_PLAY_BODY_CATALOGUE => render_catalogue_panel(),
            RASTER_PLAY_BODY_PROPERTIES => render_properties_panel(&document, view_state),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖RasterApp

//#region 🔖Manifest
fn create_raster_app() -> App {
    App::from_builder(
        App::builder(RASTER_PLAY_APP_ID, "Raster")
            .icon_id("raster")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(RASTER_PLAY_WINDOW_COMPOSITE, "Composite", RASTER_PLAY_BODY_COMPOSITE)
            .window_kind(RASTER_PLAY_WINDOW_NAVIGATOR, "Navigator", RASTER_PLAY_BODY_NAVIGATOR)
            .default_layout(create_default_layout(
                &[RASTER_PLAY_WINDOW_COMPOSITE.into(), RASTER_PLAY_WINDOW_NAVIGATOR.into()],
                "row",
                Some(&[72.0, 28.0]),
                Some(&["Composite".into(), "Navigator".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                RASTER_PLAY_BODY_LAYERS,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                RASTER_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(RASTER_PLAY_MASKS_TAB_ID, "Masks", "workbench", RASTER_PLAY_BODY_MASKS)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                RASTER_PLAY_BODY_PROPERTIES,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("empty", "Empty", serde_json::to_string(&empty_raster_document()).unwrap())
    .example("semio", "Semio", SEMIO_EXAMPLE_JSON)
    .program("raster", "Raster", "2d.raster")
}

fn raster_bundle() -> PluginBundle {
    PluginBundle::new("raster", "Raster", "0.1.0").register_app(create_raster_app(), || Box::new(RasterApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(raster_bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_raster_scene() {
        let app = RasterApp;
        let document = serde_json::to_string(&empty_raster_document()).unwrap();
        let node = app.render(RASTER_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("raster"));
    }

    #[test]
    fn renders_navigator_canvas() {
        let app = RasterApp;
        let document = serde_json::to_string(&empty_raster_document()).unwrap();
        let node = app.render(RASTER_PLAY_BODY_NAVIGATOR, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn parses_semio_example_document() {
        let document: RasterDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).expect("semio raster json");
        assert!(!document.layers.is_empty());
    }

    #[test]
    fn renders_layers_tree() {
        let app = RasterApp;
        let document = SEMIO_EXAMPLE_JSON.to_string();
        let node = app.render(RASTER_PLAY_BODY_LAYERS, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Backdrop"));
    }

    #[test]
    fn add_layer_command() {
        let mut app = RasterApp;
        let document = serde_json::to_string(&empty_raster_document()).unwrap();
        let ops = app.handle_command(
            "addLayer",
            Some(&json!({ "kind": "group" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        assert!(ops[0].contains("\"kind\":\"group\""));
    }
}
//#endregion 🧪Tests
