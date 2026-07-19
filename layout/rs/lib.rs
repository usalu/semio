//! 📄 Layout engine — document layout, WebGPU render, export.

pub use infinite_cavas as cavas;

//#region ⚠️ Errors
/// 🚧 All fallible layout-crate operations funnel through this — document parsing, scene/hit-test
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

mod document {
// #region document
use serde::{Deserialize, Serialize};

pub const LAYOUT_FIXTURE_SCHEMA: &str = "layout.fixture";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutRect {
    pub x: f64,
    pub y: f64,
    #[serde(rename = "w")]
    pub width: f64,
    #[serde(rename = "h")]
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutBounds {
    pub x: f64,
    pub y: f64,
    #[serde(rename = "w")]
    pub width: f64,
    #[serde(rename = "h")]
    pub height: f64,
    pub rotation: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageMargins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageColumns {
    pub count: u32,
    pub gutter: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    #[serde(rename = "objectIds")]
    pub object_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FrameKind {
    #[serde(rename = "rect")]
    Rect,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameBase {
    pub id: String,
    #[serde(rename = "layerId")]
    pub layer_id: String,
    pub bounds: LayoutBounds,
    pub locked: Option<bool>,
    pub visible: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RectFrame {
    #[serde(flatten)]
    pub base: FrameBase,
    pub fill: Option<[f32; 4]>,
    pub stroke: Option<[f32; 4]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextFrame {
    #[serde(flatten)]
    pub base: FrameBase,
    #[serde(rename = "storyId")]
    pub story_id: String,
    #[serde(rename = "threadNext")]
    pub thread_next: Option<String>,
    pub columns: u32,
    pub inset: LayoutRect,
    #[serde(rename = "wrapMode")]
    pub wrap_mode: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageFrame {
    #[serde(flatten)]
    pub base: FrameBase,
    #[serde(rename = "linkId")]
    pub link_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Frame {
    #[serde(rename = "rect")]
    Rect {
        id: String,
        #[serde(rename = "layerId")]
        layer_id: String,
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        fill: Option<[f32; 4]>,
        stroke: Option<[f32; 4]>,
    },
    #[serde(rename = "text")]
    Text {
        id: String,
        #[serde(rename = "layerId")]
        layer_id: String,
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[serde(rename = "storyId")]
        story_id: String,
        #[serde(rename = "threadNext")]
        thread_next: Option<String>,
        columns: u32,
        inset: LayoutRect,
        #[serde(rename = "wrapMode")]
        wrap_mode: String,
    },
    #[serde(rename = "image")]
    Image {
        id: String,
        #[serde(rename = "layerId")]
        layer_id: String,
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[serde(rename = "linkId")]
        link_id: String,
    },
}

impl Frame {
    pub fn id(&self) -> &str {
        match self {
            Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. } => id,
        }
    }

    pub fn bounds(&self) -> &LayoutBounds {
        match self {
            Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
        }
    }

    pub fn kind_str(&self) -> &str {
        match self {
            Frame::Rect { .. } => "rect",
            Frame::Text { .. } => "text",
            Frame::Image { .. } => "image",
        }
    }

    pub fn visible(&self) -> bool {
        match self {
            Frame::Rect { visible, .. } | Frame::Text { visible, .. } | Frame::Image { visible, .. } => visible.unwrap_or(true),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextStyleRun {
    pub start: usize,
    pub end: usize,
    #[serde(rename = "paragraphStyleId")]
    pub paragraph_style_id: Option<String>,
    #[serde(rename = "characterStyleId")]
    pub character_style_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextStory {
    pub id: String,
    pub content: String,
    #[serde(rename = "styleRuns")]
    pub style_runs: Vec<TextStyleRun>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParagraphStyle {
    pub id: String,
    pub name: String,
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "fontSize")]
    pub font_size: f64,
    #[serde(rename = "fontWeight")]
    pub font_weight: u32,
    pub leading: f64,
    pub tracking: f64,
    pub alignment: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageLink {
    pub id: String,
    pub path: String,
    pub hash: String,
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
    #[serde(rename = "colorProfile")]
    pub color_profile: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "proxyDataUrl")]
    pub proxy_data_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageOverride {
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub bounds: Option<LayoutBounds>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParentPage {
    pub id: String,
    pub name: String,
    pub width: f64,
    pub height: f64,
    #[serde(rename = "layerIds")]
    pub layer_ids: Vec<String>,
    pub layers: Vec<Layer>,
    pub frames: Vec<Frame>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub name: String,
    #[serde(rename = "spreadId")]
    pub spread_id: String,
    #[serde(rename = "parentPageId")]
    pub parent_page_id: Option<String>,
    pub width: f64,
    pub height: f64,
    pub margins: PageMargins,
    pub columns: PageColumns,
    pub guides: Vec<LayoutRect>,
    #[serde(rename = "layerIds")]
    pub layer_ids: Vec<String>,
    pub layers: Vec<Layer>,
    pub frames: Vec<Frame>,
    pub overrides: Vec<PageOverride>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spread {
    pub id: String,
    pub name: String,
    #[serde(rename = "pageIds")]
    pub page_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridSettings {
    #[serde(rename = "baselineGrid")]
    pub baseline_grid: f64,
    #[serde(rename = "baselineOffset")]
    pub baseline_offset: f64,
    #[serde(rename = "snapToBaseline")]
    pub snap_to_baseline: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutDocument {
    pub schema: String,
    pub name: String,
    pub camera: LayoutCamera,
    #[serde(rename = "previewCamera")]
    pub preview_camera: LayoutCamera,
    pub grid: GridSettings,
    #[serde(rename = "paragraphStyles")]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[serde(rename = "characterStyles")]
    pub character_styles: Vec<serde_json::Value>,
    pub stories: Vec<TextStory>,
    pub links: Vec<ImageLink>,
    #[serde(rename = "parentPages")]
    pub parent_pages: Vec<ParentPage>,
    pub spreads: Vec<Spread>,
    pub pages: Vec<Page>,
    #[serde(rename = "printTarget")]
    pub print_target: Option<String>,
}

pub fn parse_layout_document(json: &str) -> Result<LayoutDocument, crate::LayoutError> {
    let doc: LayoutDocument = serde_json::from_str(json)?;
    if doc.schema != LAYOUT_FIXTURE_SCHEMA {
        return Err(crate::LayoutError::UnexpectedSchema(doc.schema));
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
                frames.push(ResolvedFrame {
                    frame: frame.clone(),
                    inherited: !overridden,
                });
            }
        }
    }
    for frame in &page.frames {
        frames.push(ResolvedFrame {
            frame: frame.clone(),
            inherited: false,
        });
    }
    frames
}

// #endregion document
}

mod display {
// #region display
use crate::document::{LayoutBounds, LayoutRect, Page};

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
    vec![
        DisplayGuide {
            rect: LayoutRect {
                x: page.margins.left,
                y: page.margins.top,
                width: page.width - page.margins.left - page.margins.right,
                height: page.height - page.margins.top - page.margins.bottom,
            },
            kind: "margin".into(),
        },
    ]
}

pub fn bounds_to_display_rect(object_id: &str, bounds: &LayoutBounds, inherited: bool, selected: bool, hovered: bool, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> DisplayRect {
    DisplayRect {
        object_id: object_id.into(),
        x: bounds.x as f32,
        y: bounds.y as f32,
        width: bounds.width as f32,
        height: bounds.height as f32,
        fill: fill.map(DisplayColor),
        stroke: stroke.map(DisplayColor),
        inherited,
        selected,
        hovered,
    }
}
// #endregion display
}

mod engine {
// #region engine
use std::borrow::Cow;
use std::sync::{Arc, OnceLock};

use fontique::Blob;
use parley::{
    Alignment, AlignmentOptions, FontContext, FontStack, FontWeight, Layout, LayoutContext, LineHeight, PositionedLayoutItem,
    StyleProperty,
};
use infinite_cavas::camera::{self, Camera, Viewport};
use infinite_cavas::{Affine, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};

use crate::display::{
    bounds_to_display_rect, page_margin_guides, DisplayColor, DisplayGlyph, DisplayGuide, DisplayImage, DisplayList, DisplayTextRun,
};
use crate::document::{parse_layout_document, resolve_page, Frame, LayoutDocument, Page, ParagraphStyle, TextStory};

static LAYOUT_SANS: &[u8] = include_bytes!("../../infinite/cavas/rs/asset/MapLabelSans.ttf");

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
        Self {
            font_context: FontContext::new(),
            layout_context: LayoutContext::new(),
            fonts_ready: false,
        }
    }

    fn ensure_fonts(&mut self) {
        if self.fonts_ready {
            return;
        }
        self.font_context
            .collection
            .register_fonts(Blob::new(Arc::new(LAYOUT_SANS.to_vec())), None);
        self.fonts_ready = true;
    }

    pub fn layout_story(&mut self, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (Layout<[u8; 4]>, bool) {
        self.ensure_fonts();
        let mut builder = self
            .layout_context
            .ranged_builder(&mut self.font_context, &story.content, 1.0, true);
        builder.push_default(StyleProperty::FontSize(paragraph.font_size as f32));
        builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed("Layout Sans"))));
        builder.push_default(StyleProperty::FontWeight(FontWeight::new(paragraph.font_weight as f32)));
        builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
            (paragraph.leading / paragraph.font_size.max(1.0)) as f32,
        )));
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
    doc.paragraph_styles.first().cloned().unwrap_or(ParagraphStyle {
        id: "paragraph.body".into(),
        name: "Body".into(),
        font_family: "Layout Sans".into(),
        font_size: 12.0,
        font_weight: 400,
        leading: 14.4,
        tracking: 0.0,
        alignment: "left".into(),
    })
}

pub fn layout_story_in_frame(story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (Layout<[u8; 4]>, bool) {
    engine().lock().expect("layout engine").layout_story(story, paragraph, frame_width, frame_height)
}

pub fn build_display_list_for_page(doc: &LayoutDocument, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool) -> DisplayList {
    let resolved = resolve_page(doc, page);
    let mut rects = Vec::new();
    let mut text_runs = Vec::new();
    let mut images = Vec::new();
    let mut guides = if chrome_blueprint && page.id == active_page_id {
        page_margin_guides(page)
    } else {
        Vec::new()
    };

    if chrome_blueprint && page.id == active_page_id {
        for guide in &page.guides {
            guides.push(DisplayGuide {
                rect: guide.clone(),
                kind: "guide".into(),
            });
        }
        let col_count = page.columns.count.max(1) as f64;
        let col_width = (page.width - page.margins.left - page.margins.right - page.columns.gutter * (col_count - 1.0)) / col_count;
        for i in 0..page.columns.count {
            let x = page.margins.left + (i as f64) * (col_width + page.columns.gutter);
            guides.push(DisplayGuide {
                rect: crate::document::LayoutRect {
                    x,
                    y: page.margins.top,
                    width: col_width,
                    height: page.height - page.margins.top - page.margins.bottom,
                },
                kind: "column".into(),
            });
        }
        if doc.grid.snap_to_baseline && doc.grid.baseline_grid > 0.0 {
            let mut y = doc.grid.baseline_offset;
            while y < page.height {
                guides.push(DisplayGuide {
                    rect: crate::document::LayoutRect {
                        x: 0.0,
                        y,
                        width: page.width,
                        height: 0.0,
                    },
                    kind: "baseline".into(),
                });
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
                rects.push(bounds_to_display_rect(
                    id,
                    bounds,
                    item.inherited,
                    selected,
                    hovered,
                    *fill,
                    stroke.or(if chrome_blueprint && item.inherited {
                        Some([0.4, 0.5, 0.7, 0.8])
                    } else {
                        None
                    }),
                ));
            }
            Frame::Text { id, bounds, story_id, inset, .. } => {
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(
                        id,
                        bounds,
                        item.inherited,
                        selected,
                        hovered,
                        None,
                        Some([0.2, 0.55, 0.9, 0.9]),
                    ));
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
                                    glyphs.push(DisplayGlyph {
                                        glyph_id: glyph.id as u32,
                                        font_size,
                                        x: base_x + glyph.x,
                                        y: base_y + glyph.y,
                                        color: DisplayColor([0.0, 0.0, 0.0, 1.0]),
                                    });
                                }
                            }
                        }
                    }
                    text_runs.push(DisplayTextRun {
                        object_id: id.clone(),
                        glyphs,
                    });
                }
            }
            Frame::Image { id, bounds, link_id, .. } => {
                let link = doc.links.iter().find(|l| l.id == *link_id);
                let placeholder = link
                    .map(|l| l.state.as_deref() == Some("missing") || l.proxy_data_url.is_none())
                    .unwrap_or(true);
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(
                        id,
                        bounds,
                        item.inherited,
                        selected,
                        hovered,
                        None,
                        Some([0.85, 0.45, 0.2, 0.9]),
                    ));
                }
                images.push(DisplayImage {
                    object_id: id.clone(),
                    x: bounds.x as f32,
                    y: bounds.y as f32,
                    width: bounds.width as f32,
                    height: bounds.height as f32,
                    placeholder,
                });
            }
        }
    }

