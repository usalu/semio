//! 🖼️ Raster plugin — declarative raster board bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind,
    build_raster_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical,
    ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor,
    AppLabelsOverlay, DocumentApp, DocumentView, PanelGroup, RasterScene, UtilityCategory, UtilityDefinition,
    UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    create_default_layout, SET_ACTIVE_UTILITY_ACTION_ID,
};
use raster::{
    empty_raster_projection, find_layer, flatten_raster_layers, layer_name, layer_node_id, layer_visible,
    RasterCamera, RasterImageAsset, RasterLayerNode, RasterLayerPatch, RasterOp,
    RasterProjection as RasterDocument, RasterTransform,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};

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
/// 🧰 Fallback utility when the host has not yet asserted a session active utility for the composite window.
const RASTER_DEFAULT_UTILITY: &str = "selectMarquee";

const SEMIO_EXAMPLE_JSON: &str = include_str!("../../example/semio.raster.json");

static RASTER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Document
/// 🎛️ Ephemeral view state (selection, hover, utility/brush settings, navigator viewport) held in the
/// app struct — never in the document — so it stays out of undo history and off the op channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterPlayRuntime {
    selected_ids: Vec<String>,
    hovered_id: Option<String>,
    brush_size: f32,
    brush_opacity: f32,
    composite_viewport: Option<RasterViewportSize>,
}

