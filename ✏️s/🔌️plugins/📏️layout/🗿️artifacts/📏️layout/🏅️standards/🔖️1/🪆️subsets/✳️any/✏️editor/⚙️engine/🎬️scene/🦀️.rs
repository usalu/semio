//! 🖼️ Layout app engine — display-list construction, glyph layout, scene painting, hit-testing and
//! export (SVG/PDF/PNG/zip). Sibling topic file of `🦀️.rs` (headless compute is split across the
//! two purely because of size — see the master template's "+ sibling topic file for big engines" note).
//!
//! Relocated wholesale from the deleted artifact-tree `⚙️engine/🎬️scene` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `LayoutEngine` owns `&mut self` rendering
//! state (font/layout contexts) threaded through canvas, pointer, wasm and window-render call sites
//! across this app — the textbook D5 Behavioral case, and one of the two `*Engine` structs in this
//! ticket that IS constructed outside its own file (see the region → destination map's exception
//! clause). `parse_layout_document`/`resolve_page` (pure, artifact-level) stayed at `🧬️schema`;
//! `compose_svg_from_drawing`/`rect_path_segments`/`LayoutError` (io/codec-dispatch territory) stayed
//! at `🚪️io` — this file reaches both by qualified path, which is the normal app→artifact direction.

use crate::io::LayoutError;
use crate::standards::v1::subsets::any::schema::{parse_layout_document, resolve_page};
use crate::{Frame, LayoutBounds, LayoutRect, LayoutSnapshot, Page, ParagraphStyle, TextStory};
use infinite_canvas::camera::{self, Camera, Viewport};
use infinite_canvas::{Affine, BezPath, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};
#[cfg(test)]
use serde_json::Value;
use ui_render::{FontDependencyId, FontFamilyChoice, ShapedText, TextAlignment, TextRunStyle, TextStyle, TextSystem};

//#region 🖼️Display
#[derive(Clone, Debug)]
pub struct DisplayColor(pub [f32; 4]);

#[derive(Clone, Debug)]
pub struct DisplayGlyph {
    pub glyph_id: u32,
    pub font_size: f32,
    pub x: f32,
    pub y: f32,
    pub color: DisplayColor,
    pub italic: bool,
}

#[derive(Clone, Debug)]
pub struct DisplayRect {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub fill: Option<DisplayColor>,
    pub stroke: Option<DisplayColor>,
    pub inherited: bool,
    pub selected: bool,
    pub hovered: bool,
    pub stack: u32,
}

#[derive(Clone, Debug)]
pub struct DisplayGuide {
    pub rect: LayoutRect,
    pub kind: String,
}

