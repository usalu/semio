//! 🖼️ Layout app engine — display-list construction, glyph layout, scene painting, hit-testing and
//! export (SVG/PDF/PNG/zip). Sibling topic file of `🦀️component.rs` (headless compute is split across the
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

use std::borrow::Cow;
use std::io::{Cursor, Write};
use std::sync::Arc;

use fontique::Blob;
use image::{ImageBuffer, Rgba};
use infinite_canvas::camera::{self, Camera, Viewport};
use infinite_canvas::{Affine, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};
use parley::{Alignment, AlignmentOptions, FontContext, FontStack, FontWeight, Layout, LayoutContext, LineHeight, PositionedLayoutItem, StyleProperty};
use serde_json::Value;
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawCanvas, DrawLayer, DrawNode, DrawStyle, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};

use crate::artifacts::layout::io::{compose_svg_from_drawing, rect_path_segments, LayoutError};
use crate::artifacts::layout::schema::{parse_layout_document, resolve_page};
use crate::artifacts::layout::{Frame, LayoutBounds, LayoutSnapshot, LayoutRect, Page, ParagraphStyle, TextStory};

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
}

#[derive(Clone, Debug)]
pub struct DisplayRect {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<DisplayColor>,
    pub stroke: Option<DisplayColor>,
    pub inherited: bool,
    pub selected: bool,
    pub hovered: bool,
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
}

#[derive(Clone, Debug)]
pub struct DisplayImage {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub placeholder: bool,
}

#[derive(Clone, Debug)]
pub struct DisplayList {
    pub page_id: String,
    pub page_width: f32,
    pub page_height: f32,
    pub rects: Vec<DisplayRect>,
    pub text_runs: Vec<DisplayTextRun>,
    pub images: Vec<DisplayImage>,
    pub guides: Vec<DisplayGuide>,
}

impl DisplayList {
    pub fn hit_test(&self, x: f32, y: f32) -> Option<String> {
        for rect in self.rects.iter().rev() {
            if x >= rect.x && x <= rect.x + rect.width && y >= rect.y && y <= rect.y + rect.height {
                return Some(rect.object_id.clone());
            }
        }
        for image in self.images.iter().rev() {
            if x >= image.x && x <= image.x + image.width && y >= image.y && y <= image.y + image.height {
                return Some(image.object_id.clone());
            }
        }
        None
    }
}

pub fn page_margin_guides(page: &Page) -> Vec<DisplayGuide> {
    vec![DisplayGuide { rect: LayoutRect { x: page.margins.left, y: page.margins.top, width: page.width - page.margins.left - page.margins.right, height: page.height - page.margins.top - page.margins.bottom }, kind: "margin".into() }]
}

pub fn bounds_to_display_rect(object_id: &str, bounds: &LayoutBounds, inherited: bool, selected: bool, hovered: bool, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> DisplayRect {
    DisplayRect { object_id: object_id.into(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, fill: fill.map(DisplayColor), stroke: stroke.map(DisplayColor), inherited, selected, hovered }
}
//#endregion 🖼️Display

//#region ⚙️Scene
static LAYOUT_SANS: &[u8] = include_bytes!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🖼️assets/🔤️MapLabelSans.ttf");

pub struct LayoutEngine {
    pub font_context: FontContext,
    pub layout_context: LayoutContext<[u8; 4]>,
    fonts_ready: bool,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self { font_context: FontContext::new(), layout_context: LayoutContext::new(), fonts_ready: false }
    }

    fn ensure_fonts(&mut self) {
        if self.fonts_ready {
            return;
        }
        self.font_context.collection.register_fonts(Blob::new(Arc::new(LAYOUT_SANS.to_vec())), None);
        self.fonts_ready = true;
    }

    pub fn layout_story(&mut self, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (Layout<[u8; 4]>, bool) {
        self.ensure_fonts();
        let mut builder = self.layout_context.ranged_builder(&mut self.font_context, &story.content, 1.0, true);
        builder.push_default(StyleProperty::FontSize(paragraph.font_size as f32));
        builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed("Layout Sans"))));
        builder.push_default(StyleProperty::FontWeight(FontWeight::new(paragraph.font_weight as f32)));
        builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative((paragraph.leading / paragraph.font_size.max(1.0)) as f32)));
        builder.push_default(StyleProperty::LetterSpacing(paragraph.tracking as f32));
        let mut layout = builder.build(&story.content);
        layout.break_all_lines(Some(frame_width));
        layout.align(Some(frame_width), alignment_from_str(&paragraph.alignment), AlignmentOptions::default());
        let overset = layout.height() > frame_height;
        (layout, overset)
    }
}

