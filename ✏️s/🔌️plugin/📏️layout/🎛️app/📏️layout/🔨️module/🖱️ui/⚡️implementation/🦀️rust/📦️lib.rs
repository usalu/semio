//! 🖥️ Layout app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! flip — `LayoutPlayApp` is a unit struct; every former `LayoutConfig` field (active page,
//! selection, hover, drop-ghost, camera poses, engagement draft) now lives in
//! `layout_engine::LayoutConfig`, written via `layout_op::LayoutConfigOperation`s (real `backwards`, no
//! ad hoc inverse); every action dispatches through the single typed `layout_protocol::LayoutCommand`
//! channel via `DocumentApp::handle`.

use base64::Engine;
use layout::{Frame, FramePatch, ImageLinkPatch, LayoutCamera, LayoutDocument, PagePatch, TextStoryPatch, LAYOUT_FIXTURE_SCHEMA, Page, PageColumns, PageMargins};
use layout_engine::{build_display_list_for_page, export_document_pdf, export_document_png_cpu, export_document_svg, export_package_zip, parse_layout_document, resolve_page, DisplayList, LayoutConfig, LayoutDropPreviewState};
use layout_op::{LayoutConfigOperation, LayoutOperation};
use layout_protocol::LayoutCommand;
use semio_framework_core::kernel::HostEffect;
use semio_framework_plugin::{SurfaceKind,
    build_canvas_2d_scene, create_default_layout, engagement_token_matches, localized_label_map,
    tree_item_desc, tree_item_with_action, tree_item_with_action_draggable, ui_declarative_sections_to_tree,
    ui_inspector_groups_to_tree, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef,
    ActionArgOption, ActionDefinition, ActionKind, App, AppLabelsOverlay, AppLabelsOverlayExt,
    Canvas2dScene, ActionDescriptor, ConfigView, DocumentApp, DocumentView, Emit, LocaleLabels, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder, ArtifactKindSpec,
    IconName, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiSelectItem, UiSelectNode, UiTreeItemNode,
    WindowEngagement, WindowEngagementInput, WindowEngagementPossible, WindowEngagementStatus,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use protocol::CollectionOperation;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
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

const LAYOUT_CATALOGUE_KINDS: &[(&str, &str)] = &[("rect", "square"), ("text", "type"), ("image", "image")];

const LAYOUT_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
const LAYOUT_CATALOGUE_KIND_MIME_PREFIX: &str = "application/x-semio-catalogue-kind.";
const LAYOUT_DROP_PREVIEW_WIDTH: f64 = 200.0;
const LAYOUT_DROP_PREVIEW_HEIGHT: f64 = 120.0;
//#endregion 🔖️Constants

//#region 🔖️Types
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

//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
fn layout_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: LAYOUT_PLAY_APP_ID.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

fn active_page<'a>(doc: &'a LayoutDocument, config: &LayoutConfig) -> Option<&'a Page> {
    doc.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| doc.pages.first())
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

/// 🩹️ Builds the `PagePatch` for a `patchPage` field write from the command's text `value`; unknown
/// fields/mistyped (non-numeric where a number is expected) values yield `None`.
fn page_patch_for_field(field: &str, value: &str) -> Option<PagePatch> {
    match field {
        "name" => Some(PagePatch { name: Some(value.into()), ..Default::default() }),
        "width" => value.parse::<f64>().ok().map(|v| PagePatch { width: Some(v), ..Default::default() }),
        "height" => value.parse::<f64>().ok().map(|v| PagePatch { height: Some(v), ..Default::default() }),
        "marginTop" => value.parse::<f64>().ok().map(|v| PagePatch { margin_top: Some(v), ..Default::default() }),
        "marginRight" => value.parse::<f64>().ok().map(|v| PagePatch { margin_right: Some(v), ..Default::default() }),
        "marginBottom" => value.parse::<f64>().ok().map(|v| PagePatch { margin_bottom: Some(v), ..Default::default() }),
        "marginLeft" => value.parse::<f64>().ok().map(|v| PagePatch { margin_left: Some(v), ..Default::default() }),
        "columnsCount" => value.parse::<f64>().ok().map(|v| PagePatch { columns_count: Some(v.max(0.0) as u32), ..Default::default() }),
        "columnsGutter" => value.parse::<f64>().ok().map(|v| PagePatch { columns_gutter: Some(v), ..Default::default() }),
        _ => None,
    }
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

//#region 🔖️CanvasScene
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

fn display_list_to_host_layers(list: &DisplayList, blueprint: bool, drop_preview: &LayoutDropPreviewState) -> Vec<Value> {
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
        if !drop_preview.kind.is_empty() && drop_preview.kind != "page" {
            let segments = rect_segments(drop_preview.x, drop_preview.y, LAYOUT_DROP_PREVIEW_WIDTH, LAYOUT_DROP_PREVIEW_HEIGHT);
            let fill = drop_preview_fill(&drop_preview.kind);
            layers.push(host_layer("layout.drop-preview", segments, Some(fill), Some(([0.1, 0.45, 0.95, 0.85], 2.0, None))));
        }
    }

    layers
}

fn canvas_layers(doc: &LayoutDocument, config: &LayoutConfig, blueprint: bool) -> String {
    let page = match active_page(doc, config) {
        Some(page) => page,
        None => return "[]".into(),
    };
    let list = build_display_list_for_page(doc, page, &page.id, &config.selected_ids, config.hovered_id.as_deref(), blueprint);
    let layers = display_list_to_host_layers(&list, blueprint, &config.drop_preview);
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖️CanvasScene

//#region 🔖️PointerCamera
fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

fn screen_to_world_for_surface(config: &LayoutConfig, blueprint: bool, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_runtime = if blueprint { &config.camera } else { &config.preview_camera };
    let camera = infinite_canvas::camera::Camera { x: camera_runtime.x, y: camera_runtime.y, zoom: camera_runtime.zoom.max(0.0001) };
    let viewport = infinite_canvas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(sx, sy));
    (world.x, world.y)
}

#[allow(clippy::too_many_arguments)]
fn hit_test_at(doc: &LayoutDocument, config: &LayoutConfig, sx: f64, sy: f64, width: f64, height: f64, blueprint: bool) -> Option<String> {
    let page = active_page(doc, config)?;
    let (wx, wy) = screen_to_world_for_surface(config, blueprint, sx, sy, width, height);
    let list = build_display_list_for_page(doc, page, &page.id, &config.selected_ids, config.hovered_id.as_deref(), blueprint);
    list.hit_test(wx as f32, wy as f32)
}
//#endregion 🔖️PointerCamera

/// 🩹️ Builds the bounds `FramePatch` for an `x`/`y`/`width`/`height` frame field write.
fn frame_bounds_patch(field: &str, value: f64) -> FramePatch {
    match field {
        "x" => FramePatch { x: Some(value), ..Default::default() },
        "y" => FramePatch { y: Some(value), ..Default::default() },
        "width" | "w" => FramePatch { width: Some(value), ..Default::default() },
        "height" | "h" => FramePatch { height: Some(value), ..Default::default() },
        _ => FramePatch::default(),
    }
}

fn resolve_link_state(link: &layout::ImageLink) -> &str {
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
        if let Some(character) = doc.character_styles.iter().find(|style| style.id == character_id) {
            if let Some(font_family) = &character.font_family {
                family = font_family.clone();
            }
            if let Some(font_size) = character.font_size {
                size = font_size;
            }
        }
    }
    (family, size)
}

