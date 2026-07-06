//! 🖼️ Raster plugin — declarative raster board bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_canvas_2d_scene, build_raster_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical,
    ui_text, App, Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, RasterScene, UiInspectorFieldGroup,
    UiNode, UiSectionNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER, create_default_layout,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::RgbaImage;
use std::collections::HashMap;
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
struct RasterImageAsset {
    mime: String,
    data: String,
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
    #[serde(default)]
    assets: HashMap<String, RasterImageAsset>,
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
        assets: HashMap::new(),
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

fn layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id
        .strip_prefix(&format!("{RASTER_TREE_PREFIX}."))
        .and_then(|rest| rest.split('.').nth(1))
        .map(str::to_string)
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

fn checkerboard_rgba(width: u32, height: u32) -> Vec<u8> {
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let v = if ((x / 16) + (y / 16)) % 2 == 0 { 220u8 } else { 180u8 };
            rgba[idx] = v;
            rgba[idx + 1] = v;
            rgba[idx + 2] = v;
            rgba[idx + 3] = 255;
        }
    }
    rgba
}

fn decode_asset_rgba(asset: &RasterImageAsset) -> Option<RgbaImage> {
    let bytes = BASE64.decode(asset.data.as_bytes()).ok()?;
    image::load_from_memory(&bytes).ok().map(|img| img.to_rgba8())
}

fn layer_pixel_rgba(document: &RasterDocument, layer: &RasterLayerNode) -> Option<(u32, u32, Vec<u8>)> {
    let RasterLayerNode::Pixel {
        width,
        height,
        image_key,
        ..
    } = layer
    else {
        return None;
    };
    let width = width.unwrap_or(512);
    let height = height.unwrap_or(512);
    if let Some(key) = image_key {
        if let Some(asset) = document.assets.get(key) {
            if let Some(image) = decode_asset_rgba(asset) {
                return Some((image.width(), image.height(), image.into_raw()));
            }
        }
    }
    Some((width, height, checkerboard_rgba(width, height)))
}

fn blit_rgba(
    canvas: &mut [u8],
    canvas_w: u32,
    canvas_h: u32,
    src: &[u8],
    src_w: u32,
    src_h: u32,
    dst_x: i32,
    dst_y: i32,
    opacity: f32,
) {
    for y in 0..src_h {
        for x in 0..src_w {
            let dst_xp = dst_x + x as i32;
            let dst_yp = dst_y + y as i32;
            if dst_xp < 0 || dst_yp < 0 || dst_xp >= canvas_w as i32 || dst_yp >= canvas_h as i32 {
                continue;
            }
            let src_idx = ((y * src_w + x) * 4) as usize;
            let dst_idx = ((dst_yp as u32 * canvas_w + dst_xp as u32) * 4) as usize;
            let alpha = (src[src_idx + 3] as f32 / 255.0) * opacity;
            if alpha <= 0.0 {
                continue;
            }
            for channel in 0..3 {
                let dst = canvas[dst_idx + channel] as f32;
                let src_px = src[src_idx + channel] as f32;
                canvas[dst_idx + channel] = (dst * (1.0 - alpha) + src_px * alpha) as u8;
            }
            canvas[dst_idx + 3] = 255;
        }
    }
}

fn flatten_raster_layers(layers: &[RasterLayerNode]) -> Vec<&RasterLayerNode> {
    let mut out = Vec::new();
    fn visit<'a>(layers: &'a [RasterLayerNode], out: &mut Vec<&'a RasterLayerNode>) {
        for layer in layers {
            out.push(layer);
            if let RasterLayerNode::Group { children, .. } = layer {
                visit(children, out);
            }
        }
    }
    visit(layers, &mut out);
    out
}

