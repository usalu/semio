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
                    frames.push(ResolvedFrame { frame: frame.clone(), inherited: !overridden });
                }
            }
        }
        for frame in &page.frames {
            frames.push(ResolvedFrame { frame: frame.clone(), inherited: false });
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
        vec![DisplayGuide { rect: LayoutRect { x: page.margins.left, y: page.margins.top, width: page.width - page.margins.left - page.margins.right, height: page.height - page.margins.top - page.margins.bottom }, kind: "margin".into() }]
    }

    pub fn bounds_to_display_rect(object_id: &str, bounds: &LayoutBounds, inherited: bool, selected: bool, hovered: bool, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> DisplayRect {
        DisplayRect { object_id: object_id.into(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, fill: fill.map(DisplayColor), stroke: stroke.map(DisplayColor), inherited, selected, hovered }
    }
    // #endregion display
}

mod engine {
    // #region engine
    use std::borrow::Cow;
    use std::sync::{Arc, OnceLock};

    use fontique::Blob;
    use infinite_cavas::camera::{self, Camera, Viewport};
    use infinite_cavas::{Affine, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};
    use parley::{Alignment, AlignmentOptions, FontContext, FontStack, FontWeight, Layout, LayoutContext, LineHeight, PositionedLayoutItem, StyleProperty};

    use crate::display::{bounds_to_display_rect, page_margin_guides, DisplayColor, DisplayGlyph, DisplayGuide, DisplayImage, DisplayList, DisplayTextRun};
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
                guides.push(DisplayGuide { rect: crate::document::LayoutRect { x, y: page.margins.top, width: col_width, height: page.height - page.margins.top - page.margins.bottom }, kind: "column".into() });
            }
            if doc.grid.snap_to_baseline && doc.grid.baseline_grid > 0.0 {
                let mut y = doc.grid.baseline_offset;
                while y < page.height {
                    guides.push(DisplayGuide { rect: crate::document::LayoutRect { x: 0.0, y, width: page.width, height: 0.0 }, kind: "baseline".into() });
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

    pub fn build_scene_from_document_json(json: &str, query: &SceneQuery, drop_preview: Option<&LayoutDropPreview>) -> Result<Scene, crate::LayoutError> {
        let doc = parse_layout_document(json)?;
        let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| crate::LayoutError::PageNotFound(query.page_id.to_string()))?;
        let list = build_display_list_for_page(&doc, page, query.page_id, query.selected_ids, query.hovered_id, query.chrome_blueprint);
        Ok(display_list_to_scene(&list, query.chrome_blueprint, query.camera, query.viewport, drop_preview))
    }

    pub fn hit_test_document_json(json: &str, sx: f64, sy: f64, query: &SceneQuery) -> Result<Option<String>, crate::LayoutError> {
        let doc = parse_layout_document(json)?;
        let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| crate::LayoutError::PageNotFound(query.page_id.to_string()))?;
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

    pub fn export_document_png_cpu(doc: &LayoutDocument, page_id: &str) -> Result<Vec<u8>, crate::LayoutError> {
        let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| crate::LayoutError::PageNotFound(page_id.to_string()))?;
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

    pub fn scene_png_from_display_list(list: &DisplayList) -> Result<Vec<u8>, crate::LayoutError> {
        let camera = infinite_cavas::camera::Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = infinite_cavas::camera::Viewport { width: list.page_width.max(1.0) as u32, height: list.page_height.max(1.0) as u32, dpr: 1.0 };
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
        use crate::document::LAYOUT_FIXTURE_SCHEMA;
        use vcs::DocumentDsl;

        fn sample_document() -> LayoutDocument {
            LayoutDocument::parse_dsl(include_str!("../example/sample.layout")).expect("sample fixture parses")
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
    }
    // #endregion export
}

mod operations {
    // #region operations
    //! 🧾 Typed VCS operation vocabulary for the layout document — the operations the layout plugin emits
    //! (page/story/link collections, per-page frame add/remove/patch, and camera). Each operation computes a
    //! true pre-state inverse so undo/redo round-trips exactly. See {@link vcs::Operation}.

    use serde::{Deserialize, Serialize};
    use vcs::{apply_collection_operation, invert_collection_operation, CollectionOperation, Identified, Operation, OperationDiff, Patchable};

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

    //#region 🔖Operation
    /// 🧺 The typed layout document operation. Pages/stories/links are flat id-keyed collections; frames
    /// are nested per-page so they get bespoke add/remove/patch variants; camera is a coalesced view operation.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "operation", rename_all = "camelCase")]
    pub enum LayoutOperation {
        Pages(CollectionOperation<String, Page, PagePatch>),
        Stories(CollectionOperation<String, TextStory, TextStoryPatch>),
        Links(CollectionOperation<String, ImageLink, ImageLinkPatch>),
        AddFrame { page_id: String, index: usize, frame: Frame, layer_id: Option<String> },
        RemoveFrame { page_id: String, frame_id: String },
        PatchFrame { page_id: String, frame_id: String, patch: FramePatch },
        SetCamera { blueprint: bool, camera: LayoutCamera },
    }

    fn apply_layout_operation(doc: &mut LayoutDocument, operation: &LayoutOperation) {
        match operation {
            LayoutOperation::Pages(cop) => apply_collection_operation(&mut doc.pages, cop),
            LayoutOperation::Stories(cop) => apply_collection_operation(&mut doc.stories, cop),
            LayoutOperation::Links(cop) => apply_collection_operation(&mut doc.links, cop),
            LayoutOperation::AddFrame { page_id, index, frame, layer_id } => {
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
            LayoutOperation::RemoveFrame { page_id, frame_id } => {
                if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                    page.frames.retain(|frame| frame.id() != frame_id);
                    for layer in &mut page.layers {
                        layer.object_ids.retain(|id| id != frame_id);
                    }
                }
            }
            LayoutOperation::PatchFrame { page_id, frame_id, patch } => {
                if let Some(page) = doc.pages.iter_mut().find(|page| page.id == *page_id) {
                    if let Some(frame) = page.frames.iter_mut().find(|frame| frame.id() == frame_id) {
                        apply_frame_patch(frame, patch);
                    }
                }
            }
            LayoutOperation::SetCamera { blueprint, camera } => {
                if *blueprint {
                    doc.camera = camera.clone();
                } else {
                    doc.preview_camera = camera.clone();
                }
            }
        }
    }

    fn backwards_layout_operation(doc: &LayoutDocument, operation: &LayoutOperation) -> Vec<LayoutOperation> {
        match operation {
            LayoutOperation::Pages(cop) => vec![LayoutOperation::Pages(invert_collection_operation(&doc.pages, cop))],
            LayoutOperation::Stories(cop) => vec![LayoutOperation::Stories(invert_collection_operation(&doc.stories, cop))],
            LayoutOperation::Links(cop) => vec![LayoutOperation::Links(invert_collection_operation(&doc.links, cop))],
            LayoutOperation::AddFrame { page_id, frame, .. } => {
                vec![LayoutOperation::RemoveFrame { page_id: page_id.clone(), frame_id: frame.id().to_string() }]
            }
            LayoutOperation::RemoveFrame { page_id, frame_id } => {
                if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                    if let Some(index) = page.frames.iter().position(|frame| frame.id() == frame_id) {
                        let frame = page.frames[index].clone();
                        let layer_id = page.layers.iter().find(|layer| layer.object_ids.iter().any(|id| id == frame_id)).map(|layer| layer.id.clone());
                        return vec![LayoutOperation::AddFrame { page_id: page_id.clone(), index, frame, layer_id }];
                    }
                }
                Vec::new()
            }
            LayoutOperation::PatchFrame { page_id, frame_id, patch } => {
                if let Some(page) = doc.pages.iter().find(|page| page.id == *page_id) {
                    if let Some(frame) = page.frames.iter().find(|frame| frame.id() == frame_id) {
                        let mut clone = frame.clone();
                        let inverse = apply_frame_patch(&mut clone, patch);
                        return vec![LayoutOperation::PatchFrame { page_id: page_id.clone(), frame_id: frame_id.clone(), patch: inverse }];
                    }
                }
                Vec::new()
            }
            LayoutOperation::SetCamera { blueprint, .. } => vec![LayoutOperation::SetCamera { blueprint: *blueprint, camera: if *blueprint { doc.camera.clone() } else { doc.preview_camera.clone() } }],
        }
    }

    /// 📦 Operation-list diff: layout operations fold sequentially over a cloned projection. `absorb` concatenates —
    /// coalesced camera drags stay one edit whose forwards replay to last-wins.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct LayoutDiff {
        pub operations: Vec<LayoutOperation>,
    }

    impl OperationDiff<LayoutDocument> for LayoutDiff {
        fn apply(&self, projection: &LayoutDocument) -> LayoutDocument {
            let mut next = projection.clone();
            for operation in &self.operations {
                apply_layout_operation(&mut next, operation);
            }
            next
        }

        fn absorb(&mut self, other: Self) {
            self.operations.extend(other.operations);
        }
    }

    impl Operation<LayoutDocument> for LayoutOperation {
        type Diff = LayoutDiff;

        fn diff(&self, _projection: &LayoutDocument) -> LayoutDiff {
            LayoutDiff { operations: vec![self.clone()] }
        }

        fn backwards(&self, projection: &LayoutDocument) -> Vec<Self> {
            backwards_layout_operation(projection, self)
        }
    }
    //#endregion 🔖Operation

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::document::{parse_layout_document, Frame, LayoutBounds};

        const SAMPLE: &str = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[{"id":"story-1","content":"Hello","styleRuns":[]}],"links":[{"id":"link-1","path":"a.png","hash":"h","width":10,"height":10,"dpi":300}],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}],"printTarget":null}"#;

        fn sample_doc() -> LayoutDocument {
            parse_layout_document(SAMPLE).expect("sample doc")
        }

        fn new_rect(id: &str) -> Frame {
            Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 }, locked: None, visible: None, fill: Some([0.1, 0.2, 0.3, 1.0]), stroke: None }
        }

        fn round_trip(doc: &LayoutDocument, operation: &LayoutOperation) -> LayoutDocument {
            let forward = vcs::apply_operation(doc, operation);
            let backs = operation.backwards(doc);
            let mut restored = forward.clone();
            for back in &backs {
                restored = vcs::apply_operation(&restored, back);
            }
            assert_eq!(&restored, doc, "backwards must restore the pre-operation document");
            forward
        }

        #[test]
        fn pages_add_and_patch_round_trip() {
            let doc = sample_doc();
            let mut page_2 = doc.pages[0].clone();
            page_2.id = "page-2".into();
            let add = LayoutOperation::Pages(CollectionOperation::Add { index: 1, item: page_2 });
            let with_page = round_trip(&doc, &add);
            assert_eq!(with_page.pages.len(), 2);

            let patch = LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() } });
            let patched = round_trip(&doc, &patch);
            let page = patched.pages.iter().find(|page| page.id == "page-1").unwrap();
            assert_eq!(page.name, "Renamed");
            assert_eq!(page.width, 300.0);
            assert_eq!(page.columns.count, 3);
        }

        #[test]
        fn frame_add_remove_patch_round_trip() {
            let doc = sample_doc();
            let add = LayoutOperation::AddFrame { page_id: "page-1".into(), index: 1, frame: new_rect("frame-2"), layer_id: Some("layer-1".into()) };
            let added = round_trip(&doc, &add);
            assert_eq!(added.pages[0].frames.len(), 2);
            assert!(added.pages[0].layers[0].object_ids.iter().any(|id| id == "frame-2"));

            let remove = LayoutOperation::RemoveFrame { page_id: "page-1".into(), frame_id: "frame-1".into() };
            let removed = round_trip(&doc, &remove);
            assert!(removed.pages[0].frames.iter().all(|frame| frame.id() != "frame-1"));

            let patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), patch: FramePatch { x: Some(99.0), fill: Some(Some([0.5, 0.5, 0.5, 1.0])), ..Default::default() } };
            let patched = round_trip(&doc, &patch);
            let frame = patched.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
            assert_eq!(frame.bounds().x, 99.0);
            let Frame::Rect { fill, .. } = frame else { panic!("expected rect") };
            assert_eq!(fill.unwrap(), [0.5, 0.5, 0.5, 1.0]);
        }

        #[test]
        fn story_and_link_patch_round_trip() {
            let doc = sample_doc();
            let story = LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: Some("Edited".into()) } });
            let edited = round_trip(&doc, &story);
            assert_eq!(edited.stories[0].content, "Edited");

            let link = LayoutOperation::Links(CollectionOperation::Patch { id: "link-1".into(), patch: ImageLinkPatch { path: Some("b.png".into()) } });
            let relinked = round_trip(&doc, &link);
            assert_eq!(relinked.links[0].path, "b.png");
        }

        #[test]
        fn set_camera_round_trips_per_surface() {
            let doc = sample_doc();
            let operation = LayoutOperation::SetCamera { blueprint: true, camera: LayoutCamera { x: 5.0, y: 6.0, zoom: 2.0 } };
            let moved = round_trip(&doc, &operation);
            assert_eq!(moved.camera.x, 5.0);
            assert_eq!(moved.preview_camera.x, 0.0);
        }
    }
    // #endregion operations
}

