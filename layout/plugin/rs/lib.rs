//! 📐 Layout plugin — blueprint/preview document editor bundled as a hot-swappable WASM component.

use base64::Engine;
use layout_rs::{
    build_display_list_for_page, export_document_pdf, export_document_png_cpu, export_document_svg, export_package_zip,
    parse_layout_document, resolve_page, DisplayList, Frame, LayoutCamera, LayoutDocument, LAYOUT_FIXTURE_SCHEMA, Page,
    PageColumns, PageMargins,
};
use semio_framework_plugin::{SurfaceKind,
    build_canvas_2d_scene, create_default_layout, tool_button, tool_collection, ui_declarative_sections_to_tree,
    ui_inspector_groups_to_tree, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App,
    Canvas2dScene, ActionDescriptor, PanelGroup, PluginApp, PluginBundle, ToolNode, UiFieldNode,
    UiInputNode, UiInspectorFieldGroup, UiNode, UiSectionNode, UiSelectItem, UiSelectNode, UiTreeItemNode, UiTreeNode,
    UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_plugin::layout::{WindowEngagementPossible, WindowEngagementStatus};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖Constants
const LAYOUT_PLAY_APP_ID: &str = "layout-play";
const LAYOUT_PLAY_SURFACE_BLUEPRINT: &str = "layout.play.blueprint";
const LAYOUT_PLAY_SURFACE_PREVIEW: &str = "layout.play.preview";
const LAYOUT_PLAY_BODY_BLUEPRINT: &str = "layout.play.blueprint";
const LAYOUT_PLAY_BODY_PREVIEW: &str = "layout.play.preview";
const LAYOUT_PLAY_BODY_DOCUMENT: &str = "layout.play.document";
const LAYOUT_PLAY_BODY_CATALOGUE: &str = "layout.play.catalogue";
const LAYOUT_PLAY_BODY_INSPECTION: &str = "layout.play.inspection";
const LAYOUT_PLAY_BODY_PREFLIGHT: &str = "layout.play.preflight";
const LAYOUT_PLAY_WINDOW_BLUEPRINT: &str = "layout-blueprint";
const LAYOUT_PLAY_WINDOW_PREVIEW: &str = "layout-preview";
const LAYOUT_PLAY_PREFLIGHT_TAB_ID: &str = "layout.panel.preflight";

const LAYOUT_SAMPLE_JSON: &str = include_str!("../../example/sample.layout.json");

const LAYOUT_CATALOGUE_KINDS: &[(&str, &str, &str)] = &[
    ("rect", "Rectangle", "square"),
    ("text", "Text Frame", "type"),
    ("image", "Image Frame", "image"),
];

const LAYOUT_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
const LAYOUT_CATALOGUE_KIND_MIME_PREFIX: &str = "application/x-semio-catalogue-kind.";
const LAYOUT_DROP_PREVIEW_WIDTH: f64 = 200.0;
const LAYOUT_DROP_PREVIEW_HEIGHT: f64 = 120.0;
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LayoutPlayRuntime {
    #[serde(default = "default_active_page_id")]
    active_page_id: String,
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    hovered_id: Option<String>,
    #[serde(default)]
    drop_preview: Option<LayoutDropPreviewState>,
    #[serde(default)]
    engagement_input: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LayoutDropPreviewState {
    kind: String,
    x: f64,
    y: f64,
}

impl Default for LayoutPlayRuntime {
    fn default() -> Self {
        Self {
            active_page_id: default_active_page_id(),
            selected_ids: Vec::new(),
            hovered_id: None,
            drop_preview: None,
            engagement_input: String::new(),
        }
    }
}

fn default_active_page_id() -> String {
    "page-1".into()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LayoutPlayEnvelope {
    document: LayoutDocument,
    #[serde(default)]
    undo_stack: Vec<LayoutDocument>,
    #[serde(default)]
    redo_stack: Vec<LayoutDocument>,
    #[serde(default)]
    runtime: LayoutPlayRuntime,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreflightIssue {
    severity: String,
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_id: Option<String>,
}

//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_document() -> LayoutDocument {
    parse_layout_document(LAYOUT_SAMPLE_JSON).expect("sample layout fixture")
}

fn default_envelope() -> LayoutPlayEnvelope {
    LayoutPlayEnvelope {
        document: default_document(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
        runtime: LayoutPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> LayoutPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &LayoutPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn download_media_export_op(filename: &str, mime_type: &str, data: &str) -> String {
    json!({ "op": "downloadMediaExport", "filename": filename, "mimeType": mime_type, "data": data }).to_string()
}

fn layout_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: LAYOUT_PLAY_APP_ID.into(),
        action: action.into(),
        args,
    }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

fn active_page<'a>(doc: &'a LayoutDocument, runtime: &LayoutPlayRuntime) -> Option<&'a Page> {
    doc.pages.iter().find(|page| page.id == runtime.active_page_id).or_else(|| doc.pages.first())
}

fn frame_icon(kind: &str) -> &str {
    match kind {
        "rect" => "square",
        "text" => "type",
        "image" => "image",
        _ => "square",
    }
}

fn page_row_id(page_id: &str) -> String {
    format!("layout-document.page.{page_id}")
}

fn frame_row_id(frame_id: &str) -> String {
    format!("layout-document.frame.{frame_id}")
}

fn layer_row_id(page_id: &str, layer_id: &str) -> String {
    format!("layout-document.layer.{page_id}.{layer_id}")
}

fn spread_row_id(spread_id: &str) -> String {
    format!("layout-document.spread.{spread_id}")
}

fn parent_page_row_id(parent_page_id: &str) -> String {
    format!("layout-document.parentPage.{parent_page_id}")
}

fn story_row_id(story_id: &str) -> String {
    format!("layout-document.story.{story_id}")
}

fn link_row_id(link_id: &str) -> String {
    format!("layout-document.link.{link_id}")
}

fn style_row_id(style_id: &str) -> String {
    format!("layout-document.style.{style_id}")
}

fn push_undo(play: &mut LayoutPlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}

fn story_full_content(doc: &LayoutDocument, story_id: &str) -> String {
    doc.stories.iter().find(|story| story.id == story_id).map(|story| story.content.clone()).unwrap_or_default()
}

fn link_path(doc: &LayoutDocument, link_id: &str) -> String {
    doc.links.iter().find(|link| link.id == link_id).map(|link| link.path.clone()).unwrap_or_default()
}

fn rgba_to_text(color: &Option<[f32; 4]>) -> String {
    color
        .map(|channels| channels.iter().map(|channel| channel.to_string()).collect::<Vec<_>>().join(", "))
        .unwrap_or_default()
}

fn text_to_rgba(text: &str) -> Option<[f32; 4]> {
    let parts: Vec<f32> = text.split(',').filter_map(|part| part.trim().parse::<f32>().ok()).collect();
    (parts.len() == 4).then(|| [parts[0], parts[1], parts[2], parts[3]])
}

//#region 🔖CanvasScene
fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([
        { "kind": "move", "to": [x, y] },
        { "kind": "line", "to": [x + width, y] },
        { "kind": "line", "to": [x + width, y + height] },
        { "kind": "line", "to": [x, y + height] },
        { "kind": "close" },
    ])
}

fn line_segments(x0: f64, y0: f64, x1: f64, y1: f64) -> Value {
    json!([
        { "kind": "move", "to": [x0, y0] },
        { "kind": "line", "to": [x1, y1] },
    ])
}

fn host_layer(id: impl Into<String>, segments: Value, fill: Option<[f32; 4]>, stroke: Option<([f32; 4], f64, Option<[f64; 2]>)>) -> Value {
    let mut layer = json!({ "id": id.into(), "segments": segments });
    if let Some(color) = fill {
        layer["fill"] = json!({ "color": color });
    }
    if let Some((color, width, dash)) = stroke {
        let mut stroke_value = json!({ "color": color, "width": width });
        if let Some(dash) = dash {
            stroke_value["dash"] = json!(dash);
        }
        layer["stroke"] = stroke_value;
    }
    layer
}

fn guide_stroke_color(kind: &str) -> [f32; 4] {
    match kind {
        "margin" => [0.75, 0.2, 0.2, 0.35],
        "column" => [0.2, 0.45, 0.85, 0.25],
        "baseline" => [0.5, 0.5, 0.5, 0.2],
        _ => [0.3, 0.3, 0.3, 0.3],
    }
}

fn drop_preview_fill(kind: &str) -> [f32; 4] {
    match kind {
        "rect" => [0.85, 0.88, 0.92, 0.45],
        "text" => [0.2, 0.55, 0.9, 0.25],
        "image" => [0.85, 0.45, 0.2, 0.25],
        _ => [0.5, 0.5, 0.5, 0.3],
    }
}

fn display_list_to_host_layers(list: &DisplayList, blueprint: bool, drop_preview: Option<&LayoutDropPreviewState>) -> Vec<Value> {
    let mut layers = Vec::new();

    let page_bg = if blueprint { [0.97, 0.97, 0.98, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };
    layers.push(host_layer("layout.page-bg", rect_segments(0.0, 0.0, list.page_width as f64, list.page_height as f64), Some(page_bg), None));

    if blueprint {
        for guide in &list.guides {
            let color = guide_stroke_color(&guide.kind);
            let segments = if guide.rect.height <= 0.0 {
                line_segments(guide.rect.x, guide.rect.y, guide.rect.x + guide.rect.width, guide.rect.y)
            } else {
                rect_segments(guide.rect.x, guide.rect.y, guide.rect.width, guide.rect.height)
            };
            layers.push(host_layer(format!("layout.guide.{}", guide.kind), segments, None, Some((color, 1.0, None))));
        }
    }

    for rect in &list.rects {
        let segments = rect_segments(rect.x as f64, rect.y as f64, rect.width as f64, rect.height as f64);
        let fill = rect.fill.as_ref().map(|color| color.0);
        let dash = (blueprint && rect.inherited).then_some([4.0, 3.0]);
        let stroke = if let Some(stroke_color) = &rect.stroke {
            let width = if rect.selected { 2.5 } else if rect.hovered { 1.75 } else { 1.0 };
            Some((stroke_color.0, width, dash))
        } else if rect.selected && blueprint {
            Some(([0.1, 0.45, 0.95, 1.0], 2.0, None))
        } else if rect.hovered && blueprint {
            Some(([0.95, 0.72, 0.15, 1.0], 1.5, None))
        } else {
            None
        };
        layers.push(host_layer(rect.object_id.clone(), segments, fill, stroke));
    }

    for image in &list.images {
        let color = if image.placeholder { [0.92, 0.88, 0.84, 1.0] } else { [0.85, 0.85, 0.85, 1.0] };
        let segments = rect_segments(image.x as f64, image.y as f64, image.width as f64, image.height as f64);
        let stroke = image.placeholder.then_some(([0.75, 0.35, 0.2, 1.0], 1.0, None));
        layers.push(host_layer(format!("{}.image", image.object_id), segments, Some(color), stroke));
    }

    for run in &list.text_runs {
        if run.glyphs.is_empty() {
            continue;
        }
        let mut segments = Vec::new();
        for glyph in &run.glyphs {
            let scale = (glyph.font_size / 16.0) as f64;
            let width = 0.45 * scale;
            let height = glyph.font_size as f64 * scale;
            let x = glyph.x as f64;
            let y = glyph.y as f64;
            segments.push(json!({ "kind": "move", "to": [x, y - height] }));
            segments.push(json!({ "kind": "line", "to": [x + width, y - height] }));
            segments.push(json!({ "kind": "line", "to": [x + width, y] }));
            segments.push(json!({ "kind": "line", "to": [x, y] }));
            segments.push(json!({ "kind": "close" }));
        }
        layers.push(host_layer(format!("{}.glyphs", run.object_id), json!(segments), Some([0.0, 0.0, 0.0, 1.0]), None));
    }

    if blueprint {
        if let Some(preview) = drop_preview.filter(|preview| preview.kind != "page") {
            let segments = rect_segments(preview.x, preview.y, LAYOUT_DROP_PREVIEW_WIDTH, LAYOUT_DROP_PREVIEW_HEIGHT);
            let fill = drop_preview_fill(&preview.kind);
            layers.push(host_layer("layout.drop-preview", segments, Some(fill), Some(([0.1, 0.45, 0.95, 0.85], 2.0, None))));
        }
    }

    layers
}

fn canvas_layers(doc: &LayoutDocument, runtime: &LayoutPlayRuntime, blueprint: bool) -> String {
    let page = match active_page(doc, runtime) {
        Some(page) => page,
        None => return "[]".into(),
    };
    let list = build_display_list_for_page(doc, page, &page.id, &runtime.selected_ids, runtime.hovered_id.as_deref(), blueprint);
    let layers = display_list_to_host_layers(&list, blueprint, runtime.drop_preview.as_ref());
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖CanvasScene

//#region 🔖PointerCamera
fn surface_is_blueprint(args: Option<&Value>) -> bool {
    args.and_then(|value| value.get("surfaceId"))
        .and_then(|value| value.as_str())
        .is_none_or(|surface| surface.contains("blueprint"))
}

fn camera_for_surface<'a>(doc: &'a mut LayoutDocument, blueprint: bool) -> &'a mut LayoutCamera {
    if blueprint {
        &mut doc.camera
    } else {
        &mut doc.preview_camera
    }
}

fn screen_to_world_for_surface(play: &LayoutPlayEnvelope, blueprint: bool, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_doc = if blueprint { &play.document.camera } else { &play.document.preview_camera };
    let camera = layout_rs::cavas::camera::Camera { x: camera_doc.x, y: camera_doc.y, zoom: camera_doc.zoom.max(0.0001) };
    let viewport = layout_rs::cavas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = layout_rs::cavas::camera::screen_to_world(&camera, &viewport, layout_rs::cavas::Point::new(sx, sy));
    (world.x, world.y)
}

fn pointer_args(args: Option<&Value>) -> (f64, f64, f64, f64) {
    let x = args.and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or(0.0);
    let y = args.and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or(0.0);
    let width = args.and_then(|value| value.get("width")).and_then(Value::as_f64).unwrap_or(1.0);
    let height = args.and_then(|value| value.get("height")).and_then(Value::as_f64).unwrap_or(1.0);
    (x, y, width, height)
}

fn hit_test_at(play: &LayoutPlayEnvelope, args: Option<&Value>, blueprint: bool) -> Option<String> {
    let page = active_page(&play.document, &play.runtime)?;
    let (sx, sy, width, height) = pointer_args(args);
    let (wx, wy) = screen_to_world_for_surface(play, blueprint, sx, sy, width, height);
    let list = build_display_list_for_page(&play.document, page, &page.id, &play.runtime.selected_ids, play.runtime.hovered_id.as_deref(), blueprint);
    list.hit_test(wx as f32, wy as f32)
}
//#endregion 🔖PointerCamera

fn patch_frame_bounds(frame: &mut Frame, field: &str, value: f64) {
    let bounds = match frame {
        Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
    };
    match field {
        "x" => bounds.x = value,
        "y" => bounds.y = value,
        "width" | "w" => bounds.width = value,
        "height" | "h" => bounds.height = value,
        _ => {}
    }
}

fn resolve_link_state(link: &layout_rs::ImageLink) -> &str {
    if let Some(state) = link.state.as_deref() {
        return state;
    }
    if link.path.is_empty() || link.hash == "sha256:missing" {
        return "missing";
    }
    if link.dpi < 150 {
        return "low_resolution";
    }
    "ok"
}

fn resolve_run_style(doc: &LayoutDocument, paragraph_style_id: Option<&str>, character_style_id: Option<&str>) -> (String, f64) {
    let paragraph = paragraph_style_id
        .and_then(|id| doc.paragraph_styles.iter().find(|style| style.id == id))
        .or_else(|| doc.paragraph_styles.first());
    let (mut family, mut size) = paragraph.map(|style| (style.font_family.clone(), style.font_size)).unwrap_or_else(|| ("Layout Sans".into(), 12.0));
    if let Some(character_id) = character_style_id {
        if let Some(character) = doc.character_styles.iter().find(|value| value.get("id").and_then(Value::as_str) == Some(character_id)) {
            if let Some(font_family) = character.get("fontFamily").and_then(Value::as_str) {
                family = font_family.into();
            }
            if let Some(font_size) = character.get("fontSize").and_then(Value::as_f64) {
                size = font_size;
            }
        }
    }
    (family, size)
}

fn run_layout_preflight(doc: &LayoutDocument) -> Vec<PreflightIssue> {
    let mut issues = Vec::new();
    for page in &doc.pages {
        let resolved = resolve_page(doc, page);
        for entry in resolved {
            let frame = &entry.frame;
            if !frame.visible() {
                continue;
            }
            let bounds = frame.bounds();
            if bounds.x < 0.0 || bounds.y < 0.0 || bounds.x + bounds.width > page.width || bounds.y + bounds.height > page.height {
                issues.push(PreflightIssue {
                    severity: "warning".into(),
                    code: "object.out_of_bounds".into(),
                    message: format!("Object {} extends outside page bounds", frame.id()),
                    object_id: Some(frame.id().into()),
                    page_id: Some(page.id.clone()),
                });
            }
            match frame {
                Frame::Image { link_id, .. } => {
                    let link = doc.links.iter().find(|entry| entry.id == *link_id);
                    match link.map(resolve_link_state) {
                        Some("missing") | None => issues.push(PreflightIssue {
                            severity: "error".into(),
                            code: "asset.missing".into(),
                            message: format!("Linked asset missing for {}", frame.id()),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        Some("modified") => issues.push(PreflightIssue {
                            severity: "warning".into(),
                            code: "asset.modified".into(),
                            message: format!("Linked asset modified for {}", frame.id()),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        Some("low_resolution") => issues.push(PreflightIssue {
                            severity: "warning".into(),
                            code: "asset.low_resolution".into(),
                            message: format!("Linked asset is low resolution for {}", frame.id()),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        _ => {}
                    }
                    if link.is_some_and(|entry| entry.proxy_data_url.is_none()) && bounds.width > 0.0 && bounds.height > 0.0 {
                        issues.push(PreflightIssue {
                            severity: "info".into(),
                            code: "image.empty_frame".into(),
                            message: format!("Image frame {} has no preview", frame.id()),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        });
                    }
                }
                Frame::Text { story_id, thread_next, .. } => {
                    let Some(story) = doc.stories.iter().find(|story| story.id == *story_id) else {
                        issues.push(PreflightIssue {
                            severity: "error".into(),
                            code: "text.missing_story".into(),
                            message: format!("Text frame {} has no story", frame.id()),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        });
                        continue;
                    };
                    let styles: Vec<(String, f64)> = if story.style_runs.is_empty() {
                        vec![resolve_run_style(doc, None, None)]
                    } else {
                        story
                            .style_runs
                            .iter()
                            .map(|run| resolve_run_style(doc, run.paragraph_style_id.as_deref(), run.character_style_id.as_deref()))
                            .collect()
                    };
                    for (family, size) in &styles {
                        if *size < 8.0 {
                            issues.push(PreflightIssue {
                                severity: "warning".into(),
                                code: "text.below_minimum_size".into(),
                                message: format!("Text in {} is below minimum readable size", frame.id()),
                                object_id: Some(frame.id().into()),
                                page_id: Some(page.id.clone()),
                            });
                        }
                        let known_family = family == "Layout Sans" || doc.paragraph_styles.iter().any(|style| style.font_family == *family);
                        if !known_family {
                            issues.push(PreflightIssue {
                                severity: "error".into(),
                                code: "font.missing".into(),
                                message: format!("Font {family} used by {} is not available", frame.id()),
                                object_id: Some(frame.id().into()),
                                page_id: Some(page.id.clone()),
                            });
                        }
                    }
                    if thread_next.is_none() && story.content.len() > 400 {
                        issues.push(PreflightIssue {
                            severity: "error".into(),
                            code: "text.overset".into(),
                            message: format!("Text in {} overflows its frame", frame.id()),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        });
                    }
                }
                Frame::Rect { .. } => {}
            }
        }
    }
    if doc.print_target.as_deref() == Some("print") {
        for link in &doc.links {
            if link.color_profile.as_deref() == Some("RGB") {
                issues.push(PreflightIssue {
                    severity: "warning".into(),
                    code: "asset.rgb_in_print".into(),
                    message: format!("Linked asset {} uses RGB in a print document", link.id),
                    object_id: Some(link.id.clone()),
                    page_id: None,
                });
            }
        }
    }
    issues
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn tree_item(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<ActionDescriptor>,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id,
        selected: None,
        default_open: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        action,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_hoverable(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<ActionDescriptor>,
    hover_id: &str,
) -> UiTreeItemNode {
    let mut item = tree_item(id, label, description, icon_id, action);
    item.hover_action = Some(layout_action("setHover", Some(json!({ "id": hover_id }))));
    item.unhover_action = Some(layout_action("setHover", Some(json!({ "id": Value::Null }))));
    item
}

fn build_document_tree(play: &LayoutPlayEnvelope) -> UiNode {
    let doc = &play.document;

    let spread_items: Vec<UiTreeItemNode> = doc
        .spreads
        .iter()
        .map(|spread| tree_item(spread_row_id(&spread.id), spread.name.clone(), Some(spread.page_ids.join(", ")), Some("layout".into()), None))
        .collect();

    let page_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .map(|page| {
            tree_item_hoverable(
                page_row_id(&page.id),
                page.name.clone(),
                page.parent_page_id.as_ref().map(|parent_id| format!("parent: {parent_id}")),
                Some("file".into()),
                Some(layout_action("setActivePage", Some(json!({ "pageId": page.id })))),
                &page.id,
            )
        })
        .collect();

    let frame_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .flat_map(|page| {
            page.frames.iter().map(move |frame| {
                tree_item_hoverable(
                    frame_row_id(frame.id()),
                    frame.id(),
                    Some(format!("{} · {}", page.name, frame.kind_str())),
                    Some(frame_icon(frame.kind_str()).into()),
                    Some(layout_action("setSelection", Some(json!({ "ids": [frame.id()] })))),
                    frame.id(),
                )
            })
        })
        .collect();
    let frame_items = if frame_items.is_empty() {
        vec![tree_item("layout-document.frames.empty", "Drop catalogue items here", None, Some("inbox".into()), None)]
    } else {
        frame_items
    };

    let parent_page_items: Vec<UiTreeItemNode> = doc
        .parent_pages
        .iter()
        .map(|parent| {
            tree_item(
                parent_page_row_id(&parent.id),
                parent.name.clone(),
                Some(format!("{}×{}", parent.width as i64, parent.height as i64)),
                Some("copy".into()),
                None,
            )
        })
        .collect();

    let layer_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .flat_map(|page| {
            page.layers.iter().map(move |layer| {
                tree_item(
                    layer_row_id(&page.id, &layer.id),
                    format!("{} · {}", page.name, layer.name),
                    Some(format!("{} objects", layer.object_ids.len())),
                    Some("layers".into()),
                    None,
                )
            })
        })
        .collect();

    let story_items: Vec<UiTreeItemNode> = doc
        .stories
        .iter()
        .map(|story| tree_item(story_row_id(&story.id), story.id.clone(), Some(format!("{} chars", story.content.chars().count())), Some("file-text".into()), None))
        .collect();

    let link_items: Vec<UiTreeItemNode> = doc
        .links
        .iter()
        .map(|link| {
            let referencing_ids: Vec<String> = doc
                .pages
                .iter()
                .flat_map(|page| page.frames.iter())
                .filter_map(|frame| match frame {
                    Frame::Image { link_id, .. } if link_id == &link.id => Some(frame.id().to_string()),
                    _ => None,
                })
                .collect();
            tree_item(
                link_row_id(&link.id),
                link.path.clone(),
                Some(link.state.clone().unwrap_or_else(|| "ok".into())),
                Some("link".into()),
                (!referencing_ids.is_empty()).then(|| layout_action("setSelection", Some(json!({ "ids": referencing_ids })))),
            )
        })
        .collect();

    let mut style_items: Vec<UiTreeItemNode> = doc
        .paragraph_styles
        .iter()
        .map(|style| {
            tree_item(
                style_row_id(&style.id),
                style.name.clone(),
                Some(format!("{} · {}pt", style.font_family, style.font_size as i64)),
                Some("type".into()),
                None,
            )
        })
        .collect();
    style_items.extend(doc.character_styles.iter().enumerate().map(|(index, style)| {
        let id = style.get("id").and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| format!("character-{index}"));
        let name = style.get("name").and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| id.clone());
        let font_family = style.get("fontFamily").and_then(Value::as_str).unwrap_or("—");
        let description = match style.get("fontSize").and_then(Value::as_f64) {
            Some(size) => format!("{font_family} · {}pt", size as i64),
            None => font_family.to_string(),
        };
        tree_item(style_row_id(&id), name, Some(description), Some("type".into()), None)
    }));

    let highlighted_ids: Vec<String> = play
        .runtime
        .hovered_id
        .as_ref()
        .map(|id| vec![page_row_id(id), frame_row_id(id)])
        .unwrap_or_default();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "layout-document.document".into(),
                label: Some("Document".into()),
                default_open: Some(true),
                items: vec![tree_item(
                    "layout-document.document.root",
                    doc.name.clone(),
                    Some(LAYOUT_FIXTURE_SCHEMA.into()),
                    Some("file-text".into()),
                    None,
                )],
            },
            UiTreeSectionNode { id: "layout-document.spreads".into(), label: Some("Spreads".into()), default_open: Some(false), items: spread_items },
            UiTreeSectionNode {
                id: "layout-document.pages".into(),
                label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
                default_open: Some(true),
                items: page_items,
            },
            UiTreeSectionNode { id: "layout-document.frames".into(), label: Some("Frames".into()), default_open: Some(true), items: frame_items },
            UiTreeSectionNode { id: "layout-document.parentPages".into(), label: Some("Parent Pages".into()), default_open: Some(false), items: parent_page_items },
            UiTreeSectionNode { id: "layout-document.layers".into(), label: Some("Layers".into()), default_open: Some(false), items: layer_items },
            UiTreeSectionNode { id: "layout-document.stories".into(), label: Some("Stories".into()), default_open: Some(false), items: story_items },
            UiTreeSectionNode { id: "layout-document.links".into(), label: Some("Links".into()), default_open: Some(false), items: link_items },
            UiTreeSectionNode { id: "layout-document.styles".into(), label: Some("Styles".into()), default_open: Some(false), items: style_items },
        ],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .flat_map(|id| vec![page_row_id(id), frame_row_id(id), layer_row_id(&play.runtime.active_page_id, id)])
                .collect(),
        ),
        highlighted_ids: if highlighted_ids.is_empty() { None } else { Some(highlighted_ids) },
        selection_change: Some(layout_action("setSelection", None)),
        drop_action: None,
    })
}

fn catalogue_tree_item(kind: &str, label: &str, icon: &str) -> UiTreeItemNode {
    let action = if kind == "page" { layout_action("addPage", None) } else { layout_action("addFrame", Some(json!({ "kind": kind }))) };
    let mut drag_data = HashMap::new();
    drag_data.insert(LAYOUT_CATALOGUE_DRAG_MIME.to_string(), json!({ "kind": kind }).to_string());
    drag_data.insert(format!("{LAYOUT_CATALOGUE_KIND_MIME_PREFIX}{kind}"), String::new());
    let mut item = tree_item(format!("layout-catalogue.{kind}"), label, Some(kind.into()), Some(icon.into()), Some(action));
    item.draggable = Some(true);
    item.drag_data = Some(drag_data);
    item
}

fn build_catalogue_tree() -> UiNode {
    let mut items = vec![catalogue_tree_item("page", "Page", "file")];
    items.extend(LAYOUT_CATALOGUE_KINDS.iter().map(|(kind, label, icon)| catalogue_tree_item(kind, label, icon)));
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "layout-catalogue.kinds".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_inspector_tree(play: &LayoutPlayEnvelope) -> UiNode {
    let doc = &play.document;
    if play.runtime.selected_ids.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", LAYOUT_FIXTURE_SCHEMA)),
            ui_text(format!("Name: {}", doc.name)),
            ui_text(format!("Pages: {}", doc.pages.len())),
            ui_text(format!("Active page: {}", play.runtime.active_page_id)),
        ]);
    }
    let selected_id = &play.runtime.selected_ids[0];
    if let Some(page) = doc.pages.iter().find(|page| page.id == *selected_id) {
        let mut fields = vec![
            ui_inspector_readonly_field("layout-play-inspector.page-id", "Id", page.id.clone()),
            UiNode::Field(UiFieldNode {
                id: "layout-play-inspector.page-name".into(),
                label: "Name".into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "layout-play-inspector.page-name.input".into(),
                    input_kind: "text".into(),
                    value: page.name.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: layout_action("patchPage", Some(json!({ "pageId": page.id, "field": "name" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
        ];
        for (field, label, value) in [
            ("width", "Width", page.width),
            ("height", "Height", page.height),
            ("marginTop", "Margin Top", page.margins.top),
            ("marginRight", "Margin Right", page.margins.right),
            ("marginBottom", "Margin Bottom", page.margins.bottom),
            ("marginLeft", "Margin Left", page.margins.left),
            ("columnsGutter", "Gutter", page.columns.gutter),
        ] {
            fields.push(UiNode::Field(UiFieldNode {
                id: format!("layout-play-inspector.page-{field}"),
                label: label.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: format!("layout-play-inspector.page-{field}.input"),
                    input_kind: "number".into(),
                    value: format!("{value}"),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: layout_action("patchPage", Some(json!({ "pageId": page.id, "field": field }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }));
        }
        fields.push(UiNode::Field(UiFieldNode {
            id: "layout-play-inspector.page-columnsCount".into(),
            label: "Columns".into(),
            child: Box::new(UiNode::Input(UiInputNode {
                id: "layout-play-inspector.page-columnsCount.input".into(),
                input_kind: "number".into(),
                value: format!("{}", page.columns.count),
                placeholder: None,
                commit: Some("blur".into()),
                on_change: layout_action("patchPage", Some(json!({ "pageId": page.id, "field": "columnsCount" }))),
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
            description: None,
            required: None,
            error: None,
        }));
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "layout-play-inspector.page".into(),
            label: "Page".into(),
            default_open: Some(true),
            fields,
        }]);
    }
    for page in &doc.pages {
        if let Some(frame) = page.frames.iter().find(|frame| frame.id() == selected_id) {
            let bounds = frame.bounds();
            let frame_id = frame.id().to_string();
            let page_id = page.id.clone();
            let name_mixed = ui_inspector_mixed_text(&[frame.id().to_string()]);
            let mut fields = vec![
                ui_inspector_readonly_field("layout-play-inspector.frame-id", "Id", frame_id.clone()),
                ui_inspector_readonly_field("layout-play-inspector.frame-kind", "Kind", frame.kind_str().to_string()),
                ui_inspector_readonly_field("layout-play-inspector.frame-page", "Page", page.name.clone()),
            ];
            for (field, label, value) in [
                ("x", "X", bounds.x),
                ("y", "Y", bounds.y),
                ("width", "Width", bounds.width),
                ("height", "Height", bounds.height),
            ] {
                fields.push(UiNode::Field(UiFieldNode {
                    id: format!("layout-play-inspector.frame-{field}"),
                    label: label.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
                        id: format!("layout-play-inspector.frame-{field}.input"),
                        input_kind: "number".into(),
                        value: format!("{}", value as i64),
                        placeholder: None,
                        commit: Some("blur".into()),
                        on_change: layout_action(
                            "patchFrame",
                            Some(json!({ "frameId": frame_id, "pageId": page_id, "field": field })),
                        ),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                }));
            }
            match frame {
                Frame::Rect { fill, stroke, .. } => {
                    for (field, label, value) in [("fill", "Fill", fill), ("stroke", "Stroke", stroke)] {
                        fields.push(UiNode::Field(UiFieldNode {
                            id: format!("layout-play-inspector.frame-{field}"),
                            label: label.into(),
                            child: Box::new(UiNode::Input(UiInputNode {
                                id: format!("layout-play-inspector.frame-{field}.input"),
                                input_kind: "text".into(),
                                value: rgba_to_text(value),
                                placeholder: Some("r, g, b, a".into()),
                                commit: Some("blur".into()),
                                on_change: layout_action(
                                    "patchFrame",
                                    Some(json!({ "frameId": frame_id, "pageId": page_id, "field": field })),
                                ),
                                min: None,
                                max: None,
                                step: None,
                                accept: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                        }));
                    }
                }
                Frame::Text { story_id, wrap_mode, columns, .. } => {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: "layout-play-inspector.frame-story".into(),
                        label: "Story".into(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: "layout-play-inspector.frame-story.input".into(),
                            input_kind: "text".into(),
                            value: story_full_content(doc, story_id),
                            placeholder: None,
                            commit: Some("blur".into()),
                            on_change: layout_action(
                                "patchFrame",
                                Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "storyContent" })),
                            ),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        id: "layout-play-inspector.frame-wrapMode".into(),
                        label: "Wrap Mode".into(),
                        child: Box::new(UiNode::Select(UiSelectNode {
                            id: "layout-play-inspector.frame-wrapMode.select".into(),
                            value: wrap_mode.clone(),
                            items: vec![
                                UiSelectItem { value: "none".into(), label: "None".into() },
                                UiSelectItem { value: "box".into(), label: "Box".into() },
                                UiSelectItem { value: "contour".into(), label: "Contour".into() },
                            ],
                            placeholder: None,
                            on_change: layout_action(
                                "patchFrame",
                                Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "wrapMode" })),
                            ),
                        })),
                        description: None,
                        required: None,
                        error: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        id: "layout-play-inspector.frame-columns".into(),
                        label: "Columns".into(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: "layout-play-inspector.frame-columns.input".into(),
                            input_kind: "number".into(),
                            value: format!("{columns}"),
                            placeholder: None,
                            commit: Some("blur".into()),
                            on_change: layout_action(
                                "patchFrame",
                                Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "columns" })),
                            ),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                    }));
                }
                Frame::Image { link_id, .. } => {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: "layout-play-inspector.frame-linkPath".into(),
                        label: "Link Path".into(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: "layout-play-inspector.frame-linkPath.input".into(),
                            input_kind: "text".into(),
                            value: link_path(doc, link_id),
                            placeholder: None,
                            commit: Some("blur".into()),
                            on_change: layout_action(
                                "patchFrame",
                                Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "linkPath" })),
                            ),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                    }));
                }
            }
            let _ = name_mixed;
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
                id: "layout-play-inspector.frame".into(),
                label: "Frame".into(),
                default_open: Some(true),
                fields,
            }]);
        }
    }
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "layout-play-inspector.missing".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        children: vec![ui_text("Selection not found in document.")],
    }])
}