#[derive(Clone, Debug)]
pub struct DisplayTextRun {
    pub object_id: String,
    pub glyphs: Vec<DisplayGlyph>,
    pub content: String,
    pub origin_x: f32,
    pub origin_y: f32,
    pub font_size: f32,
    pub stack: u32,
    pub color: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct DisplayImage {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub placeholder: bool,
    pub proxy_data_url: Option<String>,
    pub preview: String,
    pub stack: u32,
}

#[derive(Clone, Debug)]
pub struct DisplayStroke {
    pub points: Vec<(f32, f32)>,
    pub closed: bool,
    pub color: [f32; 4],
    pub width: f32,
    pub fill: Option<[f32; 4]>,
    pub stack: u32,
}

pub struct DisplayList {
    pub page_id: String,
    pub page_width: f32,
    pub page_height: f32,
    pub strokes: Vec<DisplayStroke>,
    pub frame_strokes: Vec<DisplayStroke>,
    pub rects: Vec<DisplayRect>,
    pub text_runs: Vec<DisplayTextRun>,
    pub images: Vec<DisplayImage>,
    pub guides: Vec<DisplayGuide>,
}

pub enum StackPiece<'a> {
    Stroke(&'a DisplayStroke),
    Rect(&'a DisplayRect),
    Image(&'a DisplayImage),
    FrameStroke(&'a DisplayStroke),
    Text(&'a DisplayTextRun),
}

pub fn stack_pieces(list: &DisplayList) -> Vec<StackPiece<'_>> {
    let mut ranked: Vec<(u32, u8, StackPiece<'_>)> = Vec::new();
    for stroke in &list.strokes {
        ranked.push((stroke.stack, 0, StackPiece::Stroke(stroke)));
    }
    for rect in &list.rects {
        ranked.push((rect.stack, 1, StackPiece::Rect(rect)));
    }
    for image in &list.images {
        ranked.push((image.stack, 2, StackPiece::Image(image)));
    }
    for stroke in &list.frame_strokes {
        ranked.push((stroke.stack, 3, StackPiece::FrameStroke(stroke)));
    }
    for run in &list.text_runs {
        ranked.push((run.stack, 4, StackPiece::Text(run)));
    }
    ranked.sort_by_key(|(stack, rank, _)| (*stack, *rank));
    ranked.into_iter().map(|(_, _, piece)| piece).collect()
}

pub fn paint_order_ids(list: &DisplayList) -> Vec<String> {
    stack_pieces(list)
        .into_iter()
        .filter_map(|piece| match piece {
            StackPiece::Rect(rect) => Some(rect.object_id.clone()),
            StackPiece::Image(image) if !image.object_id.starts_with("drawing-") => Some(image.object_id.clone()),
            _ => None,
        })
        .collect()
}

impl DisplayList {
    pub fn hit_test(&self, x: f32, y: f32) -> Option<String> {
        for piece in stack_pieces(self).into_iter().rev() {
            match piece {
                StackPiece::Rect(rect) if x >= rect.x && x <= rect.x + rect.width && y >= rect.y && y <= rect.y + rect.height => return Some(rect.object_id.clone()),
                StackPiece::Image(image) if !image.object_id.starts_with("drawing-") && x >= image.x && x <= image.x + image.width && y >= image.y && y <= image.y + image.height => return Some(image.object_id.clone()),
                _ => {}
            }
        }
        None
    }
}

/// 👆 Topmost visible frame whose axis-aligned bounds contain `(x, y)`.
///
/// Parent frames are tested first and page frames after, matching paint order.
/// This does not construct a [`LayoutEngine`] and does not shape text.
pub fn hit_test_page_frames(doc: &LayoutSnapshot, page: &Page, x: f32, y: f32) -> Option<String> {
    let mut hit = None;
    let mut consider = |frame: &Frame| {
        if !frame.visible() {
            return;
        }
        let bounds = frame.bounds();
        if frame_bounds_contain(bounds, x, y) {
            hit = Some(frame.id().to_string());
        }
    };
    if let Some(parent_id) = &page.parent_page_id {
        if let Some(parent) = doc.parent_pages.iter().find(|parent| parent.id == *parent_id) {
            for frame in &parent.frames {
                consider(frame);
            }
        }
    }
    for frame in &page.frames {
        consider(frame);
    }
    hit
}

pub fn page_margin_guides(page: &Page) -> Vec<DisplayGuide> {
    vec![DisplayGuide { rect: LayoutRect { x: page.margins.left, y: page.margins.top, width: page.width - page.margins.left - page.margins.right, height: page.height - page.margins.top - page.margins.bottom }, kind: "margin".into() }]
}

pub fn bounds_to_display_rect(object_id: &str, bounds: &LayoutBounds, inherited: bool, selected: bool, hovered: bool, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> DisplayRect {
    DisplayRect { object_id: object_id.into(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, rotation: bounds.rotation as f32, fill: fill.map(DisplayColor), stroke: stroke.map(DisplayColor), inherited, selected, hovered, stack: 0 }
}

fn frame_bounds_contain(bounds: &LayoutBounds, x: f32, y: f32) -> bool {
    let half_w = (bounds.width as f32) * 0.5;
    let half_h = (bounds.height as f32) * 0.5;
    let cx = bounds.x as f32 + half_w;
    let cy = bounds.y as f32 + half_h;
    let dx = x - cx;
    let dy = y - cy;
    let (local_x, local_y) = if bounds.rotation.abs() < 1.0e-6 {
        (dx, dy)
    } else {
        let (sin, cos) = (-bounds.rotation).sin_cos();
        (dx * cos as f32 - dy * sin as f32, dx * sin as f32 + dy * cos as f32)
    };
    local_x.abs() <= half_w && local_y.abs() <= half_h
}
//#endregion 🖼️Display

//#region ⚙️Scene
pub static LAYOUT_SANS: &[u8] = include_bytes!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🖼️assets/🔤️MapLabelSans.ttf");

pub struct LayoutEngine {
    text: TextSystem,
    font: FontDependencyId,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    /// 🏗️ Registers the embedded `LAYOUT_SANS` font under [`TextSystem`]'s
    /// request/provide-bytes seam synchronously — never `Pending`, since the bytes are already in
    /// the binary (`include_bytes!`), unlike a real host-fetched [`FontSource`].
    pub fn new() -> Self {
        let mut text = TextSystem::new();
        let font = text.request_font("layout-sans");
        text.provide_font_bytes(font, LAYOUT_SANS);
        Self { text, font }
    }

    pub fn layout_story(&mut self, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (ShapedText, bool) {
        let style = TextRunStyle {
            base: TextStyle { family: FontFamilyChoice::Custom(self.font), size_px: paragraph.font_size as f32 },
            weight: paragraph.font_weight as f32,
            line_height_relative: (paragraph.leading / paragraph.font_size.max(1.0)) as f32,
            letter_spacing_px: paragraph.tracking as f32,
            alignment: alignment_from_str(&paragraph.alignment),
        };
        let shaped = self.text.shape_paragraph(&story.content, &style, frame_width);
        let overset = shaped.height > frame_height;
        (shaped, overset)
    }
}

fn alignment_from_str(value: &str) -> TextAlignment {
    match value {
        "center" | "middle" => TextAlignment::Middle,
        "right" => TextAlignment::Right,
        "justify" | "justified" => TextAlignment::Justified,
        _ => TextAlignment::Left,
    }
}

fn default_paragraph(doc: &LayoutSnapshot) -> ParagraphStyle {
    doc.paragraph_styles.first().cloned().unwrap_or(ParagraphStyle { id: "paragraph.body".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() })
}

pub fn layout_story_in_frame(engine: &mut LayoutEngine, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (ShapedText, bool) {
    engine.layout_story(story, paragraph, frame_width, frame_height)
}

/// 👁️ Live canvas fidelity. Below this zoom the page is a proxy: frame chrome and image
/// proxies, no story strings. At or above it, stories are emitted as text runs without shaping.
pub const INTERACTIVE_TEXT_ZOOM: f64 = 0.35;

/// 🖼️ Host-canvas display list. Never shapes paragraphs and never loads a font.
pub fn build_interactive_display_list(doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool, camera_x: f64, camera_y: f64, zoom: f64) -> DisplayList {
    assemble_display_list(None, doc, page, active_page_id, selected_ids, hovered_id, chrome_blueprint, zoom < INTERACTIVE_TEXT_ZOOM, camera_x, camera_y, zoom)
}

pub fn build_display_list_for_page(engine: &mut LayoutEngine, doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool) -> DisplayList {
    assemble_display_list(Some(engine), doc, page, active_page_id, selected_ids, hovered_id, chrome_blueprint, false, 0.0, 0.0, 1.0)
}

const INTERACTIVE_VIEW_SPAN: f64 = 2000.0;

fn outside_interactive_view(bounds: &crate::LayoutBounds, camera_x: f64, camera_y: f64, zoom: f64) -> bool {
    let span = INTERACTIVE_VIEW_SPAN / zoom.max(0.05);
    let x1 = bounds.x + bounds.width;
    let y1 = bounds.y + bounds.height;
    x1 < camera_x - span || bounds.x > camera_x + span || y1 < camera_y - span || bounds.y > camera_y + span
}


fn frame_layer_visible(doc: &LayoutSnapshot, page: &Page, frame: &Frame) -> bool {
    let layer_id = frame.layer_id();
    if let Some(layer) = page.layers.iter().find(|layer| layer.id == layer_id) {
        return layer.visible;
    }
    if let Some(parent_id) = &page.parent_page_id {
        if let Some(parent) = doc.parent_pages.iter().find(|parent| parent.id == *parent_id) {
            if let Some(layer) = parent.layers.iter().find(|layer| layer.id == layer_id) {
                return layer.visible;
            }
        }
    }
    true
}


struct DrawingLabel {
    text: String,
    x: f32,
    y: f32,
}

fn raw_drawing_labels(doc: &LayoutSnapshot) -> Vec<DrawingLabel> {
    let Some(drawing) = doc.background_drawing.as_ref() else { return Vec::new() };
    let mut labels = Vec::new();
    for layer in &drawing.content.layers {
        if !layer.visible {
            continue;
        }
        collect_drawing_text(&layer.root, 0.0, 0.0, &mut labels);
    }
    labels
}

fn collect_drawing_text(node: &semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode, tx: f64, ty: f64, labels: &mut Vec<DrawingLabel>) {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode;
    match node {
        DrawNode::Group { transform, children } => {
            let next_x = tx + transform.translation.x;
            let next_y = ty + transform.translation.y;
            for child in children {
                collect_drawing_text(child, next_x, next_y, labels);
            }
        }
        DrawNode::Text { value, at, .. } => {
            let text = value.trim();
            if !text.is_empty() && labels.len() < 32 {
                labels.push(DrawingLabel { text: text.to_string(), x: (at.x + tx) as f32, y: (at.y + ty) as f32 });
            }
        }
        DrawNode::Path { .. } | DrawNode::Image { .. } => {}
    }
}

fn fit_from_points(points: impl Iterator<Item = (f32, f32)>, page_width: f64, page_height: f64) -> Option<(f32, f32, f32)> {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for (x, y) in points {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    if !min_x.is_finite() {
        return None;
    }
    let width = (max_x - min_x).max(0.0);
    let height = (max_y - min_y).max(0.0);
    let scale_x = if width > 0.0 { page_width as f32 / width } else { 1.0 };
    let scale_y = if height > 0.0 { page_height as f32 / height } else { 1.0 };
    let scale = scale_x.min(scale_y);
    let ox = (page_width as f32 - width * scale) * 0.5 - min_x * scale;
    let oy = (page_height as f32 - height * scale) * 0.5 - min_y * scale;
    Some((scale, ox, oy))
}

fn stroke_points(strokes: &[DisplayStroke]) -> impl Iterator<Item = (f32, f32)> + '_ {
    strokes.iter().flat_map(|stroke| stroke.points.iter().copied())
}

fn place_drawing_labels(labels: &[DrawingLabel], strokes: &[DisplayStroke], bounds: &LayoutBounds) -> Vec<DrawingLabel> {
    let fit = fit_from_points(stroke_points(strokes), bounds.width, bounds.height).or_else(|| fit_from_points(labels.iter().map(|label| (label.x, label.y)), bounds.width, bounds.height));
    let Some((scale, ox, oy)) = fit else { return Vec::new() };
    let center_x = (bounds.x + bounds.width * 0.5) as f32;
    let center_y = (bounds.y + bounds.height * 0.5) as f32;
    let radians = bounds.rotation as f32;
    let (sin, cos) = radians.sin_cos();
    labels
        .iter()
        .map(|label| {
            let page_x = label.x * scale + ox + bounds.x as f32;
            let page_y = label.y * scale + oy + bounds.y as f32;
            let (x, y) = if bounds.rotation.abs() < 1.0e-6 {
                (page_x, page_y)
            } else {
                let dx = page_x - center_x;
                let dy = page_y - center_y;
                (center_x + dx * cos - dy * sin, center_y + dx * sin + dy * cos)
            };
            DrawingLabel { text: label.text.clone(), x, y }
        })
        .collect()
}

fn shape_drawing_label(engine: &mut LayoutEngine, doc: &LayoutSnapshot, text: &str, x: f32, y: f32) -> DisplayTextRun {
    let story = TextStory { id: String::new(), content: text.to_string(), style_runs: Vec::new() };
    let mut paragraph = default_paragraph(doc);
    paragraph.font_size = 11.0;
    paragraph.leading = 13.2;
    paragraph.alignment = "left".into();
    let (shaped, _) = layout_story_in_frame(engine, &story, &paragraph, 1000.0, 1000.0);
    let glyphs = shaped.glyphs.iter().map(|glyph| DisplayGlyph { glyph_id: glyph.glyph_id as u32, font_size: 11.0, x: x + glyph.x, y: y + glyph.y, color: DisplayColor([0.1, 0.1, 0.1, 1.0]), italic: false }).collect();
    DisplayTextRun { object_id: "drawing-label".into(), glyphs, content: text.to_string(), origin_x: x, origin_y: y, font_size: 11.0, stack: 0, color: [0.0, 0.0, 0.0, 1.0] }
}

fn unshaped_drawing_label(text: &str, x: f32, y: f32) -> DisplayTextRun {
    DisplayTextRun { object_id: "drawing-label".into(), glyphs: Vec::new(), content: text.to_string(), origin_x: x, origin_y: y, font_size: 11.0, stack: 0, color: [0.0, 0.0, 0.0, 1.0] }
}

struct DrawingRaster {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    data_url: String,
}

fn png_data_url(bytes: &[u8]) -> Option<String> {
    if bytes.len() < 8 || bytes.len() > 8192 {
        return None;
    }
    semio_s_artifact_stdio_png::io::decode_png(bytes).ok()?;
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut encoded = String::new();
    for chunk in bytes.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        encoded.push(TABLE[(a >> 2) as usize] as char);
        encoded.push(TABLE[((a & 3) << 4 | b >> 4) as usize] as char);
        encoded.push(if chunk.len() > 1 { TABLE[((b & 15) << 2 | c >> 6) as usize] as char } else { '=' });
        encoded.push(if chunk.len() > 2 { TABLE[(c & 63) as usize] as char } else { '=' });
    }
    Some(format!("data:image/png;base64,{encoded}"))
}

fn raw_drawing_rasters(doc: &LayoutSnapshot) -> Vec<DrawingRaster> {
    let Some(drawing) = doc.background_drawing.as_ref() else { return Vec::new() };
    let mut rasters = Vec::new();
    for layer in &drawing.content.layers {
        if !layer.visible {
            continue;
        }
        collect_drawing_rasters(&layer.root, 0.0, 0.0, &mut rasters);
    }
    rasters
}

fn collect_drawing_rasters(node: &semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode, tx: f64, ty: f64, rasters: &mut Vec<DrawingRaster>) {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode;
    match node {
        DrawNode::Group { transform, children } => {
            let next_x = tx + transform.translation.x;
            let next_y = ty + transform.translation.y;
            for child in children {
                collect_drawing_rasters(child, next_x, next_y, rasters);
            }
        }
        DrawNode::Image { at, width, height, mime, bytes } => {
            if rasters.len() >= 8 || *width <= 0.0 || *height <= 0.0 || !mime.contains("png") {
                return;
            }
            let Some(data_url) = png_data_url(bytes) else { return };
            rasters.push(DrawingRaster { x: (at.x + tx) as f32, y: (at.y + ty) as f32, width: *width as f32, height: *height as f32, data_url });
        }
        DrawNode::Path { .. } | DrawNode::Text { .. } => {}
    }
}

fn push_drawing_raster(images: &mut Vec<DisplayImage>, raster: &DrawingRaster, x: f32, y: f32, width: f32, height: f32, rotation: f32) {
    if width <= 0.0 || height <= 0.0 {
        return;
    }
    images.push(DisplayImage { object_id: format!("drawing-image-{}", images.len()), x, y, width, height, rotation, placeholder: false, proxy_data_url: Some(raster.data_url.clone()), preview: String::new(), stack: 0 });
}

fn place_drawing_rasters(rasters: &[DrawingRaster], strokes: &[DisplayStroke], bounds: &LayoutBounds) -> Vec<DisplayImage> {
    let Some((scale, ox, oy)) = fit_from_points(stroke_points(strokes), bounds.width, bounds.height).or_else(|| fit_from_points(rasters.iter().flat_map(|raster| [(raster.x, raster.y), (raster.x + raster.width, raster.y + raster.height)]), bounds.width, bounds.height)) else { return Vec::new() };
    let mut images = Vec::new();
    for raster in rasters {
        push_drawing_raster(&mut images, raster, raster.x * scale + ox + bounds.x as f32, raster.y * scale + oy + bounds.y as f32, raster.width * scale, raster.height * scale, bounds.rotation as f32);
    }
    images
}

fn raw_drawing_strokes(doc: &LayoutSnapshot) -> Vec<DisplayStroke> {
    let Some(drawing) = doc.background_drawing.as_ref() else { return Vec::new() };
    let mut raw = Vec::new();
    for layer in &drawing.content.layers {
        if !layer.visible {
            continue;
        }
        collect_draw_node(&layer.root, &drawing.content.styles, 0.0, 0.0, &mut raw);
    }
    raw
}

fn drawing_strokes(doc: &LayoutSnapshot, page_width: f64, page_height: f64) -> Vec<DisplayStroke> {
    if page_width <= 0.0 || page_height <= 0.0 {
        return Vec::new();
    }
    fit_strokes(&raw_drawing_strokes(doc), page_width, page_height)
}

fn drawing_kind(kind: &str) -> bool {
    matches!(kind, "drawing" | "dwg" | "dxf" | "svg" | "cad" | "s.draw.drawing" | "s.stdio.dwg" | "s.stdio.dxf" | "s.stdio.svg" | "s.cad.cad")
}

fn place_drawing_in_frame(raw: &[DisplayStroke], bounds: &LayoutBounds) -> Vec<DisplayStroke> {
    if raw.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
        return Vec::new();
    }
    let fitted = fit_strokes(raw, bounds.width, bounds.height);
    let center_x = (bounds.x + bounds.width * 0.5) as f32;
    let center_y = (bounds.y + bounds.height * 0.5) as f32;
    let radians = bounds.rotation as f32;
    let (sin, cos) = radians.sin_cos();
    fitted
        .into_iter()
        .map(|stroke| DisplayStroke {
            closed: stroke.closed,
            color: stroke.color,
            width: stroke.width,
            fill: stroke.fill,
            stack: stroke.stack,
            points: stroke
                .points
                .into_iter()
                .map(|(x, y)| {
                    let page_x = x + bounds.x as f32;
                    let page_y = y + bounds.y as f32;
                    if bounds.rotation.abs() < 1.0e-6 {
                        (page_x, page_y)
                    } else {
                        let dx = page_x - center_x;
                        let dy = page_y - center_y;
                        (center_x + dx * cos - dy * sin, center_y + dx * sin + dy * cos)
                    }
                })
                .collect(),
        })
        .collect()
}

fn plan_stroke_paint(styles: &[semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawStyle], name: &Option<String>) -> (Option<[f32; 4]>, [f32; 4], f32) {
    let mut color = [0.15_f32, 0.2, 0.28, 1.0];
    let mut width = 1.0_f32;
    let mut fill = None;
    if let Some(style) = name.as_ref().and_then(|name| styles.iter().find(|style| style.name == *name)) {
        if let Some(stroke) = &style.stroke {
            color = [stroke.r, stroke.g, stroke.b, stroke.a];
        }
        if let Some(paint) = &style.fill {
            fill = Some([paint.r, paint.g, paint.b, paint.a]);
        }
        if let Some(opacity) = style.opacity {
            color[3] *= opacity;
            if let Some(fill_color) = fill.as_mut() {
                fill_color[3] *= opacity;
            }
        }
        if let Some(stroke_width) = style.stroke_width {
            width = stroke_width.clamp(0.1, 64.0) as f32;
        }
    }
    (fill, color, width)
}

fn collect_draw_node(node: &semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode, styles: &[semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawStyle], tx: f64, ty: f64, out: &mut Vec<DisplayStroke>) {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, PathSegment};
    match node {
        DrawNode::Group { transform, children } => {
            let next_x = tx + transform.translation.x;
            let next_y = ty + transform.translation.y;
            for child in children {
                collect_draw_node(child, styles, next_x, next_y, out);
            }
        }
        DrawNode::Path { segments, style } => {
            let (fill, color, width) = plan_stroke_paint(styles, style);
            let mut current: Option<(f64, f64)> = None;
            let mut points = Vec::new();
            let mut closed = false;
            let flush = |points: &mut Vec<(f32, f32)>, closed: &mut bool, out: &mut Vec<DisplayStroke>| {
                if points.len() >= 2 && out.len() < 128 {
                    out.push(DisplayStroke { points: std::mem::take(points), closed: *closed, color, width, fill, stack: 0 });
                } else {
                    points.clear();
                }
                *closed = false;
            };
            for segment in segments {
                match segment {
                    PathSegment::MoveTo { to } => {
                        flush(&mut points, &mut closed, out);
                        current = Some((to.x + tx, to.y + ty));
                        points.push(((to.x + tx) as f32, (to.y + ty) as f32));
                    }
                    PathSegment::LineTo { to } => {
                        let point = (to.x + tx, to.y + ty);
                        if points.is_empty() {
                            if let Some(start) = current {
                                points.push((start.0 as f32, start.1 as f32));
                            }
                        }
                        push_point(&mut points, point);
                        current = Some(point);
                    }
                    PathSegment::CubicTo { c1, c2, to } => {
                        let start = current.unwrap_or((c1.x + tx, c1.y + ty));
                        for step in 1..=4 {
                            let t = step as f64 / 4.0;
                            push_point(&mut points, cubic_point(start, (c1.x + tx, c1.y + ty), (c2.x + tx, c2.y + ty), (to.x + tx, to.y + ty), t));
                        }
                        current = Some((to.x + tx, to.y + ty));
                    }
                    PathSegment::QuadTo { c, to } => {
                        let start = current.unwrap_or((c.x + tx, c.y + ty));
                        for step in 1..=4 {
                            let t = step as f64 / 4.0;
                            push_point(&mut points, quad_point(start, (c.x + tx, c.y + ty), (to.x + tx, to.y + ty), t));
                        }
                        current = Some((to.x + tx, to.y + ty));
                    }
                    PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
                        let end = (to.x + tx, to.y + ty);
                        let start = current.unwrap_or(end);
                        let sampled = arc_points(start, *rx, *ry, *x_rotation, *large_arc, *sweep, end);
                        if sampled.is_empty() {
                            push_point(&mut points, end);
                        } else {
                            for point in sampled {
                                push_point(&mut points, point);
                            }
                        }
                        current = Some(end);
                    }
                    PathSegment::Close => closed = true,
                }
            }
            flush(&mut points, &mut closed, out);
        }
        DrawNode::Text { .. } | DrawNode::Image { .. } => {}
    }
}

fn push_point(points: &mut Vec<(f32, f32)>, point: (f64, f64)) {
    if points.len() < 128 {
        points.push((point.0 as f32, point.1 as f32));
    }
}

fn cubic_point(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let x = u * u * u * p0.0 + 3.0 * u * u * t * p1.0 + 3.0 * u * t * t * p2.0 + t * t * t * p3.0;
    let y = u * u * u * p0.1 + 3.0 * u * u * t * p1.1 + 3.0 * u * t * t * p2.1 + t * t * t * p3.1;
    (x, y)
}

fn quad_point(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    ((u * u * p0.0) + (2.0 * u * t * p1.0) + (t * t * p2.0), (u * u * p0.1) + (2.0 * u * t * p1.1) + (t * t * p2.1))
}

fn arc_points(from: (f64, f64), rx: f64, ry: f64, x_rotation_degrees: f64, large_arc: bool, sweep: bool, to: (f64, f64)) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut start = from;
    for [c1, c2, end] in arc_cubics(from, rx, ry, x_rotation_degrees, large_arc, sweep, to) {
        let end_point = (end[0], end[1]);
        for step in 1..=4 {
            let t = step as f64 / 4.0;
            points.push(cubic_point(start, (c1[0], c1[1]), (c2[0], c2[1]), end_point, t));
        }
        start = end_point;
    }
    points
}

fn arc_cubics(from: (f64, f64), mut rx: f64, mut ry: f64, x_rotation_degrees: f64, large_arc: bool, sweep: bool, to: (f64, f64)) -> Vec<[[f64; 2]; 3]> {
    let span = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt();
    if span == 0.0 {
        return Vec::new();
    }
    rx = rx.abs();
    ry = ry.abs();
    if rx == 0.0 || ry == 0.0 {
        return vec![[[from.0, from.1], [to.0, to.1], [to.0, to.1]]];
    }
    let phi = x_rotation_degrees.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let dx = (from.0 - to.0) / 2.0;
    let dy = (from.1 - to.1) / 2.0;
    let x1 = cos_phi * dx + sin_phi * dy;
    let y1 = -sin_phi * dx + cos_phi * dy;
    let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
    if lambda > 1.0 {
        let scale = lambda.sqrt();
        rx *= scale;
        ry *= scale;
    }
    let numerator = (rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1).max(0.0);
    let denominator = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let root = if denominator > 0.0 { (numerator / denominator).sqrt() } else { 0.0 };
    let sign = if large_arc == sweep { -1.0 } else { 1.0 };
    let cx1 = sign * root * rx * y1 / ry;
    let cy1 = -sign * root * ry * x1 / rx;
    let cx = cos_phi * cx1 - sin_phi * cy1 + (from.0 + to.0) / 2.0;
    let cy = sin_phi * cx1 + cos_phi * cy1 + (from.1 + to.1) / 2.0;
    let angle = |ux: f64, uy: f64, vx: f64, vy: f64| {
        let turn = (ux * vy - uy * vx).signum();
        let cosine = ((ux * vx + uy * vy) / ((ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt())).clamp(-1.0, 1.0);
        if turn < 0.0 { -cosine.acos() } else { cosine.acos() }
    };
    let theta1 = angle(1.0, 0.0, (x1 - cx1) / rx, (y1 - cy1) / ry);
    let mut delta = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry);
    if !sweep && delta > 0.0 {
        delta -= std::f64::consts::TAU;
    } else if sweep && delta < 0.0 {
        delta += std::f64::consts::TAU;
    }
    let pieces = (delta.abs() / std::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
    let step = delta / pieces as f64;
    let handle = 4.0 / 3.0 * (step / 4.0).tan();
    let on_ellipse = |theta: f64| {
        let (sin, cos) = theta.sin_cos();
        ([cos_phi * rx * cos - sin_phi * ry * sin + cx, sin_phi * rx * cos + cos_phi * ry * sin + cy], [-cos_phi * rx * sin - sin_phi * ry * cos, -sin_phi * rx * sin + cos_phi * ry * cos])
    };
    (0..pieces)
        .map(|piece| {
            let (a, b) = (theta1 + step * piece as f64, theta1 + step * (piece + 1) as f64);
            let ((p0, d0), (p1, d1)) = (on_ellipse(a), on_ellipse(b));
            let end = if piece + 1 == pieces { [to.0, to.1] } else { p1 };
            [[p0[0] + handle * d0[0], p0[1] + handle * d0[1]], [p1[0] - handle * d1[0], p1[1] - handle * d1[1]], end]
        })
        .collect()
}

fn fit_strokes(strokes: &[DisplayStroke], page_width: f64, page_height: f64) -> Vec<DisplayStroke> {
    let Some((scale, ox, oy)) = fit_from_points(stroke_points(strokes), page_width, page_height) else { return Vec::new() };
    strokes.iter().map(|stroke| DisplayStroke { closed: stroke.closed, color: stroke.color, width: stroke.width, fill: stroke.fill, stack: stroke.stack, points: stroke.points.iter().map(|(x, y)| (x * scale + ox, y * scale + oy)).collect() }).collect()
}

fn paint_display_stroke(scene: &mut Scene, transform: Affine, stroke: &DisplayStroke) {
    if let Some(fill) = stroke.fill.filter(|color| color[3] > 0.001) {
        if stroke.points.len() >= 3 {
            let mut path = BezPath::new();
            for (index, (x, y)) in stroke.points.iter().enumerate() {
                let point = Point::new(*x as f64, *y as f64);
                if index == 0 { path.move_to(point); } else { path.line_to(point); }
            }
            path.close_path();
            scene.fill(FillRule::NonZero, transform, Color::new(fill), None, &path);
        }
    }
    let mut points = stroke.points.clone();
    if stroke.closed && points.len() >= 2 {
        points.push(points[0]);
    }
    for pair in points.windows(2) {
        scene.stroke(&Stroke::new(stroke.width.max(0.1) as f64), transform, Color::new(stroke.color), None, &Line::new(Point::new(pair[0].0 as f64, pair[0].1 as f64), Point::new(pair[1].0 as f64, pair[1].1 as f64)));
    }
}

fn assemble_display_list(mut engine: Option<&mut LayoutEngine>, doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool, proxy_band: bool, camera_x: f64, camera_y: f64, zoom: f64) -> DisplayList {
    let resolved = resolve_page(doc, page);
    let mut rects = Vec::new();
    let mut text_runs = Vec::new();
    let mut images = Vec::new();
    let mut frame_strokes = Vec::new();
    let raw_drawing = raw_drawing_strokes(doc);
    let raw_labels = raw_drawing_labels(doc);
    let raw_rasters = raw_drawing_rasters(doc);
    let mut frame_labels: Vec<(u32, DrawingLabel)> = Vec::new();
    let mut guides = if chrome_blueprint && page.id == active_page_id { page_margin_guides(page) } else { Vec::new() };

    if chrome_blueprint && page.id == active_page_id {
        for guide in &page.guides {
            guides.push(DisplayGuide { rect: guide.clone(), kind: "guide".into() });
        }
        let col_count = page.columns.count.max(1) as f64;
        let col_width = (page.width - page.margins.left - page.margins.right - page.columns.gutter * (col_count - 1.0)) / col_count;
        for i in 0..page.columns.count {
            let x = page.margins.left + (i as f64) * (col_width + page.columns.gutter);
            guides.push(DisplayGuide { rect: LayoutRect { x, y: page.margins.top, width: col_width, height: page.height - page.margins.top - page.margins.bottom }, kind: "column".into() });
        }
        if engine.is_some() && !proxy_band && doc.grid.snap_to_baseline && doc.grid.baseline_grid > 0.0 {
            let mut y = doc.grid.baseline_offset;
            while y < page.height {
                guides.push(DisplayGuide { rect: LayoutRect { x: 0.0, y, width: page.width, height: 0.0 }, kind: "baseline".into() });
                y += doc.grid.baseline_grid;
            }
        }
    }

    let mut next_stack = 1u32;
    for item in resolved {
        if !item.frame.visible() || !frame_layer_visible(doc, page, &item.frame) {
            continue;
        }
        let selected = selected_ids.iter().any(|id| id == item.frame.id());
        let hovered = hovered_id.is_some_and(|id| id == item.frame.id());
        if engine.is_none() && !selected && !hovered && outside_interactive_view(item.frame.bounds(), camera_x, camera_y, zoom) {
            continue;
        }
        let stack = next_stack;
        next_stack += 1;
        let rect_from = rects.len();
        let image_from = images.len();
        let text_from = text_runs.len();
        let stroke_from = frame_strokes.len();
        match &item.frame {
            Frame::Rect { id, bounds, fill, stroke, .. } => {
                rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, *fill, stroke.or(if chrome_blueprint && item.inherited { Some([0.4, 0.5, 0.7, 0.8]) } else { None })));
            }
            Frame::Text { id, bounds, story_id, inset, .. } => {
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.2, 0.55, 0.9, 0.9])));
                }
                if !proxy_band {
                    if let Some(story) = doc.stories.iter().find(|s| s.id == *story_id) {
                        let paragraph = default_paragraph(doc);
                        let base_x = (bounds.x + inset.x) as f32;
                        let base_y = (bounds.y + inset.y) as f32;
                        let font_size = paragraph.font_size as f32;
                        let glyphs = if let Some(engine) = engine.as_deref_mut() {
                            let frame_width = (bounds.width - inset.width - inset.x * 2.0).max(1.0) as f32;
                            let frame_height = (bounds.height - inset.height - inset.y * 2.0).max(1.0) as f32;
                            if story.style_runs.is_empty() {
                                let (shaped, _overset) = layout_story_in_frame(engine, story, &paragraph, frame_width, frame_height);
                                shaped.glyphs.iter().map(|glyph| DisplayGlyph { glyph_id: glyph.glyph_id as u32, font_size, x: base_x + glyph.x, y: base_y + glyph.y, color: DisplayColor([0.0, 0.0, 0.0, 1.0]), italic: false }).collect()
                            } else {
                                story_run_glyphs(engine, doc, story, &paragraph, frame_width, frame_height, base_x, base_y)
                            }
                        } else {
                            Vec::new()
                        };
                        let (origin_x, origin_y) = glyphs.first().map(|glyph| (glyph.x, glyph.y - font_size)).unwrap_or((base_x, base_y));
                        if glyphs.is_empty() && !story.style_runs.is_empty() {
                            let mut pen = base_x;
                            for (index, span) in style_spans(story, &paragraph, doc).into_iter().enumerate() {
                                if span.start >= span.end || !story.content.is_char_boundary(span.start) || !story.content.is_char_boundary(span.end) {
                                    continue;
                                }
                                let slice = story.content[span.start..span.end].to_string();
                                if slice.is_empty() {
                                    continue;
                                }
                                let chars = slice.chars().count() as f32;
                                text_runs.push(DisplayTextRun { object_id: format!("{id}.span.{index}"), glyphs: Vec::new(), content: slice, origin_x: pen, origin_y: base_y, font_size: span.font_size, stack: 0, color: span.color.0 });
                                pen += span.font_size * chars * 0.6 + span.tracking * chars;
                            }
                        } else {
                            text_runs.push(DisplayTextRun { object_id: id.clone(), glyphs, content: story.content.clone(), origin_x, origin_y, font_size, stack: 0, color: [0.0, 0.0, 0.0, 1.0] });
                        }
                    }
                }
            }
            Frame::Image { id, bounds, link_id, .. } => {
                let link = doc.links.iter().find(|l| l.id == *link_id);
                let proxy_data_url = link.and_then(|link| link.proxy_data_url.clone()).filter(|url| !url.is_empty());
                let placeholder = link.is_none_or(|l| l.state.as_deref() == Some("missing") || l.proxy_data_url.is_none());
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.85, 0.45, 0.2, 0.9])));
                }
                let placed = if !proxy_band && proxy_data_url.is_none() {
                    link.and_then(|link| drawing_kind(&link.artifact_kind).then(|| place_drawing_in_frame(&raw_drawing, bounds))).filter(|strokes| !strokes.is_empty())
                } else {
                    None
                };
                let preview = if proxy_band || proxy_data_url.is_some() || placed.is_some() { String::new() } else { link.map(|link| preview_mark(&link.artifact_kind)).unwrap_or_default() };
                images.push(DisplayImage { object_id: id.clone(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, rotation: bounds.rotation as f32, placeholder, proxy_data_url: proxy_data_url.clone(), preview, stack: 0 });
                if let Some(placed) = placed {
                    frame_strokes.extend(placed);
                    frame_labels.extend(place_drawing_labels(&raw_labels, &raw_drawing, bounds).into_iter().map(|label| (stack, label)));
                    images.extend(place_drawing_rasters(&raw_rasters, &raw_drawing, bounds));
                } else if !proxy_band && proxy_data_url.is_none() {
                    if let Some(kind) = link.and_then(|link| (!link.artifact_kind.is_empty()).then(|| link.artifact_kind.clone())) {
                        let glyphs = placed_kind_glyphs(engine.as_deref_mut(), doc, bounds, &kind);
                        text_runs.push(DisplayTextRun { object_id: id.clone(), glyphs, content: kind, origin_x: bounds.x as f32 + 4.0, origin_y: bounds.y as f32 + 4.0, font_size: 11.0, stack: 0, color: [0.0, 0.0, 0.0, 1.0] });
                    }
                }
            }
        }
        for rect in &mut rects[rect_from..] {
            rect.stack = stack;
        }
        for image in &mut images[image_from..] {
            image.stack = stack;
        }
        for run in &mut text_runs[text_from..] {
            run.stack = stack;
        }
        for stroke in &mut frame_strokes[stroke_from..] {
            stroke.stack = stack;
        }
    }

    let strokes = drawing_strokes(doc, page.width, page.height);
    let label_fit = fit_from_points(stroke_points(&raw_drawing), page.width, page.height).or_else(|| fit_from_points(raw_labels.iter().map(|label| (label.x, label.y)), page.width, page.height));
    if let Some(engine) = engine.as_deref_mut() {
        if let Some((scale, ox, oy)) = label_fit {
            for label in &raw_labels {
                text_runs.push(shape_drawing_label(engine, doc, &label.text, label.x * scale + ox, label.y * scale + oy));
            }
        }
        for (stack, label) in &frame_labels {
            text_runs.push(shape_drawing_label(engine, doc, &label.text, label.x, label.y));
            text_runs.last_mut().unwrap().stack = *stack;
        }
    } else if let Some((scale, ox, oy)) = label_fit {
        for label in &raw_labels {
            text_runs.push(unshaped_drawing_label(&label.text, label.x * scale + ox, label.y * scale + oy));
        }
        for (stack, label) in &frame_labels {
            text_runs.push(unshaped_drawing_label(&label.text, label.x, label.y));
            text_runs.last_mut().unwrap().stack = *stack;
        }
    }
    if let Some((scale, ox, oy)) = fit_from_points(stroke_points(&raw_drawing), page.width, page.height).or_else(|| fit_from_points(raw_rasters.iter().flat_map(|raster| [(raster.x, raster.y), (raster.x + raster.width, raster.y + raster.height)]), page.width, page.height)) {
        for raster in &raw_rasters {
            push_drawing_raster(&mut images, raster, raster.x * scale + ox, raster.y * scale + oy, raster.width * scale, raster.height * scale, 0.0);
        }
    }
    DisplayList { page_id: page.id.clone(), page_width: page.width as f32, page_height: page.height as f32, strokes, frame_strokes, rects, text_runs, images, guides }
}