mod dsl {
    //#region 🔖Dsl
    //! 🔤 Handcrafted textual DSL for `LayoutDocument` (`vcs::DocumentDsl`) and one-line op-text for
    //! `LayoutOperation` (`vcs::OpText`, see `🔖OpText`) — replaces the JSON fixture format. Grammar is a
    //! small hand-rolled tokenizer + recursive-descent parser (no external parser crate), in the spirit of
    //! `mathematical_graph_dsl::wire` and `draw_rs`'s own `🔖Dsl` region. Nested values (bounds/margins/
    //! columns/colors as `( )` tuples, id lists as `[ ]`, structured lists as `[ {..} {..} ]`, pages/
    //! layers/frames as `{ }` blocks) are self-delimiting and never contain a literal newline, so the exact
    //! same printer/parser pair works whether chunks are newline-joined (pretty `print_dsl`) or embedded
    //! inline in a one-line op (`🔖OpText`).

    use crate::document::*;
    use crate::operations::*;
    use vcs::{CollectionOperation, DocumentDsl, OpText, TextError, TextSpan};

    //#region 🔖DslLexer
    #[derive(Clone, Debug, PartialEq)]
    enum LayoutTok {
        Ident(String),
        Str(String),
        Num(f64),
        Eq,
        Colon,
        Comma,
        LParen,
        RParen,
        LBracket,
        RBracket,
        LBrace,
        RBrace,
        Eof,
    }

    #[derive(Clone, Debug)]
    struct LayoutSpannedTok {
        tok: LayoutTok,
        line: u32,
        column: u32,
    }