impl RasterPlayRuntime {
    fn new() -> Self {
        Self {
            selected_ids: Vec::new(),
            hovered_id: None,
            brush_size: 24.0,
            brush_opacity: 1.0,
            composite_viewport: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterViewportSize {
    width: f64,
    height: f64,
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

fn create_group_layer() -> RasterLayerNode {
    RasterLayerNode::Group {
        id: create_raster_id("group"),
        name: "Group".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        mask: None,
        children: Vec::new(),
    }
}

fn create_adjustment_layer() -> RasterLayerNode {
    RasterLayerNode::Adjustment {
        id: create_raster_id("adjust"),
        name: "Adjustment".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        adjustment_kind: "brightnessContrast".into(),
    }
}

fn create_layer_of_kind(kind: &str) -> RasterLayerNode {
    match kind {
        "group" => create_group_layer(),
        "adjustment" => create_adjustment_layer(),
        _ => create_pixel_layer("Layer", 512, 512),
    }
}

fn empty_raster_document() -> RasterDocument {
    let mut document = empty_raster_projection();
    document.id = "empty".into();
    document.layers = vec![create_pixel_layer("Background", 512, 512)];
    document
}

fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    format!("{RASTER_TREE_PREFIX}.{segment}.{}", layer_node_id(layer))
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

/// 📄 Duplicates a layer subtree with freshly minted ids (a new document node, not an op inverse).
fn clone_layer(layer: &RasterLayerNode) -> RasterLayerNode {
    match layer {
        RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, mask, width, height, image_key, .. } => {
            RasterLayerNode::Pixel {
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
            }
        }
        RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, mask, children, .. } => RasterLayerNode::Group {
            id: create_raster_id("group"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            children: children.iter().map(clone_layer).collect(),
        },
        RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, transform, adjustment_kind, .. } => {
            RasterLayerNode::Adjustment {
                id: create_raster_id("adjust"),
                name: format!("{name} copy"),
                visible: *visible,
                opacity: *opacity,
                blend_mode: blend_mode.clone(),
                transform: transform.clone(),
                adjustment_kind: adjustment_kind.clone(),
            }
        }
    }
}

/// 🩹 Builds a sparse {@link RasterLayerPatch} for a `patchLayer`/`patchLayers` field write.
fn layer_patch_for_field(field: &str, value: &Value, prior: &RasterLayerNode) -> Option<RasterLayerPatch> {
    let mut patch = RasterLayerPatch::default();
    let opacity_of = raster::layer_opacity(prior) as f64;
    match field {
        "name" => patch.name = Some(value.as_str().unwrap_or("").into()),
        "visible" => patch.visible = Some(value.as_bool().unwrap_or_else(|| !layer_visible(prior))),
        "opacity" => patch.opacity = Some(value.as_f64().unwrap_or(opacity_of) as f32),
        "blendMode" => patch.blend_mode = Some(value.as_str().unwrap_or("normal").into()),
        "transformX" => patch.transform_x = Some(value.as_f64().unwrap_or(0.0)),
        "transformY" => patch.transform_y = Some(value.as_f64().unwrap_or(0.0)),
        "width" => patch.width = Some(value.as_u64().unwrap_or(512) as u32),
        "height" => patch.height = Some(value.as_u64().unwrap_or(512) as u32),
        "adjustmentKind" => patch.adjustment_kind = Some(value.as_str().unwrap_or("brightnessContrast").into()),
        _ => return None,
    }
    Some(patch)
}
//#endregion 🔖Document

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the raster app; one field per label makes every locale combination compile-checked.
struct RasterLabels {
    masks: &'static str,
    no_masks: &'static str,
    mask_suffix: &'static str,
    add_pixel: &'static str,
    add_group: &'static str,
    layer_kinds: &'static str,
    layer: &'static str,
    catalogue_pixel: &'static str,
    catalogue_group: &'static str,
    catalogue_adjustment: &'static str,
    window_composite: &'static str,
    window_navigator: &'static str,
}

const RASTER_LABELS_NATIVE_EN: RasterLabels = RasterLabels {
    masks: "Masks",
    no_masks: "No masks",
    mask_suffix: "mask",
    add_pixel: "Add Pixel",
    add_group: "Add Group",
    layer_kinds: "Layer kinds",
    layer: "Layer",
    catalogue_pixel: "pixel — paintable bitmap layer",
    catalogue_group: "group — nested layer stack",
    catalogue_adjustment: "adjustment — non-destructive filter",
    window_composite: "Composite",
    window_navigator: "Navigator",
};

const RASTER_LABELS_NATIVE_DE: RasterLabels = RasterLabels {
    masks: "Masken",
    no_masks: "Keine Masken",
    mask_suffix: "Maske",
    add_pixel: "Pixel hinzufügen",
    add_group: "Gruppe hinzufügen",
    layer_kinds: "Ebenenarten",
    layer: "Ebene",
    catalogue_pixel: "pixel — bearbeitbare Bitmap-Ebene",
    catalogue_group: "group — verschachtelter Ebenenstapel",
    catalogue_adjustment: "adjustment — zerstörungsfreier Filter",
    window_composite: "Komposit",
    window_navigator: "Navigator",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
fn raster_labels(view_state: &ViewState) -> &'static RasterLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &RASTER_LABELS_NATIVE_DE } else { &RASTER_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
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

fn selection_from_runtime(runtime: &RasterPlayRuntime, view_state: &ViewState) -> Vec<String> {
    if !runtime.selected_ids.is_empty() {
        return runtime.selected_ids.clone();
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
        loading: None,
        default_open: Some(matches!(layer, RasterLayerNode::Group { .. })),
        action: Some(play_action(
            RASTER_PLAY_CONTROLLER_ID,
            "setSelection",
            Some(json!({ "ids": [layer_node_id(layer)] })),
        )),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: Some(true),
        drag_data: None,
        items: nested,
        control: None,
        is_hidden: if layer_visible(layer) { None } else { Some(true) },
    }
}

fn render_layers_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterLabels) -> UiNode {
    let toolbar = vec![
        UiTreeItemNode {
            id: "raster-play-layers.add.pixel".into(),
            label: labels.add_pixel.into(),
            description: None,
            icon_id: Some("image".into()),
            selected: None,
            loading: None,
            default_open: None,
            action: Some(play_action(
                RASTER_PLAY_CONTROLLER_ID,
                "addLayer",
                Some(json!({ "kind": "pixel" })),
            )),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        UiTreeItemNode {
            id: "raster-play-layers.add.group".into(),
            label: labels.add_group.into(),
            description: None,
            icon_id: Some("folder-plus".into()),
            selected: None,
            loading: None,
            default_open: None,
            action: Some(play_action(
                RASTER_PLAY_CONTROLLER_ID,
                "addLayer",
                Some(json!({ "kind": "group" })),
            )),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
    ];
    let layer_items: Vec<UiTreeItemNode> = document.layers.iter().map(layer_tree_item).collect();
    let selected_ids: Vec<String> = selection_from_runtime(runtime, view_state)
        .iter()
        .filter_map(|id| find_layer(&document.layers, id).map(layer_row_id))
        .collect();
    let highlighted_ids: Vec<String> = runtime
        .hovered_id
        .as_deref()
        .and_then(|id| find_layer(&document.layers, id))
        .map(|layer| vec![layer_row_id(layer)])
        .unwrap_or_default();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "raster-play-layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            loading: None,
            items: [toolbar, layer_items].concat(),
        }],
        loading: None,
        selected_ids: Some(selected_ids),
        highlighted_ids: Some(highlighted_ids),
        selection_change: Some(play_action(
            RASTER_PLAY_CONTROLLER_ID,
            "setSelection",
            None,
        )),
        drop_action: None,
    })
}