fn story_run_glyphs(engine: &mut LayoutEngine, doc: &LayoutSnapshot, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32, base_x: f32, base_y: f32) -> Vec<DisplayGlyph> {
    let mut pen = 0.0;
    let mut glyphs = Vec::new();
    for span in style_spans(story, paragraph, doc) {
        if span.start >= span.end {
            continue;
        }
        let mut local = paragraph.clone();
        let ratio = paragraph.leading / paragraph.font_size.max(1.0);
        local.font_size = span.font_size as f64;
        local.leading = span.font_size as f64 * ratio;
        local.font_weight = span.weight.max(1.0) as u32;
        local.tracking = span.tracking as f64;
        let slice = TextStory { id: String::new(), content: story.content[span.start..span.end].to_string(), style_runs: Vec::new() };
        let (shaped, _) = layout_story_in_frame(engine, &slice, &local, (frame_width - pen).max(1.0), frame_height);
        for glyph in &shaped.glyphs {
            glyphs.push(DisplayGlyph { glyph_id: glyph.glyph_id as u32, font_size: span.font_size, x: base_x + pen + glyph.x, y: base_y + glyph.y, color: span.color.clone(), italic: span.italic });
        }
        pen += shaped.width;
    }
    glyphs
}

struct StyleSpan {
    start: usize,
    end: usize,
    font_size: f32,
    color: DisplayColor,
    weight: f32,
    tracking: f32,
    italic: bool,
}