fn alignment_from_str(value: &str) -> Alignment {
    match value {
        "center" | "middle" => Alignment::Middle,
        "right" => Alignment::Right,
        "justify" | "justified" => Alignment::Justified,
        _ => Alignment::Left,
    }
}

fn default_paragraph(doc: &LayoutSnapshot) -> ParagraphStyle {
    doc.paragraph_styles.first().cloned().unwrap_or(ParagraphStyle { id: "paragraph.body".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() })
}

pub fn layout_story_in_frame(engine: &mut LayoutEngine, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (Layout<[u8; 4]>, bool) {
    engine.layout_story(story, paragraph, frame_width, frame_height)
}

pub fn build_display_list_for_page(engine: &mut LayoutEngine, doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool) -> DisplayList {
    let resolved = resolve_page(doc, page);
    let mut rects = Vec::new();
    let mut text_runs = Vec::new();
    let mut images = Vec::new();
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
        if doc.grid.snap_to_baseline && doc.grid.baseline_grid > 0.0 {
            let mut y = doc.grid.baseline_offset;
            while y < page.height {
                guides.push(DisplayGuide { rect: LayoutRect { x: 0.0, y, width: page.width, height: 0.0 }, kind: "baseline".into() });
                y += doc.grid.baseline_grid;
            }
        }
    }

    for item in resolved {
        if !item.frame.visible() {
            continue;
        }
        let selected = selected_ids.iter().any(|id| id == item.frame.id());
        let hovered = hovered_id.is_some_and(|id| id == item.frame.id());
        match &item.frame {
            Frame::Rect { id, bounds, fill, stroke, .. } => {
                rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, *fill, stroke.or(if chrome_blueprint && item.inherited { Some([0.4, 0.5, 0.7, 0.8]) } else { None })));
            }
            Frame::Text { id, bounds, story_id, inset, .. } => {
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.2, 0.55, 0.9, 0.9])));
                }
                if let Some(story) = doc.stories.iter().find(|s| s.id == *story_id) {
                    let paragraph = default_paragraph(doc);
                    let frame_width = (bounds.width - inset.width - inset.x * 2.0).max(1.0) as f32;
                    let frame_height = (bounds.height - inset.height - inset.y * 2.0).max(1.0) as f32;
                    let (layout, _overset) = layout_story_in_frame(engine, story, &paragraph, frame_width, frame_height);
                    let mut glyphs = Vec::new();
                    let base_x = (bounds.x + inset.x) as f32;
                    let base_y = (bounds.y + inset.y) as f32;
                    for line in layout.lines() {
                        for positioned in line.items() {
                            if let PositionedLayoutItem::GlyphRun(run) = positioned {
                                let font_size = paragraph.font_size as f32;
                                for glyph in run.positioned_glyphs() {
                                    glyphs.push(DisplayGlyph { glyph_id: glyph.id as u32, font_size, x: base_x + glyph.x, y: base_y + glyph.y, color: DisplayColor([0.0, 0.0, 0.0, 1.0]) });
                                }
                            }
                        }
                    }
                    text_runs.push(DisplayTextRun { object_id: id.clone(), glyphs });
                }
            }
            Frame::Image { id, bounds, link_id, .. } => {
                let link = doc.links.iter().find(|l| l.id == *link_id);
                let placeholder = link.is_none_or(|l| l.state.as_deref() == Some("missing") || l.proxy_data_url.is_none());
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.85, 0.45, 0.2, 0.9])));
                }
                images.push(DisplayImage { object_id: id.clone(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, placeholder });
            }
        }
    }

    DisplayList { page_id: page.id.clone(), page_width: page.width as f32, page_height: page.height as f32, rects, text_runs, images, guides }
}

fn color_from(c: &DisplayColor) -> Color {
    Color::new(c.0)
}