fn build_preflight_tree(play: &LayoutPlayEnvelope) -> UiNode {
    let issues = run_layout_preflight(&play.document);
    let items: Vec<UiTreeItemNode> = if issues.is_empty() {
        vec![tree_item("layout-preflight.empty", "No issues", None, Some("check-circle".into()), None)]
    } else {
        issues
            .iter()
            .map(|issue| {
                tree_item(
                    format!(
                        "layout-preflight.{}.{}",
                        issue.code,
                        issue.object_id.clone().unwrap_or_else(|| issue.message.clone())
                    ),
                    issue.message.clone(),
                    Some(format!("{} · {}", issue.severity, issue.code)),
                    Some(if issue.severity == "error" {
                        "alert-circle"
                    } else {
                        "alert-triangle"
                    }.into()),
                    Some(layout_action("focusPreflightIssue", Some(json!({ "issue": issue })))),
                )
            })
            .collect()
    };
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "layout-preflight.issues".into(),
            label: Some("Preflight".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn layout_window_engagement(play: &LayoutPlayEnvelope, label: &str) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some(format!("layout-engagement-{label}")),
            value: Some(play.runtime.engagement_input.clone()),
            placeholder: Some("undo, redo, export png".into()),
            disabled: None,
            on_change: Some(layout_action("engagementInput", None)),
            on_submit: Some(layout_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: format!("layout-status-{label}"),
            text: format!("Page {}", play.runtime.active_page_id),
        }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "layout.eng.undo".into(), label: "Undo".into(), detail: None, action: Some(layout_action("undo", None)) },
            WindowEngagementPossible { id: "layout.eng.redo".into(), label: "Redo".into(), detail: None, action: Some(layout_action("redo", None)) },
        ]),
    }
}

