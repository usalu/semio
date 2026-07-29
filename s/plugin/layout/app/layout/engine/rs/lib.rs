//! ⚙️ Layout app — headless compute (constitutional: engine).

use std::borrow::Cow;
use std::io::{Cursor, Write};
use std::sync::{Arc, OnceLock};

use fontique::Blob;
use image::{ImageBuffer, Rgba};
use infinite_cavas::camera::{self, Camera, Viewport};
use infinite_cavas::{Affine, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};
use layout::{
    Frame, GridSettings, Layer, LayoutBounds, LayoutCamera, LayoutDocument, LayoutRect, Page, PageColumns, PageMargins, ParagraphStyle, Spread, TextStory,
    LAYOUT_FIXTURE_SCHEMA,
};
use parley::{Alignment, AlignmentOptions, FontContext, FontStack, FontWeight, Layout, LayoutContext, LineHeight, PositionedLayoutItem, StyleProperty};
use serde_json::Value;
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

//#region ⚠️ Errors
/// 🚧 All fallible layout-engine operations funnel through this — document parsing, scene/hit-test
/// resolution, and export (SVG/PDF/PNG/zip package).
#[derive(Debug, thiserror::Error)]
pub enum LayoutError {
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unexpected schema {0}")]
    UnexpectedSchema(String),
    #[error("page {0} not found")]
    PageNotFound(String),
    #[error("png: {0}")]
    Png(#[from] png::EncodingError),
    #[error("zip: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}
//#endregion ⚠️ Errors

//#region 📄 Document
pub fn parse_layout_document(json: &str) -> Result<LayoutDocument, LayoutError> {
    let doc: LayoutDocument = serde_json::from_str(json)?;
    if doc.schema != LAYOUT_FIXTURE_SCHEMA {
        return Err(LayoutError::UnexpectedSchema(doc.schema));
    }
    Ok(doc)
}

pub struct ResolvedFrame {
    pub frame: Frame,
    pub inherited: bool,
}

pub fn resolve_page<'a>(doc: &'a LayoutDocument, page: &'a Page) -> Vec<ResolvedFrame> {
    let mut frames = Vec::new();
    if let Some(parent_id) = &page.parent_page_id {
        if let Some(parent) = doc.parent_pages.iter().find(|p| p.id == *parent_id) {
            for frame in &parent.frames {
                let overridden = page.overrides.iter().any(|o| o.object_id == frame.id());
                frames.push(ResolvedFrame { frame: frame.clone(), inherited: !overridden });
            }
        }
    }
    for frame in &page.frames {
        frames.push(ResolvedFrame { frame: frame.clone(), inherited: false });
    }
    frames
}
//#endregion 📄 Document

//#region 🖼️ Display
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
//#endregion 🖼️ Display

//#region ⚙️ Scene
static LAYOUT_SANS: &[u8] = include_bytes!("../../../../../../framework/kernel/infinite/canvas/rs/asset/MapLabelSans.ttf");

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

static ENGINE: OnceLock<std::sync::Mutex<LayoutEngine>> = OnceLock::new();

fn engine() -> &'static std::sync::Mutex<LayoutEngine> {
    ENGINE.get_or_init(|| std::sync::Mutex::new(LayoutEngine::new()))
}

fn alignment_from_str(value: &str) -> Alignment {
    match value {
        "center" | "middle" => Alignment::Middle,
        "right" => Alignment::Right,
        "justify" | "justified" => Alignment::Justified,
        _ => Alignment::Left,
    }
}

fn default_paragraph(doc: &LayoutDocument) -> ParagraphStyle {
    doc.paragraph_styles.first().cloned().unwrap_or(ParagraphStyle { id: "paragraph.body".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() })
}