fn style_spans(story: &TextStory, paragraph: &ParagraphStyle, doc: &LayoutSnapshot) -> Vec<StyleSpan> {
    let len = story.content.len();
    let mut owners: Vec<Option<usize>> = vec![None; len];
    for (index, run) in story.style_runs.iter().enumerate() {
        let start = usize::try_from(run.start).unwrap_or(len).min(len);
        let end = usize::try_from(run.end).unwrap_or(len).min(len);
        if start > end || !story.content.is_char_boundary(start) || !story.content.is_char_boundary(end) {
            continue;
        }
        for slot in owners.iter_mut().take(end).skip(start) {
            *slot = Some(index);
        }
    }
    let mut spans = Vec::new();
    let mut cursor = 0;
    while cursor < len {
        if !story.content.is_char_boundary(cursor) {
            cursor += 1;
            continue;
        }
        let owner = owners[cursor];
        let mut end = cursor + 1;
        while end < len && owners[end] == owner {
            end += 1;
        }
        while end < len && !story.content.is_char_boundary(end) {
            end += 1;
        }
        let (font_size, color, weight, tracking, italic) = span_style(owner, story, paragraph, doc);
        spans.push(StyleSpan { start: cursor, end, font_size, color, weight, tracking, italic });
        cursor = end;
    }
    spans
}