    DisplayList {
        page_id: page.id.clone(),
        page_width: page.width as f32,
        page_height: page.height as f32,
        rects,
        text_runs,
        images,
        guides,
    }
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
    scene.stroke(
        &Stroke::new(2.0),
        transform,
        Color::new([0.1, 0.45, 0.95, 0.85]),
        None,
        &shape,
    );
}

pub fn display_list_to_scene(
    list: &DisplayList,
    chrome_blueprint: bool,
    camera: &Camera,
    viewport: &Viewport,
    drop_preview: Option<&LayoutDropPreview>,
) -> Scene {
    let mut scene = Scene::new();
    let transform = camera::camera_content_affine(camera, viewport);
    let page_bg = if chrome_blueprint {
        Color::new([0.97, 0.97, 0.98, 1.0])
    } else {
        Color::new([1.0, 1.0, 1.0, 1.0])
    };
    scene.fill(
        FillRule::NonZero,
        transform,
        page_bg,
        None,
        &Rect::new(0.0, 0.0, list.page_width as f64, list.page_height as f64),
    );

    if chrome_blueprint {
        for guide in &list.guides {
            let stroke = match guide.kind.as_str() {
                "margin" => Color::new([0.75, 0.2, 0.2, 0.35]),
                "column" => Color::new([0.2, 0.45, 0.85, 0.25]),
                "baseline" => Color::new([0.5, 0.5, 0.5, 0.2]),
                _ => Color::new([0.3, 0.3, 0.3, 0.3]),
            };
            if guide.rect.height <= 0.0 {
                scene.stroke(
                    &Stroke::new(1.0),
                    transform,
                    stroke,
                    None,
                    &Line::new(
                        Point::new(guide.rect.x, guide.rect.y),
                        Point::new(guide.rect.x + guide.rect.width, guide.rect.y),
                    ),
                );
            } else {
                scene.stroke(
                    &Stroke::new(1.0),
                    transform,
                    stroke,
                    None,
                    &Rect::new(guide.rect.x, guide.rect.y, guide.rect.x + guide.rect.width, guide.rect.y + guide.rect.height),
                );
            }
        }
    }

    for rect in &list.rects {
        let shape = RoundedRect::new(
            Rect::new(
                rect.x as f64,
                rect.y as f64,
                (rect.x + rect.width) as f64,
                (rect.y + rect.height) as f64,
            ),
            RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0),
        );
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
            scene.stroke(
                &Stroke::new(width),
                transform,
                color_from(stroke),
                None,
                &shape,
            );
        } else if rect.selected && chrome_blueprint {
            scene.stroke(
                &Stroke::new(2.0),
                transform,
                Color::new([0.1, 0.45, 0.95, 1.0]),
                None,
                &shape,
            );
        } else if rect.hovered && chrome_blueprint {
            scene.stroke(
                &Stroke::new(1.5),
                transform,
                Color::new([0.95, 0.72, 0.15, 1.0]),
                None,
                &shape,
            );
        }
    }

    for image in &list.images {
        let color = if image.placeholder {
            Color::new([0.92, 0.88, 0.84, 1.0])
        } else {
            Color::new([0.85, 0.85, 0.85, 1.0])
        };
        let shape = Rect::new(image.x as f64, image.y as f64, (image.x + image.width) as f64, (image.y + image.height) as f64);
        scene.fill(FillRule::NonZero, transform, color, None, &shape);
        if image.placeholder {
            scene.stroke(
                &Stroke::new(1.0),
                transform,
                Color::new([0.75, 0.35, 0.2, 1.0]),
                None,
                &shape,
            );
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

pub fn build_scene_from_document_json(json: &str, query: &SceneQuery, drop_preview: Option<&LayoutDropPreview>) -> Result<Scene, crate::LayoutError> {
    let doc = parse_layout_document(json)?;
    let page = doc
        .pages
        .iter()
        .find(|p| p.id == query.page_id)
        .ok_or_else(|| crate::LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(&doc, page, query.page_id, query.selected_ids, query.hovered_id, query.chrome_blueprint);
    Ok(display_list_to_scene(&list, query.chrome_blueprint, query.camera, query.viewport, drop_preview))
}

pub fn hit_test_document_json(json: &str, sx: f64, sy: f64, query: &SceneQuery) -> Result<Option<String>, crate::LayoutError> {
    let doc = parse_layout_document(json)?;
    let page = doc
        .pages
        .iter()
        .find(|p| p.id == query.page_id)
        .ok_or_else(|| crate::LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(&doc, page, query.page_id, query.selected_ids, query.hovered_id, true);
    let world = camera::screen_to_world(query.camera, query.viewport, Point::new(sx, sy));
    Ok(list.hit_test(world.x as f32, world.y as f32))
}

pub fn screen_to_world_json(camera: &Camera, viewport: &Viewport, sx: f64, sy: f64) -> String {
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    serde_json::json!({ "x": world.x, "y": world.y }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
// #endregion engine
}

mod export {
// #region export
use std::io::{Cursor, Write};

use image::{ImageBuffer, Rgba};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::display::DisplayList;
use crate::document::LayoutDocument;
use crate::engine::{build_display_list_for_page, display_list_to_scene};

pub fn export_display_list_svg(list: &DisplayList) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        list.page_width, list.page_height, list.page_width, list.page_height
    ));
    out.push('\n');
    out.push_str(&format!(r#"<rect width="{}" height="{}" fill="white"/>"#, list.page_width, list.page_height));
    out.push('\n');
    for rect in &list.rects {
        if let Some(fill) = &rect.fill {
            out.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgba({},{},{},{})"/>"#,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                (fill.0[0] * 255.0) as u8,
                (fill.0[1] * 255.0) as u8,
                (fill.0[2] * 255.0) as u8,
                fill.0[3]
            ));
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
        out.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
            image.x, image.y, image.width, image.height, fill
        ));
        out.push('\n');
    }
    for run in &list.text_runs {
        for glyph in &run.glyphs {
            out.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                glyph.x,
                glyph.y,
                glyph.font_size * 0.45,
                glyph.font_size
            ));
            out.push('\n');
        }
    }
    out.push_str("</svg>");
    out
}