pub fn layout_story_in_frame(story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (Layout<[u8; 4]>, bool) {
    engine().lock().expect("layout engine").layout_story(story, paragraph, frame_width, frame_height)
}

pub fn build_display_list_for_page(doc: &LayoutDocument, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool) -> DisplayList {
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
                    let (layout, _overset) = layout_story_in_frame(story, &paragraph, frame_width, frame_height);
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
                let placeholder = link.map(|l| l.state.as_deref() == Some("missing") || l.proxy_data_url.is_none()).unwrap_or(true);
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

/// @emoji 👻 Catalogue drop ghost rect shown while dragging onto the canvas.
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

/// 🔭 Bundled render/hit-test context for a single page query — page, camera/viewport, and
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

pub fn build_scene_from_document_json(json: &str, query: &SceneQuery, drop_preview: Option<&LayoutDropPreview>) -> Result<Scene, LayoutError> {
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(&doc, page, query.page_id, query.selected_ids, query.hovered_id, query.chrome_blueprint);
    Ok(display_list_to_scene(&list, query.chrome_blueprint, query.camera, query.viewport, drop_preview))
}

pub fn hit_test_document_json(json: &str, sx: f64, sy: f64, query: &SceneQuery) -> Result<Option<String>, LayoutError> {
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(&doc, page, query.page_id, query.selected_ids, query.hovered_id, true);
    let world = camera::screen_to_world(query.camera, query.viewport, Point::new(sx, sy));
    Ok(list.hit_test(world.x as f32, world.y as f32))
}

pub fn screen_to_world_json(camera: &Camera, viewport: &Viewport, sx: f64, sy: f64) -> String {
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    serde_json::json!({ "x": world.x, "y": world.y }).to_string()
}
//#endregion ⚙️ Scene

//#region 📤 Export
pub fn export_display_list_svg(list: &DisplayList) -> String {
    let mut out = String::new();
    out.push_str(&format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#, list.page_width, list.page_height, list.page_width, list.page_height));
    out.push('\n');
    out.push_str(&format!(r#"<rect width="{}" height="{}" fill="white"/>"#, list.page_width, list.page_height));
    out.push('\n');
    for rect in &list.rects {
        if let Some(fill) = &rect.fill {
            out.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgba({},{},{},{})"/>"#, rect.x, rect.y, rect.width, rect.height, (fill.0[0] * 255.0) as u8, (fill.0[1] * 255.0) as u8, (fill.0[2] * 255.0) as u8, fill.0[3]));
            out.push('\n');
        }
        if let Some(stroke) = &rect.stroke {
            out.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="rgba({},{},{},{})" stroke-width="1"/>"#,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                (stroke.0[0] * 255.0) as u8,
                (stroke.0[1] * 255.0) as u8,
                (stroke.0[2] * 255.0) as u8,
                stroke.0[3]
            ));
            out.push('\n');
        }
    }
    for image in &list.images {
        let fill = if image.placeholder { "rgba(235,225,215,1)" } else { "rgba(220,220,220,1)" };
        out.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#, image.x, image.y, image.width, image.height, fill));
        out.push('\n');
    }
    for run in &list.text_runs {
        for glyph in &run.glyphs {
            out.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#, glyph.x, glyph.y, glyph.font_size * 0.45, glyph.font_size));
            out.push('\n');
        }
    }
    out.push_str("</svg>");
    out
}

pub fn export_document_svg(doc: &LayoutDocument, page_id: &str) -> Result<String, LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| LayoutError::PageNotFound(page_id.to_string()))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], None, false);
    Ok(export_display_list_svg(&list))
}

pub fn export_document_pdf(doc: &LayoutDocument, page_id: &str) -> Result<Vec<u8>, LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| LayoutError::PageNotFound(page_id.to_string()))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], None, false);
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

pub fn export_document_png_cpu(doc: &LayoutDocument, page_id: &str) -> Result<Vec<u8>, LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| LayoutError::PageNotFound(page_id.to_string()))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], None, false);
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
    let doc: LayoutDocument = serde_json::from_str(doc_json)?;
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file("document.json", options)?;
    zip.write_all(doc_json.as_bytes())?;
    zip.start_file("preflight-report.json", options)?;
    zip.write_all(preflight_json.as_bytes())?;
    let manifest_links: Vec<serde_json::Value> = doc
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
//#endregion 📤 Export

//#region 🔖 DocumentHelpers
const LAYOUT_SAMPLE_TEXT: &str = layout_dsl::LAYOUT_SAMPLE_TEXT;

/// 📄 The bundled sample fixture, parsed once — the source of truth for `LayoutPlayApp::initial_projection`
/// and the app manifest's `.example(...)` document.
pub fn default_document() -> LayoutDocument {
    <LayoutDocument as store::DocumentDsl>::parse_dsl(LAYOUT_SAMPLE_TEXT).expect("sample layout fixture")
}

/// 🌉 JSON bridge for `semio_framework_plugin`'s `App::example`, which hardcodes `serde_json::from_str`
/// on its `document_json` parameter (shared framework machinery, out of scope for this DSL migration) —
/// derives the JSON from the DSL fixture rather than keeping a second, redundant JSON copy of it on disk.
pub fn layout_sample_document_json() -> String {
    serde_json::to_string(&default_document()).unwrap_or_default()
}
//#endregion 🔖 DocumentHelpers

//#region 🔖 MediaImportExport
pub fn layout_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
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
pub fn layout_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
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
                layers: vec![Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }],
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
        grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: vec![Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids }],
        pages,
        print_target: None,
    };
    serde_json::to_value(document).map_err(|e| e.to_string())
}
//#endregion 🔖 MediaImportExport