fn layout_toolbar_tools() -> Vec<ToolNode> {
    vec![
        tool_collection(
            "layout-tools-document",
            "file-text",
            "Document",
            vec![
                tool_button("layout-tools-undo", "rotate-ccw", "Undo", layout_action("undo", None)),
                tool_button("layout-tools-redo", "rotate-cw", "Redo", layout_action("redo", None)),
            ],
        ),
        tool_collection(
            "layout-tools-export",
            "download",
            "Export",
            vec![
                tool_button("layout-tools-export-png", "image", "PNG", layout_action("exportPng", None)),
                tool_button("layout-tools-export-svg", "file-code", "SVG", layout_action("exportSvg", None)),
                tool_button("layout-tools-export-pdf", "file-text", "PDF", layout_action("exportPdf", None)),
                tool_button("layout-tools-export-package", "archive", "Package", layout_action("exportPackage", None)),
            ],
        ),
    ]
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_blueprint(play: &LayoutPlayEnvelope) -> UiNode {
    let camera = &play.document.camera;
    build_canvas_2d_scene(
        LAYOUT_PLAY_SURFACE_BLUEPRINT,
        LAYOUT_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            layers_json: canvas_layers(&play.document, &play.runtime, true),
        },
    )
}

fn render_preview(play: &LayoutPlayEnvelope) -> UiNode {
    let camera = &play.document.preview_camera;
    build_canvas_2d_scene(
        LAYOUT_PLAY_SURFACE_PREVIEW,
        LAYOUT_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            layers_json: canvas_layers(&play.document, &play.runtime, false),
        },
    )
}
//#endregion 🔖Render