pub fn export_document_svg(doc: &LayoutDocument, page_id: &str) -> Result<String, crate::LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| crate::LayoutError::PageNotFound(page_id.to_string()))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], None, false);
    Ok(export_display_list_svg(&list))
}

pub fn export_document_pdf(doc: &LayoutDocument, page_id: &str) -> Result<Vec<u8>, crate::LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| crate::LayoutError::PageNotFound(page_id.to_string()))?;
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
        format!(
            "3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R /Resources<< /Font<< /F1 5 0 R >> >> >>endobj\n",
            page.width, page.height
        ),
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

pub fn export_document_png_cpu(doc: &LayoutDocument, page_id: &str) -> Result<Vec<u8>, crate::LayoutError> {
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| crate::LayoutError::PageNotFound(page_id.to_string()))?;
    let list = build_display_list_for_page(doc, page, page_id, &[], None, false);
    let width = list.page_width.max(1.0) as u32;
    let height = list.page_height.max(1.0) as u32;
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));
    for rect in &list.rects {
        if let Some(fill) = &rect.fill {
            let color = Rgba([
                (fill.0[0] * 255.0) as u8,
                (fill.0[1] * 255.0) as u8,
                (fill.0[2] * 255.0) as u8,
                (fill.0[3] * 255.0) as u8,
            ]);
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

pub fn export_package_zip(doc_json: &str, preflight_json: &str) -> Result<Vec<u8>, crate::LayoutError> {
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
            let hash = if link.hash.is_empty() {
                format!("sha256:{:x}", Sha256::digest(link.path.as_bytes()))
            } else {
                link.hash.clone()
            };
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

pub fn scene_png_from_display_list(list: &DisplayList) -> Result<Vec<u8>, crate::LayoutError> {
    let camera = infinite_cavas::camera::Camera { x: 0.0, y: 0.0, zoom: 1.0 };
    let viewport = infinite_cavas::camera::Viewport {
        width: list.page_width.max(1.0) as u32,
        height: list.page_height.max(1.0) as u32,
        dpr: 1.0,
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{parse_layout_document, LAYOUT_FIXTURE_SCHEMA};

    #[test]
    fn png_cpu_export_writes_valid_rgba_png() {
        let json = include_str!("../example/sample.layout.json");
        let doc = parse_layout_document(json).expect("sample fixture parses");
        let bytes = export_document_png_cpu(&doc, "page-1").expect("png export succeeds");
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
    }

    #[test]
    fn pdf_export_writes_pdf_header() {
        let json = include_str!("../example/sample.layout.json");
        let doc = parse_layout_document(json).expect("sample fixture parses");
        let bytes = export_document_pdf(&doc, "page-1").expect("pdf export succeeds");
        assert!(bytes.starts_with(b"%PDF-1.4"));
    }

    #[test]
    fn package_zip_bundles_document_and_preflight() {
        let json = include_str!("../example/sample.layout.json");
        let doc = parse_layout_document(json).expect("sample fixture parses");
        let bytes = export_package_zip(json, "[]").expect("package export succeeds");
        assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
        assert!(bytes.starts_with(b"PK"));
    }
}
// #endregion export
}


mod ops {
// #region ops
//! 🧾 Typed VCS operation vocabulary for the layout document — the ops the layout plugin emits
//! (page/story/link collections, per-page frame add/remove/patch, and camera). Each op computes a
//! true pre-state inverse so undo/redo round-trips exactly. See {@link vcs::Operation}.

use serde::{Deserialize, Serialize};
use vcs::{
    apply_collection_op, invert_collection_op, CollectionOp, Identified, Operation, OperationDiff, Patchable,
};

use crate::document::{Frame, ImageLink, LayoutCamera, LayoutDocument, Page, TextStory};

//#region 🔖Ids
impl Identified<String> for Page {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for TextStory {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for ImageLink {
    fn id(&self) -> &String {
        &self.id
    }
}
//#endregion 🔖Ids

//#region 🔖Patches
/// 📄 Sparse scalar patch for a {@link Page} (name, size, margins, columns).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagePatch {
    pub name: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub margin_top: Option<f64>,
    pub margin_right: Option<f64>,
    pub margin_bottom: Option<f64>,
    pub margin_left: Option<f64>,
    pub columns_count: Option<u32>,
    pub columns_gutter: Option<f64>,
}

impl Patchable<PagePatch> for Page {
    fn apply_patch(&mut self, patch: &PagePatch) -> PagePatch {
        let inverse = PagePatch {
            name: patch.name.as_ref().map(|_| self.name.clone()),
            width: patch.width.map(|_| self.width),
            height: patch.height.map(|_| self.height),
            margin_top: patch.margin_top.map(|_| self.margins.top),
            margin_right: patch.margin_right.map(|_| self.margins.right),
            margin_bottom: patch.margin_bottom.map(|_| self.margins.bottom),
            margin_left: patch.margin_left.map(|_| self.margins.left),
            columns_count: patch.columns_count.map(|_| self.columns.count),
            columns_gutter: patch.columns_gutter.map(|_| self.columns.gutter),
        };
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(value) = patch.width {
            self.width = value;
        }
        if let Some(value) = patch.height {
            self.height = value;
        }
        if let Some(value) = patch.margin_top {
            self.margins.top = value;
        }
        if let Some(value) = patch.margin_right {
            self.margins.right = value;
        }
        if let Some(value) = patch.margin_bottom {
            self.margins.bottom = value;
        }
        if let Some(value) = patch.margin_left {
            self.margins.left = value;
        }
        if let Some(value) = patch.columns_count {
            self.columns.count = value;
        }
        if let Some(value) = patch.columns_gutter {
            self.columns.gutter = value;
        }
        inverse
    }
}

/// 📝 Sparse patch for a {@link TextStory}'s body content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStoryPatch {
    pub content: Option<String>,
}

impl Patchable<TextStoryPatch> for TextStory {
    fn apply_patch(&mut self, patch: &TextStoryPatch) -> TextStoryPatch {
        let inverse = TextStoryPatch { content: patch.content.as_ref().map(|_| self.content.clone()) };
        if let Some(content) = &patch.content {
            self.content = content.clone();
        }
        inverse
    }
}

/// 🔗 Sparse patch for an {@link ImageLink}'s file path.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageLinkPatch {
    pub path: Option<String>,
}

impl Patchable<ImageLinkPatch> for ImageLink {
    fn apply_patch(&mut self, patch: &ImageLinkPatch) -> ImageLinkPatch {
        let inverse = ImageLinkPatch { path: patch.path.as_ref().map(|_| self.path.clone()) };
        if let Some(path) = &patch.path {
            self.path = path.clone();
        }
        inverse
    }
}

/// 🖼️ Sparse patch for a {@link Frame}: bounds for any kind, fill/stroke for rects, wrap-mode/columns
/// for text. The doubly-optional `fill`/`stroke` distinguishes "unchanged" (outer `None`) from
/// "cleared" (inner `None`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FramePatch {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub fill: Option<Option<[f32; 4]>>,
    pub stroke: Option<Option<[f32; 4]>>,
    pub wrap_mode: Option<String>,
    pub columns: Option<u32>,
}

/// 🩹 Applies a {@link FramePatch} in place and returns the patch that undoes it.
pub fn apply_frame_patch(frame: &mut Frame, patch: &FramePatch) -> FramePatch {
    let mut inverse = FramePatch::default();
    {
        let bounds = match frame {
            Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
        };
        if patch.x.is_some() {
            inverse.x = Some(bounds.x);
        }
        if patch.y.is_some() {
            inverse.y = Some(bounds.y);
        }
        if patch.width.is_some() {
            inverse.width = Some(bounds.width);
        }
        if patch.height.is_some() {
            inverse.height = Some(bounds.height);
        }
        if let Some(value) = patch.x {
            bounds.x = value;
        }
        if let Some(value) = patch.y {
            bounds.y = value;
        }
        if let Some(value) = patch.width {
            bounds.width = value;
        }
        if let Some(value) = patch.height {
            bounds.height = value;
        }
    }
    match frame {
        Frame::Rect { fill, stroke, .. } => {
            if patch.fill.is_some() {
                inverse.fill = Some(*fill);
            }
            if patch.stroke.is_some() {
                inverse.stroke = Some(*stroke);
            }
            if let Some(new) = patch.fill {
                *fill = new;
            }
            if let Some(new) = patch.stroke {
                *stroke = new;
            }
        }
        Frame::Text { wrap_mode, columns, .. } => {
            if patch.wrap_mode.is_some() {
                inverse.wrap_mode = Some(wrap_mode.clone());
            }
            if patch.columns.is_some() {
                inverse.columns = Some(*columns);
            }
            if let Some(new) = &patch.wrap_mode {
                *wrap_mode = new.clone();
            }
            if let Some(new) = patch.columns {
                *columns = new;
            }
        }
        Frame::Image { .. } => {}
    }
    inverse
}
//#endregion 🔖Patches

//#region 🔖Op
/// 🧺 The typed layout document operation. Pages/stories/links are flat id-keyed collections; frames
/// are nested per-page so they get bespoke add/remove/patch variants; camera is a coalesced view op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum LayoutOp {
    Pages(CollectionOp<String, Page, PagePatch>),
    Stories(CollectionOp<String, TextStory, TextStoryPatch>),
    Links(CollectionOp<String, ImageLink, ImageLinkPatch>),
    AddFrame { page_id: String, index: usize, frame: Frame, layer_id: Option<String> },
    RemoveFrame { page_id: String, frame_id: String },
    PatchFrame { page_id: String, frame_id: String, patch: FramePatch },
    SetCamera { blueprint: bool, camera: LayoutCamera },
}

fn apply_layout_op(doc: &mut LayoutDocument, op: &LayoutOp) {
    match op {
        LayoutOp::Pages(cop) => apply_collection_op(&mut doc.pages, cop),
        LayoutOp::Stories(cop) => apply_collection_op(&mut doc.stories, cop),
        LayoutOp::Links(cop) => apply_collection_op(&mut doc.links, cop),
        LayoutOp::AddFrame { page_id, index, frame, layer_id } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                let at = (*index).min(page.frames.len());
                page.frames.insert(at, frame.clone());
                if let Some(layer_id) = layer_id {
                    if let Some(layer) = page.layers.iter_mut().find(|layer| layer.id == *layer_id) {
                        layer.object_ids.push(frame.id().to_string());
                    }
                }
            }
        }
        LayoutOp::RemoveFrame { page_id, frame_id } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                page.frames.retain(|frame| frame.id() != frame_id);
                for layer in &mut page.layers {
                    layer.object_ids.retain(|id| id != frame_id);
                }
            }
        }
        LayoutOp::PatchFrame { page_id, frame_id, patch } => {
            if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                if let Some(frame) = page.frames.iter_mut().find(|frame| frame.id() == frame_id) {
                    apply_frame_patch(frame, patch);
                }
            }
        }
        LayoutOp::SetCamera { blueprint, camera } => {
            if *blueprint {
                doc.camera = camera.clone();
            } else {
                doc.preview_camera = camera.clone();
            }
        }
    }
}