//#region 🧪 Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn rect_frame(id: &str, visible: Option<bool>) -> Frame {
        Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible, fill: None, stroke: None }
    }

    fn base_doc() -> LayoutDocument {
        LayoutDocument {
            schema: LAYOUT_FIXTURE_SCHEMA.into(),
            name: "t".into(),
            camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
        }
    }

    fn sample_document() -> LayoutDocument {
        layout_dsl::parse_dsl(layout_dsl::LAYOUT_SAMPLE_TEXT).expect("sample fixture parses")
    }

    #[test]
    fn resolve_page_marks_overridden_parent_frames_and_ignores_missing_parent() {
        let mut doc = base_doc();
        doc.parent_pages.push(layout::ParentPage {
            id: "parent-1".into(),
            name: "Master".into(),
            width: 100.0,
            height: 100.0,
            layer_ids: vec!["layer-1".into()],
            layers: Vec::new(),
            frames: vec![rect_frame("frame-a", None), rect_frame("frame-b", None)],
        });

        let page_with_parent = Page {
            id: "page-1".into(),
            name: "P1".into(),
            spread_id: "spread-1".into(),
            parent_page_id: Some("parent-1".into()),
            width: 100.0,
            height: 100.0,
            margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
            columns: PageColumns { count: 1, gutter: 0.0 },
            guides: Vec::new(),
            layer_ids: Vec::new(),
            layers: Vec::new(),
            frames: Vec::new(),
            overrides: vec![layout::PageOverride { object_id: "frame-a".into(), bounds: None, visible: None, locked: None }],
        };
        let resolved = resolve_page(&doc, &page_with_parent);
        assert_eq!(resolved.len(), 2);
        let a = resolved.iter().find(|r| r.frame.id() == "frame-a").expect("frame-a resolved");
        assert!(!a.inherited, "overridden parent frame must not be marked inherited");
        let b = resolved.iter().find(|r| r.frame.id() == "frame-b").expect("frame-b resolved");
        assert!(b.inherited, "non-overridden parent frame stays inherited");

        let mut page_missing_parent = page_with_parent.clone();
        page_missing_parent.parent_page_id = Some("no-such-parent".into());
        assert!(resolve_page(&doc, &page_missing_parent).is_empty());

        let mut page_no_parent = page_with_parent;
        page_no_parent.parent_page_id = None;
        page_no_parent.frames = vec![rect_frame("frame-own", None)];
        let own_only = resolve_page(&doc, &page_no_parent);
        assert_eq!(own_only.len(), 1);
        assert!(!own_only[0].inherited);
    }

    #[test]
    fn parse_layout_document_rejects_wrong_schema_and_invalid_json() {
        let wrong_schema = r#"{"schema":"other.schema","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[]}"#;
        let error = parse_layout_document(wrong_schema).expect_err("wrong schema must fail");
        assert!(matches!(error, LayoutError::UnexpectedSchema(schema) if schema == "other.schema"));

        let invalid_json = "not json";
        let error = parse_layout_document(invalid_json).expect_err("invalid json must fail");
        assert!(matches!(error, LayoutError::Json(_)));
    }

    #[test]
    fn builds_scene_from_empty_document() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
        let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
        let scene = build_scene_from_document_json(json, &query, None).expect("scene");
        let _ = scene;
    }

    #[test]
    fn hit_test_respects_camera_zoom() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 0.5 };
        let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
        let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
        let hit = hit_test_document_json(json, 210.0, 160.0, &query).expect("hit");
        assert_eq!(hit.as_deref(), Some("frame-1"));
    }

    #[test]
    fn marks_hovered_frame_rect() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let list = build_display_list_for_page(&doc, page, "page-1", &[], Some("frame-1"), true);
        assert!(list.rects.iter().any(|rect| rect.object_id == "frame-1" && rect.hovered));
        assert!(list.rects.iter().all(|rect| rect.object_id != "frame-1" || rect.hovered));
    }

    #[test]
    fn scene_and_hit_test_error_when_page_missing() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":100,"height":100,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 100, height: 100, dpr: 1.0 };
        let query = SceneQuery { page_id: "missing-page", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
        assert!(matches!(build_scene_from_document_json(json, &query, None), Err(LayoutError::PageNotFound(id)) if id == "missing-page"));
        let hit = hit_test_document_json(json, 0.0, 0.0, &query);
        assert!(matches!(hit, Err(LayoutError::PageNotFound(id)) if id == "missing-page"));
    }

    #[test]
    fn hit_test_returns_none_for_empty_space() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 400, height: 400, dpr: 1.0 };
        let query = SceneQuery { page_id: "page-1", selected_ids: &[], hovered_id: None, chrome_blueprint: false, camera: &camera, viewport: &viewport };
        let hit = hit_test_document_json(json, 300.0, 300.0, &query).expect("hit test");
        assert!(hit.is_none());
    }

    #[test]
    fn display_list_hit_test_matches_image_bounds_and_misses_elsewhere() {
        let list = DisplayList { page_id: "page-1".into(), page_width: 100.0, page_height: 100.0, rects: Vec::new(), text_runs: Vec::new(), images: vec![DisplayImage { object_id: "img-1".into(), x: 10.0, y: 10.0, width: 20.0, height: 20.0, placeholder: false }], guides: Vec::new() };
        assert_eq!(list.hit_test(15.0, 15.0).as_deref(), Some("img-1"));
        assert!(list.hit_test(90.0, 90.0).is_none());
    }

    #[test]
    fn guides_omitted_for_non_active_page_even_with_chrome_blueprint() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":10,"right":10,"bottom":10,"left":10},"columns":{"count":2,"gutter":4},"guides":[{"x":5,"y":5,"w":1,"h":1}],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let list = build_display_list_for_page(&doc, page, "different-active-page", &[], None, true);
        assert!(list.guides.is_empty(), "guides must only render for the active blueprint page");
    }

    #[test]
    fn baseline_guides_only_emitted_when_grid_snaps() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let list = build_display_list_for_page(&doc, page, "page-1", &[], None, true);
        assert!(list.guides.iter().all(|guide| guide.kind != "baseline"));
    }

    #[test]
    fn image_placeholder_reflects_link_lookup_and_state() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[{"id":"link-missing","path":"a.png","hash":"h","width":1,"height":1,"dpi":72,"state":"missing"},{"id":"link-ready","path":"b.png","hash":"h","width":1,"height":1,"dpi":72,"state":"ready","proxyDataUrl":"data:image/png;base64,AA=="}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["img-missing","img-ready","img-unlinked"]}],"frames":[{"id":"img-missing","layerId":"layer-1","kind":"image","bounds":{"x":0,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-missing"},{"id":"img-ready","layerId":"layer-1","kind":"image","bounds":{"x":20,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-ready"},{"id":"img-unlinked","layerId":"layer-1","kind":"image","bounds":{"x":40,"y":0,"w":10,"h":10,"rotation":0},"linkId":"link-gone"}],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let list = build_display_list_for_page(&doc, page, "page-1", &[], None, false);
        let by_id = |id: &str| list.images.iter().find(|i| i.object_id == id).expect("image present");
        assert!(by_id("img-missing").placeholder, "missing-state link stays a placeholder");
        assert!(!by_id("img-ready").placeholder, "ready link with a proxy is not a placeholder");
        assert!(by_id("img-unlinked").placeholder, "unresolved link falls back to placeholder");
    }

    #[test]
    fn layout_story_in_frame_resolves_alignment_variants_and_detects_overset() {
        let story = TextStory { id: "story-1".into(), content: "Hello layout engine, this line should wrap across several lines of text.".into(), style_runs: Vec::new() };
        for alignment in ["left", "center", "middle", "right", "justify", "justified", "unrecognized"] {
            let paragraph = ParagraphStyle { id: "p".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: alignment.into() };
            let (layout, overset) = layout_story_in_frame(&story, &paragraph, 80.0, 10.0);
            assert!(layout.height() > 0.0, "alignment {alignment} should still measure a positive height");
            assert!(overset, "narrow/short frame with long content should overset for alignment {alignment}");
        }
        let paragraph = ParagraphStyle { id: "p".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() };
        let (_, not_overset) = layout_story_in_frame(&story, &paragraph, 2000.0, 2000.0);
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
                DisplayRect { object_id: "r-explicit-stroke".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, fill: Some(DisplayColor([1.0, 1.0, 1.0, 1.0])), stroke: Some(DisplayColor([0.0, 0.0, 0.0, 1.0])), inherited: false, selected: true, hovered: false },
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
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid json point");
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
        assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
        assert!(bytes.starts_with(b"PK"));
    }

    #[test]
    fn svg_export_contains_rect_and_wraps_a_valid_document() {
        let doc = sample_document();
        let svg = export_document_svg(&doc, "page-1").expect("svg export succeeds");
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("<rect"));
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
        let list = build_display_list_for_page(&doc, page, "page-1", &[], None, false);
        let bytes = scene_png_from_display_list(&list).expect("scene png export succeeds");
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
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
//#endregion 🧪 Tests