fn span_style(owner: Option<usize>, story: &TextStory, paragraph: &ParagraphStyle, doc: &LayoutSnapshot) -> (f32, DisplayColor, f32, f32, bool) {
    let plain = (paragraph.font_size as f32, DisplayColor([0.0, 0.0, 0.0, 1.0]), paragraph.font_weight as f32, paragraph.tracking as f32, false);
    let Some(index) = owner else { return plain };
    let run = &story.style_runs[index];
    let mut size = paragraph.font_size;
    let mut weight = paragraph.font_weight as f64;
    let mut tracking = paragraph.tracking;
    let mut italic = false;
    let mut color = [0.0, 0.0, 0.0, 1.0];
    if let Some(id) = &run.paragraph_style_id {
        if let Some(style) = doc.paragraph_styles.iter().find(|style| style.id == *id) {
            size = style.font_size;
            weight = style.font_weight as f64;
            tracking = style.tracking;
        }
    }
    if let Some(id) = &run.character_style_id {
        if let Some(style) = doc.character_styles.iter().find(|style| style.id == *id) {
            if let Some(font_size) = style.font_size {
                size = font_size;
            }
            if let Some(font_weight) = style.font_weight {
                weight = font_weight as f64;
            }
            if let Some(value) = style.tracking {
                tracking = value;
            }
            if let Some(value) = style.italic {
                italic = value;
            }
            if let Some(rgba) = style.color {
                color = rgba;
            }
        }
    }
    (size as f32, DisplayColor(color), weight as f32, tracking as f32, italic)
}