fn render_masks_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterLabels) -> UiNode {
    let mut items = Vec::new();
    fn collect_masks(layer: &RasterLayerNode, items: &mut Vec<UiTreeItemNode>, labels: &RasterLabels) {
        if let RasterLayerNode::Pixel { id, name, mask, .. }
        | RasterLayerNode::Group { id, name, mask, .. } = layer
        {
            if mask.as_ref().is_some_and(|mask| mask.enabled) {
                items.push(UiTreeItemNode {
                    id: mask_row_id(id),
                    label: format!("{name} {}", labels.mask_suffix),
                    description: Some("mask".into()),
                    icon_id: Some("scan".into()),
                    selected: None,
                    loading: None,
                    default_open: None,
                    action: Some(play_action(
                        RASTER_PLAY_CONTROLLER_ID,
                        "setSelection",
                        Some(json!({ "ids": [id] })),
                    )),
                    hover_action: None,
                    unhover_action: None,
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
                collect_masks(child, items, labels);
            }
        }
    }
    for layer in &document.layers {
        collect_masks(layer, &mut items, labels);
    }
    if items.is_empty() {
        items.push(UiTreeItemNode {
            id: "raster-play-masks.empty".into(),
            label: labels.no_masks.into(),
            description: None,
            icon_id: Some("scan".into()),
            selected: None,
            loading: None,
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
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
            label: Some(labels.masks.into()),
            default_open: Some(true),
            loading: None,
            items,
        }],
        loading: None,
        selected_ids: Some(
            selection_from_runtime(runtime, view_state)
                .iter()
                .map(|id| mask_row_id(id))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn render_catalogue_panel(labels: &RasterLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "raster-catalogue".into(),
        label: Some(labels.layer_kinds.into()),
        default_open: Some(true),
        loading: None,
        children: vec![
            ui_text(labels.catalogue_pixel),
            ui_text(labels.catalogue_group),
            ui_text(labels.catalogue_adjustment),
        ],
    }])
}

fn render_properties_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterLabels) -> UiNode {
    let selected = selection_from_runtime(runtime, view_state);
    let layers: Vec<&RasterLayerNode> = selected
        .iter()
        .filter_map(|id| find_layer(&document.layers, id))
        .collect();
    if layers.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", document.schema)),
            ui_text(format!("Brush: {} @ {}", runtime.brush_size, runtime.brush_opacity)),
        ]);
    }
    let names: Vec<String> = layers.iter().map(|layer| layer_name(*layer).into()).collect();
    let opacities: Vec<f64> = layers.iter().map(|layer| raster::layer_opacity(layer) as f64).collect();
    let mixed_name = ui_inspector_mixed_text(&names);
    let mixed_opacity = ui_inspector_mixed_number(&opacities);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "raster-properties.layer".into(),
        label: labels.layer.into(),
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
/// 📡 Document JSON for the WASM compositor, omitting embedded assets/camera/utility/brush — mirrors premigration `rasterDocumentToSyncJson`.
fn document_sync_json(document: &RasterDocument) -> String {
    let mut value = serde_json::to_value(document).unwrap_or(Value::Null);
    if let Value::Object(ref mut map) = value {
        map.remove("assets");
        map.remove("camera");
        map.remove("activeTool");
        map.remove("brushSize");
        map.remove("brushOpacity");
    }
    value.to_string()
}

fn raster_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str, view_mode: &str) -> RasterScene {
    RasterScene {
        document_sync_json: document_sync_json(document),
        assets_json: serde_json::to_string(&document.assets).unwrap_or_else(|_| "{}".into()),
        camera_json: serde_json::to_string(&document.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into()),
        selection_json: serde_json::to_string(&runtime.selected_ids).unwrap_or_else(|_| "[]".into()),
        hovered_id: runtime.hovered_id.clone(),
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size as f64,
        brush_opacity: runtime.brush_opacity as f64,
        view_mode: view_mode.into(),
        composite_viewport_json: runtime
            .composite_viewport
            .as_ref()
            .map(|viewport| serde_json::to_string(viewport).unwrap_or_else(|_| "{}".into())),
    }
}

fn render_composite_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str) -> UiNode {
    build_raster_scene(RASTER_PLAY_SURFACE_COMPOSITE, RASTER_PLAY_CONTROLLER_ID, raster_scene(document, runtime, active_utility, "composite"))
}

fn render_navigator_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str) -> UiNode {
    build_raster_scene(RASTER_PLAY_SURFACE_NAVIGATOR, RASTER_PLAY_CONTROLLER_ID, raster_scene(document, runtime, active_utility, "navigator"))
}
//#endregion 🔖Scenes

//#region 🔖RasterApp
#[derive(Default)]
struct RasterApp {
    runtime: RasterPlayRuntime,
}