fn backwards_layout_op(doc: &LayoutDocument, op: &LayoutOp) -> Vec<LayoutOp> {
    match op {
        LayoutOp::Pages(cop) => vec![LayoutOp::Pages(invert_collection_op(&doc.pages, cop))],
        LayoutOp::Stories(cop) => vec![LayoutOp::Stories(invert_collection_op(&doc.stories, cop))],
        LayoutOp::Links(cop) => vec![LayoutOp::Links(invert_collection_op(&doc.links, cop))],
        LayoutOp::AddFrame { page_id, frame, .. } => {
            vec![LayoutOp::RemoveFrame { page_id: page_id.clone(), frame_id: frame.id().to_string() }]
        }
        LayoutOp::RemoveFrame { page_id, frame_id } => {
            if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                if let Some(index) = page.frames.iter().position(|frame| frame.id() == frame_id) {
                    let frame = page.frames[index].clone();
                    let layer_id = page
                        .layers
                        .iter()
                        .find(|layer| layer.object_ids.iter().any(|id| id == frame_id))
                        .map(|layer| layer.id.clone());
                    return vec![LayoutOp::AddFrame { page_id: page_id.clone(), index, frame, layer_id }];
                }
            }
            Vec::new()
        }
        LayoutOp::PatchFrame { page_id, frame_id, patch } => {
            if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                if let Some(frame) = page.frames.iter().find(|frame| frame.id() == frame_id) {
                    let mut clone = frame.clone();
                    let inverse = apply_frame_patch(&mut clone, patch);
                    return vec![LayoutOp::PatchFrame {
                        page_id: page_id.clone(),
                        frame_id: frame_id.clone(),
                        patch: inverse,
                    }];
                }
            }
            Vec::new()
        }
        LayoutOp::SetCamera { blueprint, .. } => vec![LayoutOp::SetCamera {
            blueprint: *blueprint,
            camera: if *blueprint { doc.camera.clone() } else { doc.preview_camera.clone() },
        }],
    }
}