fn placed_kind_glyphs(engine: Option<&mut LayoutEngine>, doc: &LayoutSnapshot, bounds: &LayoutBounds, kind: &str) -> Vec<DisplayGlyph> {
    let Some(engine) = engine else { return Vec::new() };
    let story = TextStory { id: String::new(), content: kind.to_string(), style_runs: Vec::new() };
    let mut paragraph = default_paragraph(doc);
    paragraph.font_size = 11.0;
    paragraph.leading = 13.2;
    paragraph.alignment = "left".into();
    let (shaped, _) = layout_story_in_frame(engine, &story, &paragraph, (bounds.width - 8.0).max(1.0) as f32, (bounds.height - 8.0).max(1.0) as f32);
    shaped.glyphs.iter().map(|glyph| DisplayGlyph { glyph_id: glyph.glyph_id as u32, font_size: 11.0, x: bounds.x as f32 + 4.0 + glyph.x, y: bounds.y as f32 + 4.0 + glyph.y, color: DisplayColor([0.1, 0.1, 0.1, 1.0]), italic: false }).collect()
}

fn paint_preview_mark(scene: &mut Scene, transform: Affine, image: &DisplayImage) {
    let inset = 4.0_f64.min(image.width as f64 * 0.2).min(image.height as f64 * 0.2);
    if image.preview.is_empty() || inset < 0.5 {
        return;
    }
    let left = image.x as f64 + inset;
    let top = image.y as f64 + inset;
    let right = image.x as f64 + image.width as f64 - inset;
    let bottom = image.y as f64 + image.height as f64 - inset;
    let color = Color::new([0.2, 0.25, 0.3, 1.0]);
    let stroke = Stroke::new(1.5);
    match image.preview.as_str() {
        "page" => {
            scene.stroke(&stroke, transform, color, None, &Rect::new(left, top, right, bottom));
        }
        "stroke" => {
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, top), Point::new(right, bottom)));
        }
        "map" => {
            let mid_x = (left + right) * 0.5;
            let mid_y = (top + bottom) * 0.5;
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(mid_x, top), Point::new(mid_x, bottom)));
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, mid_y), Point::new(right, mid_y)));
        }
        "curve" => {
            let mid_x = (left + right) * 0.5;
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, bottom), Point::new(mid_x, top)));
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(mid_x, top), Point::new(right, bottom)));
        }
        "grid" => {
            let step_x = (right - left) / 3.0;
            let step_y = (bottom - top) / 3.0;
            for step in 1..3 {
                let y = top + step_y * step as f64;
                let x = left + step_x * step as f64;
                scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, y), Point::new(right, y)));
                scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(x, top), Point::new(x, bottom)));
            }
        }
        _ => {}
    }
}