impl RasterApp {
    /// 🩹 Builds `PatchLayer` ops for a `patchLayer`/`patchLayers` field write across ids.
    fn patch_layer_ops(&self, document: &RasterDocument, layer_ids: &[String], field: &str, value: &Value) -> Vec<RasterOp> {
        layer_ids
            .iter()
            .filter_map(|layer_id| {
                let prior = find_layer(&document.layers, layer_id)?;
                let patch = layer_patch_for_field(field, value, prior)?;
                Some(RasterOp::PatchLayer { layer_id: layer_id.clone(), patch })
            })
            .collect()
    }
}

impl Default for RasterPlayRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentApp for RasterApp {
    type Projection = RasterDocument;
    type Op = RasterOp;

    fn app_id(&self) -> &str {
        RASTER_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        RASTER_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> RasterDocument {
        empty_raster_document()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, RasterDocument>,
        _view_state: &ViewState,
    ) -> ActionEmit<RasterOp> {
        let document = doc.projection;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no ops.
            "setBrushSize" => {
                if let Some(size) = args.and_then(|value| value.get("brushSize")).and_then(|value| value.as_f64()) {
                    self.runtime.brush_size = size as f32;
                }
                ActionEmit::default()
            }
            "setBrushOpacity" => {
                if let Some(opacity) = args.and_then(|value| value.get("opacity")).and_then(|value| value.as_f64()) {
                    self.runtime.brush_opacity = (opacity as f32).clamp(0.0, 1.0);
                }
                ActionEmit::default()
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Host-owned utility switch: the active utility lives in session view state (never the
                // document). There is no plugin-side paint scratch to clear — brush strokes are painted
                // host-side in the WASM canvas — so this simply acknowledges with no ops or history.
                ActionEmit::default()
            }
            "setSelection" => {
                self.runtime.selected_ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                ActionEmit::default()
            }
            "setHover" => {
                self.runtime.hovered_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                ActionEmit::default()
            }
            "setCompositeViewport" => {
                if let (Some(width), Some(height)) = (
                    args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()),
                    args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()),
                ) {
                    self.runtime.composite_viewport = Some(RasterViewportSize { width, height });
                }
                ActionEmit::default()
            }
            "selectAll" => {
                self.runtime.selected_ids = flatten_raster_layers(&document.layers)
                    .into_iter()
                    .map(|layer| layer_node_id(layer).to_string())
                    .collect();
                ActionEmit::default()
            }
            // 📷 Camera — a coalesced scalar op so a pan/zoom gesture is one undo step.
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")).and_then(|value| serde_json::from_value::<RasterCamera>(value.clone()).ok()) {
                    return ActionEmit { ops: vec![RasterOp::SetCamera { camera }], coalesce_key: Some("camera".into()), ..Default::default() };
                }
                if let Some(zoom) = args.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()) {
                    let camera = RasterCamera { zoom, ..document.camera.clone() };
                    return ActionEmit { ops: vec![RasterOp::SetCamera { camera }], coalesce_key: Some("camera".into()), ..Default::default() };
                }
                ActionEmit::default()
            }
            // ✏️ Operations — dispatched as VCS operations with a true inverse.
            "setDocument" => match args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<RasterDocument>(value.clone()).ok()) {
                Some(replacement) => ActionEmit::ops(vec![RasterOp::ReplaceDocument { document: replacement }]),
                None => ActionEmit::default(),
            },
            "setLayerVisible" | "toggleLayerVisible" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                let Some(layer) = find_layer(&document.layers, target_id) else { return ActionEmit::default() };
                let visible = args
                    .and_then(|value| value.get("visible"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or_else(|| !layer_visible(layer));
                ActionEmit::ops(vec![RasterOp::PatchLayer {
                    layer_id: target_id.into(),
                    patch: RasterLayerPatch { visible: Some(visible), ..Default::default() },
                }])
            }
            "addLayer" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("pixel");
                let layer = create_layer_of_kind(kind);
                self.runtime.selected_ids = vec![layer_node_id(&layer).to_string()];
                ActionEmit::ops(vec![RasterOp::AddLayer { parent_id: None, index: document.layers.len(), layer }])
            }
            "dropLayerKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("pixel");
                let layer = create_layer_of_kind(kind);
                self.runtime.selected_ids = vec![layer_node_id(&layer).to_string()];
                ActionEmit::ops(vec![RasterOp::AddLayer { parent_id: None, index: document.layers.len(), layer }])
            }
            "deleteLayer" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                if find_layer(&document.layers, target_id).is_none() {
                    return ActionEmit::default();
                }
                self.runtime.selected_ids.retain(|id| id != target_id);
                ActionEmit::ops(vec![RasterOp::RemoveLayer { layer_id: target_id.into() }])
            }
            "duplicateLayer" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                match find_layer(&document.layers, target_id) {
                    Some(layer) => {
                        let copy = clone_layer(layer);
                        self.runtime.selected_ids = vec![layer_node_id(&copy).to_string()];
                        ActionEmit::ops(vec![RasterOp::AddLayer { parent_id: None, index: document.layers.len(), layer: copy }])
                    }
                    None => ActionEmit::default(),
                }
            }
            "patchLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if layer_id.is_empty() || field.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(self.patch_layer_ops(document, &[layer_id.to_string()], field, &value))
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
                if field.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(self.patch_layer_ops(document, &layer_ids, field, &value))
            }
            "moveLayer" => {
                let Some(layer_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                if find_layer(&document.layers, layer_id).is_none() {
                    return ActionEmit::default();
                }
                let target_row_id = args.and_then(|value| value.get("targetRowId")).and_then(|value| value.as_str()).unwrap_or("raster-play-layers");
                let drop_position = args.and_then(|value| value.get("dropPosition")).and_then(|value| value.as_str()).unwrap_or("after");
                let parent_id = layer_id_from_tree_row_id(target_row_id).and_then(|id| {
                    find_layer(&document.layers, &id).and_then(|entry| matches!(entry, RasterLayerNode::Group { .. }).then_some(id))
                });
                let index = if drop_position == "before" {
                    0
                } else if let Some(parent) = &parent_id {
                    match find_layer(&document.layers, parent) {
                        Some(RasterLayerNode::Group { children, .. }) => children.len(),
                        _ => 0,
                    }
                } else {
                    document.layers.len()
                };
                ActionEmit::ops(vec![RasterOp::MoveLayer { layer_id: layer_id.into(), parent_id, index }])
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RasterDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = raster_labels(view_state);
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(RASTER_DEFAULT_UTILITY);
        match body_key {
            RASTER_PLAY_BODY_COMPOSITE => render_composite_scene(document, &self.runtime, active_utility),
            RASTER_PLAY_BODY_NAVIGATOR => render_navigator_scene(document, &self.runtime, active_utility),
            RASTER_PLAY_BODY_LAYERS => render_layers_panel(document, &self.runtime, view_state, labels),
            RASTER_PLAY_BODY_MASKS => render_masks_panel(document, &self.runtime, view_state, labels),
            RASTER_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            RASTER_PLAY_BODY_PROPERTIES => render_properties_panel(document, &self.runtime, view_state, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = raster_labels(view_state);
        AppLabelsOverlay {
            app_label: None,
            window_kind_labels: HashMap::from([
                (RASTER_PLAY_WINDOW_COMPOSITE.to_string(), labels.window_composite.to_string()),
                (RASTER_PLAY_WINDOW_NAVIGATOR.to_string(), labels.window_navigator.to_string()),
            ]),
            panel_tab_labels: HashMap::new(),
            mode_labels: HashMap::new(),
            action_labels: HashMap::new(),
            utility_labels: HashMap::new(),
        }
    }
}
//#endregion 🔖RasterApp

//#region 🔖Manifest
/// 🛠️ An internal (non-palette) action declaration — the panel/pointer/gesture-bound vocabulary
/// dispatched by the layer tree, catalogue drops, camera and inspector, never a palette command.
fn raster_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, kind) }
}