/// 📦 Op-list diff: layout ops fold sequentially over a cloned projection. `absorb` concatenates —
/// coalesced camera drags stay one edit whose forwards replay to last-wins.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LayoutDiff {
    pub ops: Vec<LayoutOp>,
}

impl OperationDiff<LayoutDocument> for LayoutDiff {
    fn apply(&self, projection: &LayoutDocument) -> LayoutDocument {
        let mut next = projection.clone();
        for op in &self.ops {
            apply_layout_op(&mut next, op);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.ops.extend(other.ops);
    }
}

impl Operation<LayoutDocument> for LayoutOp {
    type Diff = LayoutDiff;

    fn diff(&self, _projection: &LayoutDocument) -> LayoutDiff {
        LayoutDiff { ops: vec![self.clone()] }
    }

    fn backwards(&self, projection: &LayoutDocument) -> Vec<Self> {
        backwards_layout_op(projection, self)
    }
}
//#endregion 🔖Op

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{parse_layout_document, Frame, LayoutBounds};

    const SAMPLE: &str = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[{"id":"story-1","content":"Hello","styleRuns":[]}],"links":[{"id":"link-1","path":"a.png","hash":"h","width":10,"height":10,"dpi":300}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}],"printTarget":null}"#;

    fn sample_doc() -> LayoutDocument {
        parse_layout_document(SAMPLE).expect("sample doc")
    }

    fn new_rect(id: &str) -> Frame {
        Frame::Rect {
            id: id.into(),
            layer_id: "layer-1".into(),
            bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 },
            locked: None,
            visible: None,
            fill: Some([0.1, 0.2, 0.3, 1.0]),
            stroke: None,
        }
    }

    fn round_trip(doc: &LayoutDocument, op: &LayoutOp) -> LayoutDocument {
        let forward = vcs::apply_operation(doc, op);
        let backs = op.backwards(doc);
        let mut restored = forward.clone();
        for back in &backs {
            restored = vcs::apply_operation(&restored, back);
        }
        assert_eq!(&restored, doc, "backwards must restore the pre-op document");
        forward
    }

    #[test]
    fn pages_add_and_patch_round_trip() {
        let doc = sample_doc();
        let mut page_2 = doc.pages[0].clone();
        page_2.id = "page-2".into();
        let add = LayoutOp::Pages(CollectionOp::Add { index: 1, item: page_2 });
        let with_page = round_trip(&doc, &add);
        assert_eq!(with_page.pages.len(), 2);

        let patch = LayoutOp::Pages(CollectionOp::Patch {
            id: "page-1".into(),
            patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() },
        });
        let patched = round_trip(&doc, &patch);
        let page = patched.pages.iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.name, "Renamed");
        assert_eq!(page.width, 300.0);
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    fn frame_add_remove_patch_round_trip() {
        let doc = sample_doc();
        let add = LayoutOp::AddFrame { page_id: "page-1".into(), index: 1, frame: new_rect("frame-2"), layer_id: Some("layer-1".into()) };
        let added = round_trip(&doc, &add);
        assert_eq!(added.pages[0].frames.len(), 2);
        assert!(added.pages[0].layers[0].object_ids.iter().any(|id| id == "frame-2"));

        let remove = LayoutOp::RemoveFrame { page_id: "page-1".into(), frame_id: "frame-1".into() };
        let removed = round_trip(&doc, &remove);
        assert!(removed.pages[0].frames.iter().all(|frame| frame.id() != "frame-1"));

        let patch = LayoutOp::PatchFrame {
            page_id: "page-1".into(),
            frame_id: "frame-1".into(),
            patch: FramePatch { x: Some(99.0), fill: Some(Some([0.5, 0.5, 0.5, 1.0])), ..Default::default() },
        };
        let patched = round_trip(&doc, &patch);
        let frame = patched.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
        assert_eq!(frame.bounds().x, 99.0);
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect") };
        assert_eq!(fill.unwrap(), [0.5, 0.5, 0.5, 1.0]);
    }

    #[test]
    fn story_and_link_patch_round_trip() {
        let doc = sample_doc();
        let story = LayoutOp::Stories(CollectionOp::Patch {
            id: "story-1".into(),
            patch: TextStoryPatch { content: Some("Edited".into()) },
        });
        let edited = round_trip(&doc, &story);
        assert_eq!(edited.stories[0].content, "Edited");

        let link = LayoutOp::Links(CollectionOp::Patch {
            id: "link-1".into(),
            patch: ImageLinkPatch { path: Some("b.png".into()) },
        });
        let relinked = round_trip(&doc, &link);
        assert_eq!(relinked.links[0].path, "b.png");
    }

    #[test]
    fn set_camera_round_trips_per_surface() {
        let doc = sample_doc();
        let op = LayoutOp::SetCamera { blueprint: true, camera: LayoutCamera { x: 5.0, y: 6.0, zoom: 2.0 } };
        let moved = round_trip(&doc, &op);
        assert_eq!(moved.camera.x, 5.0);
        assert_eq!(moved.preview_camera.x, 0.0);
    }
}
// #endregion ops
}