fn preview_mark(kind: &str) -> String {
    if kind.is_empty() {
        return String::new();
    }
    let mark = if kind.contains("pdf") || kind.contains("note") || kind.contains("writer") || kind.contains("forms") || kind.contains("sequence") || kind.contains("presentation") {
        "page"
    } else if kind.contains("draw") || kind.contains("dwg") || kind.contains("dxf") || kind.contains("cad") {
        "stroke"
    } else if kind.contains("map") || kind.contains("gis") || kind.contains("terrain") {
        "map"
    } else if kind.contains("svg") || kind.contains("fem") || kind.contains("equation") || kind.contains("flow") || kind.contains("wires") {
        "curve"
    } else {
        "grid"
    };
    mark.into()
}

fn color_from(c: &DisplayColor) -> Color {
    Color::new(c.0)
}

/// 👻️ Catalogue drop ghost rect shown while dragging onto the canvas.
#[derive(Clone, Debug)]
pub struct LayoutDropPreview {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

const DROP_PREVIEW_WIDTH: f64 = 200.0;
const DROP_PREVIEW_HEIGHT: f64 = 120.0;

fn append_drop_preview(scene: &mut Scene, transform: Affine, preview: &LayoutDropPreview) {
    if preview.kind == "page" {
        return;
    }
    let shape = Rect::new(preview.x, preview.y, preview.x + DROP_PREVIEW_WIDTH, preview.y + DROP_PREVIEW_HEIGHT);
    let fill = match preview.kind.as_str() {
        "rect" => Color::new([0.85, 0.88, 0.92, 0.45]),
        "text" => Color::new([0.2, 0.55, 0.9, 0.25]),
        "image" => Color::new([0.85, 0.45, 0.2, 0.25]),
        _ => Color::new([0.5, 0.5, 0.5, 0.3]),
    };
    scene.fill(FillRule::NonZero, transform, fill, None, &shape);
    scene.stroke(&Stroke::new(2.0), transform, Color::new([0.1, 0.45, 0.95, 0.85]), None, &shape);
}

pub fn display_list_to_scene(list: &DisplayList, chrome_blueprint: bool, camera: &Camera, viewport: &Viewport, drop_preview: Option<&LayoutDropPreview>) -> Scene {
    let mut scene = Scene::new();
    let transform = camera::camera_content_affine(camera, viewport);
    let page_bg = if chrome_blueprint { Color::new([0.97, 0.97, 0.98, 1.0]) } else { Color::new([1.0, 1.0, 1.0, 1.0]) };
    scene.fill(FillRule::NonZero, transform, page_bg, None, &Rect::new(0.0, 0.0, list.page_width as f64, list.page_height as f64));

    if chrome_blueprint {
        for guide in &list.guides {
            let stroke = match guide.kind.as_str() {
                "margin" => Color::new([0.75, 0.2, 0.2, 0.35]),
                "column" => Color::new([0.2, 0.45, 0.85, 0.25]),
                "baseline" => Color::new([0.5, 0.5, 0.5, 0.2]),
                _ => Color::new([0.3, 0.3, 0.3, 0.3]),
            };
            if guide.rect.height <= 0.0 {
                scene.stroke(&Stroke::new(1.0), transform, stroke, None, &Line::new(Point::new(guide.rect.x, guide.rect.y), Point::new(guide.rect.x + guide.rect.width, guide.rect.y)));
            } else {
                scene.stroke(&Stroke::new(1.0), transform, stroke, None, &Rect::new(guide.rect.x, guide.rect.y, guide.rect.x + guide.rect.width, guide.rect.y + guide.rect.height));
            }
        }
    }

    for piece in stack_pieces(list) {
        match piece {
            StackPiece::Stroke(stroke) | StackPiece::FrameStroke(stroke) => paint_display_stroke(&mut scene, transform, stroke),
            StackPiece::Rect(rect) => {
                let shape = RoundedRect::new(Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.width) as f64, (rect.y + rect.height) as f64), RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0));
                if let Some(fill) = &rect.fill {
                    scene.fill(FillRule::NonZero, transform, color_from(fill), None, &shape);
                }
                if let Some(stroke) = &rect.stroke {
                    let width = if rect.selected { 2.5 } else if rect.hovered { 1.75 } else { 1.0 };
                    scene.stroke(&Stroke::new(width), transform, color_from(stroke), None, &shape);
                } else if rect.selected && chrome_blueprint {
                    scene.stroke(&Stroke::new(2.0), transform, Color::new([0.1, 0.45, 0.95, 1.0]), None, &shape);
                } else if rect.hovered && chrome_blueprint {
                    scene.stroke(&Stroke::new(1.5), transform, Color::new([0.95, 0.72, 0.15, 1.0]), None, &shape);
                }
            }
            StackPiece::Image(image) => {
                let color = if image.placeholder { Color::new([0.92, 0.88, 0.84, 1.0]) } else { Color::new([0.85, 0.85, 0.85, 1.0]) };
                let shape = Rect::new(image.x as f64, image.y as f64, (image.x + image.width) as f64, (image.y + image.height) as f64);
                scene.fill(FillRule::NonZero, transform, color, None, &shape);
                if image.placeholder {
                    scene.stroke(&Stroke::new(1.0), transform, Color::new([0.75, 0.35, 0.2, 1.0]), None, &shape);
                }
                paint_preview_mark(&mut scene, transform, image);
            }
            StackPiece::Text(run) => {
                for glyph in &run.glyphs {
                    let skew = if glyph.italic { Affine::new([1.0, 0.0, 0.25, 1.0, 0.0, 0.0]) } else { Affine::IDENTITY };
                    scene.fill(FillRule::NonZero, transform * Affine::IDENTITY.translate(Vec2::new(glyph.x as f64, glyph.y as f64)) * skew * Affine::IDENTITY.scale((glyph.font_size / 16.0) as f64), color_from(&glyph.color), None, &Rect::new(0.0, -glyph.font_size as f64, 0.45, 0.0));
                }
            }
        }
    }

    if let Some(preview) = drop_preview {
        append_drop_preview(&mut scene, transform, preview);
    }

    scene
}