fn run_layout_preflight(doc: &LayoutDocument, labels: &LayoutLabels) -> Vec<PreflightIssue> {
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
                    message: preflight_msg(labels.preflight_out_of_bounds, &[frame.id()]),
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
                            message: preflight_msg(labels.preflight_asset_missing, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        Some("modified") => issues.push(PreflightIssue {
                            severity: "warning".into(),
                            code: "asset.modified".into(),
                            message: preflight_msg(labels.preflight_asset_modified, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        Some("low_resolution") => issues.push(PreflightIssue {
                            severity: "warning".into(),
                            code: "asset.low_resolution".into(),
                            message: preflight_msg(labels.preflight_asset_low_resolution, &[frame.id()]),
                            object_id: Some(frame.id().into()),
                            page_id: Some(page.id.clone()),
                        }),
                        _ => {}
                    }
                    if link.is_some_and(|entry| entry.proxy_data_url.is_none()) && bounds.width > 0.0 && bounds.height > 0.0 {
                        issues.push(PreflightIssue {
                            severity: "info".into(),
                            code: "image.empty_frame".into(),
                            message: preflight_msg(labels.preflight_image_empty_frame, &[frame.id()]),
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
                            message: preflight_msg(labels.preflight_text_missing_story, &[frame.id()]),
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
                                message: preflight_msg(labels.preflight_text_below_minimum_size, &[frame.id()]),
                                object_id: Some(frame.id().into()),
                                page_id: Some(page.id.clone()),
                            });
                        }
                        let known_family = family == "Layout Sans" || doc.paragraph_styles.iter().any(|style| style.font_family == *family);
                        if !known_family {
                            issues.push(PreflightIssue {
                                severity: "error".into(),
                                code: "font.missing".into(),
                                message: preflight_msg(labels.preflight_font_missing, &[family, frame.id()]),
                                object_id: Some(frame.id().into()),
                                page_id: Some(page.id.clone()),
                            });
                        }
                    }
                    if thread_next.is_none() && story.content.len() > 400 {
                        issues.push(PreflightIssue {
                            severity: "error".into(),
                            code: "text.overset".into(),
                            message: preflight_msg(labels.preflight_text_overset, &[frame.id()]),
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
                    message: preflight_msg(labels.preflight_asset_rgb_in_print, &[&link.id]),
                    object_id: Some(link.id.clone()),
                    page_id: None,
                });
            }
        }
    }
    issues
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the layout app; one field per label makes every locale combination compile-checked.
    struct LayoutLabels {
        document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        spreads: native_en "Spreads", native_de "Doppelseiten", reuse_en "Spreads", reuse_de "Doppelseiten";
        frames: native_en "Frames", native_de "Rahmen", reuse_en "Frames", reuse_de "Rahmen";
        parent_pages: native_en "Parent Pages", native_de "Übergeordnete Seiten", reuse_en "Parent Pages", reuse_de "Übergeordnete Seiten";
        layers: native_en "Layers", native_de "Ebenen", reuse_en "Layers", reuse_de "Ebenen";
        stories: native_en "Stories", native_de "Textflüsse", reuse_en "Stories", reuse_de "Textflüsse";
        links: native_en "Links", native_de "Verknüpfungen", reuse_en "Links", reuse_de "Verknüpfungen";
        styles: native_en "Styles", native_de "Formate", reuse_en "Styles", reuse_de "Formate";
        drop_here: native_en "Drop catalogue items here", native_de "Katalogelemente hier ablegen", reuse_en "Drop catalogue items here", reuse_de "Katalogelemente hier ablegen";
        catalogue_page: native_en "Page", native_de "Seite", reuse_en "Page", reuse_de "Seite";
        kind_rect: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        kind_text: native_en "Text Frame", native_de "Textrahmen", reuse_en "Text Frame", reuse_de "Textrahmen";
        kind_image: native_en "Image Frame", native_de "Bildrahmen", reuse_en "Image Frame", reuse_de "Bildrahmen";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        pages: native_en "Pages", native_de "Seiten", reuse_en "Pages", reuse_de "Seiten";
        active_page: native_en "Active page", native_de "Aktive Seite", reuse_en "Active page", reuse_de "Aktive Seite";
        id: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        margin_top: native_en "Margin Top", native_de "Rand oben", reuse_en "Margin Top", reuse_de "Rand oben";
        margin_right: native_en "Margin Right", native_de "Rand rechts", reuse_en "Margin Right", reuse_de "Rand rechts";
        margin_bottom: native_en "Margin Bottom", native_de "Rand unten", reuse_en "Margin Bottom", reuse_de "Rand unten";
        margin_left: native_en "Margin Left", native_de "Rand links", reuse_en "Margin Left", reuse_de "Rand links";
        gutter: native_en "Gutter", native_de "Spaltenabstand", reuse_en "Gutter", reuse_de "Spaltenabstand";
        columns: native_en "Columns", native_de "Spalten", reuse_en "Columns", reuse_de "Spalten";
        page: native_en "Page", native_de "Seite", reuse_en "Page", reuse_de "Seite";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        fill: native_en "Fill", native_de "Füllung", reuse_en "Fill", reuse_de "Füllung";
        stroke: native_en "Stroke", native_de "Kontur", reuse_en "Stroke", reuse_de "Kontur";
        story: native_en "Story", native_de "Textfluss", reuse_en "Story", reuse_de "Textfluss";
        wrap_mode: native_en "Wrap Mode", native_de "Textumfluss", reuse_en "Wrap Mode", reuse_de "Textumfluss";
        wrap_none: native_en "None", native_de "Kein", reuse_en "None", reuse_de "Kein";
        wrap_box: native_en "Box", native_de "Rechteck", reuse_en "Box", reuse_de "Rechteck";
        wrap_contour: native_en "Contour", native_de "Kontur", reuse_en "Contour", reuse_de "Kontur";
        link_path: native_en "Link Path", native_de "Verknüpfungspfad", reuse_en "Link Path", reuse_de "Verknüpfungspfad";
        group_page: native_en "Page", native_de "Seite", reuse_en "Page", reuse_de "Seite";
        group_frame: native_en "Frame", native_de "Rahmen", reuse_en "Frame", reuse_de "Rahmen";
        selection_not_found: native_en "Selection not found in document.", native_de "Auswahl im Dokument nicht gefunden.", reuse_en "Selection not found in document.", reuse_de "Auswahl im Dokument nicht gefunden.";
        preflight: native_en "Preflight", native_de "Preflight", reuse_en "Preflight", reuse_de "Preflight";
        no_issues: native_en "No issues", native_de "Keine Probleme", reuse_en "No issues", reuse_de "Keine Probleme";
        window_blueprint: native_en "Blueprint", native_de "Entwurf", reuse_en "Blueprint", reuse_de "Entwurf";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        parent: native_en "parent", native_de "übergeordnet", reuse_en "parent", reuse_de "übergeordnet";
        objects: native_en "objects", native_de "Objekte", reuse_en "objects", reuse_de "Objekte";
        chars: native_en "chars", native_de "Zeichen", reuse_en "chars", reuse_de "Zeichen";
        undo: native_en "Undo", native_de "Rückgängig", reuse_en "Undo", reuse_de "Rückgängig";
        redo: native_en "Redo", native_de "Wiederholen", reuse_en "Redo", reuse_de "Wiederholen";
        preflight_out_of_bounds: native_en "Object {} extends outside page bounds", native_de "Objekt {} liegt außerhalb der Seitengrenzen", reuse_en "Object {} extends outside page bounds", reuse_de "Objekt {} liegt außerhalb der Seitengrenzen";
        preflight_asset_missing: native_en "Linked asset missing for {}", native_de "Verknüpftes Element fehlt für {}", reuse_en "Linked asset missing for {}", reuse_de "Verknüpftes Element fehlt für {}";
        preflight_asset_modified: native_en "Linked asset modified for {}", native_de "Verknüpftes Element geändert für {}", reuse_en "Linked asset modified for {}", reuse_de "Verknüpftes Element geändert für {}";
        preflight_asset_low_resolution: native_en "Linked asset is low resolution for {}", native_de "Verknüpftes Element hat niedrige Auflösung für {}", reuse_en "Linked asset is low resolution for {}", reuse_de "Verknüpftes Element hat niedrige Auflösung für {}";
        preflight_image_empty_frame: native_en "Image frame {} has no preview", native_de "Bildrahmen {} hat keine Vorschau", reuse_en "Image frame {} has no preview", reuse_de "Bildrahmen {} hat keine Vorschau";
        preflight_text_missing_story: native_en "Text frame {} has no story", native_de "Textrahmen {} hat keinen Textfluss", reuse_en "Text frame {} has no story", reuse_de "Textrahmen {} hat keinen Textfluss";
        preflight_text_below_minimum_size: native_en "Text in {} is below minimum readable size", native_de "Text in {} ist kleiner als die Mindestlesbarkeitsgröße", reuse_en "Text in {} is below minimum readable size", reuse_de "Text in {} ist kleiner als die Mindestlesbarkeitsgröße";
        preflight_font_missing: native_en "Font {} used by {} is not available", native_de "Schriftart {} verwendet von {} ist nicht verfügbar", reuse_en "Font {} used by {} is not available", reuse_de "Schriftart {} verwendet von {} ist nicht verfügbar";
        preflight_text_overset: native_en "Text in {} overflows its frame", native_de "Text in {} läuft über den Rahmen hinaus", reuse_en "Text in {} overflows its frame", reuse_de "Text in {} läuft über den Rahmen hinaus";
        preflight_asset_rgb_in_print: native_en "Linked asset {} uses RGB in a print document", native_de "Verknüpftes Element {} verwendet RGB in einem Druckdokument", reuse_en "Linked asset {} uses RGB in a print document", reuse_de "Verknüpftes Element {} verwendet RGB in einem Druckdokument";
    }
}