//#region 🔖LayoutPlayApp
struct LayoutPlayApp;

impl PluginApp for LayoutPlayApp {
    fn app_id(&self) -> &str {
        LAYOUT_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("layout envelope json")
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match action {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<LayoutPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                play.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&play)];
            }
            "setActivePage" => {
                if let Some(page_id) = args.and_then(|value| value.get("pageId")).and_then(|value| value.as_str()) {
                    play.runtime.active_page_id = page_id.into();
                    return vec![set_document_op(&play)];
                }
            }
            "setHover" => {
                play.runtime.hovered_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&play)];
            }
            "focusPreflightIssue" => {
                if let Some(issue) = args.and_then(|value| value.get("issue")) {
                    if let Some(object_id) = issue.get("objectId").and_then(|value| value.as_str()) {
                        play.runtime.selected_ids = vec![object_id.into()];
                    }
                    if let Some(page_id) = issue.get("pageId").and_then(|value| value.as_str()) {
                        play.runtime.active_page_id = page_id.into();
                    }
                    return vec![set_document_op(&play)];
                }
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
            "addFrame" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("rect");
                let drop_x = args.and_then(|value| value.get("x")).and_then(Value::as_f64);
                let drop_y = args.and_then(|value| value.get("y")).and_then(Value::as_f64);
                let page_id = play.runtime.active_page_id.clone();
                let page_index = play.document.pages.iter().position(|page| page.id == page_id);
                if let Some(page_index) = page_index {
                    let frame_id = format!("frame-{}", play.document.pages[page_index].frames.len() + 1);
                    let layer_id = play.document.pages[page_index]
                        .layer_ids
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "layer-1".into());
                    push_undo(&mut play);
                    let page = &mut play.document.pages[page_index];
                    let frame = match kind {
                        "text" => Frame::Text {
                            id: frame_id.clone(),
                            layer_id,
                            bounds: layout_rs::LayoutBounds {
                                x: drop_x.unwrap_or(48.0),
                                y: drop_y.unwrap_or(120.0),
                                width: 200.0,
                                height: 120.0,
                                rotation: 0.0,
                            },
                            locked: None,
                            visible: None,
                            story_id: play.document.stories.first().map(|story| story.id.clone()).unwrap_or_else(|| "story-1".into()),
                            thread_next: None,
                            columns: 1,
                            inset: layout_rs::LayoutRect { x: 4.0, y: 4.0, width: 192.0, height: 112.0 },
                            wrap_mode: "box".into(),
                        },
                        "image" => Frame::Image {
                            id: frame_id.clone(),
                            layer_id,
                            bounds: layout_rs::LayoutBounds {
                                x: drop_x.unwrap_or(48.0),
                                y: drop_y.unwrap_or(280.0),
                                width: 160.0,
                                height: 120.0,
                                rotation: 0.0,
                            },
                            locked: None,
                            visible: None,
                            link_id: play.document.links.first().map(|link| link.id.clone()).unwrap_or_else(|| "link-missing".into()),
                        },
                        _ => Frame::Rect {
                            id: frame_id.clone(),
                            layer_id,
                            bounds: layout_rs::LayoutBounds {
                                x: drop_x.unwrap_or(48.0),
                                y: drop_y.unwrap_or(48.0),
                                width: 120.0,
                                height: 64.0,
                                rotation: 0.0,
                            },
                            locked: None,
                            visible: None,
                            fill: Some([0.2, 0.24, 0.3, 1.0]),
                            stroke: None,
                        },
                    };
                    if let Some(layer_id_target) = page.layer_ids.first().cloned() {
                        if let Some(layer) = page.layers.iter_mut().find(|layer| layer.id == layer_id_target) {
                            layer.object_ids.push(frame_id.clone());
                        }
                    }
                    page.frames.push(frame);
                    play.runtime.selected_ids = vec![frame_id];
                    return vec![set_document_op(&play)];
                }
            }
            "addPage" => {
                push_undo(&mut play);
                let template = play
                    .document
                    .pages
                    .iter()
                    .find(|page| page.id == play.runtime.active_page_id)
                    .or_else(|| play.document.pages.first());
                let (width, height, spread_id, parent_page_id, margins, columns) = template
                    .map(|page| {
                        (
                            page.width,
                            page.height,
                            page.spread_id.clone(),
                            page.parent_page_id.clone(),
                            page.margins.clone(),
                            page.columns.clone(),
                        )
                    })
                    .unwrap_or((
                        595.0,
                        842.0,
                        "spread-1".into(),
                        None,
                        PageMargins {
                            top: 48.0,
                            right: 36.0,
                            bottom: 48.0,
                            left: 36.0,
                        },
                        PageColumns { count: 1, gutter: 0.0 },
                    ));
                let page_id = format!("page-{}", play.document.pages.len() + 1);
                let layer_id = format!("layer-{page_id}");
                play.document.pages.push(Page {
                    id: page_id.clone(),
                    name: format!("Page {}", play.document.pages.len() + 1),
                    spread_id,
                    parent_page_id,
                    width,
                    height,
                    margins,
                    columns,
                    guides: Vec::new(),
                    layer_ids: vec![layer_id.clone()],
                    layers: vec![layout_rs::Layer {
                        id: layer_id,
                        name: "Content".into(),
                        visible: true,
                        locked: false,
                        object_ids: Vec::new(),
                    }],
                    frames: Vec::new(),
                    overrides: Vec::new(),
                });
                play.runtime.active_page_id = page_id.clone();
                play.runtime.selected_ids = vec![page_id];
                return vec![set_document_op(&play)];
            }
            "patchPage" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| play.runtime.active_page_id.clone());
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                push_undo(&mut play);
                if let Some(page) = play.document.pages.iter_mut().find(|page| page.id == page_id) {
                    match field.as_str() {
                        "name" => {
                            if let Some(name) = value.as_str() {
                                page.name = name.into();
                            }
                        }
                        "width" => {
                            if let Some(width) = value.as_f64() {
                                page.width = width;
                            }
                        }
                        "height" => {
                            if let Some(height) = value.as_f64() {
                                page.height = height;
                            }
                        }
                        "marginTop" => {
                            if let Some(margin) = value.as_f64() {
                                page.margins.top = margin;
                            }
                        }
                        "marginRight" => {
                            if let Some(margin) = value.as_f64() {
                                page.margins.right = margin;
                            }
                        }
                        "marginBottom" => {
                            if let Some(margin) = value.as_f64() {
                                page.margins.bottom = margin;
                            }
                        }
                        "marginLeft" => {
                            if let Some(margin) = value.as_f64() {
                                page.margins.left = margin;
                            }
                        }
                        "columnsCount" => {
                            if let Some(count) = value.as_f64() {
                                page.columns.count = count.max(0.0) as u32;
                            }
                        }
                        "columnsGutter" => {
                            if let Some(gutter) = value.as_f64() {
                                page.columns.gutter = gutter;
                            }
                        }
                        _ => {}
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "patchFrame" => {
                let frame_id = args.and_then(|value| value.get("frameId")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| play.runtime.active_page_id.clone());
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if frame_id.is_empty() {
                    return Vec::new();
                }
                let Some(page_index) = play.document.pages.iter().position(|page| page.id == page_id) else {
                    return Vec::new();
                };
                let Some(frame_index) = play.document.pages[page_index].frames.iter().position(|frame| frame.id() == frame_id) else {
                    return Vec::new();
                };
                push_undo(&mut play);
                match field.as_str() {
                    "x" | "y" | "width" | "w" | "height" | "h" => {
                        if let Some(number) = value.as_f64() {
                            patch_frame_bounds(&mut play.document.pages[page_index].frames[frame_index], &field, number);
                        }
                    }
                    "fill" | "stroke" => {
                        if let Frame::Rect { fill, stroke, .. } = &mut play.document.pages[page_index].frames[frame_index] {
                            let rgba = text_to_rgba(value.as_str().unwrap_or(""));
                            if field == "fill" {
                                *fill = rgba;
                            } else {
                                *stroke = rgba;
                            }
                        }
                    }
                    "wrapMode" => {
                        if let (Frame::Text { wrap_mode, .. }, Some(mode)) =
                            (&mut play.document.pages[page_index].frames[frame_index], value.as_str())
                        {
                            *wrap_mode = mode.into();
                        }
                    }
                    "columns" => {
                        if let (Frame::Text { columns, .. }, Some(count)) =
                            (&mut play.document.pages[page_index].frames[frame_index], value.as_f64())
                        {
                            *columns = count.max(0.0) as u32;
                        }
                    }
                    "storyContent" => {
                        let story_id = match &play.document.pages[page_index].frames[frame_index] {
                            Frame::Text { story_id, .. } => Some(story_id.clone()),
                            _ => None,
                        };
                        if let (Some(story_id), Some(content)) = (story_id, value.as_str()) {
                            if let Some(story) = play.document.stories.iter_mut().find(|story| story.id == story_id) {
                                story.content = content.into();
                            }
                        }
                    }
                    "linkPath" => {
                        let link_id = match &play.document.pages[page_index].frames[frame_index] {
                            Frame::Image { link_id, .. } => Some(link_id.clone()),
                            _ => None,
                        };
                        if let (Some(link_id), Some(path)) = (link_id, value.as_str()) {
                            if let Some(link) = play.document.links.iter_mut().find(|link| link.id == link_id) {
                                link.path = path.into();
                            }
                        }
                    }
                    _ => {}
                }
                return vec![set_document_op(&play)];
            }
            "exportPng" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(play.runtime.active_page_id.as_str())
                    .to_string();
                return match export_document_png_cpu(&play.document, &page_id) {
                    Ok(bytes) => vec![download_media_export_op(
                        &format!("{page_id}.png"),
                        "image/png",
                        &base64::engine::general_purpose::STANDARD.encode(bytes),
                    )],
                    Err(_) => Vec::new(),
                };
            }
            "exportSvg" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(play.runtime.active_page_id.as_str())
                    .to_string();
                return match export_document_svg(&play.document, &page_id) {
                    Ok(svg) => vec![download_media_export_op(&format!("{page_id}.svg"), "image/svg+xml", &svg)],
                    Err(_) => Vec::new(),
                };
            }
            "exportPdf" => {
                let page_id = args
                    .and_then(|value| value.get("pageId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(play.runtime.active_page_id.as_str())
                    .to_string();
                return match export_document_pdf(&play.document, &page_id) {
                    Ok(bytes) => vec![download_media_export_op(
                        &format!("{page_id}.pdf"),
                        "application/pdf",
                        &base64::engine::general_purpose::STANDARD.encode(bytes),
                    )],
                    Err(_) => Vec::new(),
                };
            }
            "exportPackage" => {
                let preflight_json = serde_json::to_string(&run_layout_preflight(&play.document)).unwrap_or_else(|_| "[]".into());
                let doc_json = serde_json::to_string(&play.document).unwrap_or_default();
                return match export_package_zip(&doc_json, &preflight_json) {
                    Ok(bytes) => vec![download_media_export_op(
                        &format!("{}.layout-package.zip", play.document.name),
                        "application/zip",
                        &base64::engine::general_purpose::STANDARD.encode(bytes),
                    )],
                    Err(_) => Vec::new(),
                };
            }
            "setCamera" => {
                let blueprint = surface_is_blueprint(args);
                if let Some(camera_value) = args.and_then(|value| value.get("camera")) {
                    let x = camera_value.get("x").and_then(Value::as_f64);
                    let y = camera_value.get("y").and_then(Value::as_f64);
                    let zoom = camera_value.get("zoom").and_then(Value::as_f64);
                    if let (Some(x), Some(y), Some(zoom)) = (x, y, zoom) {
                        let camera = camera_for_surface(&mut play.document, blueprint);
                        camera.x = x;
                        camera.y = y;
                        camera.zoom = zoom;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "canvasPointerDown" => {
                let blueprint = surface_is_blueprint(args);
                let button = args.and_then(|value| value.get("button")).and_then(Value::as_i64).unwrap_or(0);
                if !blueprint || button != 0 {
                    return Vec::new();
                }
                let extend = args.and_then(|value| value.get("extend")).and_then(Value::as_bool).unwrap_or(false);
                let hit = hit_test_at(&play, args, blueprint);
                play.runtime.selected_ids = match hit {
                    Some(id) if extend => {
                        let mut ids = play.runtime.selected_ids.clone();
                        if let Some(position) = ids.iter().position(|existing| *existing == id) {
                            ids.remove(position);
                        } else {
                            ids.push(id);
                        }
                        ids
                    }
                    Some(id) => vec![id],
                    None => Vec::new(),
                };
                return vec![set_document_op(&play)];
            }
            "canvasPointerMove" => {
                let blueprint = surface_is_blueprint(args);
                if !blueprint {
                    return Vec::new();
                }
                let hit = hit_test_at(&play, args, blueprint);
                if hit == play.runtime.hovered_id {
                    return Vec::new();
                }
                play.runtime.hovered_id = hit;
                return vec![set_document_op(&play)];
            }
            "canvasPointerUp" => {
                return Vec::new();
            }
            "canvasDragOver" => {
                let blueprint = surface_is_blueprint(args);
                if !blueprint {
                    return Vec::new();
                }
                let kind = args
                    .and_then(|value| value.get("types"))
                    .and_then(|value| value.as_array())
                    .and_then(|types| {
                        types
                            .iter()
                            .find_map(|entry| entry.as_str().and_then(|type_value| type_value.strip_prefix(LAYOUT_CATALOGUE_KIND_MIME_PREFIX)).map(str::to_string))
                    })
                    .unwrap_or_else(|| "unknown".into());
                let (sx, sy, width, height) = pointer_args(args);
                let (wx, wy) = screen_to_world_for_surface(&play, blueprint, sx, sy, width, height);
                let unchanged = play
                    .runtime
                    .drop_preview
                    .as_ref()
                    .is_some_and(|preview| preview.kind == kind && (preview.x - wx).abs() < 1.0 && (preview.y - wy).abs() < 1.0);
                if unchanged {
                    return Vec::new();
                }
                play.runtime.drop_preview = Some(LayoutDropPreviewState { kind, x: wx, y: wy });
                return vec![set_document_op(&play)];
            }
            "canvasDragLeave" => {
                if play.runtime.drop_preview.is_none() {
                    return Vec::new();
                }
                play.runtime.drop_preview = None;
                return vec![set_document_op(&play)];
            }
            "canvasDrop" => {
                let blueprint = surface_is_blueprint(args);
                play.runtime.drop_preview = None;
                let cleared_json = serde_json::to_string(&play).unwrap_or_default();
                if !blueprint {
                    return vec![set_document_op(&play)];
                }
                let Some(payload) = args
                    .and_then(|value| value.get("dragData"))
                    .and_then(Value::as_str)
                    .and_then(|raw| serde_json::from_str::<Value>(raw).ok())
                else {
                    return vec![set_document_op(&play)];
                };
                let Some(kind) = payload.get("kind").and_then(Value::as_str).map(str::to_string) else {
                    return vec![set_document_op(&play)];
                };
                let (sx, sy, width, height) = pointer_args(args);
                let (wx, wy) = screen_to_world_for_surface(&play, blueprint, sx, sy, width, height);
                if kind == "page" {
                    return self.handle_action_patch_ops("addPage", None, &cleared_json, _view_state);
                }
                return self.handle_action_patch_ops("addFrame", Some(&json!({ "kind": kind, "x": wx, "y": wy })), &cleared_json, _view_state);
            }
            "engagementInput" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_str) {
                    play.runtime.engagement_input = value.into();
                    return vec![set_document_op(&play)];
                }
            }
            "engagementSubmit" => {
                let typed = args.and_then(|value| value.get("value")).and_then(Value::as_str).map(str::trim).map(str::to_lowercase).unwrap_or_default();
                let action = match typed.as_str() {
                    "undo" => Some("undo"),
                    "redo" => Some("redo"),
                    "export png" | "png" => Some("exportPng"),
                    "export svg" | "svg" => Some("exportSvg"),
                    "export pdf" | "pdf" => Some("exportPdf"),
                    "export package" | "package" => Some("exportPackage"),
                    _ => None,
                };
                if let Some(action) = action {
                    return self.handle_action_patch_ops(action, None, document_json, _view_state);
                }
                return Vec::new();
            }
            _ => {}
        }
        Vec::new()
    }

    fn tools(&self, _document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        layout_toolbar_tools()
    }

    fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let play = parse_envelope(document_json);
        HashMap::from([
            (LAYOUT_PLAY_WINDOW_BLUEPRINT.to_string(), layout_window_engagement(&play, "blueprint")),
            (LAYOUT_PLAY_WINDOW_PREVIEW.to_string(), layout_window_engagement(&play, "preview")),
        ])
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => render_blueprint(&play),
            LAYOUT_PLAY_BODY_PREVIEW => render_preview(&play),
            LAYOUT_PLAY_BODY_DOCUMENT => build_document_tree(&play),
            LAYOUT_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            LAYOUT_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            LAYOUT_PLAY_BODY_PREFLIGHT => build_preflight_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖LayoutPlayApp

//#region 🔖AppFactory
fn create_layout_app() -> App {
    App::from_builder(
        App::builder(LAYOUT_PLAY_APP_ID, "Layout").document(["semio", "layout"])
            .icon_id("layout")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(LAYOUT_PLAY_WINDOW_BLUEPRINT, "Blueprint", LAYOUT_PLAY_BODY_BLUEPRINT, SurfaceKind::Canvas2d)
            .window_kind(LAYOUT_PLAY_WINDOW_PREVIEW, "Preview", LAYOUT_PLAY_BODY_PREVIEW, SurfaceKind::Canvas2d)
            .default_layout(create_default_layout(
                &[LAYOUT_PLAY_WINDOW_BLUEPRINT.into(), LAYOUT_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[55.0, 45.0]),
                Some(&["Blueprint".into(), "Preview".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                LAYOUT_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                LAYOUT_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(LAYOUT_PLAY_PREFLIGHT_TAB_ID, "Preflight", PanelGroup::Workbench, LAYOUT_PLAY_BODY_PREFLIGHT)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                LAYOUT_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("sample", "Sample", LAYOUT_SAMPLE_JSON)
    .program("layout", "Layout", "layout")
}

fn layout_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::pages_rects_svg(value, "Layout")
}

/// 📥 Extracts axis-aligned rectangular boundaries from closed 4-vertex `LwPolyline`s and frames one page per rectangle, falling back to a single page framed to the drawing extents.
fn dwg_rect_pages(drawing: &semio_framework_os::DwgDrawing) -> Vec<(f64, f64, f64, f64)> {
    let mut rects = Vec::new();
    for entity in &drawing.entities {
        let semio_framework_os::DwgGeometry::LwPolyline { closed: true, vertices, .. } = &entity.geometry else { continue };
        if vertices.len() != 4 {
            continue;
        }
        let (min_x, max_x) = vertices.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), v| (min.min(v[0]), max.max(v[0])));
        let (min_y, max_y) = vertices.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), v| (min.min(v[1]), max.max(v[1])));
        let is_axis_aligned = vertices
            .iter()
            .all(|v| ((v[0] - min_x).abs() < 1e-6 || (v[0] - max_x).abs() < 1e-6) && ((v[1] - min_y).abs() < 1e-6 || (v[1] - max_y).abs() < 1e-6));
        if is_axis_aligned && max_x > min_x && max_y > min_y {
            rects.push((min_x, min_y, max_x - min_x, max_y - min_y));
        }
    }
    if rects.is_empty() {
        rects.push((
            drawing.extmin[0],
            drawing.extmin[1],
            (drawing.extmax[0] - drawing.extmin[0]).max(1.0),
            (drawing.extmax[1] - drawing.extmin[1]).max(1.0),
        ));
    }
    rects
}

/// 📥 Builds a schema-valid layout document from a parsed DWG drawing, framing one page per rectangular boundary found.
fn layout_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let pages: Vec<Page> = dwg_rect_pages(drawing)
        .into_iter()
        .enumerate()
        .map(|(index, (_x, _y, width, height))| {
            let id = format!("page-{}", index + 1);
            let layer_id = format!("layer-{id}");
            Page {
                id: id.clone(),
                name: format!("Page {}", index + 1),
                spread_id: "spread-1".into(),
                parent_page_id: None,
                width,
                height,
                margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: vec![layer_id.clone()],
                layers: vec![layout_rs::Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }],
                frames: Vec::new(),
                overrides: Vec::new(),
            }
        })
        .collect();
    let page_ids = pages.iter().map(|page| page.id.clone()).collect();
    let document = LayoutDocument {
        schema: LAYOUT_FIXTURE_SCHEMA.into(),
        name: "Imported DWG".into(),
        camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        preview_camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        grid: layout_rs::GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: vec![layout_rs::Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids }],
        pages,
        print_target: None,
    };
    serde_json::to_value(document).map_err(|e| e.to_string())
}