/// 🔭️ Bundled render/hit-test context for a single page query — page, camera/viewport, and
/// selection state. Groups {@link build_scene_from_document_json} and
/// {@link hit_test_document_json}'s shared arguments under `clippy::too_many_arguments`.
pub struct SceneQuery<'a> {
    pub page_id: &'a str,
    pub selected_ids: &'a [String],
    pub hovered_id: Option<&'a str>,
    pub chrome_blueprint: bool,
    pub camera: &'a Camera,
    pub viewport: &'a Viewport,
}

pub fn build_scene_from_document_json(engine: &mut LayoutEngine, json: &str, query: &SceneQuery<'_>, drop_preview: Option<&LayoutDropPreview>) -> Result<Scene, LayoutError> {
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(engine, &doc, page, query.page_id, query.selected_ids, query.hovered_id, query.chrome_blueprint);
    Ok(display_list_to_scene(&list, query.chrome_blueprint, query.camera, query.viewport, drop_preview))
}

pub fn hit_test_document_json(engine: &mut LayoutEngine, json: &str, sx: f64, sy: f64, query: &SceneQuery<'_>) -> Result<Option<String>, LayoutError> {
    let _ = engine;
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let world = camera::screen_to_world(query.camera, query.viewport, Point::new(sx, sy));
    Ok(hit_test_page_frames(&doc, page, world.x as f32, world.y as f32))
}

pub fn screen_to_world_json(camera: &Camera, viewport: &Viewport, sx: f64, sy: f64) -> String {
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    serde_json::json!({ "x": world.x, "y": world.y }).to_string()
}
//#endregion ⚙️Scene

//#region 📤️Export
#[cfg(test)]
pub use crate::editor::layout::engine::export::{export_document_pdf_headless_batch, export_document_png_headless_batch, export_document_svg_headless_batch, export_package_zip_headless_batch};
//#endregion 📤️Export

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