pub use document::*;
pub use display::*;
pub use engine::*;
pub use export::*;
pub use ops::*;

#[cfg(target_arch = "wasm32")]
mod wasm_session {
// #region wasm_session
use std::cell::RefCell;
use std::rc::Rc;

use infinite_cavas::camera::{self, Camera, Viewport};
use js_sys::Promise;
use infinite_cavas::Point;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::HtmlCanvasElement;

use crate::document::parse_layout_document;
use crate::engine::{build_scene_from_document_json, hit_test_document_json, screen_to_world_json, LayoutDropPreview};
use crate::export::{export_document_pdf, export_document_png_cpu, export_document_svg, export_package_zip};

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
    gpu: infinite_cavas::gpu_session::CanvasGpuSession,
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
                gpu: infinite_cavas::gpu_session::CanvasGpuSession::default(),
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
            let (render_ctx, renderer, surface) = infinite_cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                .await
                .map_err(|err| JsValue::from_str(&err))?;
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
        self.state.borrow_mut().drop_preview = Some(LayoutDropPreview {
            kind: kind.to_string(),
            x,
            y,
        });
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
        inner.interaction = LayoutInteraction::Pan {
            origin: inner.camera.clone(),
            start_screen: Point::new(sx, sy),
        };
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
        let query = SceneQuery {
            page_id: &inner.page_id,
            selected_ids: &inner.selected_ids,
            hovered_id: hovered,
            chrome_blueprint: inner.chrome_blueprint,
            camera: &inner.camera,
            viewport: &inner.viewport,
        };
        let scene = build_scene_from_document_json(&inner.document_json, &query, drop_preview.as_ref()).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let clear = infinite_cavas::theme::default_raster_clear();
        inner.gpu.render_frame(&scene, clear).map_err(|e| e)
    }

    #[wasm_bindgen(js_name = hitTest)]
    pub fn hit_test(&self, sx: f32, sy: f32) -> Result<JsValue, JsValue> {
        let inner = self.state.borrow();
        let hovered = inner.hovered_id.as_deref();
        let query = SceneQuery {
            page_id: &inner.page_id,
            selected_ids: &inner.selected_ids,
            hovered_id: hovered,
            chrome_blueprint: true,
            camera: &inner.camera,
            viewport: &inner.viewport,
        };
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