fn register_layout_exports() {
    semio_framework_os::register_2d_export_handlers("2d.layout", "layout", layout_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.layout", layout_document_json_from_dwg);
}

fn bundle() -> PluginBundle {
    register_layout_exports();
    PluginBundle::new("layout", "Layout", "0.1.0").register_app(create_layout_app(), || Box::new(LayoutPlayApp))
}

semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_blueprint_canvas_scene() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_preview_canvas_scene() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn document_lists_sample_pages() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("layout-document.page.page-1"));
        assert!(json.contains("Page 1"));
    }

    #[test]
    fn catalogue_lists_frame_kinds() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("layout-catalogue.rect"));
        assert!(json.contains("Text Frame"));
    }

    #[test]
    fn preflight_finds_missing_asset() {
        let play = default_envelope();
        let issues = run_layout_preflight(&play.document);
        assert!(issues.iter().any(|issue| issue.code == "asset.missing"));
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_PREFLIGHT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("asset.missing") || json.contains("Linked asset missing"));
    }

    #[test]
    fn set_selection_updates_runtime() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "setSelection",
            Some(&json!({ "ids": ["frame-text-1"] })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.selected_ids, vec!["frame-text-1".to_string()]);
    }

    #[test]
    fn sample_fixture_parses() {
        let doc = parse_layout_document(LAYOUT_SAMPLE_JSON).expect("sample fixture");
        assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn add_frame_action_appends_rect() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let before: LayoutPlayEnvelope = serde_json::from_str(&document).expect("parse envelope");
        let before_count = before.document.pages[0].frames.len();
        let ops = app.handle_action_patch_ops("addFrame", Some(&json!({ "kind": "rect" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.document.pages[0].frames.len(), before_count + 1);
    }

    #[test]
    fn patch_page_supports_margins_and_columns() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        for (field, value) in [
            ("marginTop", 60.0),
            ("marginRight", 40.0),
            ("marginBottom", 60.0),
            ("marginLeft", 40.0),
            ("columnsGutter", 18.0),
        ] {
            let ops = app.handle_action_patch_ops(
                "patchPage",
                Some(&json!({ "pageId": "page-1", "field": field, "value": value })),
                &document,
                &ViewState::default(),
            );
            assert_eq!(ops.len(), 1, "field {field} should apply");
        }
        let ops = app.handle_action_patch_ops(
            "patchPage",
            Some(&json!({ "pageId": "page-1", "field": "columnsCount", "value": 3 })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let page = next.document.pages.iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    fn patch_frame_supports_rect_fill_and_stroke() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("addFrame", Some(&json!({ "kind": "rect" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let after_add: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let document = serde_json::to_string(&after_add).unwrap();
        let frame_id = after_add.runtime.selected_ids[0].clone();

        let ops = app.handle_action_patch_ops(
            "patchFrame",
            Some(&json!({ "frameId": frame_id, "pageId": "page-1", "field": "fill", "value": "0.5, 0.4, 0.3, 1" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let frame = next.document.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect frame") };
        assert_eq!(fill.unwrap(), [0.5, 0.4, 0.3, 1.0]);
    }

    #[test]
    fn patch_frame_supports_text_story_content_and_wrap_mode() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "patchFrame",
            Some(&json!({ "frameId": "frame-text-1", "pageId": "page-1", "field": "storyContent", "value": "Edited story body." })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let story = next.document.stories.iter().find(|story| story.id == "story-1").unwrap();
        assert_eq!(story.content, "Edited story body.");

        let document = serde_json::to_string(&next).unwrap();
        let ops = app.handle_action_patch_ops(
            "patchFrame",
            Some(&json!({ "frameId": "frame-text-1", "pageId": "page-1", "field": "wrapMode", "value": "contour" })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let frame = next.document.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
        let Frame::Text { wrap_mode, .. } = frame else { panic!("expected text frame") };
        assert_eq!(wrap_mode, "contour");
    }

    #[test]
    fn patch_frame_supports_image_link_path() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "patchFrame",
            Some(&json!({ "frameId": "frame-image-1", "pageId": "page-1", "field": "linkPath", "value": "assets/updated.png" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let link = next.document.links.iter().find(|link| link.id == "link-missing").unwrap();
        assert_eq!(link.path, "assets/updated.png");
    }

    #[test]
    fn export_actions_wire_to_real_layout_rs_exporters() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        for (action, mime_type) in [
            ("exportPng", "image/png"),
            ("exportSvg", "image/svg+xml"),
            ("exportPdf", "application/pdf"),
            ("exportPackage", "application/zip"),
        ] {
            let ops = app.handle_action_patch_ops(action, Some(&json!({ "pageId": "page-1" })), &document, &ViewState::default());
            assert_eq!(ops.len(), 1, "{action} should emit a download op");
            let payload: Value = serde_json::from_str(&ops[0]).unwrap();
            assert_eq!(payload["op"], "downloadMediaExport");
            assert_eq!(payload["mimeType"], mime_type);
            assert!(!payload["data"].as_str().unwrap_or("").is_empty());
        }
    }

    fn test_screen_point(camera_x: f64, camera_y: f64, zoom: f64, width: f64, height: f64, world_x: f64, world_y: f64) -> (f64, f64) {
        let camera = layout_rs::cavas::camera::Camera { x: camera_x, y: camera_y, zoom };
        let viewport = layout_rs::cavas::camera::Viewport { width: width as u32, height: height as u32, dpr: 1.0 };
        let screen = layout_rs::cavas::camera::world_to_screen(&camera, &viewport, layout_rs::cavas::Point::new(world_x, world_y));
        (screen.x, screen.y)
    }

    fn scene_layers_json(node: &UiNode) -> String {
        let value: Value = serde_json::to_value(node).unwrap();
        value["canvas2d"]["layersJson"].as_str().expect("layersJson string").to_string()
    }

    #[test]
    fn blueprint_scene_has_page_background_and_guides() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("layout.page-bg"));
        assert!(layers_json.contains("0.97"));
        assert!(layers_json.contains("layout.guide.margin"));
        assert!(layers_json.contains("layout.guide.column"));
        assert!(layers_json.contains("\"segments\""));
        assert!(layers_json.contains("\"fill\":{\"color\""));
        assert!(!layers_json.contains("\"linkId\""));
    }

    #[test]
    fn preview_scene_has_white_background_and_no_guides() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("layout.page-bg"));
        assert!(!layers_json.contains("layout.guide."));
    }

    #[test]
    fn inherited_frame_gets_dashed_stroke_in_blueprint() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("\"dash\":[4.0,3.0]"));
    }

    #[test]
    fn selected_and_hovered_frames_get_chrome_strokes() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setSelection", Some(&json!({ "ids": ["frame-text-1"] })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let selected: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let document = serde_json::to_string(&selected).unwrap();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("2.5"));

        let ops = app.handle_action_patch_ops("setHover", Some(&json!({ "id": "frame-image-1" })), &document, &ViewState::default());
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let hovered: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let document = serde_json::to_string(&hovered).unwrap();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("1.75"));
    }

    #[test]
    fn set_camera_updates_surface_camera() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "setCamera",
            Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "camera": { "x": 10.0, "y": 20.0, "zoom": 1.5 } })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.document.camera.x, 10.0);
        assert_eq!(next.document.camera.y, 20.0);
        assert_eq!(next.document.camera.zoom, 1.5);
        assert_eq!(next.document.preview_camera.x, 0.0);
    }

    #[test]
    fn pointer_down_selects_frame_via_hit_test() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let (sx, sy) = test_screen_point(0.0, 0.0, 0.5, 800.0, 600.0, 136.0, 435.0);
        let ops = app.handle_action_patch_ops(
            "canvasPointerDown",
            Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": sx, "y": sy, "width": 800.0, "height": 600.0, "button": 0 })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.selected_ids, vec!["frame-image-1".to_string()]);
    }

    #[test]
    fn pointer_move_hover_is_change_only() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let (sx, sy) = test_screen_point(0.0, 0.0, 0.5, 800.0, 600.0, 156.0, 220.0);
        let args = json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": sx, "y": sy, "width": 800.0, "height": 600.0 });
        let ops = app.handle_action_patch_ops("canvasPointerMove", Some(&args), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.hovered_id.as_deref(), Some("frame-text-1"));
        let document = serde_json::to_string(&next).unwrap();
        let ops = app.handle_action_patch_ops("canvasPointerMove", Some(&args), &document, &ViewState::default());
        assert!(ops.is_empty());
    }

    #[test]
    fn canvas_drop_adds_frame_at_world_coords() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let (sx, sy) = test_screen_point(0.0, 0.0, 0.5, 800.0, 600.0, 100.0, 200.0);
        let drag_data = json!({ "kind": "rect" }).to_string();
        let ops = app.handle_action_patch_ops(
            "canvasDrop",
            Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": sx, "y": sy, "width": 800.0, "height": 600.0, "dragData": drag_data })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let frame_id = next.runtime.selected_ids[0].clone();
        let frame = next.document.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
        let bounds = frame.bounds();
        assert!((bounds.x - 100.0).abs() < 0.01);
        assert!((bounds.y - 200.0).abs() < 0.01);
    }

    #[test]
    fn canvas_drop_page_kind_adds_page() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let before: LayoutPlayEnvelope = serde_json::from_str(&document).unwrap();
        let before_count = before.document.pages.len();
        let drag_data = json!({ "kind": "page" }).to_string();
        let ops = app.handle_action_patch_ops(
            "canvasDrop",
            Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT, "x": 0.0, "y": 0.0, "width": 800.0, "height": 600.0, "dragData": drag_data })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.document.pages.len(), before_count + 1);
    }

    #[test]
    fn drag_over_emits_ghost_and_leave_clears() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "canvasDragOver",
            Some(&json!({
                "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT,
                "x": 400.0, "y": 300.0, "width": 800.0, "height": 600.0,
                "types": [format!("{LAYOUT_CATALOGUE_KIND_MIME_PREFIX}rect")],
            })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let with_preview: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert!(with_preview.runtime.drop_preview.is_some());
        let document = serde_json::to_string(&with_preview).unwrap();
        let scene = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let json_str = serde_json::to_string(&scene).unwrap();
        assert!(json_str.contains("layout.drop-preview"));

        let ops = app.handle_action_patch_ops("canvasDragLeave", Some(&json!({ "surfaceId": LAYOUT_PLAY_SURFACE_BLUEPRINT })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let cleared: LayoutPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert!(cleared.runtime.drop_preview.is_none());
    }

    #[test]
    fn catalogue_items_are_draggable() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains(LAYOUT_CATALOGUE_DRAG_MIME));
        assert!(json_str.contains("\"draggable\":true"));
        assert!(json_str.contains("layout-catalogue.page"));
    }

    #[test]
    fn document_tree_has_nine_sections() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let node = app.render(LAYOUT_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json_str = serde_json::to_string(&node).unwrap();
        for section_id in [
            "layout-document.document",
            "layout-document.spreads",
            "layout-document.pages",
            "layout-document.frames",
            "layout-document.parentPages",
            "layout-document.layers",
            "layout-document.stories",
            "layout-document.links",
            "layout-document.styles",
        ] {
            assert!(json_str.contains(section_id), "missing section {section_id}");
        }
    }

    #[test]
    fn preflight_reports_all_expected_issue_codes() {
        let json = r#"{
            "schema": "layout.fixture",
            "name": "Preflight Fixture",
            "camera": {"x":0,"y":0,"zoom":1},
            "previewCamera": {"x":0,"y":0,"zoom":1},
            "grid": {"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},
            "paragraphStyles": [{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],
            "characterStyles": [
                {"id":"character.small","fontFamily":"Layout Sans","fontSize":6},
                {"id":"character.exotic","fontFamily":"Comic Sans","fontSize":10}
            ],
            "stories": [
                {"id":"story-small","content":"Small caption text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.small"}]},
                {"id":"story-exotic","content":"Exotic font text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.exotic"}]},
                {"id":"story-overset","content":"placeholder","styleRuns":[]}
            ],
            "links": [
                {"id":"link-missing","path":"a.png","hash":"sha256:missing","width":100,"height":100,"dpi":300,"state":"missing"},
                {"id":"link-modified","path":"b.png","hash":"sha256:abc","width":100,"height":100,"dpi":300,"state":"modified"},
                {"id":"link-lowres","path":"c.png","hash":"sha256:def","width":100,"height":100,"dpi":72},
                {"id":"link-rgb","path":"d.png","hash":"sha256:ghi","width":100,"height":100,"dpi":300,"colorProfile":"RGB"}
            ],
            "parentPages": [],
            "spreads": [{"id":"spread-1","name":"Spread 1","pageIds":["page-1"]}],
            "pages": [{
                "id":"page-1","name":"Page 1","spreadId":"spread-1","width":200,"height":200,
                "margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},
                "guides":[], "layerIds":["layer-1"],
                "layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-oob","frame-missing","frame-modified","frame-lowres","frame-no-story","frame-small","frame-exotic","frame-overset"]}],
                "frames":[
                    {"id":"frame-oob","layerId":"layer-1","kind":"rect","bounds":{"x":150,"y":150,"w":100,"h":100,"rotation":0},"fill":[0,0,0,1]},
                    {"id":"frame-missing","layerId":"layer-1","kind":"image","bounds":{"x":0,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-missing"},
                    {"id":"frame-modified","layerId":"layer-1","kind":"image","bounds":{"x":20,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-modified"},
                    {"id":"frame-lowres","layerId":"layer-1","kind":"image","bounds":{"x":40,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-lowres"},
                    {"id":"frame-no-story","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":40,"w":50,"h":20,"rotation":0},"storyId":"story-absent","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-small","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":60,"w":50,"h":20,"rotation":0},"storyId":"story-small","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-exotic","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":80,"w":50,"h":20,"rotation":0},"storyId":"story-exotic","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-overset","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":100,"w":50,"h":20,"rotation":0},"storyId":"story-overset","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"}
                ],
                "overrides":[]
            }],
            "printTarget":"print"
        }"#;
        let mut doc = parse_layout_document(json).expect("preflight fixture");
        if let Some(story) = doc.stories.iter_mut().find(|story| story.id == "story-overset") {
            story.content = "a".repeat(450);
        }
        let issues = run_layout_preflight(&doc);
        let codes: Vec<&str> = issues.iter().map(|issue| issue.code.as_str()).collect();
        for expected in [
            "object.out_of_bounds",
            "asset.missing",
            "asset.modified",
            "asset.low_resolution",
            "image.empty_frame",
            "text.missing_story",
            "text.below_minimum_size",
            "font.missing",
            "text.overset",
            "asset.rgb_in_print",
        ] {
            assert!(codes.contains(&expected), "missing preflight code: {expected}");
        }
    }

    #[test]
    fn window_engagements_cover_both_windows() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let engagements = app.window_engagements(&document, &ViewState::default());
        let blueprint = engagements.get(LAYOUT_PLAY_WINDOW_BLUEPRINT).expect("blueprint engagement");
        let status = blueprint.status.as_ref().and_then(|rows| rows.first()).expect("status");
        assert!(status.text.contains("Page"));
        let input = blueprint.input.as_ref().expect("input");
        assert_eq!(input.placeholder.as_deref(), Some("undo, redo, export png"));
        assert!(engagements.contains_key(LAYOUT_PLAY_WINDOW_PREVIEW));
    }

    #[test]
    fn tools_expose_undo_redo_and_exports() {
        let app = LayoutPlayApp;
        let document = app.initial_document_json();
        let tools = app.tools(&document, &ViewState::default());
        let json_str = serde_json::to_string(&tools).unwrap();
        for needle in [
            "layout-tools-undo",
            "layout-tools-redo",
            "layout-tools-export-png",
            "layout-tools-export-svg",
            "layout-tools-export-pdf",
            "layout-tools-export-package",
        ] {
            assert!(json_str.contains(needle), "missing tool {needle}");
        }
    }

    #[test]
    fn engagement_submit_triggers_export() {
        let mut app = LayoutPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("engagementSubmit", Some(&json!({ "value": "export png" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        assert_eq!(payload["op"], "downloadMediaExport");
        assert_eq!(payload["mimeType"], "image/png");
    }

    #[test]
    fn dwg_import_frames_page_to_rectangular_polyline() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer: 0,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::LwPolyline {
                closed: true,
                elevation: 0.0,
                vertices: vec![[10.0, 20.0], [110.0, 20.0], [110.0, 70.0], [10.0, 70.0]],
                bulges: vec![0.0; 4],
            },
        });
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutDocument = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 100.0);
        assert_eq!(document.pages[0].height, 50.0);
    }

    #[test]
    fn dwg_import_without_rectangles_falls_back_to_extents() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer: 0,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [200.0, 150.0, 0.0] },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [200.0, 150.0, 0.0];
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutDocument = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 200.0);
        assert_eq!(document.pages[0].height, 150.0);
    }
}
//#endregion 🧪Tests