/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` (mirrors `shooting_ui`'s identical B1 fix).
fn is_de_locale(cfg: &LayoutConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &LayoutConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}

/// 🗣️ Resolves the active label set from the config-carried locale; unknown locales fall back to native English.
fn layout_labels(cfg: &LayoutConfig) -> &'static LayoutLabels {
    resolve_labels::<LayoutLabels>(cfg)
}

/// 🗣️ Resolves a catalogue frame kind's display label from its stable id; unknown kinds fall back to the kind id itself.
fn catalogue_kind_label(kind: &'static str, labels: &LayoutLabels) -> &'static str {
    match kind {
        "rect" => labels.kind_rect,
        "text" => labels.kind_text,
        "image" => labels.kind_image,
        _ => kind,
    }
}

/// 🗣️ Fills a localized preflight message template's positional `{}` placeholders, in order, with the given values.
fn preflight_msg(template: &str, args: &[&str]) -> String {
    let mut result = template.to_string();
    for arg in args {
        result = result.replacen("{}", arg, 1);
    }
    result
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in `create_layout_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn layout_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addFrame", "Add Frame", "Rahmen hinzufügen"),
        ("addPage", "Add Page", "Seite hinzufügen"),
        ("exportPng", "Export Png", "Png exportieren"),
        ("exportSvg", "Export Svg", "Svg exportieren"),
        ("exportPdf", "Export Pdf", "Pdf exportieren"),
        ("exportPackage", "Export Package", "Paket exportieren"),
        ("patchPage", "Patch Page", "Seite aktualisieren"),
        ("patchFrame", "Patch Frame", "Rahmen aktualisieren"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("canvasDrop", "Canvas Drop", "Ablegen auf Leinwand"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("setActivePage", "Set Active Page", "Aktive Seite festlegen"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("focusPreflightIssue", "Focus Preflight Issue", "Preflight-Problem fokussieren"),
        ("engagementInput", "Engagement Input", "Eingabe"),
        ("canvasPointerDown", "Canvas Pointer Down", "Leinwand-Zeiger gedrückt"),
        ("canvasPointerMove", "Canvas Pointer Move", "Leinwand-Zeiger bewegen"),
        ("canvasPointerUp", "Canvas Pointer Up", "Leinwand-Zeiger losgelassen"),
        ("canvasDragOver", "Canvas Drag Over", "Ziehen über Leinwand"),
        ("canvasDragLeave", "Canvas Drag Leave", "Ziehen verlässt Leinwand"),
        ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
    ];
    localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_layout_app`.
/// The layout app currently declares no utilities, so this returns an empty map — kept for parity with the
/// other crates' overlay-wiring shape and to compile-check the moment a utility is added.
fn layout_utility_labels(_is_de: bool) -> HashMap<String, String> {
    HashMap::new()
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
/// 🌳️ Layout's row shape (id/label/description/icon/optional-action) over the SDK's
/// `tree_item_desc`/`tree_item_with_action` — the icon assignment is the only bit the SDK helpers
/// don't cover, since not every plugin's rows carry one.
fn layout_tree_item(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<ActionDescriptor>,
) -> UiTreeItemNode {
    let mut item = match action {
        Some(action) => tree_item_with_action(id, label, description, action),
        None => tree_item_desc(id, label, description),
    };
    item.icon_id = icon_id.and_then(|id| IconName::from_str(&id));
    item
}

/// 🌳️ A `layout_tree_item` that additionally dispatches `setHover`/clear-hover on hover/unhover —
/// used by the document tree's page and frame rows to drive canvas hover highlighting.
fn layout_tree_item_hoverable(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    icon_id: Option<String>,
    action: Option<ActionDescriptor>,
    hover_id: &str,
) -> UiTreeItemNode {
    let mut item = layout_tree_item(id, label, description, icon_id, action);
    item.hover_action = Some(layout_action("setHover", Some(json!({ "id": hover_id }))));
    item.unhover_action = Some(layout_action("setHover", Some(json!({ "id": Value::Null }))));
    item
}

fn build_document_tree(doc: &LayoutDocument, config: &LayoutConfig, labels: &LayoutLabels) -> UiNode {

    let spread_items: Vec<UiTreeItemNode> = doc
        .spreads
        .iter()
        .map(|spread| layout_tree_item(spread_row_id(&spread.id), spread.name.clone(), Some(spread.page_ids.join(", ")), Some("layout".into()), None))
        .collect();

    let page_items: Vec<UiTreeItemNode> = doc
        .pages
        .iter()
        .map(|page| {
            layout_tree_item_hoverable(
                page_row_id(&page.id),
                page.name.clone(),
                page.parent_page_id.as_ref().map(|parent_id| format!("{}: {parent_id}", labels.parent)),
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
                layout_tree_item_hoverable(
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
        vec![layout_tree_item("layout-document.frames.empty", labels.drop_here, None, Some("inbox".into()), None)]
    } else {
        frame_items
    };

    let parent_page_items: Vec<UiTreeItemNode> = doc
        .parent_pages
        .iter()
        .map(|parent| {
            layout_tree_item(
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
                layout_tree_item(
                    layer_row_id(&page.id, &layer.id),
                    format!("{} · {}", page.name, layer.name),
                    Some(format!("{} {}", layer.object_ids.len(), labels.objects)),
                    Some("layers".into()),
                    None,
                )
            })
        })
        .collect();

    let story_items: Vec<UiTreeItemNode> = doc
        .stories
        .iter()
        .map(|story| layout_tree_item(story_row_id(&story.id), story.id.clone(), Some(format!("{} {}", story.content.chars().count(), labels.chars)), Some("file-text".into()), None))
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
            layout_tree_item(
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
            layout_tree_item(
                style_row_id(&style.id),
                style.name.clone(),
                Some(format!("{} · {}pt", style.font_family, style.font_size as i64)),
                Some("type".into()),
                None,
            )
        })
        .collect();
    style_items.extend(doc.character_styles.iter().map(|style| {
        let name = style.name.clone().unwrap_or_else(|| style.id.clone());
        let font_family = style.font_family.as_deref().unwrap_or("—");
        let description = match style.font_size {
            Some(size) => format!("{font_family} · {}pt", size as i64),
            None => font_family.to_string(),
        };
        layout_tree_item(style_row_id(&style.id), name, Some(description), Some("type".into()), None)
    }));

    let highlighted_ids: Vec<String> = config
        .hovered_id
        .as_ref()
        .map(|id| vec![page_row_id(id), frame_row_id(id)])
        .unwrap_or_default();
    let mut builder = PanelTreeBuilder::new("layout-document")
        .section(
            "layout-document.document",
            Some(labels.document.into()),
            true,
            vec![layout_tree_item("layout-document.document.root", doc.name.clone(), Some(LAYOUT_FIXTURE_SCHEMA.into()), Some("file-text".into()), None)],
        )
        .section("layout-document.spreads", Some(labels.spreads.into()), false, spread_items)
        .section("layout-document.pages", Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, page_items)
        .section("layout-document.frames", Some(labels.frames.into()), true, frame_items)
        .section("layout-document.parentPages", Some(labels.parent_pages.into()), false, parent_page_items)
        .section("layout-document.layers", Some(labels.layers.into()), false, layer_items)
        .section("layout-document.stories", Some(labels.stories.into()), false, story_items)
        .section("layout-document.links", Some(labels.links.into()), false, link_items)
        .section("layout-document.styles", Some(labels.styles.into()), false, style_items)
        .selected(config.selected_ids.iter().flat_map(|id| vec![page_row_id(id), frame_row_id(id), layer_row_id(&config.active_page_id, id)]).collect())
        .selection_change(layout_action("setSelection", None));
    if !highlighted_ids.is_empty() {
        builder = builder.highlighted(highlighted_ids);
    }
    builder.build()
}

fn catalogue_tree_item(kind: &str, label: &str, icon: &str) -> UiTreeItemNode {
    let action = if kind == "page" { layout_action("addPage", None) } else { layout_action("addFrame", Some(json!({ "kind": kind }))) };
    let mut drag_data_entries = serde_json::Map::new();
    drag_data_entries.insert(LAYOUT_CATALOGUE_DRAG_MIME.to_string(), json!(json!({ "kind": kind }).to_string()));
    drag_data_entries.insert(format!("{LAYOUT_CATALOGUE_KIND_MIME_PREFIX}{kind}"), json!(""));
    let drag_data = Value::Object(drag_data_entries);
    let mut item = tree_item_with_action_draggable(format!("layout-catalogue.{kind}"), label, Some(kind.into()), action, &drag_data);
    item.icon_id = Some(icon.into());
    item
}

fn build_catalogue_tree(labels: &LayoutLabels) -> UiNode {
    let mut items = vec![catalogue_tree_item("page", labels.catalogue_page, "file")];
    items.extend(LAYOUT_CATALOGUE_KINDS.iter().map(|(kind, icon)| catalogue_tree_item(kind, catalogue_kind_label(kind, labels), icon)));
    PanelTreeBuilder::new("layout-catalogue")
        .section("layout-catalogue.kinds", Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()), true, items)
        .build()
}

fn build_inspector_tree(doc: &LayoutDocument, config: &LayoutConfig, labels: &LayoutLabels) -> UiNode {
    if config.selected_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "layout-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![
                ui_text(format!("{}: {}", labels.schema, LAYOUT_FIXTURE_SCHEMA)),
                ui_text(format!("{}: {}", labels.name, doc.name)),
                ui_text(format!("{}: {}", labels.pages, doc.pages.len())),
                ui_text(format!("{}: {}", labels.active_page, config.active_page_id)),
            ],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let selected_id = &config.selected_ids[0];
    if let Some(page) = doc.pages.iter().find(|page| page.id == *selected_id) {
        let mut fields = vec![
            ui_inspector_readonly_field("layout-play-inspector.page-id", labels.id, page.id.clone()),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "layout-play-inspector.page-name".into(),
                label: labels.name.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ];
        for (field, label, value) in [
            ("width", labels.width, page.width),
            ("height", labels.height, page.height),
            ("marginTop", labels.margin_top, page.margins.top),
            ("marginRight", labels.margin_right, page.margins.right),
            ("marginBottom", labels.margin_bottom, page.margins.bottom),
            ("marginLeft", labels.margin_left, page.margins.left),
            ("columnsGutter", labels.gutter, page.columns.gutter),
        ] {
            fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("layout-play-inspector.page-{field}"),
                label: label.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
        }
        fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
            id: "layout-play-inspector.page-columnsCount".into(),
            label: labels.columns.into(),
            child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            menu: None,
        }));
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(),
            id: "layout-play-inspector.page".into(),
            label: labels.group_page.into(),
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
                ui_inspector_readonly_field("layout-play-inspector.frame-id", labels.id, frame_id.clone()),
                ui_inspector_readonly_field("layout-play-inspector.frame-kind", labels.kind, frame.kind_str().to_string()),
                ui_inspector_readonly_field("layout-play-inspector.frame-page", labels.page, page.name.clone()),
            ];
            for (field, label, value) in [
                ("x", labels.x, bounds.x),
                ("y", labels.y, bounds.y),
                ("width", labels.width, bounds.width),
                ("height", labels.height, bounds.height),
            ] {
                fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                    id: format!("layout-play-inspector.frame-{field}"),
                    label: label.into(),
                    child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }));
            }
            match frame {
                Frame::Rect { fill, stroke, .. } => {
                    for (field, label, value) in [("fill", labels.fill, fill), ("stroke", labels.stroke, stroke)] {
                        fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                            id: format!("layout-play-inspector.frame-{field}"),
                            label: label.into(),
                            child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                                menu: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                            menu: None,
                        }));
                    }
                }
                Frame::Text { story_id, wrap_mode, columns, .. } => {
                    fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-story".into(),
                        label: labels.story.into(),
                        child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-wrapMode".into(),
                        label: labels.wrap_mode.into(),
                        child: Box::new(UiNode::Select(UiSelectNode {presence: UiPresence::default(),
                            id: "layout-play-inspector.frame-wrapMode.select".into(),
                            value: wrap_mode.clone(),
                            items: vec![
                                UiSelectItem { value: "none".into(), label: labels.wrap_none.into(),
        },
                                UiSelectItem { value: "box".into(), label: labels.wrap_box.into(),
        },
                                UiSelectItem { value: "contour".into(), label: labels.wrap_contour.into(),
        },
                            ],
                            placeholder: None,
                            on_change: layout_action(
                                "patchFrame",
                                Some(json!({ "frameId": frame_id, "pageId": page_id, "field": "wrapMode" })),
                            ),
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-columns".into(),
                        label: labels.columns.into(),
                        child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                }
                Frame::Image { link_id, .. } => {
                    fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                        id: "layout-play-inspector.frame-linkPath".into(),
                        label: labels.link_path.into(),
                        child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    }));
                }
            }
            let _ = name_mixed;
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(),
                id: "layout-play-inspector.frame".into(),
                label: labels.group_frame.into(),
                default_open: Some(true),
                fields,
            }]);
        }
    }
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "layout-play-inspector.missing".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        children: vec![ui_text(labels.selection_not_found)],
        presence: UiPresence::default(),
        menu: None,
    }])
}

fn build_preflight_tree(doc: &LayoutDocument, labels: &LayoutLabels) -> UiNode {
    let issues = run_layout_preflight(doc, labels);
    let items: Vec<UiTreeItemNode> = if issues.is_empty() {
        vec![layout_tree_item("layout-preflight.empty", labels.no_issues, None, Some("check-circle".into()), None)]
    } else {
        issues
            .iter()
            .map(|issue| {
                layout_tree_item(
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
    PanelTreeBuilder::new("layout-preflight")
        .section("layout-preflight.issues", Some(labels.preflight.into()), true, items)
        .build()
}

fn layout_window_engagement(config: &LayoutConfig, label: &str, labels: &LayoutLabels) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some(format!("layout-engagement-{label}")),
            value: Some(config.engagement_input.clone()),
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
            text: format!("{} {}", labels.page, config.active_page_id),
        }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "layout.eng.undo".into(), label: labels.undo.into(), detail: None, action: Some(layout_action("undo", None)) },
            WindowEngagementPossible { id: "layout.eng.redo".into(), label: labels.redo.into(), detail: None, action: Some(layout_action("redo", None)) },
        ]),
    }
}

//#endregion 🔖️Panels

//#region 🔖️Render
fn render_blueprint(doc: &LayoutDocument, config: &LayoutConfig) -> UiNode {
    let camera = &config.camera;
    build_canvas_2d_scene(
        LAYOUT_PLAY_SURFACE_BLUEPRINT,
        LAYOUT_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            layers_json: canvas_layers(doc, config, true),
        },
    )
}

fn render_preview(doc: &LayoutDocument, config: &LayoutConfig) -> UiNode {
    let camera = &config.preview_camera;
    build_canvas_2d_scene(
        LAYOUT_PLAY_SURFACE_PREVIEW,
        LAYOUT_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            layers_json: canvas_layers(doc, config, false),
        },
    )
}
//#endregion 🔖️Render

//#region 🔖️LayoutPlayApp
/// 🧪️ B1: unit struct — every former `LayoutPlayRuntime` field now lives in
/// `layout_engine::LayoutConfig`, written through `layout_op::LayoutConfigOperation`s.
#[derive(Default)]
pub struct LayoutPlayApp;

impl DocumentApp for LayoutPlayApp {
    type Projection = LayoutDocument;
    type Operation = LayoutOperation;
    type Config = LayoutConfig;
    type ConfigOperation = LayoutConfigOperation;
    type Command = LayoutCommand;

    fn app_id(&self) -> &str {
        LAYOUT_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        LAYOUT_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> LayoutDocument {
        layout_engine::default_document()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(layout_engine::layout_io())
    }

    /// 🏷️ Maps each `LayoutCommand` variant back to the action id it was declared under in
    /// `create_layout_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &LayoutCommand) -> &str {
        match command {
            LayoutCommand::SetSelection { .. } => "setSelection",
            LayoutCommand::SetActivePage { .. } => "setActivePage",
            LayoutCommand::SetHover { .. } => "setHover",
            LayoutCommand::FocusPreflightIssue { .. } => "focusPreflightIssue",
            LayoutCommand::EngagementInput { .. } => "engagementInput",
            LayoutCommand::CanvasPointerDown { .. } => "canvasPointerDown",
            LayoutCommand::CanvasPointerMove { .. } => "canvasPointerMove",
            LayoutCommand::CanvasPointerUp => "canvasPointerUp",
            LayoutCommand::CanvasDragOver { .. } => "canvasDragOver",
            LayoutCommand::CanvasDragLeave => "canvasDragLeave",
            LayoutCommand::SetCamera { .. } => "setCamera",
            LayoutCommand::SetLocale { .. } => "setLocale",
            LayoutCommand::AddFrame { .. } => "addFrame",
            LayoutCommand::AddPage => "addPage",
            LayoutCommand::PatchPage { .. } => "patchPage",
            LayoutCommand::PatchFrame { .. } => "patchFrame",
            LayoutCommand::CanvasDrop { .. } => "canvasDrop",
            LayoutCommand::ExportPng { .. } => "exportPng",
            LayoutCommand::ExportSvg { .. } => "exportSvg",
            LayoutCommand::ExportPdf { .. } => "exportPdf",
            LayoutCommand::ExportPackage => "exportPackage",
            LayoutCommand::EngagementSubmit { .. } => "engagementSubmit",
        }
    }

    fn handle(&self, command: &LayoutCommand, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Emit<LayoutOperation, LayoutConfigOperation> {
        let document = doc.projection;
        let config = cfg.projection;
        match command {
            //#region 👁️View
            LayoutCommand::SetSelection { ids } => Emit::config(vec![LayoutConfigOperation::SetSelection { ids: ids.clone() }]),
            LayoutCommand::SetActivePage { page_id } => Emit::config(vec![LayoutConfigOperation::SetActivePage { page_id: page_id.clone() }]),
            LayoutCommand::SetHover { id } => Emit::config(vec![LayoutConfigOperation::SetHover { id: id.clone() }]),
            LayoutCommand::FocusPreflightIssue { object_id, page_id } => {
                let mut config_operations = Vec::new();
                if let Some(object_id) = object_id {
                    config_operations.push(LayoutConfigOperation::SetSelection { ids: vec![object_id.clone()] });
                }
                if let Some(page_id) = page_id {
                    config_operations.push(LayoutConfigOperation::SetActivePage { page_id: page_id.clone() });
                }
                Emit::config(config_operations)
            }
            LayoutCommand::EngagementInput { value } => Emit::config(vec![LayoutConfigOperation::SetEngagementInput { value: value.clone() }]),
            LayoutCommand::CanvasPointerDown { surface_id, button, extend, x, y, width, height } => {
                let blueprint = surface_is_blueprint(surface_id.as_deref());
                if !blueprint || *button != 0 {
                    return Emit::default();
                }
                let hit = hit_test_at(document, config, *x, *y, *width, *height, blueprint);
                let ids = match hit {
                    Some(id) if *extend => {
                        let mut ids = config.selected_ids.clone();
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
                Emit::config(vec![LayoutConfigOperation::SetSelection { ids }])
            }
            LayoutCommand::CanvasPointerMove { surface_id, x, y, width, height } => {
                let blueprint = surface_is_blueprint(surface_id.as_deref());
                if !blueprint {
                    return Emit::default();
                }
                Emit::config(vec![LayoutConfigOperation::SetHover { id: hit_test_at(document, config, *x, *y, *width, *height, blueprint) }])
            }
            LayoutCommand::CanvasPointerUp => Emit::default(),
            LayoutCommand::CanvasDragOver { surface_id, kind, x, y, width, height } => {
                let blueprint = surface_is_blueprint(surface_id.as_deref());
                if !blueprint {
                    return Emit::default();
                }
                let (wx, wy) = screen_to_world_for_surface(config, blueprint, *x, *y, *width, *height);
                Emit::config(vec![LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState { kind: kind.clone(), x: wx, y: wy } }])
            }
            LayoutCommand::CanvasDragLeave => Emit::config(vec![LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState::default() }]),
            LayoutCommand::SetCamera { surface_id, camera } => {
                let blueprint = surface_is_blueprint(surface_id.as_deref());
                if blueprint {
                    Emit::config(vec![LayoutConfigOperation::SetCamera { camera: camera.clone() }])
                } else {
                    Emit::config(vec![LayoutConfigOperation::SetPreviewCamera { camera: camera.clone() }])
                }
            }
            LayoutCommand::SetLocale { value } => Emit::config(vec![LayoutConfigOperation::SetLocale { value: value.clone() }]),
            //#endregion 👁️View
            //#region 🔧️Operations
            LayoutCommand::AddFrame { kind, x, y } => {
                let page_id = config.active_page_id.clone();
                let Some(page) = document.pages.iter().find(|page| page.id == page_id) else {
                    return Emit::default();
                };
                let index = page.frames.len();
                let frame_id = format!("frame-{}", index + 1);
                let layer_id = page.layer_ids.first().cloned().unwrap_or_else(|| "layer-1".into());
                let frame = match kind.as_str() {
                    "text" => Frame::Text {
                        id: frame_id.clone(),
                        layer_id: layer_id.clone(),
                        bounds: layout::LayoutBounds { x: x.unwrap_or(48.0), y: y.unwrap_or(120.0), width: 200.0, height: 120.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        story_id: document.stories.first().map(|story| story.id.clone()).unwrap_or_else(|| "story-1".into()),
                        thread_next: None,
                        columns: 1,
                        inset: layout::LayoutRect { x: 4.0, y: 4.0, width: 192.0, height: 112.0 },
                        wrap_mode: "box".into(),
                    },
                    "image" => Frame::Image {
                        id: frame_id.clone(),
                        layer_id: layer_id.clone(),
                        bounds: layout::LayoutBounds { x: x.unwrap_or(48.0), y: y.unwrap_or(280.0), width: 160.0, height: 120.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        link_id: document.links.first().map(|link| link.id.clone()).unwrap_or_else(|| "link-missing".into()),
                    },
                    _ => Frame::Rect {
                        id: frame_id.clone(),
                        layer_id: layer_id.clone(),
                        bounds: layout::LayoutBounds { x: x.unwrap_or(48.0), y: y.unwrap_or(48.0), width: 120.0, height: 64.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        fill: Some([0.2, 0.24, 0.3, 1.0]),
                        stroke: None,
                    },
                };
                Emit {
                    document_operations: vec![LayoutOperation::AddFrame { page_id, index, frame, layer_id: Some(layer_id) }],
                    config_operations: vec![LayoutConfigOperation::SetSelection { ids: vec![frame_id] }],
                    ..Default::default()
                }
            }
            LayoutCommand::AddPage => {
                let template = document.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| document.pages.first());
                let (width, height, spread_id, parent_page_id, margins, columns) = template
                    .map(|page| (page.width, page.height, page.spread_id.clone(), page.parent_page_id.clone(), page.margins.clone(), page.columns.clone()))
                    .unwrap_or((595.0, 842.0, "spread-1".into(), None, PageMargins { top: 48.0, right: 36.0, bottom: 48.0, left: 36.0 }, PageColumns { count: 1, gutter: 0.0 }));
                let page_id = format!("page-{}", document.pages.len() + 1);
                let layer_id = format!("layer-{page_id}");
                let page = Page {
                    id: page_id.clone(),
                    name: format!("Page {}", document.pages.len() + 1),
                    spread_id,
                    parent_page_id,
                    width,
                    height,
                    margins,
                    columns,
                    guides: Vec::new(),
                    layer_ids: vec![layer_id.clone()],
                    layers: vec![layout::Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }],
                    frames: Vec::new(),
                    overrides: Vec::new(),
                };
                Emit {
                    document_operations: vec![LayoutOperation::Pages(CollectionOperation::Add { id: page.id.clone(), item: page, at: document.pages.len() })],
                    config_operations: vec![LayoutConfigOperation::SetActivePage { page_id: page_id.clone() }, LayoutConfigOperation::SetSelection { ids: vec![page_id] }],
                    ..Default::default()
                }
            }
            LayoutCommand::PatchPage { page_id, field, value } => {
                let page_id = page_id.clone().unwrap_or_else(|| config.active_page_id.clone());
                match page_patch_for_field(field, value) {
                    Some(patch) if document.pages.iter().any(|page| page.id == page_id) => Emit::operations(vec![LayoutOperation::Pages(CollectionOperation::Patch { id: page_id, patch })]),
                    _ => Emit::default(),
                }
            }
            LayoutCommand::PatchFrame { frame_id, page_id, field, value } => {
                let page_id = page_id.clone().unwrap_or_else(|| config.active_page_id.clone());
                if frame_id.is_empty() {
                    return Emit::default();
                }
                let Some(page) = document.pages.iter().find(|page| page.id == page_id) else {
                    return Emit::default();
                };
                let Some(frame) = page.frames.iter().find(|frame| frame.id() == frame_id) else {
                    return Emit::default();
                };
                match field.as_str() {
                    "x" | "y" | "width" | "w" | "height" | "h" => match value.parse::<f64>() {
                        Ok(number) => Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: frame_id.clone(), patch: frame_bounds_patch(field, number) }]),
                        Err(_) => Emit::default(),
                    },
                    "fill" | "stroke" => {
                        let rgba = text_to_rgba(value);
                        let patch = if field == "fill" { FramePatch { fill: Some(rgba), ..Default::default() } } else { FramePatch { stroke: Some(rgba), ..Default::default() } };
                        Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: frame_id.clone(), patch }])
                    }
                    "wrapMode" => Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: frame_id.clone(), patch: FramePatch { wrap_mode: Some(value.clone()), ..Default::default() } }]),
                    "columns" => match value.parse::<f64>() {
                        Ok(count) => Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: frame_id.clone(), patch: FramePatch { columns: Some(count.max(0.0) as u32), ..Default::default() } }]),
                        Err(_) => Emit::default(),
                    },
                    "storyContent" => {
                        let story_id = match frame {
                            Frame::Text { story_id, .. } => Some(story_id.clone()),
                            _ => None,
                        };
                        match story_id {
                            Some(story_id) if document.stories.iter().any(|story| story.id == story_id) => {
                                Emit::operations(vec![LayoutOperation::Stories(CollectionOperation::Patch { id: story_id, patch: TextStoryPatch { content: Some(value.clone()) } })])
                            }
                            _ => Emit::default(),
                        }
                    }
                    "linkPath" => {
                        let link_id = match frame {
                            Frame::Image { link_id, .. } => Some(link_id.clone()),
                            _ => None,
                        };
                        match link_id {
                            Some(link_id) if document.links.iter().any(|link| link.id == link_id) => {
                                Emit::operations(vec![LayoutOperation::Links(CollectionOperation::Patch { id: link_id, patch: ImageLinkPatch { path: Some(value.clone()) } })])
                            }
                            _ => Emit::default(),
                        }
                    }
                    _ => Emit::default(),
                }
            }
            LayoutCommand::CanvasDrop { surface_id, kind, x, y, width, height } => {
                let blueprint = surface_is_blueprint(surface_id.as_deref());
                let clear_preview = Emit::config(vec![LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState::default() }]);
                if !blueprint {
                    return clear_preview;
                }
                let (wx, wy) = screen_to_world_for_surface(config, blueprint, *x, *y, *width, *height);
                let mut emitted = if kind == "page" {
                    self.handle(&LayoutCommand::AddPage, doc, cfg)
                } else {
                    self.handle(&LayoutCommand::AddFrame { kind: kind.clone(), x: Some(wx), y: Some(wy) }, doc, cfg)
                };
                emitted.config_operations.push(LayoutConfigOperation::SetDropPreview { preview: LayoutDropPreviewState::default() });
                emitted
            }
            //#endregion 🔧️Operations
            //#region 🐚️Shell
            LayoutCommand::ExportPng { page_id } => {
                let page_id = page_id.clone().unwrap_or_else(|| config.active_page_id.clone());
                match export_document_png_cpu(document, &page_id) {
                    Ok(bytes) => Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{page_id}.png"), mime_type: "image/png".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), encoding: Some("base64".into()) }),
                    Err(_) => Emit::default(),
                }
            }
            LayoutCommand::ExportSvg { page_id } => {
                let page_id = page_id.clone().unwrap_or_else(|| config.active_page_id.clone());
                match export_document_svg(document, &page_id) {
                    Ok(svg) => Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{page_id}.svg"), mime_type: "image/svg+xml".into(), data: svg, encoding: None }),
                    Err(_) => Emit::default(),
                }
            }
            LayoutCommand::ExportPdf { page_id } => {
                let page_id = page_id.clone().unwrap_or_else(|| config.active_page_id.clone());
                match export_document_pdf(document, &page_id) {
                    Ok(bytes) => Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{page_id}.pdf"), mime_type: "application/pdf".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), encoding: Some("base64".into()) }),
                    Err(_) => Emit::default(),
                }
            }
            LayoutCommand::ExportPackage => {
                let preflight_json = serde_json::to_string(&run_layout_preflight(document, layout_labels(config))).unwrap_or_else(|_| "[]".into());
                let doc_json = serde_json::to_string(document).unwrap_or_default();
                match export_package_zip(&doc_json, &preflight_json) {
                    Ok(bytes) => Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.layout-package.zip", document.name), mime_type: "application/zip".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), encoding: Some("base64".into()) }),
                    Err(_) => Emit::default(),
                }
            }
            LayoutCommand::EngagementSubmit { value } => {
                let typed = value.trim();
                let export = if engagement_token_matches(typed, "export png") || engagement_token_matches(typed, "png") {
                    Some(LayoutCommand::ExportPng { page_id: None })
                } else if engagement_token_matches(typed, "export svg") || engagement_token_matches(typed, "svg") {
                    Some(LayoutCommand::ExportSvg { page_id: None })
                } else if engagement_token_matches(typed, "export pdf") || engagement_token_matches(typed, "pdf") {
                    Some(LayoutCommand::ExportPdf { page_id: None })
                } else if engagement_token_matches(typed, "export package") || engagement_token_matches(typed, "package") {
                    Some(LayoutCommand::ExportPackage)
                } else {
                    None
                };
                match export {
                    Some(export) => self.handle(&export, doc, cfg),
                    None => Emit::default(),
                }
            }
            //#endregion 🐚️Shell
        }
    }

    //#region 🔖️Media
    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `document:out` replicates the trait default
    /// exactly (overriding `export_media` for `layout:out` forfeits the default's dispatch); `layout:out`
    /// re-exports the current layout's first page as `2d.layout` vector/SVG — reuses `export_document_svg`
    /// (the same exporter `exportSvg`/`LayoutCommand::ExportSvg` use). No `cfg` parameter reaches this
    /// method, so there is no config-carried "active page" to prefer over the first page.
    fn export_media(&self, port: &str, doc: &DocumentView<'_, LayoutDocument>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let bytes = store::DocumentPack::encode_pack(doc.projection);
                Ok(Media {
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    payload: MediaPayload::Structured { schema: LAYOUT_FIXTURE_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "layout:out" => {
                let document = doc.projection;
                let page = document.pages.first().ok_or_else(|| MediaError::Payload(port.to_string(), "layout has no pages to export".into()))?;
                let svg = export_document_svg(document, &page.id).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.layout".into(), json: svg } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `fields:in` binds the incoming `form.dictionary`
    /// values into `LayoutDocument::data_fields_json` — layout has no existing text-interpolation/
    /// field-binding concept for frames/stories yet, so this stores the dictionary verbatim as a new
    /// named data source (see `layout::LayoutDocument::data_fields_json`'s doc) rather than wiring it
    /// into rendering today.
    fn import_media(&self, port: &str, media: &Media, _doc: &DocumentView<'_, LayoutDocument>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, MediaError> {
        match port {
            "fields:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "fields:in only accepts a Structured (JSON object) payload".into()));
                };
                Ok(Emit::operations(vec![LayoutOperation::SetDataFields { json: Some(json.clone()) }]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    fn render(&self, body_key: &str, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let labels = layout_labels(config);
        match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => render_blueprint(document, config),
            LAYOUT_PLAY_BODY_PREVIEW => render_preview(document, config),
            LAYOUT_PLAY_BODY_DOCUMENT => build_document_tree(document, config, labels),
            LAYOUT_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            LAYOUT_PLAY_BODY_INSPECTION => build_inspector_tree(document, config, labels),
            LAYOUT_PLAY_BODY_PREFLIGHT => build_preflight_tree(document, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, _doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        let labels = layout_labels(config);
        HashMap::from([
            (LAYOUT_PLAY_WINDOW_BLUEPRINT.to_string(), layout_window_engagement(config, "blueprint", labels)),
            (LAYOUT_PLAY_WINDOW_PREVIEW.to_string(), layout_window_engagement(config, "preview", labels)),
        ])
    }

    fn app_labels(&self, cfg: &ConfigView<'_, LayoutConfig>) -> AppLabelsOverlay {
        let config = cfg.projection;
        let labels = layout_labels(config);
        let is_de = is_de_locale(config);
        AppLabelsOverlay::default()
            .window_kind_label(LAYOUT_PLAY_WINDOW_BLUEPRINT, labels.window_blueprint)
            .window_kind_label(LAYOUT_PLAY_WINDOW_PREVIEW, labels.window_preview)
            .panel_tab_label(LAYOUT_PLAY_PREFLIGHT_TAB_ID, labels.preflight)
            .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
            .action_labels(layout_action_labels(is_de))
            .utility_labels(layout_utility_labels(is_de))
            .example_labels(HashMap::from([("sample".to_string(), (if is_de { "Beispiel" } else { "Sample" }).to_string())]))
    }
}
//#endregion 🔖️LayoutPlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/inspector/DnD/engagement-bound
/// vocabulary dispatched by the canvas and panels, never surfaced as a standalone palette command.
fn layout_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

pub fn create_layout_app() -> App {
    App::from_builder(
        App::builder(LAYOUT_PLAY_APP_ID, "Layout").document(["semio", "layout"])
            .artifact_kind(ArtifactKindSpec {
                id: "2d.layout".into(),
                name: "Layout".into(),
                source_format: "layout.fixture".into(),
                component_kind: "layout".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                schema: "layout.fixture".into(),
                export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
            })
            .icon_id("layout")
            .mode("edit", "Edit", "pencil")
            .default_mode_id("edit")
            .window_kind(LAYOUT_PLAY_WINDOW_BLUEPRINT, "Blueprint", LAYOUT_PLAY_BODY_BLUEPRINT, SurfaceKind::Canvas2d, "layout")
            .window_kind(LAYOUT_PLAY_WINDOW_PREVIEW, "Preview", LAYOUT_PLAY_BODY_PREVIEW, SurfaceKind::Canvas2d, "preview")
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
            // ✏️ Palette-visible content commands — dispatched as VCS operations with a true inverse.
            .operation("addFrame", "Add Frame")
            .operation("addPage", "Add Page")
            .action_args("addFrame", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("rect", "Rectangle"),
                    ActionArgOption::new("text", "Text Frame"),
                    ActionArgOption::new("image", "Image Frame"),
                ]).default_value("rect"),
                ActionArgDef::number("x", "X"),
                ActionArgDef::number("y", "Y"),
            ])
            // 🐚️ Palette-visible shell exports — round-trip through the host.
            .shell_action("exportPng", "Export Png")
            .shell_action("exportSvg", "Export Svg")
            .shell_action("exportPdf", "Export Pdf")
            .shell_action("exportPackage", "Export Package")
            // 🔧️ Internal document operations — inspector/DnD-bound, not palette commands.
            .action_with(layout_internal_action("patchPage", "Patch Page", ActionKind::Operation))
            .action_with(layout_internal_action("patchFrame", "Patch Frame", ActionKind::Operation))
            .action_with(layout_internal_action("canvasDrop", "Canvas Drop", ActionKind::Operation))
            // 👁️ Ephemeral view state — selection, hover, active page, drop ghost, pointer, camera, engagement draft.
            .action_with(layout_internal_action("setSelection", "Set Selection", ActionKind::View))
            .action_with(layout_internal_action("setActivePage", "Set Active Page", ActionKind::View))
            .action_with(layout_internal_action("setHover", "Set Hover", ActionKind::View))
            .action_with(layout_internal_action("focusPreflightIssue", "Focus Preflight Issue", ActionKind::View))
            .action_with(layout_internal_action("engagementInput", "Engagement Input", ActionKind::View))
            .action_with(layout_internal_action("canvasPointerDown", "Canvas Pointer Down", ActionKind::View))
            .action_with(layout_internal_action("canvasPointerMove", "Canvas Pointer Move", ActionKind::View))
            .action_with(layout_internal_action("canvasPointerUp", "Canvas Pointer Up", ActionKind::View))
            .action_with(layout_internal_action("canvasDragOver", "Canvas Drag Over", ActionKind::View))
            .action_with(layout_internal_action("canvasDragLeave", "Canvas Drag Leave", ActionKind::View))
            .action_with(layout_internal_action("setCamera", "Set Camera", ActionKind::View))
            // 🐚️ Engagement submit — routes typed export intents through the host, emits only shell effects.
            .action_with(layout_internal_action("engagementSubmit", "Engagement Submit", ActionKind::Shell))
            // 📇️ Per-window action scoping — the content-authoring operations only make sense on the
            // interactive Blueprint surface; the read-only Preview surface renders output and never
            // creates or edits frames/pages. Exports, camera, pointer/drag, selection and hover are
            // surface-discriminated (via `surfaceId`) or global, so they stay unscoped orphans and
            // appear on both windows.
            .window_kind_actions(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![
                "addFrame".into(), "addPage".into(), "patchPage".into(), "patchFrame".into(),
            ])
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS) — `config_spec()`/`layout_io()`
            // are this same information's single source of truth, reused here rather than duplicated.
            .config(LayoutPlayApp::default().config_spec())
            .io(layout_engine::layout_io()),
    )
    .example("sample", "Sample", layout_engine::layout_sample_document_json(), "cylinder")
    .workflow("layout", "Layout", "layout")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    // #region wasm_session
    use std::cell::RefCell;
    use std::rc::Rc;

    use infinite_canvas::camera::{self, Camera, Viewport};
    use infinite_canvas::Point;
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    use layout_engine::{build_scene_from_document_json, hit_test_document_json, screen_to_world_json, LayoutDropPreview, SceneQuery};
    use layout_engine::{export_document_pdf, export_document_png_cpu, export_document_svg, export_package_zip, parse_layout_document};

    #[derive(Clone, Debug)]
    enum LayoutInteraction {
        None,
        Pan { origin: Camera, start_screen: Point },
    }

    struct LayoutSessionInner {
        document_json: String,
        page_id: String,
        selected_ids: Vec<String>,
        hovered_id: Option<String>,
        chrome_blueprint: bool,
        camera: Camera,
        viewport: Viewport,
        interaction: LayoutInteraction,
        drop_preview: Option<LayoutDropPreview>,
        gpu: infinite_canvas::gpu_session::CanvasGpuSession,
    }

    #[wasm_bindgen]
    pub struct LayoutSession {
        state: Rc<RefCell<LayoutSessionInner>>,
    }

    #[wasm_bindgen]
    impl LayoutSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self {
                state: Rc::new(RefCell::new(LayoutSessionInner {
                    document_json: String::new(),
                    page_id: "page-1".into(),
                    selected_ids: Vec::new(),
                    hovered_id: None,
                    chrome_blueprint: true,
                    camera: Camera::default(),
                    viewport: Viewport::default(),
                    interaction: LayoutInteraction::None,
                    drop_preview: None,
                    gpu: infinite_canvas::gpu_session::CanvasGpuSession::default(),
                })),
            }
        }

        #[wasm_bindgen(js_name = gpuReady)]
        pub fn gpu_ready(&self) -> bool {
            self.state.borrow().gpu.gpu_ready()
        }

        #[wasm_bindgen(js_name = attachCanvas)]
        pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> Promise {
            let inner = self.state.clone();
            if inner.borrow().gpu.gpu_ready() {
                return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
            }
            let lw = logical_w.max(1);
            let lh = logical_h.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            let canvas = canvas.clone();
            future_to_promise(async move {
                let (render_ctx, renderer, surface) = infinite_canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                if g.gpu.gpu_ready() {
                    return Err(JsValue::from_str("canvas surface already attached"));
                }
                g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
                g.viewport.set_size(lw, lh, dpr);
                Ok(JsValue::UNDEFINED)
            })
        }

        #[wasm_bindgen(js_name = setSize)]
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            let lw = width.max(1);
            let lh = height.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            let mut inner = self.state.borrow_mut();
            inner.viewport.set_size(lw, lh, dpr);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = setCamera)]
        pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
            let mut inner = self.state.borrow_mut();
            inner.camera.x = x;
            inner.camera.y = y;
            inner.camera.zoom = camera::clamp_zoom(zoom);
        }

        #[wasm_bindgen(js_name = setDocumentJson)]
        pub fn set_document_json(&mut self, json: &str) -> Result<(), JsValue> {
            parse_layout_document(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.state.borrow_mut().document_json = json.to_string();
            Ok(())
        }

        #[wasm_bindgen(js_name = setPageId)]
        pub fn set_page_id(&mut self, page_id: &str) {
            self.state.borrow_mut().page_id = page_id.to_string();
        }

        #[wasm_bindgen(js_name = setSelectedIdsJson)]
        pub fn set_selected_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
            let ids: Vec<String> = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.state.borrow_mut().selected_ids = ids;
            Ok(())
        }

        #[wasm_bindgen(js_name = setHoveredId)]
        pub fn set_hovered_id(&mut self, hovered_id: Option<String>) {
            self.state.borrow_mut().hovered_id = hovered_id;
        }

        #[wasm_bindgen(js_name = setChromeMode)]
        pub fn set_chrome_mode(&mut self, blueprint: bool) {
            self.state.borrow_mut().chrome_blueprint = blueprint;
        }

        #[wasm_bindgen(js_name = setDropPreview)]
        pub fn set_drop_preview(&mut self, kind: &str, x: f64, y: f64) {
            self.state.borrow_mut().drop_preview = Some(LayoutDropPreview { kind: kind.to_string(), x, y });
        }

        #[wasm_bindgen(js_name = clearDropPreview)]
        pub fn clear_drop_preview(&mut self) {
            self.state.borrow_mut().drop_preview = None;
        }

        #[wasm_bindgen(js_name = pointerDownScreen)]
        pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
            if button != 1 {
                return;
            }
            let mut inner = self.state.borrow_mut();
            inner.interaction = LayoutInteraction::Pan { origin: inner.camera.clone(), start_screen: Point::new(sx, sy) };
        }

        #[wasm_bindgen(js_name = pointerMoveScreen)]
        pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
            let mut inner = self.state.borrow_mut();
            let LayoutInteraction::Pan { origin, start_screen } = inner.interaction.clone() else {
                return;
            };
            let delta = Point::new(sx, sy) - start_screen;
            inner.camera.x = origin.x - delta.x / origin.zoom;
            inner.camera.y = origin.y - delta.y / origin.zoom;
            inner.interaction = LayoutInteraction::Pan { origin, start_screen };
        }

        #[wasm_bindgen(js_name = pointerUpScreen)]
        pub fn pointer_up_screen(&mut self, _sx: f64, _sy: f64) {
            self.state.borrow_mut().interaction = LayoutInteraction::None;
        }

        #[wasm_bindgen(js_name = wheelScreen)]
        pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
            let mut inner = self.state.borrow_mut();
            let viewport = inner.viewport.clone();
            camera::wheel_screen(&mut inner.camera, &viewport, sx, sy, delta_y);
        }

        #[wasm_bindgen(js_name = screenToWorld)]
        pub fn screen_to_world(&self, sx: f64, sy: f64) -> String {
            let inner = self.state.borrow();
            screen_to_world_json(&inner.camera, &inner.viewport, sx, sy)
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let hovered = inner.hovered_id.as_deref();
            let drop_preview = inner.drop_preview.clone();
            let query = SceneQuery { page_id: &inner.page_id, selected_ids: &inner.selected_ids, hovered_id: hovered, chrome_blueprint: inner.chrome_blueprint, camera: &inner.camera, viewport: &inner.viewport };
            let scene = build_scene_from_document_json(&inner.document_json, &query, drop_preview.as_ref()).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let clear = infinite_canvas::theme::default_raster_clear();
            inner.gpu.render_frame(&scene, clear).map_err(|e| e)
        }

        #[wasm_bindgen(js_name = hitTest)]
        pub fn hit_test(&self, sx: f32, sy: f32) -> Result<JsValue, JsValue> {
            let inner = self.state.borrow();
            let hovered = inner.hovered_id.as_deref();
            let query = SceneQuery { page_id: &inner.page_id, selected_ids: &inner.selected_ids, hovered_id: hovered, chrome_blueprint: true, camera: &inner.camera, viewport: &inner.viewport };
            let hit = hit_test_document_json(&inner.document_json, sx as f64, sy as f64, &query).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(hit.map(|id| JsValue::from_str(&id)).unwrap_or(JsValue::NULL))
        }

        #[wasm_bindgen(js_name = exportPng)]
        pub fn export_png(&self, page_id: &str) -> Result<Vec<u8>, JsValue> {
            let inner = self.state.borrow();
            let doc = parse_layout_document(&inner.document_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            export_document_png_cpu(&doc, page_id).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = exportSvg)]
        pub fn export_svg(&self, page_id: &str) -> Result<String, JsValue> {
            let inner = self.state.borrow();
            let doc = parse_layout_document(&inner.document_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            export_document_svg(&doc, page_id).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = exportPdf)]
        pub fn export_pdf(&self, page_id: &str) -> Result<Vec<u8>, JsValue> {
            let inner = self.state.borrow();
            let doc = parse_layout_document(&inner.document_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            export_document_pdf(&doc, page_id).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = exportPackage)]
        pub fn export_package(&self, preflight_json: &str) -> Result<Vec<u8>, JsValue> {
            let inner = self.state.borrow();
            export_package_zip(&inner.document_json, preflight_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
    // #endregion wasm_session
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::LayoutSession;
//#endregion 🔖️WasmSession

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, ViewState, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<LayoutPlayApp> {
        testkit::new_app::<LayoutPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so kind discipline runs.
    fn new_app_with_registry() -> VcsDocumentApp<LayoutPlayApp> {
        testkit::new_app_with_registry::<LayoutPlayApp>(create_layout_app)
    }

    fn render_json(app: &mut VcsDocumentApp<LayoutPlayApp>, body: &str) -> String {
        let node = app.render(body, None, &ViewState::default()).expect("render");
        serde_json::to_string(&node).unwrap()
    }

    fn render_json_locale(app: &mut VcsDocumentApp<LayoutPlayApp>, body: &str, locale: &str) -> String {
        app.dispatch_typed(LayoutCommand::SetLocale { value: locale.into() }, &testkit::meta("local")).expect("set locale");
        render_json(app, body)
    }

    fn scene_layers_json(node: &UiNode) -> String {
        let value: Value = serde_json::to_value(node).unwrap();
        value["canvas2d"]["layersJson"].as_str().expect("layersJson string").to_string()
    }

    fn test_screen_point(camera_x: f64, camera_y: f64, zoom: f64, width: f64, height: f64, world_x: f64, world_y: f64) -> (f64, f64) {
        let camera = infinite_canvas::camera::Camera { x: camera_x, y: camera_y, zoom };
        let viewport = infinite_canvas::camera::Viewport { width: width as u32, height: height as u32, dpr: 1.0 };
        let screen = infinite_canvas::camera::world_to_screen(&camera, &viewport, infinite_canvas::Point::new(world_x, world_y));
        (screen.x, screen.y)
    }

    #[test]
    fn renders_blueprint_canvas_scene() {
        let mut app = new_app();
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("canvas-2d"));
    }

    #[test]
    fn renders_preview_canvas_scene() {
        let mut app = new_app();
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_PREVIEW).contains("canvas-2d"));
    }

    #[test]
    fn window_kind_actions_scope_authoring_to_blueprint_only() {
        let definition = create_layout_app().definition;
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window)
                .into_iter()
                .map(|action| action.id.clone())
                .collect()
        };
        let blueprint = resolve(LAYOUT_PLAY_WINDOW_BLUEPRINT);
        let preview = resolve(LAYOUT_PLAY_WINDOW_PREVIEW);
        for authoring in ["addFrame", "addPage", "patchPage", "patchFrame"] {
            assert!(blueprint.contains(&authoring.to_string()), "Blueprint must expose {authoring}");
            assert!(!preview.contains(&authoring.to_string()), "Preview must NOT expose {authoring}");
        }
        for shared in ["exportPng", "exportPdf", "setCamera"] {
            assert!(blueprint.contains(&shared.to_string()) && preview.contains(&shared.to_string()), "{shared} stays on both windows");
        }
    }

    #[test]
    fn document_lists_sample_pages() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.page.page-1"));
        assert!(json.contains("Page 1"));
    }

    #[test]
    fn catalogue_lists_frame_kinds() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(json.contains("layout-catalogue.rect"));
        assert!(json.contains("Text Frame"));
    }

    #[test]
    fn layout_labels_resolve_native_english_by_default() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("\"Frames\""));
        assert!(json.contains("\"Layers\""));
        let catalogue = render_json(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(catalogue.contains("Rectangle"));
        assert!(!json.contains("Rahmen"));
    }

    #[test]
    fn layout_labels_translate_document_tree_in_german() {
        let mut app = new_app();
        let json = render_json_locale(&mut app, LAYOUT_PLAY_BODY_DOCUMENT, "de");
        assert!(json.contains("\"Rahmen\""));
        assert!(json.contains("\"Ebenen\""));
        let catalogue = render_json_locale(&mut app, LAYOUT_PLAY_BODY_CATALOGUE, "de");
        assert!(catalogue.contains("Rechteck"));
        assert!(!json.contains("\"Frames\""));
    }

    #[test]
    fn preflight_finds_missing_asset() {
        let issues = run_layout_preflight(&layout_engine::default_document(), &LayoutLabels::EN);
        assert!(issues.iter().any(|issue| issue.code == "asset.missing"));
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_PREFLIGHT);
        assert!(json.contains("asset.missing") || json.contains("Linked asset missing"));
    }

    #[test]
    fn set_selection_reflects_in_inspector() {
        let mut app = new_app();
        app.dispatch_typed(LayoutCommand::SetSelection { ids: vec!["frame-text-1".into()] }, &testkit::meta("local")).expect("select");
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_INSPECTION);
        assert!(json.contains("frame-text-1"));
    }

    #[test]
    fn sample_fixture_parses() {
        let doc = layout_dsl::parse_dsl(layout_dsl::LAYOUT_SAMPLE_TEXT).expect("sample fixture");
        assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn add_frame_action_appends_rect() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        let result = app.dispatch_typed(LayoutCommand::AddFrame { kind: "rect".into(), x: None, y: None }, &testkit::meta("local")).expect("add");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").pages[0].frames.len(), before + 1);
    }

    #[test]
    fn undo_redo_round_trips_add_frame() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            LayoutCommand::AddFrame { kind: "rect".into(), x: None, y: None },
            |app| app.projection().expect("projection").pages[0].frames.len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn patch_page_supports_margins_and_columns() {
        let mut app = new_app();
        for (field, value) in [
            ("marginTop", 60.0),
            ("marginRight", 40.0),
            ("marginBottom", 60.0),
            ("marginLeft", 40.0),
            ("columnsGutter", 18.0),
        ] {
            let result = app
                .dispatch_typed(LayoutCommand::PatchPage { page_id: Some("page-1".into()), field: field.into(), value: value.to_string() }, &testkit::meta("local"))
                .expect("patch");
            assert_eq!(result.operations.len(), 1, "field {field} should apply");
        }
        app.dispatch_typed(LayoutCommand::PatchPage { page_id: Some("page-1".into()), field: "columnsCount".into(), value: "3".into() }, &testkit::meta("local")).expect("cols");
        let page = app.projection().expect("projection").pages.into_iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    fn patch_frame_supports_rect_fill_and_stroke() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        app.dispatch_typed(LayoutCommand::AddFrame { kind: "rect".into(), x: None, y: None }, &testkit::meta("local")).expect("add");
        let frame_id = format!("frame-{}", before + 1);
        let result = app
            .dispatch_typed(
                LayoutCommand::PatchFrame { frame_id: frame_id.clone(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() },
                &testkit::meta("local"),
            )
            .expect("patch");
        assert_eq!(result.operations.len(), 1);
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect frame") };
        assert_eq!(fill.unwrap(), [0.5, 0.4, 0.3, 1.0]);
    }

    #[test]
    fn patch_frame_supports_text_story_content_and_wrap_mode() {
        let mut app = new_app();
        app.dispatch_typed(
            LayoutCommand::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "storyContent".into(), value: "Edited story body.".into() },
            &testkit::meta("local"),
        )
        .expect("story");
        let story = app.projection().expect("projection").stories.into_iter().find(|story| story.id == "story-1").unwrap();
        assert_eq!(story.content, "Edited story body.");

        app.dispatch_typed(
            LayoutCommand::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "wrapMode".into(), value: "contour".into() },
            &testkit::meta("local"),
        )
        .expect("wrap");
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
        let Frame::Text { wrap_mode, .. } = frame else { panic!("expected text frame") };
        assert_eq!(wrap_mode, "contour");
    }

    #[test]
    fn patch_frame_supports_image_link_path() {
        let mut app = new_app();
        app.dispatch_typed(
            LayoutCommand::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "linkPath".into(), value: "assets/updated.png".into() },
            &testkit::meta("local"),
        )
        .expect("link");
        let link = app.projection().expect("projection").links.into_iter().find(|link| link.id == "link-missing").unwrap();
        assert_eq!(link.path, "assets/updated.png");
    }

    #[test]
    fn export_actions_wire_to_real_layout_rs_exporters() {
        let mut app = new_app();
        let exports: Vec<(LayoutCommand, &str)> = vec![
            (LayoutCommand::ExportPng { page_id: Some("page-1".into()) }, "image/png"),
            (LayoutCommand::ExportSvg { page_id: Some("page-1".into()) }, "image/svg+xml"),
            (LayoutCommand::ExportPdf { page_id: Some("page-1".into()) }, "application/pdf"),
            (LayoutCommand::ExportPackage, "application/zip"),
        ];
        for (command, mime_type) in exports {
            let result = app.dispatch_typed(command, &testkit::meta("local")).expect("export");
            match result.requested_effects.first() {
                Some(HostEffect::DownloadMediaExport { mime_type: mime, data, .. }) => {
                    assert_eq!(mime, mime_type);
                    assert!(!data.is_empty(), "export data");
                }
                other => panic!("expected DownloadMediaExport, got {other:?}"),
            }
        }
    }

    #[test]
    fn blueprint_scene_has_page_background_and_guides() {
        let mut app = new_app();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, None, &ViewState::default()).expect("render");
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
        let mut app = new_app();
        let node = app.render(LAYOUT_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("render");
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("layout.page-bg"));
        assert!(!layers_json.contains("layout.guide."));
    }

    #[test]
    fn inherited_frame_gets_dashed_stroke_in_blueprint() {
        let mut app = new_app();
        let node = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, None, &ViewState::default()).expect("render");
        let layers_json = scene_layers_json(&node);
        assert!(layers_json.contains("\"dash\":[4.0,3.0]"));
    }

    #[test]
    fn selected_and_hovered_frames_get_chrome_strokes() {
        let mut app = new_app();
        app.dispatch_typed(LayoutCommand::SetSelection { ids: vec!["frame-text-1".into()] }, &testkit::meta("local")).expect("select");
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("2.5"));

        app.dispatch_typed(LayoutCommand::SetHover { id: Some("frame-image-1".into()) }, &testkit::meta("local")).expect("hover");
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("1.75"));
    }

    #[test]
    fn set_camera_mutates_config_and_emits_no_operations() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        let result = app
            .dispatch_typed(
                LayoutCommand::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 } },
                &testkit::meta("local"),
            )
            .expect("camera");
        assert!(result.operations.is_empty(), "camera is a config action and emits no operations");
        assert_eq!(app.projection().expect("projection"), before, "camera never mutates the document");
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT);
        assert!(json.contains(r#""cameraX":10.0"#), "blueprint scene reflects config camera: {json}");
        assert!(json.contains(r#""cameraY":20.0"#), "blueprint scene reflects config camera: {json}");
        assert!(json.contains(r#""zoom":1.5"#), "blueprint scene reflects config camera: {json}");
    }

    #[test]
    fn set_camera_preview_surface_updates_independently_of_blueprint() {
        let mut app = new_app();
        let result = app
            .dispatch_typed(
                LayoutCommand::SetCamera { surface_id: Some(LAYOUT_PLAY_SURFACE_PREVIEW.into()), camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 } },
                &testkit::meta("local"),
            )
            .expect("camera");
        assert!(result.operations.is_empty(), "camera is a config action and emits no operations");
        let preview_json = render_json(&mut app, LAYOUT_PLAY_BODY_PREVIEW);
        assert!(preview_json.contains(r#""cameraX":3.0"#), "preview scene reflects config camera: {preview_json}");
        assert!(preview_json.contains(r#""zoom":2.0"#), "preview scene reflects config camera: {preview_json}");
        let blueprint_json = render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT);
        assert!(blueprint_json.contains(r#""cameraX":0.0"#), "blueprint surface camera stays independent: {blueprint_json}");
        assert!(blueprint_json.contains(r#""zoom":1.0"#), "blueprint surface camera stays at default zoom: {blueprint_json}");
    }

    #[test]
    fn pointer_down_selects_frame_via_hit_test() {
        let mut app = new_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 136.0, 435.0);
        app.dispatch_typed(
            LayoutCommand::CanvasPointerDown { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), button: 0, extend: false, x: sx, y: sy, width: 800.0, height: 600.0 },
            &testkit::meta("local"),
        )
        .expect("pointer");
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.frame.frame-image-1"));
    }

    #[test]
    fn pointer_move_updates_hover_highlight() {
        let mut app = new_app();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
        let result = app
            .dispatch_typed(LayoutCommand::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0 }, &testkit::meta("local"))
            .expect("move");
        assert!(result.operations.is_empty(), "hover is a config action, not an operation");
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
        assert!(json.contains("layout-document.frame.frame-text-1"));
    }

    #[test]
    fn canvas_drop_adds_frame_at_world_coords() {
        let mut app = new_app();
        // 👁️ Camera pose is now config-default (see `LayoutCamera::default`), not a document field —
        // the screen point is computed against that same default (x=0 y=0 zoom=1.0), not the app's
        // former fixture-authored zoom=0.5.
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 100.0, 200.0);
        let result = app
            .dispatch_typed(
                LayoutCommand::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: sx, y: sy, width: 800.0, height: 600.0 },
                &testkit::meta("local"),
            )
            .expect("drop");
        assert_eq!(result.operations.len(), 1);
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.last().unwrap();
        let bounds = frame.bounds();
        assert!((bounds.x - 100.0).abs() < 0.01);
        assert!((bounds.y - 200.0).abs() < 0.01);
    }

    #[test]
    fn canvas_drop_page_kind_adds_page() {
        let mut app = new_app();
        let before = app.projection().expect("projection").pages.len();
        let result = app
            .dispatch_typed(
                LayoutCommand::CanvasDrop { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "page".into(), x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
                &testkit::meta("local"),
            )
            .expect("drop");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").pages.len(), before + 1);
    }

    #[test]
    fn drag_over_emits_ghost_and_leave_clears() {
        let mut app = new_app();
        app.dispatch_typed(
            LayoutCommand::CanvasDragOver { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), kind: "rect".into(), x: 400.0, y: 300.0, width: 800.0, height: 600.0 },
            &testkit::meta("local"),
        )
        .expect("over");
        assert!(render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));

        app.dispatch_typed(LayoutCommand::CanvasDragLeave, &testkit::meta("local")).expect("leave");
        assert!(!render_json(&mut app, LAYOUT_PLAY_BODY_BLUEPRINT).contains("layout.drop-preview"));
    }

    #[test]
    fn catalogue_items_are_draggable() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(json.contains(LAYOUT_CATALOGUE_DRAG_MIME));
        assert!(json.contains("\"draggable\":true"));
        assert!(json.contains("layout-catalogue.page"));
    }

    #[test]
    fn document_tree_has_nine_sections() {
        let mut app = new_app();
        let json = render_json(&mut app, LAYOUT_PLAY_BODY_DOCUMENT);
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
            assert!(json.contains(section_id), "missing section {section_id}");
        }
    }

    #[test]
    fn preflight_reports_all_expected_issue_codes() {
        let json = r#"{
            "schema": "layout.fixture",
            "name": "Preflight Fixture",
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
        let issues = run_layout_preflight(&doc, &LayoutLabels::EN);
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
        let mut app = new_app();
        let engagements = app.window_engagements();
        let blueprint = engagements.get(LAYOUT_PLAY_WINDOW_BLUEPRINT).expect("blueprint engagement");
        let status = blueprint.status.as_ref().and_then(|rows| rows.first()).expect("status");
        assert!(status.text.contains("Page"));
        let input = blueprint.input.as_ref().expect("input");
        assert_eq!(input.placeholder.as_deref(), Some("undo, redo, export png"));
        assert!(engagements.contains_key(LAYOUT_PLAY_WINDOW_PREVIEW));
    }

    #[test]
    fn registry_backed_engagement_submit_is_shell_effect_not_operation() {
        // 🧬️ engagementSubmit is declared `Shell`: through the real registry the kind-discipline
        // check must accept it because its handler only routes an export `HostEffect`, never operations.
        let mut app = new_app_with_registry();
        let result = app
            .dispatch_typed(LayoutCommand::EngagementSubmit { value: "export png".into() }, &testkit::meta("local"))
            .expect("engagementSubmit passes registry kind discipline");
        assert!(result.operations.is_empty(), "Shell action must not emit document operations");
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }

    #[test]
    fn registry_backed_add_frame_emits_operation() {
        // 🧬️ addFrame is declared `Operation`: the registry-backed wrapper must let its operations through.
        let mut app = new_app_with_registry();
        let result = app
            .dispatch_typed(LayoutCommand::AddFrame { kind: "rect".into(), x: None, y: None }, &testkit::meta("local"))
            .expect("addFrame passes registry kind discipline");
        assert_eq!(result.operations.len(), 1);
    }

    #[test]
    fn registry_backed_pointer_move_is_view_only() {
        // 🧬️ canvasPointerMove is declared `View`: it mutates only config hover state and must
        // never emit an operation, which the registry kind-discipline check enforces.
        let mut app = new_app_with_registry();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
        let result = app
            .dispatch_typed(
                LayoutCommand::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0 },
                &testkit::meta("local"),
            )
            .expect("canvasPointerMove passes registry kind discipline");
        assert!(result.operations.is_empty(), "View action must not emit document operations");
    }

    #[test]
    fn engagement_submit_triggers_export() {
        let mut app = new_app();
        let result = app.dispatch_typed(LayoutCommand::EngagementSubmit { value: "export png".into() }, &testkit::meta("local")).expect("submit");
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }

    #[test]
    fn engagement_submit_triggers_export_from_normalized_shell_draft() {
        // The React shell PascalCases and strips separators from every draft before submitting it
        // (`normalizeEngagementActionText`), so "export png" arrives as "ExportPng".
        let mut app = new_app();
        let result = app.dispatch_typed(LayoutCommand::EngagementSubmit { value: "ExportPng".into() }, &testkit::meta("local")).expect("submit");
        assert!(matches!(result.requested_effects.first(), Some(HostEffect::DownloadMediaExport { mime_type, .. }) if mime_type == "image/png"));
    }

    //#region 🧪️MediaPorts
    #[test]
    fn export_media_layout_out_returns_svg_of_first_page() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = LayoutPlayApp.export_media("layout:out", &doc).expect("export layout:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.layout");
        assert!(json.starts_with("<svg"));
    }

    #[test]
    fn export_media_document_out_round_trips_through_pack() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = LayoutPlayApp.export_media("document:out", &doc).expect("export document:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, LAYOUT_FIXTURE_SCHEMA);
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
        let decoded = <LayoutDocument as store::DocumentPack>::decode_pack(&bytes).expect("decode pack");
        assert_eq!(decoded, document);
    }

    #[test]
    fn import_media_fields_in_sets_data_fields_json() {
        let mut app = new_app();
        let media = Media {
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            payload: MediaPayload::Structured { schema: "form.dictionary".into(), json: r#"{"name":"Ada"}"#.into() },
        };
        app.import_media("fields:in", &media, &testkit::meta("local")).expect("import fields:in");
        let document = app.projection().expect("projection");
        assert_eq!(document.data_fields_json.as_deref(), Some(r#"{"name":"Ada"}"#));
    }

    #[test]
    fn layout_io_exposes_declared_ports() {
        let io = LayoutPlayApp.io().expect("layout declares io");
        assert!(io.ports.iter().any(|port| port.id == "fields:in"));
        assert!(io.ports.iter().any(|port| port.id == "layout:out"));
    }
    //#endregion 🧪️MediaPorts
}
//#endregion 🧪️Tests