fn composite_raster_pixels(document: &RasterDocument) -> (u32, u32, Vec<u8>) {
    let (width, height) = raster_document_bounds(document);
    let mut canvas = vec![255u8; (width * height * 4) as usize];
    for channel in canvas.chunks_exact_mut(4) {
        channel[0] = 32;
        channel[1] = 34;
        channel[2] = 40;
        channel[3] = 255;
    }
    for layer in flatten_raster_layers(&document.layers) {
        if !layer_visible(layer) {
            continue;
        }
        let RasterLayerNode::Pixel {
            opacity,
            transform,
            ..
        } = layer
        else {
            continue;
        };
        let Some((src_w, src_h, src)) = layer_pixel_rgba(document, layer) else {
            continue;
        };
        let dst_x = transform.x.round() as i32;
        let dst_y = transform.y.round() as i32;
        blit_rgba(
            &mut canvas,
            width,
            height,
            &src,
            src_w,
            src_h,
            dst_x,
            dst_y,
            *opacity as f32,
        );
    }
    (width, height, canvas)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterPlayEnvelope {
    #[serde(flatten)]
    document: RasterDocument,
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    undo_stack: Vec<RasterDocument>,
    #[serde(default)]
    redo_stack: Vec<RasterDocument>,
}

fn parse_envelope(document_json: &str) -> RasterPlayEnvelope {
    if let Ok(envelope) = serde_json::from_str::<RasterPlayEnvelope>(document_json) {
        return envelope;
    }
    let document: RasterDocument = serde_json::from_str(document_json).unwrap_or_else(|_| empty_raster_document());
    RasterPlayEnvelope {
        document,
        selected_ids: Vec::new(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
    }
}

fn set_document_op(envelope: &RasterPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn push_undo_raster(play: &mut RasterPlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}

fn patch_layer_field(document: &mut RasterDocument, layer_id: &str, field: &str, value: &Value) -> bool {
    update_layer_in_tree(&mut document.layers, layer_id, &mut |layer| match layer {
        RasterLayerNode::Pixel {
            name,
            visible,
            opacity,
            blend_mode,
            transform,
            width,
            height,
            ..
        } => {
            match field {
                "name" => *name = value.as_str().unwrap_or("").into(),
                "visible" => *visible = value.as_bool().unwrap_or(*visible),
                "opacity" => *opacity = value.as_f64().unwrap_or(*opacity as f64) as f32,
                "blendMode" => *blend_mode = value.as_str().unwrap_or(blend_mode).into(),
                "transformX" => transform.x = value.as_f64().unwrap_or(transform.x),
                "transformY" => transform.y = value.as_f64().unwrap_or(transform.y),
                "width" => *width = Some(value.as_u64().unwrap_or(width.unwrap_or(512) as u64) as u32),
                "height" => *height = Some(value.as_u64().unwrap_or(height.unwrap_or(512) as u64) as u32),
                _ => {}
            }
        }
        RasterLayerNode::Group {
            name,
            visible,
            opacity,
            blend_mode,
            transform,
            ..
        } => {
            match field {
                "name" => *name = value.as_str().unwrap_or("").into(),
                "visible" => *visible = value.as_bool().unwrap_or(*visible),
                "opacity" => *opacity = value.as_f64().unwrap_or(*opacity as f64) as f32,
                "blendMode" => *blend_mode = value.as_str().unwrap_or(blend_mode).into(),
                "transformX" => transform.x = value.as_f64().unwrap_or(transform.x),
                "transformY" => transform.y = value.as_f64().unwrap_or(transform.y),
                _ => {}
            }
        }
        RasterLayerNode::Adjustment {
            name,
            visible,
            opacity,
            blend_mode,
            adjustment_kind,
            ..
        } => {
            match field {
                "name" => *name = value.as_str().unwrap_or("").into(),
                "visible" => *visible = value.as_bool().unwrap_or(*visible),
                "opacity" => *opacity = value.as_f64().unwrap_or(*opacity as f64) as f32,
                "blendMode" => *blend_mode = value.as_str().unwrap_or(blend_mode).into(),
                "adjustmentKind" => *adjustment_kind = value.as_str().unwrap_or(adjustment_kind).into(),
                _ => {}
            }
        }
    })
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

fn selection_from_envelope(play: &RasterPlayEnvelope, view_state: &ViewState) -> Vec<String> {
    if !play.selected_ids.is_empty() {
        return play.selected_ids.clone();
    }
    selection_from_view(view_state)
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
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: Some(true),
        drag_data: None,
        items: nested,
        control: None,
        is_hidden: if layer_visible(layer) { None } else { Some(true) },
    }
}

fn render_layers_panel(document: &RasterDocument, play: &RasterPlayEnvelope, view_state: &ViewState) -> UiNode {
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
            hover_command: None,
            unhover_command: None,
            actions: None,
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
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
    ];
    let layer_items: Vec<UiTreeItemNode> = document.layers.iter().map(layer_tree_item).collect();
    let selected_ids: Vec<String> = selection_from_envelope(play, view_state)
        .iter()
        .filter_map(|id| find_layer(&document.layers, id).map(layer_row_id))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "raster-play-layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
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

fn render_masks_panel(document: &RasterDocument, play: &RasterPlayEnvelope, view_state: &ViewState) -> UiNode {
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
                    hover_command: None,
                    unhover_command: None,
                    actions: None,
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
            hover_command: None,
            unhover_command: None,
            actions: None,
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
            selection_from_envelope(play, view_state)
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
    let (width, height, pixels) = composite_raster_pixels(document);
    build_raster_scene(
        RASTER_PLAY_SURFACE_COMPOSITE,
        RASTER_PLAY_CONTROLLER_ID,
        RasterScene {
            width,
            height,
            pixels_base64: BASE64.encode(pixels),
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
        serde_json::to_string(&RasterPlayEnvelope {
            document: empty_raster_document(),
            selected_ids: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
        .expect("raster document json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<RasterPlayEnvelope>(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                    if let Ok(parsed) = serde_json::from_value::<RasterDocument>(next.clone()) {
                        play.document = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setBrushSize" => {
                if let Some(size) = args.and_then(|value| value.get("brushSize")).and_then(|value| value.as_f64()) {
                    play.document.brush_size = Some(size as f32);
                    return vec![set_document_op(&play)];
                }
            }
            "setBrushOpacity" => {
                if let Some(opacity) = args.and_then(|value| value.get("opacity")).and_then(|value| value.as_f64()) {
                    play.document.brush_opacity = Some(opacity as f32);
                    return vec![set_document_op(&play)];
                }
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    play.document.active_tool = Some(tool.into());
                    return vec![set_document_op(&play)];
                }
            }
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        play.document.camera = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
                if let Some(zoom) = args.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()) {
                    play.document.camera.zoom = zoom;
                    return vec![set_document_op(&play)];
                }
            }
            "setLayerVisible" | "toggleLayerVisible" => {
                if let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    let visible = args
                        .and_then(|value| value.get("visible"))
                        .and_then(|value| value.as_bool())
                        .unwrap_or_else(|| {
                            find_layer(&play.document.layers, target_id)
                                .map(|layer| !layer_visible(layer))
                                .unwrap_or(true)
                        });
                    update_layer_in_tree(&mut play.document.layers, target_id, &mut |layer| match layer {
                        RasterLayerNode::Pixel { visible: v, .. }
                        | RasterLayerNode::Group { visible: v, .. }
                        | RasterLayerNode::Adjustment { visible: v, .. } => *v = visible,
                    });
                    return vec![set_document_op(&play)];
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
                push_undo_raster(&mut play);
                play.document.layers.push(layer);
                return vec![set_document_op(&play)];
            }
            "deleteLayer" => {
                if let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    push_undo_raster(&mut play);
                    remove_layer_from_tree(&mut play.document.layers, target_id);
                    play.selected_ids.retain(|id| id != target_id);
                    return vec![set_document_op(&play)];
                }
            }
            "duplicateLayer" => {
                if let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    if let Some(layer) = find_layer(&play.document.layers, target_id).cloned() {
                        push_undo_raster(&mut play);
                        play.document.layers.push(clone_layer(&layer));
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setSelection" => {
                play.selected_ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                return vec![set_document_op(&play)];
            }
            "setHover" => {}
            "selectAll" => {
                play.selected_ids = flatten_raster_layers(&play.document.layers)
                    .into_iter()
                    .filter_map(|layer| match layer {
                        RasterLayerNode::Pixel { id, .. }
                        | RasterLayerNode::Group { id, .. }
                        | RasterLayerNode::Adjustment { id, .. } => Some(id.clone()),
                    })
                    .collect();
                return vec![set_document_op(&play)];
            }
            "patchLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if !layer_id.is_empty() && !field.is_empty() {
                    push_undo_raster(&mut play);
                    patch_layer_field(&mut play.document, layer_id, field, &value);
                    return vec![set_document_op(&play)];
                }
            }
            "patchLayers" => {
                let layer_ids: Vec<String> = args
                    .and_then(|value| value.get("layerIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if !field.is_empty() {
                    push_undo_raster(&mut play);
                    for layer_id in layer_ids {
                        patch_layer_field(&mut play.document, &layer_id, field, &value);
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "dropLayerKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("pixel");
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
                let select_id = layer_node_id(&layer).to_string();
                push_undo_raster(&mut play);
                play.document.layers.push(layer);
                play.selected_ids = vec![select_id];
                return vec![set_document_op(&play)];
            }
            "moveLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str());
                let target_row_id = args
                    .and_then(|value| value.get("targetRowId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("raster-play-layers");
                let drop_position = args
                    .and_then(|value| value.get("dropPosition"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("after");
                let Some(layer_id) = layer_id else {
                    return Vec::new();
                };
                let Some(layer) = find_layer(&play.document.layers, layer_id).cloned() else {
                    return Vec::new();
                };
                push_undo_raster(&mut play);
                remove_layer_from_tree(&mut play.document.layers, layer_id);
                let parent_id = layer_id_from_tree_row_id(target_row_id).and_then(|id| {
                    find_layer(&play.document.layers, &id).and_then(|entry| {
                        if matches!(entry, RasterLayerNode::Group { .. }) {
                            Some(id)
                        } else {
                            None
                        }
                    })
                });
                let index = if drop_position == "before" {
                    0
                } else if let Some(ref parent) = parent_id {
                    find_layer(&play.document.layers, parent)
                        .and_then(|entry| match entry {
                            RasterLayerNode::Group { children, .. } => Some(children.len()),
                            _ => None,
                        })
                        .unwrap_or(0)
                } else {
                    play.document.layers.len()
                };
                insert_layer(&mut play.document.layers, parent_id.as_deref(), index, layer);
                return vec![set_document_op(&play)];
            }
            "undo" => {
                if let Some(previous) = play.undo_stack.pop() {
                    play.redo_stack.push(play.document.clone());
                    play.document = previous;
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.redo_stack.pop() {
                    play.undo_stack.push(play.document.clone());
                    play.document = next;
                    return vec![set_document_op(&play)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            RASTER_PLAY_BODY_COMPOSITE => render_composite_scene(&play.document),
            RASTER_PLAY_BODY_NAVIGATOR => render_navigator_scene(&play.document),
            RASTER_PLAY_BODY_LAYERS => render_layers_panel(&play.document, &play, view_state),
            RASTER_PLAY_BODY_MASKS => render_masks_panel(&play.document, &play, view_state),
            RASTER_PLAY_BODY_CATALOGUE => render_catalogue_panel(),
            RASTER_PLAY_BODY_PROPERTIES => render_properties_panel(&play.document, view_state),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖RasterApp

//#region 🔖Manifest
fn create_raster_app() -> App {
    App::from_builder(
        App::builder(RASTER_PLAY_APP_ID, "Raster").document(["semio", "raster"])
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
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
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

fn raster_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Raster", 1024, 1024)
}

fn register_raster_exports() {
    semio_framework_os::register_2d_svg_png_export_handlers("2d.raster", "raster", raster_document_json_to_svg);
}

fn raster_bundle() -> PluginBundle {
    register_raster_exports();
    PluginBundle::new("raster", "Raster", "0.1.0").register_app(create_raster_app(), || Box::new(RasterApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(raster_bundle()));

semio_framework_plugin::plugin_exports!();
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
    fn composite_scene_encodes_pixels() {
        let app = RasterApp;
        let document = SEMIO_EXAMPLE_JSON.to_string();
        let node = app.render(RASTER_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("raster"));
        assert!(!json.contains("\"pixelsBase64\":\"\""));
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