/// @emoji 👻️ Catalogue drop ghost rect shown while dragging onto the canvas.
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

    for rect in &list.rects {
        let shape = RoundedRect::new(Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.width) as f64, (rect.y + rect.height) as f64), RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0));
        if let Some(fill) = &rect.fill {
            scene.fill(FillRule::NonZero, transform, color_from(fill), None, &shape);
        }
        if let Some(stroke) = &rect.stroke {
            let width = if rect.selected {
                2.5
            } else if rect.hovered {
                1.75
            } else {
                1.0
            };
            scene.stroke(&Stroke::new(width), transform, color_from(stroke), None, &shape);
        } else if rect.selected && chrome_blueprint {
            scene.stroke(&Stroke::new(2.0), transform, Color::new([0.1, 0.45, 0.95, 1.0]), None, &shape);
        } else if rect.hovered && chrome_blueprint {
            scene.stroke(&Stroke::new(1.5), transform, Color::new([0.95, 0.72, 0.15, 1.0]), None, &shape);
        }
    }

    for image in &list.images {
        let color = if image.placeholder { Color::new([0.92, 0.88, 0.84, 1.0]) } else { Color::new([0.85, 0.85, 0.85, 1.0]) };
        let shape = Rect::new(image.x as f64, image.y as f64, (image.x + image.width) as f64, (image.y + image.height) as f64);
        scene.fill(FillRule::NonZero, transform, color, None, &shape);
        if image.placeholder {
            scene.stroke(&Stroke::new(1.0), transform, Color::new([0.75, 0.35, 0.2, 1.0]), None, &shape);
        }
    }

    for run in &list.text_runs {
        for glyph in &run.glyphs {
            scene.fill(
                FillRule::NonZero,
                transform * Affine::IDENTITY.translate(Vec2::new(glyph.x as f64, glyph.y as f64)) * Affine::IDENTITY.scale((glyph.font_size / 16.0) as f64),
                color_from(&glyph.color),
                None,
                &Rect::new(0.0, -glyph.font_size as f64, 0.45, 0.0),
            );
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
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(engine, &doc, page, query.page_id, query.selected_ids, query.hovered_id, true);
    let world = camera::screen_to_world(query.camera, query.viewport, Point::new(sx, sy));
    Ok(list.hit_test(world.x as f32, world.y as f32))
}

pub fn screen_to_world_json(camera: &Camera, viewport: &Viewport, sx: f64, sy: f64) -> String {
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    serde_json::json!({ "x": world.x, "y": world.y }).to_string()
}
//#endregion ⚙️Scene

//#region 📤️Export
/// 🌉️ Maps a rendered `DisplayList` onto a real `SemioDrawingSnapshot`: one white background rect,
/// one rect-shaped `DrawNode::Path` per filled/stroked `DisplayRect` (real fill/stroke color, same
/// two-pass fill-then-stroke behavior the previous hand-rolled SVG had), one colored rect per
/// `DisplayImage` (placeholder vs. resolved tint), and one small filled rect per `DisplayGlyph` —
/// same "glyph as a small box" fidelity the previous string builder had (this engine never emits
/// real font outlines to SVG on either path).
fn display_list_to_semio_drawing(list: &DisplayList) -> SemioDrawingSnapshot {
    let mut styles = vec![DrawStyle { name: "background".into(), fill: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None }];
    let mut children = vec![DrawNode::Path { segments: rect_path_segments(0.0, 0.0, list.page_width as f64, list.page_height as f64), style: Some("background".into()) }];

    for (index, rect) in list.rects.iter().enumerate() {
        let segments = rect_path_segments(rect.x as f64, rect.y as f64, rect.width as f64, rect.height as f64);
        if let Some(fill) = &rect.fill {
            let name = format!("rect-fill-{index}");
            styles.push(DrawStyle { name: name.clone(), fill: Some(color_to_semio_rgba(fill)), stroke: None, stroke_width: None, opacity: None });
            children.push(DrawNode::Path { segments: segments.clone(), style: Some(name) });
        }
        if let Some(stroke) = &rect.stroke {
            let name = format!("rect-stroke-{index}");
            styles.push(DrawStyle { name: name.clone(), fill: None, stroke: Some(color_to_semio_rgba(stroke)), stroke_width: Some(1.0), opacity: None });
            children.push(DrawNode::Path { segments, style: Some(name) });
        }
    }

    for (index, image) in list.images.iter().enumerate() {
        let color = if image.placeholder { SemioRgba { r: 0.92, g: 0.88, b: 0.84, a: 1.0 } } else { SemioRgba { r: 0.86, g: 0.86, b: 0.86, a: 1.0 } };
        let name = format!("image-{index}");
        styles.push(DrawStyle { name: name.clone(), fill: Some(color), stroke: None, stroke_width: None, opacity: None });
        children.push(DrawNode::Path { segments: rect_path_segments(image.x as f64, image.y as f64, image.width as f64, image.height as f64), style: Some(name) });
    }

    for run in &list.text_runs {
        for (index, glyph) in run.glyphs.iter().enumerate() {
            let name = format!("glyph-{}-{index}", run.object_id);
            styles.push(DrawStyle { name: name.clone(), fill: Some(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None });
            children.push(DrawNode::Path {
                segments: rect_path_segments(glyph.x as f64, glyph.y as f64, (glyph.font_size * 0.45) as f64, glyph.font_size as f64),
                style: Some(name),
            });
        }
    }

    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: list.page_width as f64, height: list.page_height as f64, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles,
        layers: vec![DrawLayer { id: list.page_id.clone(), name: list.page_id.clone(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

fn color_to_semio_rgba(color: &DisplayColor) -> SemioRgba {
    SemioRgba { r: color.0[0], g: color.0[1], b: color.0[2], a: color.0[3] }
}

/// 🌉️ Composes through stdio's real `drawing↔svg` bridge (`io_dispatch`, via
/// `crate::artifacts::layout::io::compose_svg_from_drawing`) — no hand-rolled SVG string here anymore.
pub fn export_display_list_svg(list: &DisplayList) -> Result<String, LayoutError> {
    let drawing = display_list_to_semio_drawing(list);
    compose_svg_from_drawing(&drawing).map_err(LayoutError::Svg)
}

pub fn export_document_svg(doc: &LayoutSnapshot, page_id: &str) -> Result<String, LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| LayoutError::PageNotFound(page_id.to_string()))?;
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, doc, page, page_id, &[], None, false);
    export_display_list_svg(&list)
}

pub fn export_document_pdf(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| LayoutError::PageNotFound(page_id.to_string()))?;
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, doc, page, page_id, &[], None, false);
    let mut body = String::new();
    body.push_str("BT\n/F1 12 Tf\n");
    body.push_str(&format!("{} {} {} {} re\nf\n", 0, 0, page.width, page.height));
    for rect in &list.rects {
        if rect.fill.is_some() {
            body.push_str(&format!("{} {} {} {} re\nf\n", rect.x, rect.y, rect.width, rect.height));
        }
    }
    body.push_str("ET\n");
    let objects = vec![
        "1 0 obj<< /Type /Catalog /Pages 2 0 R >>endobj\n".to_string(),
        "2 0 obj<< /Type /Pages /Kids [3 0 R] /Count 1 >>endobj\n".to_string(),
        format!("3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R /Resources<< /Font<< /F1 5 0 R >> >> >>endobj\n", page.width, page.height),
        format!("4 0 obj<< /Length {} >>stream\n{}endstream\nendobj\n", body.len(), body),
        "5 0 obj<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>endobj\n".to_string(),
    ];
    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = vec![0usize];
    for object in &objects {
        offsets.push(pdf.len());
        pdf.push_str(object);
    }
    let xref_start = pdf.len();
    pdf.push_str(&format!("xref\n0 {}\n", objects.len() + 1));
    pdf.push_str("0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        pdf.push_str(&format!("{:010} 00000 n \n", offset));
    }
    pdf.push_str(&format!("trailer<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", objects.len() + 1, xref_start));
    Ok(pdf.into_bytes())
}

pub fn export_document_png_cpu(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| LayoutError::PageNotFound(page_id.to_string()))?;
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, doc, page, page_id, &[], None, false);
    let width = list.page_width.max(1.0) as u32;
    let height = list.page_height.max(1.0) as u32;
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));
    for rect in &list.rects {
        if let Some(fill) = &rect.fill {
            let color = Rgba([(fill.0[0] * 255.0) as u8, (fill.0[1] * 255.0) as u8, (fill.0[2] * 255.0) as u8, (fill.0[3] * 255.0) as u8]);
            let x0 = rect.x.max(0.0) as u32;
            let y0 = rect.y.max(0.0) as u32;
            let x1 = (rect.x + rect.width).min(width as f32) as u32;
            let y1 = (rect.y + rect.height).min(height as f32) as u32;
            for y in y0..y1 {
                for x in x0..x1 {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(img.as_raw())?;
    }
    Ok(bytes)
}

pub fn export_package_zip(doc_json: &str, preflight_json: &str) -> Result<Vec<u8>, LayoutError> {
    let doc: LayoutSnapshot = serde_json::from_str(doc_json)?;
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file("document.json", options)?;
    zip.write_all(doc_json.as_bytes())?;
    zip.start_file("preflight-report.json", options)?;
    zip.write_all(preflight_json.as_bytes())?;
    let manifest_links: Vec<Value> = doc
        .links
        .iter()
        .map(|link| {
            let hash = if link.hash.is_empty() { format!("sha256:{:x}", Sha256::digest(link.path.as_bytes())) } else { link.hash.clone() };
            serde_json::json!({
                "id": link.id,
                "path": link.path,
                "hash": hash,
                "state": link.state,
                "missing": link.state.as_deref() == Some("missing"),
            })
        })
        .collect();
    let manifest = serde_json::json!({
        "schema": "layout.package-manifest/v1",
        "document": "document.json",
        "preflight": "preflight-report.json",
        "links": manifest_links,
        "generatedAt": "2026-07-02T00:00:00Z",
    });
    zip.start_file("package-manifest.json", options)?;
    zip.write_all(manifest.to_string().as_bytes())?;
    let data = zip.finish()?;
    Ok(data.into_inner())
}

pub fn scene_png_from_display_list(list: &DisplayList) -> Result<Vec<u8>, LayoutError> {
    let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
    let viewport = Viewport { width: list.page_width.max(1.0) as u32, height: list.page_height.max(1.0) as u32, dpr: 1.0 };
    let _scene = display_list_to_scene(list, false, &camera, &viewport, None);
    let width = list.page_width.max(1.0) as u32;
    let height = list.page_height.max(1.0) as u32;
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(img.as_raw())?;
    }
    Ok(bytes)
}
//#endregion 📤️Export

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document() -> LayoutSnapshot {
        crate::artifacts::layout::dsl::parse_dsl(crate::artifacts::layout::dsl::LAYOUT_SAMPLE_TEXT).expect("sample fixture parses")
    }

    #[test]
    fn builds_scene_from_empty_document() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
        let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
        let mut engine = LayoutEngine::new();
        let scene = build_scene_from_document_json(&mut engine, json, &query, None).expect("scene");
        let _ = scene;
    }

    #[test]
    fn hit_test_respects_camera_zoom() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 0.5 };
        let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
        let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
        let mut engine = LayoutEngine::new();
        let hit = hit_test_document_json(&mut engine, json, 210.0, 160.0, &query).expect("hit");
        assert_eq!(hit.as_deref(), Some("frame-1"));
    }

    #[test]
    fn marks_hovered_frame_rect() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let mut engine = LayoutEngine::new();
        let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], Some("frame-1"), true);
        assert!(list.rects.iter().any(|rect| rect.object_id == "frame-1" && rect.hovered));
        assert!(list.rects.iter().all(|rect| rect.object_id != "frame-1" || rect.hovered));
    }

    #[test]
    fn scene_and_hit_test_error_when_page_missing() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":100,"height":100,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 100, height: 100, dpr: 1.0 };
        let query = SceneQuery { page_id: "missing-page", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
        let mut engine = LayoutEngine::new();
        assert!(matches!(build_scene_from_document_json(&mut engine, json, &query, None), Err(LayoutError::PageNotFound(id)) if id == "missing-page"));
        let hit = hit_test_document_json(&mut engine, json, 0.0, 0.0, &query);
        assert!(matches!(hit, Err(LayoutError::PageNotFound(id)) if id == "missing-page"));
    }

    #[test]
    fn hit_test_returns_none_for_empty_space() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 400, height: 400, dpr: 1.0 };
        let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: false, camera: &camera, viewport: &viewport };
        let mut engine = LayoutEngine::new();
        let hit = hit_test_document_json(&mut engine, json, 300.0, 300.0, &query).expect("hit test");
        assert!(hit.is_none());
    }

    #[test]
    fn display_list_hit_test_matches_image_bounds_and_misses_elsewhere() {
        let list = DisplayList {
            page_id: "page-1".into(),
            page_width: 100.0,
            page_height: 100.0,
            rects: Vec::new(),
            text_runs: Vec::new(),
            images: vec![DisplayImage { object_id: "img-1".into(), x: 10.0, y: 10.0, width: 20.0, height: 20.0, placeholder: false }],
            guides: Vec::new(),
        };
        assert_eq!(list.hit_test(15.0, 15.0).as_deref(), Some("img-1"));
        assert!(list.hit_test(90.0, 90.0).is_none());
    }

    #[test]
    fn guides_omitted_for_non_active_page_even_with_chrome_blueprint() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":10,"right":10,"bottom":10,"left":10},"columns":{"count":2,"gutter":4},"guides":[{"x":5,"y":5,"w":1,"h":1}],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let mut engine = LayoutEngine::new();
        let list = build_display_list_for_page(&mut engine, &doc, page, "different-active-page", &[], None, true);
        assert!(list.guides.is_empty(), "guides must only render for the active blueprint page");
    }

    #[test]
    fn baseline_guides_only_emitted_when_grid_snaps() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let mut engine = LayoutEngine::new();
        let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], None, true);
        assert!(list.guides.iter().all(|guide| guide.kind != "baseline"));
    }

    #[test]
    fn image_placeholder_reflects_link_lookup_and_state() {
        let json = r#"{"schema":"layout.layout","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[{"id":"link-missing","path":"a.png","hash":"h","width":1,"height":1,"dpi":72,"state":"missing"},{"id":"link-ready","path":"b.png","hash":"h","width":1,"height":1,"dpi":72,"state":"ready","proxyDataUrl":"data:image/png;base64,AA=="}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["img-missing","img-ready","img-unlinked"]}],"frames":[{"id":"img-missing","layerId":"layer-1","kind":"image","bounds":{"x":0,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-missing"},{"id":"img-ready","layerId":"layer-1","kind":"image","bounds":{"x":20,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-ready"},{"id":"img-unlinked","layerId":"layer-1","kind":"image","bounds":{"x":40,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-gone"}],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let mut engine = LayoutEngine::new();
        let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], None, false);
        let by_id = |id: &str| list.images.iter().find(|i| i.object_id == id).expect("image present");
        assert!(by_id("img-missing").placeholder, "missing-state link stays a placeholder");
        assert!(!by_id("img-ready").placeholder, "ready link with a proxy is not a placeholder");
        assert!(by_id("img-unlinked").placeholder, "unresolved link falls back to placeholder");
    }

    #[test]
    fn layout_story_in_frame_resolves_alignment_variants_and_detects_overset() {
        let mut engine = LayoutEngine::new();
        let story = TextStory { id: "story-1".into(), content: "Hello layout engine, this line should wrap across several lines of text.".into(), style_runs: Vec::new() };
        for alignment in ["left", "center", "middle", "right", "justify", "justified", "unrecognized"] {
            let paragraph = ParagraphStyle { id: "p".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: alignment.into() };
            let (layout, overset) = layout_story_in_frame(&mut engine, &story, &paragraph, 80.0, 10.0);
            assert!(layout.height() > 0.0, "alignment {alignment} should still measure a positive height");
            assert!(overset, "narrow/short frame with long content should overset for alignment {alignment}");
        }
        let paragraph = ParagraphStyle { id: "p".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() };
        let (_, not_overset) = layout_story_in_frame(&mut engine, &story, &paragraph, 2000.0, 2000.0);
        assert!(!not_overset);
    }

    #[test]
    fn display_list_to_scene_handles_drop_preview_variants_and_rect_styles() {
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 200, height: 200, dpr: 1.0 };
        let list = DisplayList {
            page_id: "page-1".into(),
            page_width: 200.0,
            page_height: 200.0,
            rects: vec![
                DisplayRect {
                    object_id: "r-explicit-stroke".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 10.0,
                    height: 10.0,
                    fill: Some(DisplayColor([1.0, 1.0, 1.0, 1.0])),
                    stroke: Some(DisplayColor([0.0, 0.0, 0.0, 1.0])),
                    inherited: false,
                    selected: true,
                    hovered: false,
                },
                DisplayRect { object_id: "r-implicit-hover".into(), x: 20.0, y: 0.0, width: 10.0, height: 10.0, fill: None, stroke: None, inherited: false, selected: false, hovered: true },
                DisplayRect { object_id: "r-implicit-select".into(), x: 40.0, y: 0.0, width: 10.0, height: 10.0, fill: None, stroke: None, inherited: false, selected: true, hovered: false },
            ],
            text_runs: vec![DisplayTextRun { object_id: "text-1".into(), glyphs: vec![DisplayGlyph { glyph_id: 1, font_size: 12.0, x: 0.0, y: 0.0, color: DisplayColor([0.0, 0.0, 0.0, 1.0]) }] }],
            images: vec![DisplayImage { object_id: "img-1".into(), x: 0.0, y: 60.0, width: 10.0, height: 10.0, placeholder: true }],
            guides: vec![DisplayGuide { rect: LayoutRect { x: 0.0, y: 0.0, width: 10.0, height: 0.0 }, kind: "unrecognized".into() }],
        };
        for kind in ["page", "rect", "text", "image", "unrecognized"] {
            let preview = LayoutDropPreview { kind: kind.into(), x: 5.0, y: 5.0 };
            let scene = display_list_to_scene(&list, true, &camera, &viewport, Some(&preview));
            let _ = scene;
        }
        let scene = display_list_to_scene(&list, false, &camera, &viewport, None);
        let _ = scene;
    }

    #[test]
    fn screen_to_world_json_returns_a_point_object() {
        let camera = Camera { x: 100.0, y: 50.0, zoom: 2.0 };
        let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
        let json = screen_to_world_json(&camera, &viewport, 210.0, 160.0);
        let parsed: Value = serde_json::from_str(&json).expect("valid json point");
        assert!(parsed["x"].is_number());
        assert!(parsed["y"].is_number());
    }

    #[test]
    fn png_cpu_export_writes_valid_rgba_png() {
        let doc = sample_document();
        let bytes = export_document_png_cpu(&doc, "page-1").expect("png export succeeds");
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
    }

    #[test]
    fn pdf_export_writes_pdf_header() {
        let doc = sample_document();
        let bytes = export_document_pdf(&doc, "page-1").expect("pdf export succeeds");
        assert!(bytes.starts_with(b"%PDF-1.4"));
    }

    #[test]
    fn package_zip_bundles_document_and_preflight() {
        let doc = sample_document();
        let json = serde_json::to_string(&doc).expect("serialize sample document to json");
        let bytes = export_package_zip(&json, "[]").expect("package export succeeds");
        assert_eq!(doc.schema, crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA);
        assert!(bytes.starts_with(b"PK"));
    }

    #[test]
    fn svg_export_contains_path_and_wraps_a_valid_document() {
        crate::artifacts::layout::io::ensure_stdio_semio_drawing_registered();
        let doc = sample_document();
        let svg = export_document_svg(&doc, "page-1").expect("svg export succeeds");
        assert!(svg.starts_with("<svg"));
        // 🌉️ Composed through stdio's real semio/drawing→svg bridge now — `DrawNode::Path` always
        // lowers to an SVG `<path>` element (the bridge's vocabulary has no `<rect>`), unlike the
        // previous hand-rolled string builder this replaced.
        assert!(svg.contains("<path"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn exports_error_when_page_missing() {
        let doc = sample_document();
        assert!(matches!(export_document_svg(&doc, "no-such-page"), Err(LayoutError::PageNotFound(id)) if id == "no-such-page"));
        assert!(matches!(export_document_pdf(&doc, "no-such-page"), Err(LayoutError::PageNotFound(_))));
        assert!(matches!(export_document_png_cpu(&doc, "no-such-page"), Err(LayoutError::PageNotFound(_))));
    }

    #[test]
    fn package_zip_rejects_invalid_document_json() {
        let error = export_package_zip("not json", "[]").expect_err("invalid json must fail");
        assert!(matches!(error, LayoutError::Json(_)));
    }

    #[test]
    fn scene_png_from_display_list_writes_a_valid_png() {
        let doc = sample_document();
        let page = doc.pages.iter().find(|p| p.id == "page-1").expect("page-1");
        let mut engine = LayoutEngine::new();
        let list = build_display_list_for_page(&mut engine, &doc, page, "page-1", &[], None, false);
        let bytes = scene_png_from_display_list(&list).expect("scene png export succeeds");
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
    }
}
//#endregion 🧪️Tests