    /// 🔍 Hand-rolled char-by-char tokenizer for the layout DSL/op-text grammar; tracks line/column so
    /// parse errors carry a `TextSpan` a dev can jump to.
    fn lex_layout_dsl(input: &str) -> Result<Vec<LayoutSpannedTok>, TextError> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        let mut line: u32 = 1;
        let mut col: u32 = 1;
        while i < chars.len() {
            let c = chars[i];
            if c == '\n' {
                i += 1;
                line += 1;
                col = 1;
                continue;
            }
            if c.is_whitespace() {
                i += 1;
                col += 1;
                continue;
            }
            let (start_line, start_col) = (line, col);
            match c {
                '=' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::Eq, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                ':' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::Colon, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                ',' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::Comma, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '(' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::LParen, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                ')' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::RParen, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '[' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::LBracket, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                ']' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::RBracket, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '{' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::LBrace, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(LayoutSpannedTok { tok: LayoutTok::RBrace, line: start_line, column: start_col });
                    i += 1;
                    col += 1;
                }
                '"' => {
                    i += 1;
                    col += 1;
                    let mut value = String::new();
                    loop {
                        if i >= chars.len() {
                            return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
                        }
                        let ch = chars[i];
                        if ch == '"' {
                            i += 1;
                            col += 1;
                            break;
                        }
                        if ch == '\\' && i + 1 < chars.len() {
                            match chars[i + 1] {
                                'n' => value.push('\n'),
                                '"' => value.push('"'),
                                '\\' => value.push('\\'),
                                other => value.push(other),
                            }
                            i += 2;
                            col += 2;
                        } else if ch == '\n' {
                            value.push('\n');
                            i += 1;
                            line += 1;
                            col = 1;
                        } else {
                            value.push(ch);
                            i += 1;
                            col += 1;
                        }
                    }
                    out.push(LayoutSpannedTok { tok: LayoutTok::Str(value), line: start_line, column: start_col });
                }
                '-' | '0'..='9' => {
                    let start = i;
                    if c == '-' {
                        i += 1;
                        col += 1;
                    }
                    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        i += 1;
                        col += 1;
                    }
                    if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                        i += 1;
                        col += 1;
                        if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                            i += 1;
                            col += 1;
                        }
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            i += 1;
                            col += 1;
                        }
                    }
                    let text: String = chars[start..i].iter().collect();
                    let value: f64 = text.parse().map_err(|_| TextError::new(format!("invalid number '{text}'"), TextSpan::at(start_line, start_col)))?;
                    out.push(LayoutSpannedTok { tok: LayoutTok::Num(value), line: start_line, column: start_col });
                }
                other if other.is_ascii_alphabetic() || other == '_' => {
                    let start = i;
                    while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '-' || chars[i] == '.') {
                        i += 1;
                        col += 1;
                    }
                    let text: String = chars[start..i].iter().collect();
                    out.push(LayoutSpannedTok { tok: LayoutTok::Ident(text), line: start_line, column: start_col });
                }
                other => return Err(TextError::new(format!("unexpected character '{other}'"), TextSpan::at(start_line, start_col))),
            }
        }
        out.push(LayoutSpannedTok { tok: LayoutTok::Eof, line, column: col });
        Ok(out)
    }

    /// 🔐 Escapes `\`, `"` and newlines for embedding a string inside a `"..."` DSL literal.
    fn escape_str(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out
    }

    /// 🔢 Prints an `f64` via its shortest round-trippable `Display` form, named for call-site clarity
    /// next to `escape_str`.
    fn fmt_num(value: f64) -> String {
        value.to_string()
    }
    //#endregion 🔖DslLexer

    //#region 🔖DslParser
    struct LayoutDslParser {
        toks: Vec<LayoutSpannedTok>,
        pos: usize,
    }

    impl LayoutDslParser {
        fn new(toks: Vec<LayoutSpannedTok>) -> Self {
            Self { toks, pos: 0 }
        }

        fn peek(&self) -> &LayoutTok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> TextSpan {
            let tok = &self.toks[self.pos];
            TextSpan::at(tok.line, tok.column)
        }

        fn bump(&mut self) -> LayoutTok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn expect_tok(&mut self, expected: &LayoutTok, label: &str) -> Result<(), TextError> {
            let span = self.span();
            let got = self.bump();
            if &got == expected {
                Ok(())
            } else {
                Err(TextError::expected(format!("expected '{label}'"), span, format!("{got:?}")))
            }
        }

        fn expect_ident(&mut self) -> Result<String, TextError> {
            let span = self.span();
            match self.bump() {
                LayoutTok::Ident(value) => Ok(value),
                other => Err(TextError::expected("expected identifier", span, format!("{other:?}"))),
            }
        }

        fn expect_str(&mut self) -> Result<String, TextError> {
            let span = self.span();
            match self.bump() {
                LayoutTok::Str(value) => Ok(value),
                other => Err(TextError::expected("expected string literal", span, format!("{other:?}"))),
            }
        }

        fn expect_num(&mut self) -> Result<f64, TextError> {
            let span = self.span();
            match self.bump() {
                LayoutTok::Num(value) => Ok(value),
                other => Err(TextError::expected("expected number", span, format!("{other:?}"))),
            }
        }

        fn expect_bool(&mut self) -> Result<bool, TextError> {
            let span = self.span();
            match self.bump() {
                LayoutTok::Ident(value) if value == "true" => Ok(true),
                LayoutTok::Ident(value) if value == "false" => Ok(false),
                other => Err(TextError::expected("expected 'true' or 'false'", span, format!("{other:?}"))),
            }
        }

        fn at_ident(&self, value: &str) -> bool {
            matches!(self.peek(), LayoutTok::Ident(candidate) if candidate == value)
        }

        fn peek_at(&self, offset: usize) -> &LayoutTok {
            let idx = (self.pos + offset).min(self.toks.len() - 1);
            &self.toks[idx].tok
        }

        fn eat_keyword(&mut self, value: &str) -> Result<(), TextError> {
            let span = self.span();
            match self.bump() {
                LayoutTok::Ident(candidate) if candidate == value => Ok(()),
                other => Err(TextError::expected(format!("expected '{value}'"), span, format!("{other:?}"))),
            }
        }

        /// 🔎 True when the parser sits at the start of a `key=value` attribute (one token of lookahead
        /// past the identifier) — the signal every sparse-patch attr-loop uses to decide whether to keep
        /// consuming optional fields.
        fn at_attr(&self) -> bool {
            matches!(self.peek(), LayoutTok::Ident(_)) && matches!(self.peek_at(1), LayoutTok::Eq)
        }

        fn peek_attr_key(&self) -> Option<&str> {
            if let LayoutTok::Ident(value) = self.peek() {
                Some(value.as_str())
            } else {
                None
            }
        }
    }

    /// 🏷️ Consumes `key=` (assumes `at_attr()` already confirmed the shape) and returns `key`.
    fn take_attr_key(p: &mut LayoutDslParser) -> Result<String, TextError> {
        let key = p.expect_ident()?;
        p.expect_tok(&LayoutTok::Eq, "=")?;
        Ok(key)
    }

    /// 🏷️ Consumes `expected=` and errors if the attribute name doesn't match.
    fn expect_key(p: &mut LayoutDslParser, expected: &str) -> Result<(), TextError> {
        let span = p.span();
        let key = take_attr_key(p)?;
        if key == expected {
            Ok(())
        } else {
            Err(TextError::expected(format!("expected key '{expected}'"), span, key))
        }
    }

    /// 🏷️ Consumes `expected:` — the field-label marker used before an inline nested value (a whole
    /// `Page`/`TextStory`/`ImageLink`/frame) embedded in a one-line op, decoupled from the value's own
    /// self-delimiting grammar so the label never collides with the value's leading token.
    fn expect_key_colon(p: &mut LayoutDslParser, expected: &str) -> Result<(), TextError> {
        let span = p.span();
        let key = p.expect_ident()?;
        if key != expected {
            return Err(TextError::expected(format!("expected '{expected}:'"), span, key));
        }
        p.expect_tok(&LayoutTok::Colon, ":")?;
        Ok(())
    }

    fn parse_kv_ident(p: &mut LayoutDslParser, key: &str) -> Result<String, TextError> {
        expect_key(p, key)?;
        p.expect_ident()
    }

    fn parse_kv_str(p: &mut LayoutDslParser, key: &str) -> Result<String, TextError> {
        expect_key(p, key)?;
        p.expect_str()
    }

    fn parse_kv_num(p: &mut LayoutDslParser, key: &str) -> Result<f64, TextError> {
        expect_key(p, key)?;
        p.expect_num()
    }

    fn parse_kv_bool(p: &mut LayoutDslParser, key: &str) -> Result<bool, TextError> {
        expect_key(p, key)?;
        p.expect_bool()
    }

    /// 🕳️ `none` sentinel for an absent `Option<String>` identifier field (ids, `threadNext`, …).
    fn parse_opt_ident(p: &mut LayoutDslParser) -> Result<Option<String>, TextError> {
        if p.at_ident("none") {
            p.bump();
            return Ok(None);
        }
        Ok(Some(p.expect_ident()?))
    }

    fn parse_kv_opt_ident(p: &mut LayoutDslParser, key: &str) -> Result<Option<String>, TextError> {
        expect_key(p, key)?;
        parse_opt_ident(p)
    }

    fn print_opt_ident(value: &Option<String>) -> String {
        value.clone().unwrap_or_else(|| "none".to_string())
    }

    /// 🕳️ `none` sentinel for an absent `Option<String>` free-text field (paths, hashes, labels, …).
    fn parse_opt_str(p: &mut LayoutDslParser) -> Result<Option<String>, TextError> {
        if p.at_ident("none") {
            p.bump();
            return Ok(None);
        }
        Ok(Some(p.expect_str()?))
    }

    fn parse_kv_opt_str(p: &mut LayoutDslParser, key: &str) -> Result<Option<String>, TextError> {
        expect_key(p, key)?;
        parse_opt_str(p)
    }

    fn print_opt_str(value: &Option<String>) -> String {
        match value {
            Some(s) => format!("\"{}\"", escape_str(s)),
            None => "none".to_string(),
        }
    }

    /// 🕳️ `none` sentinel for an absent `Option<bool>` field (`locked`/`visible` on frames/overrides).
    fn parse_opt_bool(p: &mut LayoutDslParser) -> Result<Option<bool>, TextError> {
        if p.at_ident("none") {
            p.bump();
            return Ok(None);
        }
        Ok(Some(p.expect_bool()?))
    }

    fn parse_kv_opt_bool(p: &mut LayoutDslParser, key: &str) -> Result<Option<bool>, TextError> {
        expect_key(p, key)?;
        parse_opt_bool(p)
    }

    fn print_opt_bool(value: &Option<bool>) -> String {
        match value {
            Some(true) => "true".to_string(),
            Some(false) => "false".to_string(),
            None => "none".to_string(),
        }
    }
    //#endregion 🔖DslParser

    //#region 🔖DslValues
    /// 🔢 Reads `(n0,n1,...,n{count-1})` — the shared tuple grammar backing bounds/margins/columns/color.
    fn parse_num_tuple(p: &mut LayoutDslParser, count: usize) -> Result<Vec<f64>, TextError> {
        p.expect_tok(&LayoutTok::LParen, "(")?;
        let mut values = Vec::with_capacity(count);
        for i in 0..count {
            if i > 0 {
                p.expect_tok(&LayoutTok::Comma, ",")?;
            }
            values.push(p.expect_num()?);
        }
        p.expect_tok(&LayoutTok::RParen, ")")?;
        Ok(values)
    }

    fn print_num_tuple(values: &[f64]) -> String {
        format!("({})", values.iter().map(|v| fmt_num(*v)).collect::<Vec<_>>().join(","))
    }

    fn parse_bounds(p: &mut LayoutDslParser) -> Result<LayoutBounds, TextError> {
        let v = parse_num_tuple(p, 5)?;
        Ok(LayoutBounds { x: v[0], y: v[1], width: v[2], height: v[3], rotation: v[4] })
    }

    fn print_bounds(b: &LayoutBounds) -> String {
        print_num_tuple(&[b.x, b.y, b.width, b.height, b.rotation])
    }

    fn parse_opt_bounds(p: &mut LayoutDslParser) -> Result<Option<LayoutBounds>, TextError> {
        if p.at_ident("none") {
            p.bump();
            return Ok(None);
        }
        Ok(Some(parse_bounds(p)?))
    }

    fn print_opt_bounds(b: &Option<LayoutBounds>) -> String {
        match b {
            Some(v) => print_bounds(v),
            None => "none".to_string(),
        }
    }

    fn parse_margins(p: &mut LayoutDslParser) -> Result<PageMargins, TextError> {
        let v = parse_num_tuple(p, 4)?;
        Ok(PageMargins { top: v[0], right: v[1], bottom: v[2], left: v[3] })
    }

    fn print_margins(m: &PageMargins) -> String {
        print_num_tuple(&[m.top, m.right, m.bottom, m.left])
    }

    fn parse_columns(p: &mut LayoutDslParser) -> Result<PageColumns, TextError> {
        let v = parse_num_tuple(p, 2)?;
        Ok(PageColumns { count: v[0] as u32, gutter: v[1] })
    }

    fn print_columns(c: &PageColumns) -> String {
        print_num_tuple(&[c.count as f64, c.gutter])
    }

    fn parse_rect(p: &mut LayoutDslParser) -> Result<LayoutRect, TextError> {
        let v = parse_num_tuple(p, 4)?;
        Ok(LayoutRect { x: v[0], y: v[1], width: v[2], height: v[3] })
    }

    fn print_rect(r: &LayoutRect) -> String {
        print_num_tuple(&[r.x, r.y, r.width, r.height])
    }

    fn parse_color4(p: &mut LayoutDslParser) -> Result<[f32; 4], TextError> {
        let v = parse_num_tuple(p, 4)?;
        Ok([v[0] as f32, v[1] as f32, v[2] as f32, v[3] as f32])
    }

    fn print_color4(c: &[f32; 4]) -> String {
        print_num_tuple(&[c[0] as f64, c[1] as f64, c[2] as f64, c[3] as f64])
    }

    fn parse_opt_color4(p: &mut LayoutDslParser) -> Result<Option<[f32; 4]>, TextError> {
        if p.at_ident("none") {
            p.bump();
            return Ok(None);
        }
        Ok(Some(parse_color4(p)?))
    }

    fn print_opt_color4(c: &Option<[f32; 4]>) -> String {
        match c {
            Some(v) => print_color4(v),
            None => "none".to_string(),
        }
    }

    fn parse_camera_fields(p: &mut LayoutDslParser) -> Result<LayoutCamera, TextError> {
        let x = parse_kv_num(p, "x")?;
        let y = parse_kv_num(p, "y")?;
        let zoom = parse_kv_num(p, "zoom")?;
        Ok(LayoutCamera { x, y, zoom })
    }

    fn print_camera_line(prefix: &str, c: &LayoutCamera) -> String {
        format!("{prefix} x={} y={} zoom={}", fmt_num(c.x), fmt_num(c.y), fmt_num(c.zoom))
    }

    /// ⚡ Tuple-form `LayoutCamera` used inline in `setCamera` op-text (the pretty top-level `camera`/
    /// `previewCamera` lines use the attr-style `parse_camera_fields`/`print_camera_line` instead).
    fn parse_camera_tuple(p: &mut LayoutDslParser) -> Result<LayoutCamera, TextError> {
        let v = parse_num_tuple(p, 3)?;
        Ok(LayoutCamera { x: v[0], y: v[1], zoom: v[2] })
    }

    fn print_camera_tuple(c: &LayoutCamera) -> String {
        print_num_tuple(&[c.x, c.y, c.zoom])
    }

    /// 🕳️ Parses `[id,id,...]` (or `[]`) — the shared grammar for `layerIds`/`objectIds`/`pageIds`.
    fn parse_id_list(p: &mut LayoutDslParser) -> Result<Vec<String>, TextError> {
        p.expect_tok(&LayoutTok::LBracket, "[")?;
        let mut ids = Vec::new();
        if !matches!(p.peek(), LayoutTok::RBracket) {
            loop {
                ids.push(p.expect_ident()?);
                if matches!(p.peek(), LayoutTok::Comma) {
                    p.bump();
                } else {
                    break;
                }
            }
        }
        p.expect_tok(&LayoutTok::RBracket, "]")?;
        Ok(ids)
    }

    fn print_id_list(ids: &[String]) -> String {
        format!("[{}]", ids.join(","))
    }

    fn parse_rect_list(p: &mut LayoutDslParser) -> Result<Vec<LayoutRect>, TextError> {
        p.expect_tok(&LayoutTok::LBracket, "[")?;
        let mut rects = Vec::new();
        if !matches!(p.peek(), LayoutTok::RBracket) {
            loop {
                rects.push(parse_rect(p)?);
                if matches!(p.peek(), LayoutTok::Comma) {
                    p.bump();
                } else {
                    break;
                }
            }
        }
        p.expect_tok(&LayoutTok::RBracket, "]")?;
        Ok(rects)
    }

    fn print_rect_list(rects: &[LayoutRect]) -> String {
        format!("[{}]", rects.iter().map(print_rect).collect::<Vec<_>>().join(","))
    }

    fn parse_style_run(p: &mut LayoutDslParser) -> Result<TextStyleRun, TextError> {
        p.expect_tok(&LayoutTok::LBrace, "{")?;
        let start = parse_kv_num(p, "start")? as usize;
        let end = parse_kv_num(p, "end")? as usize;
        let paragraph_style_id = parse_kv_opt_ident(p, "paragraphStyleId")?;
        let character_style_id = parse_kv_opt_ident(p, "characterStyleId")?;
        p.expect_tok(&LayoutTok::RBrace, "}")?;
        Ok(TextStyleRun { start, end, paragraph_style_id, character_style_id })
    }

    fn print_style_run(r: &TextStyleRun) -> String {
        format!(
            "{{start={} end={} paragraphStyleId={} characterStyleId={}}}",
            r.start,
            r.end,
            print_opt_ident(&r.paragraph_style_id),
            print_opt_ident(&r.character_style_id)
        )
    }

    fn parse_style_runs(p: &mut LayoutDslParser) -> Result<Vec<TextStyleRun>, TextError> {
        p.expect_tok(&LayoutTok::LBracket, "[")?;
        let mut runs = Vec::new();
        while !matches!(p.peek(), LayoutTok::RBracket) {
            runs.push(parse_style_run(p)?);
        }
        p.expect_tok(&LayoutTok::RBracket, "]")?;
        Ok(runs)
    }

    fn print_style_runs(runs: &[TextStyleRun]) -> String {
        format!("[{}]", runs.iter().map(print_style_run).collect::<Vec<_>>().join(" "))
    }

    fn parse_override(p: &mut LayoutDslParser) -> Result<PageOverride, TextError> {
        p.expect_tok(&LayoutTok::LBrace, "{")?;
        let object_id = parse_kv_ident(p, "objectId")?;
        expect_key(p, "bounds")?;
        let bounds = parse_opt_bounds(p)?;
        let visible = parse_kv_opt_bool(p, "visible")?;
        let locked = parse_kv_opt_bool(p, "locked")?;
        p.expect_tok(&LayoutTok::RBrace, "}")?;
        Ok(PageOverride { object_id, bounds, visible, locked })
    }

    fn print_override(o: &PageOverride) -> String {
        format!("{{objectId={} bounds={} visible={} locked={}}}", o.object_id, print_opt_bounds(&o.bounds), print_opt_bool(&o.visible), print_opt_bool(&o.locked))
    }

    fn parse_overrides(p: &mut LayoutDslParser) -> Result<Vec<PageOverride>, TextError> {
        p.expect_tok(&LayoutTok::LBracket, "[")?;
        let mut overrides = Vec::new();
        while !matches!(p.peek(), LayoutTok::RBracket) {
            overrides.push(parse_override(p)?);
        }
        p.expect_tok(&LayoutTok::RBracket, "]")?;
        Ok(overrides)
    }

    fn print_overrides(overrides: &[PageOverride]) -> String {
        format!("[{}]", overrides.iter().map(print_override).collect::<Vec<_>>().join(" "))
    }
    //#endregion 🔖DslValues

    //#region 🔖DslEntities
    fn parse_paragraph_style(p: &mut LayoutDslParser) -> Result<ParagraphStyle, TextError> {
        p.eat_keyword("paragraphStyle")?;
        let id = parse_kv_ident(p, "id")?;
        let name = parse_kv_str(p, "name")?;
        let font_family = parse_kv_str(p, "fontFamily")?;
        let font_size = parse_kv_num(p, "fontSize")?;
        let font_weight = parse_kv_num(p, "fontWeight")? as u32;
        let leading = parse_kv_num(p, "leading")?;
        let tracking = parse_kv_num(p, "tracking")?;
        let alignment = parse_kv_str(p, "alignment")?;
        Ok(ParagraphStyle { id, name, font_family, font_size, font_weight, leading, tracking, alignment })
    }

    fn print_paragraph_style(s: &ParagraphStyle) -> String {
        format!(
            "paragraphStyle id={} name=\"{}\" fontFamily=\"{}\" fontSize={} fontWeight={} leading={} tracking={} alignment=\"{}\"",
            s.id,
            escape_str(&s.name),
            escape_str(&s.font_family),
            fmt_num(s.font_size),
            s.font_weight,
            fmt_num(s.leading),
            fmt_num(s.tracking),
            escape_str(&s.alignment)
        )
    }

    /// 🌫️ `characterStyles` is an untyped `Vec<serde_json::Value>` on `LayoutDocument` (no fixed schema
    /// yet) — round-tripped as an opaque JSON blob embedded in a quoted DSL string rather than inventing
    /// grammar for a shape nothing constrains.
    fn parse_character_style(p: &mut LayoutDslParser) -> Result<serde_json::Value, TextError> {
        p.eat_keyword("characterStyle")?;
        let span = p.span();
        let raw = parse_kv_str(p, "json")?;
        serde_json::from_str(&raw).map_err(|error| TextError::new(format!("invalid characterStyle json: {error}"), span))
    }

    fn print_character_style(v: &serde_json::Value) -> String {
        let raw = serde_json::to_string(v).unwrap_or_else(|_| "null".to_string());
        format!("characterStyle json=\"{}\"", escape_str(&raw))
    }

    fn parse_story_fields(p: &mut LayoutDslParser) -> Result<TextStory, TextError> {
        let id = parse_kv_ident(p, "id")?;
        let content = parse_kv_str(p, "content")?;
        expect_key(p, "styleRuns")?;
        let style_runs = parse_style_runs(p)?;
        Ok(TextStory { id, content, style_runs })
    }

    fn print_story_fields(s: &TextStory) -> String {
        format!("id={} content=\"{}\" styleRuns={}", s.id, escape_str(&s.content), print_style_runs(&s.style_runs))
    }

    fn parse_story(p: &mut LayoutDslParser) -> Result<TextStory, TextError> {
        p.eat_keyword("story")?;
        parse_story_fields(p)
    }

    fn print_story(s: &TextStory) -> String {
        format!("story {}", print_story_fields(s))
    }

    fn parse_link_fields(p: &mut LayoutDslParser) -> Result<ImageLink, TextError> {
        let id = parse_kv_ident(p, "id")?;
        let path = parse_kv_str(p, "path")?;
        let hash = parse_kv_str(p, "hash")?;
        let width = parse_kv_num(p, "width")? as u32;
        let height = parse_kv_num(p, "height")? as u32;
        let dpi = parse_kv_num(p, "dpi")? as u32;
        let color_profile = parse_kv_opt_str(p, "colorProfile")?;
        let state = parse_kv_opt_str(p, "state")?;
        let proxy_data_url = parse_kv_opt_str(p, "proxyDataUrl")?;
        Ok(ImageLink { id, path, hash, width, height, dpi, color_profile, state, proxy_data_url })
    }

    fn print_link_fields(l: &ImageLink) -> String {
        format!(
            "id={} path=\"{}\" hash=\"{}\" width={} height={} dpi={} colorProfile={} state={} proxyDataUrl={}",
            l.id,
            escape_str(&l.path),
            escape_str(&l.hash),
            l.width,
            l.height,
            l.dpi,
            print_opt_str(&l.color_profile),
            print_opt_str(&l.state),
            print_opt_str(&l.proxy_data_url)
        )
    }

    fn parse_link(p: &mut LayoutDslParser) -> Result<ImageLink, TextError> {
        p.eat_keyword("link")?;
        parse_link_fields(p)
    }

    fn print_link(l: &ImageLink) -> String {
        format!("link {}", print_link_fields(l))
    }

    fn parse_layer_stmt(p: &mut LayoutDslParser) -> Result<Layer, TextError> {
        p.eat_keyword("layer")?;
        let id = parse_kv_ident(p, "id")?;
        let name = parse_kv_str(p, "name")?;
        let visible = parse_kv_bool(p, "visible")?;
        let locked = parse_kv_bool(p, "locked")?;
        expect_key(p, "objectIds")?;
        let object_ids = parse_id_list(p)?;
        Ok(Layer { id, name, visible, locked, object_ids })
    }

    fn print_layer_stmt(l: &Layer) -> String {
        format!("layer id={} name=\"{}\" visible={} locked={} objectIds={}", l.id, escape_str(&l.name), l.visible, l.locked, print_id_list(&l.object_ids))
    }

    /// 🖼️ Reads a `Frame` starting directly at its kind ident (`rect`/`text`/`image`, no wrapping
    /// keyword) — self-delimiting the same way `draw_rs`'s `DrawLayerNode` kind tag is, so it can sit
    /// either inside a page's `{ }` block or right after an op-text `frame:` field label.
    fn parse_frame_value(p: &mut LayoutDslParser) -> Result<Frame, TextError> {
        let span = p.span();
        let kind = p.expect_ident()?;
        let id = parse_kv_ident(p, "id")?;
        let layer_id = parse_kv_ident(p, "layerId")?;
        expect_key(p, "bounds")?;
        let bounds = parse_bounds(p)?;
        let locked = parse_kv_opt_bool(p, "locked")?;
        let visible = parse_kv_opt_bool(p, "visible")?;
        match kind.as_str() {
            "rect" => {
                expect_key(p, "fill")?;
                let fill = parse_opt_color4(p)?;
                expect_key(p, "stroke")?;
                let stroke = parse_opt_color4(p)?;
                Ok(Frame::Rect { id, layer_id, bounds, locked, visible, fill, stroke })
            }
            "text" => {
                let story_id = parse_kv_ident(p, "storyId")?;
                let thread_next = parse_kv_opt_ident(p, "threadNext")?;
                let columns = parse_kv_num(p, "columns")? as u32;
                expect_key(p, "inset")?;
                let inset = parse_rect(p)?;
                let wrap_mode = parse_kv_str(p, "wrapMode")?;
                Ok(Frame::Text { id, layer_id, bounds, locked, visible, story_id, thread_next, columns, inset, wrap_mode })
            }
            "image" => {
                let link_id = parse_kv_ident(p, "linkId")?;
                Ok(Frame::Image { id, layer_id, bounds, locked, visible, link_id })
            }
            other => Err(TextError::expected(format!("unknown frame kind '{other}'"), span, "rect|text|image")),
        }
    }

    fn print_frame_value(f: &Frame) -> String {
        match f {
            Frame::Rect { id, layer_id, bounds, locked, visible, fill, stroke } => format!(
                "rect id={} layerId={} bounds={} locked={} visible={} fill={} stroke={}",
                id,
                layer_id,
                print_bounds(bounds),
                print_opt_bool(locked),
                print_opt_bool(visible),
                print_opt_color4(fill),
                print_opt_color4(stroke)
            ),
            Frame::Text { id, layer_id, bounds, locked, visible, story_id, thread_next, columns, inset, wrap_mode } => format!(
                "text id={} layerId={} bounds={} locked={} visible={} storyId={} threadNext={} columns={} inset={} wrapMode=\"{}\"",
                id,
                layer_id,
                print_bounds(bounds),
                print_opt_bool(locked),
                print_opt_bool(visible),
                story_id,
                print_opt_ident(thread_next),
                columns,
                print_rect(inset),
                escape_str(wrap_mode)
            ),
            Frame::Image { id, layer_id, bounds, locked, visible, link_id } => {
                format!("image id={} layerId={} bounds={} locked={} visible={} linkId={}", id, layer_id, print_bounds(bounds), print_opt_bool(locked), print_opt_bool(visible), link_id)
            }
        }
    }

    /// 🧱 Reads the `{ layer... rect|text|image... }` block shared by `Page` and `ParentPage` — zero or
    /// more `layer` statements followed by zero or more frame values, discriminated purely by their own
    /// leading keyword/kind (no extra wrapper needed).
    fn parse_layers_and_frames(p: &mut LayoutDslParser) -> Result<(Vec<Layer>, Vec<Frame>), TextError> {
        p.expect_tok(&LayoutTok::LBrace, "{")?;
        let mut layers = Vec::new();
        let mut frames = Vec::new();
        loop {
            match p.peek().clone() {
                LayoutTok::RBrace => break,
                LayoutTok::Ident(kw) if kw == "layer" => layers.push(parse_layer_stmt(p)?),
                LayoutTok::Ident(kw) if kw == "rect" || kw == "text" || kw == "image" => frames.push(parse_frame_value(p)?),
                other => return Err(TextError::expected("expected 'layer', a frame kind, or '}'", p.span(), format!("{other:?}"))),
            }
        }
        p.expect_tok(&LayoutTok::RBrace, "}")?;
        Ok((layers, frames))
    }

    fn print_layers_and_frames(layers: &[Layer], frames: &[Frame]) -> String {
        let mut parts = Vec::with_capacity(layers.len() + frames.len());
        for l in layers {
            parts.push(print_layer_stmt(l));
        }
        for f in frames {
            parts.push(print_frame_value(f));
        }
        format!("{{ {} }}", parts.join(" "))
    }

    fn parse_parent_page(p: &mut LayoutDslParser) -> Result<ParentPage, TextError> {
        p.eat_keyword("parentPage")?;
        let id = parse_kv_ident(p, "id")?;
        let name = parse_kv_str(p, "name")?;
        let width = parse_kv_num(p, "width")?;
        let height = parse_kv_num(p, "height")?;
        expect_key(p, "layerIds")?;
        let layer_ids = parse_id_list(p)?;
        let (layers, frames) = parse_layers_and_frames(p)?;
        Ok(ParentPage { id, name, width, height, layer_ids, layers, frames })
    }

    fn print_parent_page(pp: &ParentPage) -> String {
        format!(
            "parentPage id={} name=\"{}\" width={} height={} layerIds={} {}",
            pp.id,
            escape_str(&pp.name),
            fmt_num(pp.width),
            fmt_num(pp.height),
            print_id_list(&pp.layer_ids),
            print_layers_and_frames(&pp.layers, &pp.frames)
        )
    }

    fn parse_page_fields(p: &mut LayoutDslParser) -> Result<Page, TextError> {
        let id = parse_kv_ident(p, "id")?;
        let name = parse_kv_str(p, "name")?;
        let spread_id = parse_kv_ident(p, "spreadId")?;
        let parent_page_id = parse_kv_opt_ident(p, "parentPageId")?;
        let width = parse_kv_num(p, "width")?;
        let height = parse_kv_num(p, "height")?;
        expect_key(p, "margins")?;
        let margins = parse_margins(p)?;
        expect_key(p, "columns")?;
        let columns = parse_columns(p)?;
        expect_key(p, "guides")?;
        let guides = parse_rect_list(p)?;
        expect_key(p, "layerIds")?;
        let layer_ids = parse_id_list(p)?;
        let (layers, frames) = parse_layers_and_frames(p)?;
        expect_key(p, "overrides")?;
        let overrides = parse_overrides(p)?;
        Ok(Page { id, name, spread_id, parent_page_id, width, height, margins, columns, guides, layer_ids, layers, frames, overrides })
    }

    fn print_page_fields(pg: &Page) -> String {
        format!(
            "id={} name=\"{}\" spreadId={} parentPageId={} width={} height={} margins={} columns={} guides={} layerIds={} {} overrides={}",
            pg.id,
            escape_str(&pg.name),
            pg.spread_id,
            print_opt_ident(&pg.parent_page_id),
            fmt_num(pg.width),
            fmt_num(pg.height),
            print_margins(&pg.margins),
            print_columns(&pg.columns),
            print_rect_list(&pg.guides),
            print_id_list(&pg.layer_ids),
            print_layers_and_frames(&pg.layers, &pg.frames),
            print_overrides(&pg.overrides)
        )
    }

    fn parse_page(p: &mut LayoutDslParser) -> Result<Page, TextError> {
        p.eat_keyword("page")?;
        parse_page_fields(p)
    }

    fn print_page(pg: &Page) -> String {
        format!("page {}", print_page_fields(pg))
    }

    fn parse_spread(p: &mut LayoutDslParser) -> Result<Spread, TextError> {
        p.eat_keyword("spread")?;
        let id = parse_kv_ident(p, "id")?;
        let name = parse_kv_str(p, "name")?;
        expect_key(p, "pageIds")?;
        let page_ids = parse_id_list(p)?;
        Ok(Spread { id, name, page_ids })
    }

    fn print_spread(s: &Spread) -> String {
        format!("spread id={} name=\"{}\" pageIds={}", s.id, escape_str(&s.name), print_id_list(&s.page_ids))
    }
    //#endregion 🔖DslEntities

    //#region 🔖DslDocument
    /// 📤 Renders `doc` as a list of self-delimited top-level statements (`doc`/`camera`/`previewCamera`/
    /// `grid`/one per style/story/link/parent page/spread/page/`printTarget`) — joined with `"\n"` for
    /// the pretty `print_dsl`.
    fn print_dsl_chunks(doc: &LayoutDocument) -> Vec<String> {
        let mut chunks = Vec::new();
        chunks.push(format!("doc schema={} name=\"{}\"", doc.schema, escape_str(&doc.name)));
        chunks.push(print_camera_line("camera", &doc.camera));
        chunks.push(print_camera_line("previewCamera", &doc.preview_camera));
        chunks.push(format!("grid baselineGrid={} baselineOffset={} snapToBaseline={}", fmt_num(doc.grid.baseline_grid), fmt_num(doc.grid.baseline_offset), doc.grid.snap_to_baseline));
        for s in &doc.paragraph_styles {
            chunks.push(print_paragraph_style(s));
        }
        for c in &doc.character_styles {
            chunks.push(print_character_style(c));
        }
        for s in &doc.stories {
            chunks.push(print_story(s));
        }
        for l in &doc.links {
            chunks.push(print_link(l));
        }
        for pp in &doc.parent_pages {
            chunks.push(print_parent_page(pp));
        }
        for sp in &doc.spreads {
            chunks.push(print_spread(sp));
        }
        for pg in &doc.pages {
            chunks.push(print_page(pg));
        }
        chunks.push(format!("printTarget={}", print_opt_str(&doc.print_target)));
        chunks
    }

    fn parse_document(p: &mut LayoutDslParser) -> Result<LayoutDocument, TextError> {
        p.eat_keyword("doc")?;
        let schema = parse_kv_ident(p, "schema")?;
        let name = parse_kv_str(p, "name")?;
        p.eat_keyword("camera")?;
        let camera = parse_camera_fields(p)?;
        p.eat_keyword("previewCamera")?;
        let preview_camera = parse_camera_fields(p)?;
        p.eat_keyword("grid")?;
        let baseline_grid = parse_kv_num(p, "baselineGrid")?;
        let baseline_offset = parse_kv_num(p, "baselineOffset")?;
        let snap_to_baseline = parse_kv_bool(p, "snapToBaseline")?;
        let grid = GridSettings { baseline_grid, baseline_offset, snap_to_baseline };
        let mut paragraph_styles = Vec::new();
        while p.at_ident("paragraphStyle") {
            paragraph_styles.push(parse_paragraph_style(p)?);
        }
        let mut character_styles = Vec::new();
        while p.at_ident("characterStyle") {
            character_styles.push(parse_character_style(p)?);
        }
        let mut stories = Vec::new();
        while p.at_ident("story") {
            stories.push(parse_story(p)?);
        }
        let mut links = Vec::new();
        while p.at_ident("link") {
            links.push(parse_link(p)?);
        }
        let mut parent_pages = Vec::new();
        while p.at_ident("parentPage") {
            parent_pages.push(parse_parent_page(p)?);
        }
        let mut spreads = Vec::new();
        while p.at_ident("spread") {
            spreads.push(parse_spread(p)?);
        }
        let mut pages = Vec::new();
        while p.at_ident("page") {
            pages.push(parse_page(p)?);
        }
        p.eat_keyword("printTarget")?;
        p.expect_tok(&LayoutTok::Eq, "=")?;
        let print_target = parse_opt_str(p)?;
        Ok(LayoutDocument { schema, name, camera, preview_camera, grid, paragraph_styles, character_styles, stories, links, parent_pages, spreads, pages, print_target })
    }

    impl DocumentDsl for LayoutDocument {
        const EXTENSION: &'static str = "layout";

        fn parse_dsl(text: &str) -> Result<Self, TextError> {
            let tokens = lex_layout_dsl(text)?;
            let mut parser = LayoutDslParser::new(tokens);
            parse_document(&mut parser)
        }

        fn print_dsl(&self) -> String {
            print_dsl_chunks(self).join("\n")
        }
    }
    //#endregion 🔖DslDocument
    //#endregion 🔖Dsl

    //#region 🔖OpText
    /// 🩹 Sparse `key=value` attr-loop reader/printer for `PagePatch` — reused verbatim by both the
    /// `pagesPatch` op-text line and (indirectly, via the same field set) nothing else, since `Page` has
    /// no other sparse-patch surface.
    fn parse_page_patch(p: &mut LayoutDslParser) -> Result<PagePatch, TextError> {
        let mut patch = PagePatch::default();
        while p.at_attr() {
            let key = p.peek_attr_key().expect("at_attr confirmed an identifier").to_string();
            match key.as_str() {
                "name" => {
                    take_attr_key(p)?;
                    patch.name = Some(p.expect_str()?);
                }
                "width" => {
                    take_attr_key(p)?;
                    patch.width = Some(p.expect_num()?);
                }
                "height" => {
                    take_attr_key(p)?;
                    patch.height = Some(p.expect_num()?);
                }
                "marginTop" => {
                    take_attr_key(p)?;
                    patch.margin_top = Some(p.expect_num()?);
                }
                "marginRight" => {
                    take_attr_key(p)?;
                    patch.margin_right = Some(p.expect_num()?);
                }
                "marginBottom" => {
                    take_attr_key(p)?;
                    patch.margin_bottom = Some(p.expect_num()?);
                }
                "marginLeft" => {
                    take_attr_key(p)?;
                    patch.margin_left = Some(p.expect_num()?);
                }
                "columnsCount" => {
                    take_attr_key(p)?;
                    patch.columns_count = Some(p.expect_num()? as u32);
                }
                "columnsGutter" => {
                    take_attr_key(p)?;
                    patch.columns_gutter = Some(p.expect_num()?);
                }
                _ => break,
            }
        }
        Ok(patch)
    }

    fn print_page_patch(patch: &PagePatch) -> String {
        let mut parts = Vec::new();
        if let Some(v) = &patch.name {
            parts.push(format!("name=\"{}\"", escape_str(v)));
        }
        if let Some(v) = patch.width {
            parts.push(format!("width={}", fmt_num(v)));
        }
        if let Some(v) = patch.height {
            parts.push(format!("height={}", fmt_num(v)));
        }
        if let Some(v) = patch.margin_top {
            parts.push(format!("marginTop={}", fmt_num(v)));
        }
        if let Some(v) = patch.margin_right {
            parts.push(format!("marginRight={}", fmt_num(v)));
        }
        if let Some(v) = patch.margin_bottom {
            parts.push(format!("marginBottom={}", fmt_num(v)));
        }
        if let Some(v) = patch.margin_left {
            parts.push(format!("marginLeft={}", fmt_num(v)));
        }
        if let Some(v) = patch.columns_count {
            parts.push(format!("columnsCount={v}"));
        }
        if let Some(v) = patch.columns_gutter {
            parts.push(format!("columnsGutter={}", fmt_num(v)));
        }
        parts.join(" ")
    }

    /// 🩹 Sparse `key=value` attr-loop reader/printer for `FramePatch` — `fill`/`stroke` are doubly
    /// optional (outer presence = "this key was touched", inner `none`/color = "cleared"/"set"), matching
    /// `FramePatch`'s own `Option<Option<[f32;4]>>` shape.
    fn parse_frame_patch(p: &mut LayoutDslParser) -> Result<FramePatch, TextError> {
        let mut patch = FramePatch::default();
        while p.at_attr() {
            let key = p.peek_attr_key().expect("at_attr confirmed an identifier").to_string();
            match key.as_str() {
                "x" => {
                    take_attr_key(p)?;
                    patch.x = Some(p.expect_num()?);
                }
                "y" => {
                    take_attr_key(p)?;
                    patch.y = Some(p.expect_num()?);
                }
                "width" => {
                    take_attr_key(p)?;
                    patch.width = Some(p.expect_num()?);
                }
                "height" => {
                    take_attr_key(p)?;
                    patch.height = Some(p.expect_num()?);
                }
                "fill" => {
                    take_attr_key(p)?;
                    patch.fill = Some(parse_opt_color4(p)?);
                }
                "stroke" => {
                    take_attr_key(p)?;
                    patch.stroke = Some(parse_opt_color4(p)?);
                }
                "wrapMode" => {
                    take_attr_key(p)?;
                    patch.wrap_mode = Some(p.expect_str()?);
                }
                "columns" => {
                    take_attr_key(p)?;
                    patch.columns = Some(p.expect_num()? as u32);
                }
                _ => break,
            }
        }
        Ok(patch)
    }

    fn print_frame_patch(patch: &FramePatch) -> String {
        let mut parts = Vec::new();
        if let Some(v) = patch.x {
            parts.push(format!("x={}", fmt_num(v)));
        }
        if let Some(v) = patch.y {
            parts.push(format!("y={}", fmt_num(v)));
        }
        if let Some(v) = patch.width {
            parts.push(format!("width={}", fmt_num(v)));
        }
        if let Some(v) = patch.height {
            parts.push(format!("height={}", fmt_num(v)));
        }
        if let Some(v) = &patch.fill {
            parts.push(format!("fill={}", print_opt_color4(v)));
        }
        if let Some(v) = &patch.stroke {
            parts.push(format!("stroke={}", print_opt_color4(v)));
        }
        if let Some(v) = &patch.wrap_mode {
            parts.push(format!("wrapMode=\"{}\"", escape_str(v)));
        }
        if let Some(v) = patch.columns {
            parts.push(format!("columns={v}"));
        }
        parts.join(" ")
    }

    /// ⚡ One-line textual encoding of every `LayoutOperation` variant (`vcs::OpText`). Reuses the value
    /// grammars from `🔖Dsl` (page/story/link/frame/patch) so a full nested item embeds inline on one
    /// line — the DSL grammar never depends on newlines, so this is the same text either way.
    impl OpText for LayoutOperation {
        fn parse_op(line: &str) -> Result<Self, TextError> {
            let tokens = lex_layout_dsl(line)?;
            let mut p = LayoutDslParser::new(tokens);
            let span = p.span();
            let op_name = p.expect_ident()?;
            let operation = match op_name.as_str() {
                "pagesAdd" => {
                    let index = parse_kv_num(&mut p, "index")? as usize;
                    expect_key_colon(&mut p, "page")?;
                    let item = parse_page_fields(&mut p)?;
                    LayoutOperation::Pages(CollectionOperation::Add { index, item })
                }
                "pagesRemove" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    LayoutOperation::Pages(CollectionOperation::Remove { id })
                }
                "pagesMove" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    let to_index = parse_kv_num(&mut p, "toIndex")? as usize;
                    LayoutOperation::Pages(CollectionOperation::Move { id, to_index })
                }
                "pagesPatch" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    let patch = parse_page_patch(&mut p)?;
                    LayoutOperation::Pages(CollectionOperation::Patch { id, patch })
                }
                "storiesAdd" => {
                    let index = parse_kv_num(&mut p, "index")? as usize;
                    expect_key_colon(&mut p, "story")?;
                    let item = parse_story_fields(&mut p)?;
                    LayoutOperation::Stories(CollectionOperation::Add { index, item })
                }
                "storiesRemove" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    LayoutOperation::Stories(CollectionOperation::Remove { id })
                }
                "storiesMove" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    let to_index = parse_kv_num(&mut p, "toIndex")? as usize;
                    LayoutOperation::Stories(CollectionOperation::Move { id, to_index })
                }
                "storiesPatch" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    let content = if p.at_attr() { Some(parse_kv_str(&mut p, "content")?) } else { None };
                    LayoutOperation::Stories(CollectionOperation::Patch { id, patch: TextStoryPatch { content } })
                }
                "linksAdd" => {
                    let index = parse_kv_num(&mut p, "index")? as usize;
                    expect_key_colon(&mut p, "link")?;
                    let item = parse_link_fields(&mut p)?;
                    LayoutOperation::Links(CollectionOperation::Add { index, item })
                }
                "linksRemove" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    LayoutOperation::Links(CollectionOperation::Remove { id })
                }
                "linksMove" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    let to_index = parse_kv_num(&mut p, "toIndex")? as usize;
                    LayoutOperation::Links(CollectionOperation::Move { id, to_index })
                }
                "linksPatch" => {
                    let id = parse_kv_ident(&mut p, "id")?;
                    let path = if p.at_attr() { Some(parse_kv_str(&mut p, "path")?) } else { None };
                    LayoutOperation::Links(CollectionOperation::Patch { id, patch: ImageLinkPatch { path } })
                }
                "addFrame" => {
                    let page_id = parse_kv_ident(&mut p, "pageId")?;
                    let index = parse_kv_num(&mut p, "index")? as usize;
                    let layer_id = parse_kv_opt_ident(&mut p, "layerId")?;
                    expect_key_colon(&mut p, "frame")?;
                    let frame = parse_frame_value(&mut p)?;
                    LayoutOperation::AddFrame { page_id, index, frame, layer_id }
                }
                "removeFrame" => {
                    let page_id = parse_kv_ident(&mut p, "pageId")?;
                    let frame_id = parse_kv_ident(&mut p, "frameId")?;
                    LayoutOperation::RemoveFrame { page_id, frame_id }
                }
                "patchFrame" => {
                    let page_id = parse_kv_ident(&mut p, "pageId")?;
                    let frame_id = parse_kv_ident(&mut p, "frameId")?;
                    let patch = parse_frame_patch(&mut p)?;
                    LayoutOperation::PatchFrame { page_id, frame_id, patch }
                }
                "setCamera" => {
                    let blueprint = parse_kv_bool(&mut p, "blueprint")?;
                    expect_key(&mut p, "camera")?;
                    let camera = parse_camera_tuple(&mut p)?;
                    LayoutOperation::SetCamera { blueprint, camera }
                }
                other => return Err(TextError::expected(format!("unknown layout operation '{other}'"), span, "known LayoutOperation variant")),
            };
            Ok(operation)
        }

        fn print_op(&self) -> String {
            match self {
                LayoutOperation::Pages(cop) => match cop {
                    CollectionOperation::Add { index, item } => format!("pagesAdd index={index} page:{}", print_page_fields(item)),
                    CollectionOperation::Remove { id } => format!("pagesRemove id={id}"),
                    CollectionOperation::Move { id, to_index } => format!("pagesMove id={id} toIndex={to_index}"),
                    CollectionOperation::Patch { id, patch } => {
                        let fields = print_page_patch(patch);
                        if fields.is_empty() {
                            format!("pagesPatch id={id}")
                        } else {
                            format!("pagesPatch id={id} {fields}")
                        }
                    }
                },
                LayoutOperation::Stories(cop) => match cop {
                    CollectionOperation::Add { index, item } => format!("storiesAdd index={index} story:{}", print_story_fields(item)),
                    CollectionOperation::Remove { id } => format!("storiesRemove id={id}"),
                    CollectionOperation::Move { id, to_index } => format!("storiesMove id={id} toIndex={to_index}"),
                    CollectionOperation::Patch { id, patch } => match &patch.content {
                        Some(v) => format!("storiesPatch id={id} content=\"{}\"", escape_str(v)),
                        None => format!("storiesPatch id={id}"),
                    },
                },
                LayoutOperation::Links(cop) => match cop {
                    CollectionOperation::Add { index, item } => format!("linksAdd index={index} link:{}", print_link_fields(item)),
                    CollectionOperation::Remove { id } => format!("linksRemove id={id}"),
                    CollectionOperation::Move { id, to_index } => format!("linksMove id={id} toIndex={to_index}"),
                    CollectionOperation::Patch { id, patch } => match &patch.path {
                        Some(v) => format!("linksPatch id={id} path=\"{}\"", escape_str(v)),
                        None => format!("linksPatch id={id}"),
                    },
                },
                LayoutOperation::AddFrame { page_id, index, frame, layer_id } => {
                    format!("addFrame pageId={page_id} index={index} layerId={} frame:{}", print_opt_ident(layer_id), print_frame_value(frame))
                }
                LayoutOperation::RemoveFrame { page_id, frame_id } => format!("removeFrame pageId={page_id} frameId={frame_id}"),
                LayoutOperation::PatchFrame { page_id, frame_id, patch } => {
                    let fields = print_frame_patch(patch);
                    if fields.is_empty() {
                        format!("patchFrame pageId={page_id} frameId={frame_id}")
                    } else {
                        format!("patchFrame pageId={page_id} frameId={frame_id} {fields}")
                    }
                }
                LayoutOperation::SetCamera { blueprint, camera } => format!("setCamera blueprint={blueprint} camera={}", print_camera_tuple(camera)),
            }
        }
    }
    //#endregion 🔖OpText

    #[cfg(test)]
    mod tests {
        use super::*;
        use vcs::{create_document_vcs_envelope, test_support, DocumentVcsCommand, DocumentVcsStore};

        fn sample_document() -> LayoutDocument {
            LayoutDocument::parse_dsl(include_str!("../example/sample.layout")).expect("sample fixture parses")
        }

        #[test]
        fn dsl_round_trips_sample_fixture() {
            let doc = sample_document();
            assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
            assert_eq!(doc.pages.len(), 2);
            test_support::assert_dsl_round_trip(&doc);
        }

        #[test]
        fn dsl_round_trips_minimal_document_with_character_style() {
            let doc = LayoutDocument {
                schema: LAYOUT_FIXTURE_SCHEMA.into(),
                name: "Empty".into(),
                camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
                preview_camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
                grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
                paragraph_styles: Vec::new(),
                character_styles: vec![serde_json::json!({"id": "char.emph", "italic": true})],
                stories: Vec::new(),
                links: Vec::new(),
                parent_pages: Vec::new(),
                spreads: Vec::new(),
                pages: Vec::new(),
                print_target: None,
            };
            test_support::assert_dsl_round_trip(&doc);
        }

        #[test]
        fn op_text_round_trips_every_layout_operation_variant() {
            let doc = sample_document();

            let mut page_2 = doc.pages[0].clone();
            page_2.id = "page-3".into();
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Add { index: 1, item: page_2 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Remove { id: "page-1".into() }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Move { id: "page-1".into(), to_index: 1 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch {
                id: "page-1".into(),
                patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() },
            }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch::default() }));

            let mut story_2 = doc.stories[0].clone();
            story_2.id = "story-2".into();
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Add { index: 1, item: story_2 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Remove { id: "story-1".into() }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Move { id: "story-1".into(), to_index: 0 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: Some("Edited".into()) } }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: None } }));

            let mut link_2 = doc.links[0].clone();
            link_2.id = "link-2".into();
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Add { index: 1, item: link_2 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Remove { id: "link-missing".into() }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Move { id: "link-missing".into(), to_index: 0 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Patch { id: "link-missing".into(), patch: ImageLinkPatch { path: Some("b.png".into()) } }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Patch { id: "link-missing".into(), patch: ImageLinkPatch { path: None } }));

            let rect_frame = Frame::Rect {
                id: "frame-new".into(),
                layer_id: "layer-1".into(),
                bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 },
                locked: None,
                visible: Some(true),
                fill: Some([0.1, 0.2, 0.3, 1.0]),
                stroke: None,
            };
            test_support::assert_op_line_round_trip(&LayoutOperation::AddFrame { page_id: "page-1".into(), index: 1, frame: rect_frame, layer_id: Some("layer-1".into()) });
            let image_frame = Frame::Image {
                id: "frame-img".into(),
                layer_id: "layer-1".into(),
                bounds: LayoutBounds { x: 1.0, y: 2.0, width: 3.0, height: 4.0, rotation: 5.0 },
                locked: Some(false),
                visible: None,
                link_id: "link-missing".into(),
            };
            test_support::assert_op_line_round_trip(&LayoutOperation::AddFrame { page_id: "page-1".into(), index: 1, frame: image_frame, layer_id: None });
            test_support::assert_op_line_round_trip(&LayoutOperation::RemoveFrame { page_id: "page-1".into(), frame_id: "frame-text-1".into() });
            test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame {
                page_id: "page-1".into(),
                frame_id: "frame-text-1".into(),
                patch: FramePatch { x: Some(10.0), fill: Some(Some([0.5, 0.5, 0.5, 1.0])), stroke: Some(None), ..Default::default() },
            });
            test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-text-1".into(), patch: FramePatch::default() });
            test_support::assert_op_line_round_trip(&LayoutOperation::SetCamera { blueprint: true, camera: LayoutCamera { x: 5.0, y: -6.5, zoom: 2.25 } });
            test_support::assert_op_line_round_trip(&LayoutOperation::SetCamera { blueprint: false, camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 } });
        }

        #[test]
        fn document_text_round_trips_a_store_with_applied_operations() {
            let initial = sample_document();
            let envelope = create_document_vcs_envelope(LAYOUT_FIXTURE_SCHEMA, "layout-doc-text-test", initial, None);
            let mut store: DocumentVcsStore<LayoutDocument, LayoutOperation> = DocumentVcsStore::new(envelope);
            store
                .dispatch(DocumentVcsCommand::Apply { operations: vec![LayoutOperation::SetCamera { blueprint: true, camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 } }], description: Some("pan camera".into()) })
                .expect("apply set camera");
            store
                .dispatch(DocumentVcsCommand::Apply {
                    operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } })],
                    description: Some("rename page".into()),
                })
                .expect("apply patch page");
            test_support::assert_document_text_round_trip(&store);
            test_support::assert_live_equals_replay(&store);
        }
    }
}

pub use display::*;
pub use document::*;
pub use engine::*;
pub use export::*;
pub use operations::*;

#[cfg(target_arch = "wasm32")]
mod wasm_session {
    // #region wasm_session
    use std::cell::RefCell;
    use std::rc::Rc;

    use infinite_cavas::camera::{self, Camera, Viewport};
    use infinite_cavas::Point;
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    use crate::document::parse_layout_document;
    use crate::engine::{build_scene_from_document_json, hit_test_document_json, screen_to_world_json, LayoutDropPreview, SceneQuery};
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
                let (render_ctx, renderer, surface) = infinite_cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
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
            let clear = infinite_cavas::theme::default_raster_clear();
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