/// 🧰 One composite-window utility declaration; ids must stay host-compatible (`paint*` prefix paints,
/// `paintEraser` erases, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: &str, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

fn create_raster_app() -> App {
    App::from_builder(
        App::builder(RASTER_PLAY_APP_ID, "Raster").document(["semio", "raster"])
            .icon_id("raster")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(RASTER_PLAY_WINDOW_COMPOSITE, "Composite", RASTER_PLAY_BODY_COMPOSITE, SurfaceKind::Raster)
            .window_kind(RASTER_PLAY_WINDOW_NAVIGATOR, "Navigator", RASTER_PLAY_BODY_NAVIGATOR, SurfaceKind::Raster)
            .default_layout(create_default_layout(
                &[RASTER_PLAY_WINDOW_COMPOSITE.into(), RASTER_PLAY_WINDOW_NAVIGATOR.into()],
                "row",
                Some(&[72.0, 28.0]),
                Some(&["Composite".into(), "Navigator".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                RASTER_PLAY_BODY_LAYERS,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                RASTER_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(RASTER_PLAY_MASKS_TAB_ID, "Masks", PanelGroup::Workbench, RASTER_PLAY_BODY_MASKS)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                RASTER_PLAY_BODY_PROPERTIES,
            )
            // ✏️ Palette-visible content operations.
            .operation("addLayer", "Add Layer")
            .operation("setDocument", "Set Document")
            // 🔧 Internal content operations — layer-tree / catalogue-drop / camera / inspector bound.
            .action_with(raster_internal_action("setCamera", "Set Camera", ActionKind::Operation))
            .action_with(raster_internal_action("setCameraZoom", "Set Camera Zoom", ActionKind::Operation))
            .action_with(raster_internal_action("setLayerVisible", "Set Layer Visible", ActionKind::Operation))
            .action_with(raster_internal_action("toggleLayerVisible", "Toggle Layer Visible", ActionKind::Operation))
            .action_with(raster_internal_action("dropLayerKind", "Drop Layer Kind", ActionKind::Operation))
            .action_with(raster_internal_action("deleteLayer", "Delete Layer", ActionKind::Operation))
            .action_with(raster_internal_action("duplicateLayer", "Duplicate Layer", ActionKind::Operation))
            .action_with(raster_internal_action("patchLayer", "Patch Layer", ActionKind::Operation))
            .action_with(raster_internal_action("patchLayers", "Patch Layers", ActionKind::Operation))
            .action_with(raster_internal_action("moveLayer", "Move Layer", ActionKind::Operation))
            // 👁️ Ephemeral view state — selection, hover, live brush controls, navigator viewport.
            .view_action("selectAll", "Select All")
            .action_with(raster_internal_action("setSelection", "Set Selection", ActionKind::View))
            .action_with(raster_internal_action("setHover", "Set Hover", ActionKind::View))
            .action_with(raster_internal_action("setBrushSize", "Set Brush Size", ActionKind::View))
            .action_with(raster_internal_action("setBrushOpacity", "Set Brush Opacity", ActionKind::View))
            .action_with(raster_internal_action("setCompositeViewport", "Set Composite Viewport", ActionKind::View))
            // 📝 Staged palette-form arguments for the two palette operations.
            .action_args("addLayer", vec![
                ActionArgDef::select("kind", "Layer Kind", vec![
                    ActionArgOption::new("pixel", "Pixel"),
                    ActionArgOption::new("group", "Group"),
                    ActionArgOption::new("adjustment", "Adjustment"),
                ]).required().default_value("pixel"),
            ])
            .action_args("setDocument", vec![
                ActionArgDef::text("document", "Document"),
            ])
            // 🧰 Composite-window utilities — one exclusive set, active utility host-owned (never a document op).
            .utility(raster_utility("selectMarquee", "Marquee Select", "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", "Brush", "brush", "Paint", UtilityCategory::Tools))
            .utility(raster_utility("paintEraser", "Eraser", "eraser", "Paint", UtilityCategory::Tools))
            .window_kind_utilities(RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(),
            ])
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

/// 📥 Rasterizes a DWG drawing's flat SVG projection into a single-layer raster document.
fn raster_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let data = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height)?;
    let asset_key = create_raster_id("dwg-asset");
    let mut layer = create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let mut assets = HashMap::new();
    assets.insert(asset_key, RasterImageAsset { mime: "image/png".into(), data });
    let document = RasterDocument {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: create_raster_id("dwg-import"),
        title: Some("DWG Import".into()),
        camera: RasterCamera::default(),
        layers: vec![layer],
        assets,
    };
    serde_json::to_value(&document).map_err(|error| error.to_string())
}

fn register_raster_exports() {
    semio_framework_os::register_2d_export_handlers("2d.raster", "raster", raster_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.raster", raster_document_json_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "raster", label: "Raster", version: "0.1.0",
    setup: register_raster_exports,
    apps: [ create_raster_app => RasterApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
    use semio_framework_plugin::app::AppActionRegistry;
    use vcs::{Backbone, BackboneMessage, MemoryBackbone};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<RasterApp> {
        VcsDocumentApp::new(RasterApp::default())
    }

    /// 🧬 A wrapper carrying the real registry so kind discipline (View-emits-ops rejection) runs.
    fn new_app_with_registry() -> VcsDocumentApp<RasterApp> {
        let definition = create_raster_app().definition;
        VcsDocumentApp::with_registry(RasterApp::default(), AppActionRegistry::from_definition(&definition))
    }

    fn semio_app() -> VcsDocumentApp<RasterApp> {
        let mut app = new_app();
        let document: RasterDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).expect("semio raster json");
        app.load_document(
            &serde_json::to_string(&vcs::create_document_vcs_envelope::<RasterDocument, RasterOp>(
                RASTER_DOCUMENT_SCHEMA,
                "raster",
                document,
                None,
            ))
            .unwrap(),
        )
        .expect("load semio");
        app
    }

    #[test]
    fn renders_raster_scene() {
        let mut app = new_app();
        let node = app.render(RASTER_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("raster"));
    }

    #[test]
    fn renders_navigator_scene() {
        let mut app = new_app();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_NAVIGATOR, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"componentKind\":\"raster\""));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[test]
    fn parses_semio_example_document() {
        let document: RasterDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).expect("semio raster json");
        assert!(!document.layers.is_empty());
    }

    #[test]
    fn imports_dwg_polyline_into_raster_document() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::LwPolyline {
                closed: true,
                elevation: 0.0,
                vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]],
                bulges: vec![0.0, 0.0, 0.0, 0.0],
            },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [10.0, 10.0, 0.0];
        let value = raster_document_json_from_dwg(&drawing).expect("dwg import");
        let document: RasterDocument = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert_eq!(asset.mime, "image/png");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn imports_empty_dwg_into_blank_raster_document() {
        let drawing = semio_framework_os::DwgDrawing::default();
        let value = raster_document_json_from_dwg(&drawing).expect("empty dwg import");
        let document: RasterDocument = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        assert_eq!(*width, Some(1));
        assert_eq!(*height, Some(1));
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn empty_document_background_layer_has_identity_scale() {
        let document = empty_raster_document();
        let json = document_sync_json(&document);
        assert!(json.contains(r#""scaleX":1.0"#), "expected identity scale in {json}");
        assert!(json.contains(r#""scaleY":1.0"#), "expected identity scale in {json}");
        assert!(!json.contains(r#""scaleX":0.0"#), "layer must not collapse to zero size");
    }

    #[test]
    fn renders_layers_tree() {
        let mut app = semio_app();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Backdrop"));
    }

    #[test]
    fn raster_labels_resolve_native_english_by_default() {
        let mut app = new_app();
        let layers_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(layers_json.contains("Add Pixel"));
        assert!(layers_json.contains("Add Group"));
        let masks_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_MASKS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(masks_json.contains("Masks"));
        assert!(masks_json.contains("No masks"));
        let catalogue_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(catalogue_json.contains("Layer kinds"));
        let properties_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_PROPERTIES, None, &ViewState::default()).expect("render")).unwrap();
        assert!(properties_json.contains("Schema:"));
    }

    #[test]
    fn raster_labels_resolve_german_locale() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let layers_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &view_state).expect("render")).unwrap();
        assert!(layers_json.contains("Pixel hinzufügen"));
        assert!(layers_json.contains("Gruppe hinzufügen"));
        let masks_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_MASKS, None, &view_state).expect("render")).unwrap();
        assert!(masks_json.contains("Masken"));
        assert!(masks_json.contains("Keine Masken"));
        let catalogue_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
        assert!(catalogue_json.contains("Ebenenarten"));
    }

    #[test]
    fn composite_scene_syncs_document_and_assets() {
        let mut app = semio_app();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"componentKind\":\"raster\""));
        assert!(json.contains("\"viewMode\":\"composite\""));
        assert!(!json.contains("\"assetsJson\":\"{}\""), "semio fixture has embedded assets");
        let document: RasterDocument = serde_json::from_str(SEMIO_EXAMPLE_JSON).unwrap();
        let sync_json = document_sync_json(&document);
        assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
        assert!(!sync_json.contains("\"camera\""), "sync json must omit camera");
    }

    #[test]
    fn set_hover_highlights_layer_row_via_runtime() {
        let mut app = semio_app();
        let layer_id = layer_node_id(&app.projection().expect("projection").layers[0]).to_string();
        let result = app.handle_action("setHover", Some(&json!({ "id": layer_id })), &ViewState::default(), &meta("local")).expect("hover");
        assert!(result.operations.is_empty(), "hover is a view action and emits no ops");
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"highlightedIds\":[\"raster-play-layers."));
    }

    #[test]
    fn set_composite_viewport_feeds_navigator_scene() {
        let mut app = new_app();
        app.handle_action("setCompositeViewport", Some(&json!({ "width": 640.0, "height": 480.0 })), &ViewState::default(), &meta("local")).expect("viewport");
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_NAVIGATOR, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("compositeViewportJson"));
        assert!(json.contains(r#"\"width\":640.0"#));
        assert!(json.contains(r#"\"height\":480.0"#));
    }

    #[test]
    fn add_layer_action_appends_and_undo_removes() {
        let mut app = new_app();
        let before = app.projection().expect("projection").layers.len();
        app.handle_action("addLayer", Some(&json!({ "kind": "group" })), &ViewState::default(), &meta("local")).expect("add");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.layers.len(), before + 1);
        assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").layers.len(), before);
    }

    #[test]
    fn patch_layer_renames_and_toggles_visibility_round_trip() {
        let mut app = new_app();
        let layer_id = layer_node_id(&app.projection().expect("projection").layers[0]).to_string();
        app.handle_action("patchLayer", Some(&json!({ "layerId": layer_id, "field": "name", "value": "Renamed" })), &ViewState::default(), &meta("local")).expect("rename");
        assert_eq!(layer_name(&app.projection().expect("projection").layers[0]), "Renamed");
        app.handle_action("toggleLayerVisible", Some(&json!({ "layerId": layer_id })), &ViewState::default(), &meta("local")).expect("toggle");
        assert!(!layer_visible(&app.projection().expect("projection").layers[0]));
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo toggle");
        assert!(layer_visible(&app.projection().expect("projection").layers[0]));
    }

    #[test]
    fn move_layer_into_group() {
        let mut app = new_app();
        app.handle_action("addLayer", Some(&json!({ "kind": "group" })), &ViewState::default(), &meta("local")).expect("add group");
        let (group_id, pixel_id) = {
            let projection = app.projection().expect("projection");
            let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
            let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
            (layer_node_id(group).to_string(), layer_node_id(pixel).to_string())
        };
        let target_row = format!("{RASTER_TREE_PREFIX}.group.{group_id}");
        app.handle_action("moveLayer", Some(&json!({ "layerId": pixel_id, "targetRowId": target_row })), &ViewState::default(), &meta("local")).expect("move");
        let projection = app.projection().expect("projection");
        let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| layer_node_id(layer) == group_id).unwrap() else {
            panic!("expected group");
        };
        assert_eq!(children.len(), 1);
        assert_eq!(layer_node_id(&children[0]), pixel_id);
    }

    /// 🧪 The definitional merge proof: A adds a layer while B renames the background layer — disjoint
    /// tree edits on one backbone that must both survive on both instances.
    #[test]
    fn two_instances_converge_disjoint_layer_edits_via_backbone() {
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        // Seed both from an identical base projection (a background layer with a fixed id) so B's
        // rename targets the same layer A holds — per-instance `initial_projection` mints fresh ids.
        let mut base = empty_raster_projection();
        base.layers = vec![RasterLayerNode::Pixel {
            id: "bg".into(),
            name: "Background".into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: RasterTransform::default(),
            mask: None,
            width: Some(512),
            height: Some(512),
            image_key: None,
        }];
        let base_envelope = serde_json::to_string(&vcs::create_document_vcs_envelope::<RasterDocument, RasterOp>(
            RASTER_DOCUMENT_SCHEMA,
            "raster",
            base,
            None,
        ))
        .unwrap();
        instance_a.load_document(&base_envelope).expect("load a");
        instance_b.load_document(&base_envelope).expect("load b");
        let background_id = "bg".to_string();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.handle_action("addLayer", Some(&json!({ "kind": "pixel" })), &ViewState::default(), &meta("actor-a")).expect("a adds layer");
        instance_b.handle_action("patchLayer", Some(&json!({ "layerId": background_id, "field": "name", "value": "Renamed By B" })), &ViewState::default(), &meta("actor-b")).expect("b renames");

        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
        assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
        assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
        assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        let mut sender = new_app();
        let (near, mut far) = MemoryBackbone::pair("mem://raster-doc", "mem://raster-doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        sender.handle_action("addLayer", Some(&json!({ "kind": "pixel" })), &ViewState::default(), &meta("local")).expect("add");
        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Ops { envelopes: ops } = message {
                envelopes.extend(ops);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied op on the channel");
        let operations_json = serde_json::to_string(&envelopes).expect("serialize");
        let mut receiver = new_app();
        let before = receiver.projection().expect("projection").layers.len();
        receiver.ingest_operations(&operations_json).expect("ingest once");
        receiver.ingest_operations(&operations_json).expect("ingest twice");
        assert_eq!(receiver.projection().expect("projection").layers.len(), before + 1, "no double-apply");
    }

    #[test]
    fn set_active_utility_switch_emits_no_ops_and_reads_from_view_state() {
        let mut app = new_app_with_registry();
        let before = app.projection().expect("projection");
        let view = ViewState { active_utility_id: Some("paintBrush".into()), ..ViewState::default() };
        // Switching utilities is the framework View action: no document ops, nothing to sync/undo.
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "paintBrush" })), &view, &meta("local"))
            .expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document ops");
        assert_eq!(app.projection().expect("projection"), before, "utility switching does not mutate the document");
        // The composite scene reads the host-owned active utility from session view state, not the runtime.
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_COMPOSITE, None, &view).expect("render")).unwrap();
        assert!(json.contains("\"activeUtility\":\"paintBrush\""), "scene reflects host-owned active utility: {json}");
    }

    #[test]
    fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
        let definition = create_raster_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser"]);
        // The marquee carries the Selection category; the paint utilities are Tools.
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee"]);
        let composite = definition.window_kinds.iter().find(|window| window.id == RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
        assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
        // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc op survives.
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.actions.iter().any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }
}
//#endregion 🧪Tests
