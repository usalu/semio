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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct LayoutCamera {
        pub x: f64,
        pub y: f64,
        pub zoom: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct LayoutRect {
        pub x: f64,
        pub y: f64,
        #[serde(rename = "w")]
        pub width: f64,
        #[serde(rename = "h")]
        pub height: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct LayoutBounds {
        pub x: f64,
        pub y: f64,
        #[serde(rename = "w")]
        pub width: f64,
        #[serde(rename = "h")]
        pub height: f64,
        pub rotation: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct PageMargins {
        pub top: f64,
        pub right: f64,
        pub bottom: f64,
        pub left: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct PageColumns {
        pub count: u32,
        pub gutter: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
    #[serde(tag = "kind")]
    pub enum Frame {
        #[serde(rename = "rect")]
        Rect {
            id: String,
            #[serde(rename = "layerId")]
            layer_id: String,
            #[dsl(block)]
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
            #[dsl(block)]
            bounds: LayoutBounds,
            locked: Option<bool>,
            visible: Option<bool>,
            #[serde(rename = "storyId")]
            story_id: String,
            #[serde(rename = "threadNext")]
            thread_next: Option<String>,
            columns: u32,
            #[dsl(block)]
            inset: LayoutRect,
            #[serde(rename = "wrapMode")]
            wrap_mode: String,
        },
        #[serde(rename = "image")]
        Image {
            id: String,
            #[serde(rename = "layerId")]
            layer_id: String,
            #[dsl(block)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct TextStyleRun {
        pub start: usize,
        pub end: usize,
        #[serde(rename = "paragraphStyleId")]
        pub paragraph_style_id: Option<String>,
        #[serde(rename = "characterStyleId")]
        pub character_style_id: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct TextStory {
        pub id: String,
        pub content: String,
        #[serde(rename = "styleRuns")]
        #[dsl(table)]
        pub style_runs: Vec<TextStyleRun>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct PageOverride {
        #[serde(rename = "objectId")]
        pub object_id: String,
        #[dsl(block)]
        pub bounds: Option<LayoutBounds>,
        pub visible: Option<bool>,
        pub locked: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ParentPage {
        pub id: String,
        pub name: String,
        pub width: f64,
        pub height: f64,
        #[serde(rename = "layerIds")]
        pub layer_ids: Vec<String>,
        #[dsl(table)]
        pub layers: Vec<Layer>,
        #[dsl(statements, block)]
        pub frames: Vec<Frame>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct Page {
        pub id: String,
        pub name: String,
        #[serde(rename = "spreadId")]
        pub spread_id: String,
        #[serde(rename = "parentPageId")]
        pub parent_page_id: Option<String>,
        pub width: f64,
        pub height: f64,
        #[dsl(block)]
        pub margins: PageMargins,
        #[dsl(block)]
        pub columns: PageColumns,
        #[dsl(table)]
        pub guides: Vec<LayoutRect>,
        #[serde(rename = "layerIds")]
        pub layer_ids: Vec<String>,
        #[dsl(table)]
        pub layers: Vec<Layer>,
        #[dsl(statements, block)]
        pub frames: Vec<Frame>,
        #[dsl(table)]
        pub overrides: Vec<PageOverride>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct Spread {
        pub id: String,
        pub name: String,
        #[serde(rename = "pageIds")]
        pub page_ids: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct GridSettings {
        #[serde(rename = "baselineGrid")]
        pub baseline_grid: f64,
        #[serde(rename = "baselineOffset")]
        pub baseline_offset: f64,
        #[serde(rename = "snapToBaseline")]
        pub snap_to_baseline: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
    #[dsl(extension = "layout", layout = "lines")]
    pub struct LayoutDocument {
        pub schema: String,
        pub name: String,
        #[dsl(block)]
        pub camera: LayoutCamera,
        #[serde(rename = "previewCamera")]
        #[dsl(block)]
        pub preview_camera: LayoutCamera,
        #[dsl(block)]
        pub grid: GridSettings,
        #[serde(rename = "paragraphStyles")]
        #[dsl(table)]
        pub paragraph_styles: Vec<ParagraphStyle>,
        #[serde(rename = "characterStyles")]
        pub character_styles: Vec<serde_json::Value>,
        #[dsl(table)]
        pub stories: Vec<TextStory>,
        #[dsl(table)]
        pub links: Vec<ImageLink>,
        #[serde(rename = "parentPages")]
        pub parent_pages: Vec<ParentPage>,
        #[dsl(table)]
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

    #[cfg(test)]
    mod tests {
        use super::*;

        fn rect_frame(id: &str, visible: Option<bool>) -> Frame {
            Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible, fill: None, stroke: None }
        }

        #[test]
        fn frame_helpers_report_id_bounds_kind_and_visibility() {
            let rect = rect_frame("frame-1", Some(false));
            assert_eq!(rect.id(), "frame-1");
            assert_eq!(rect.kind_str(), "rect");
            assert!(!rect.visible());

            let default_visible = rect_frame("frame-2", None);
            assert!(default_visible.visible());
            assert_eq!(default_visible.bounds().width, 10.0);

            let text = Frame::Text { id: "frame-3".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0 }, locked: None, visible: None, story_id: "story-1".into(), thread_next: None, columns: 1, inset: LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, wrap_mode: "box".into() };
            assert_eq!(text.kind_str(), "text");

            let image = Frame::Image { id: "frame-4".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0 }, locked: None, visible: Some(true), link_id: "link-1".into() };
            assert_eq!(image.kind_str(), "image");
            assert!(image.visible());
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

        #[test]
        fn resolve_page_marks_overridden_parent_frames_and_ignores_missing_parent() {
            let mut doc = base_doc();
            doc.parent_pages.push(ParentPage {
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
                overrides: vec![PageOverride { object_id: "frame-a".into(), bounds: None, visible: None, locked: None }],
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
            assert!(matches!(error, crate::LayoutError::UnexpectedSchema(schema) if schema == "other.schema"));

            let invalid_json = "not json";
            let error = parse_layout_document(invalid_json).expect_err("invalid json must fail");
            assert!(matches!(error, crate::LayoutError::Json(_)));
        }
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

        #[test]
        fn scene_and_hit_test_error_when_page_missing() {
            let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":100,"height":100,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
            let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
            let viewport = Viewport { width: 100, height: 100, dpr: 1.0 };
            let query = SceneQuery { page_id: "missing-page", selected_ids: &[], hovered_id: None, chrome_blueprint: true, camera: &camera, viewport: &viewport };
            assert!(matches!(build_scene_from_document_json(json, &query, None), Err(crate::LayoutError::PageNotFound(id)) if id == "missing-page"));
            let hit = hit_test_document_json(json, 0.0, 0.0, &query);
            assert!(matches!(hit, Err(crate::LayoutError::PageNotFound(id)) if id == "missing-page"));
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
                    crate::display::DisplayRect { object_id: "r-explicit-stroke".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, fill: Some(DisplayColor([1.0, 1.0, 1.0, 1.0])), stroke: Some(DisplayColor([0.0, 0.0, 0.0, 1.0])), inherited: false, selected: true, hovered: false },
                    crate::display::DisplayRect { object_id: "r-implicit-hover".into(), x: 20.0, y: 0.0, width: 10.0, height: 10.0, fill: None, stroke: None, inherited: false, selected: false, hovered: true },
                    crate::display::DisplayRect { object_id: "r-implicit-select".into(), x: 40.0, y: 0.0, width: 10.0, height: 10.0, fill: None, stroke: None, inherited: false, selected: true, hovered: false },
                ],
                text_runs: vec![DisplayTextRun { object_id: "text-1".into(), glyphs: vec![DisplayGlyph { glyph_id: 1, font_size: 12.0, x: 0.0, y: 0.0, color: DisplayColor([0.0, 0.0, 0.0, 1.0]) }] }],
                images: vec![DisplayImage { object_id: "img-1".into(), x: 0.0, y: 60.0, width: 10.0, height: 10.0, placeholder: true }],
                guides: vec![DisplayGuide { rect: crate::document::LayoutRect { x: 0.0, y: 0.0, width: 10.0, height: 0.0 }, kind: "unrecognized".into() }],
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
        use store::DocumentDsl;

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
            assert!(matches!(export_document_svg(&doc, "no-such-page"), Err(crate::LayoutError::PageNotFound(id)) if id == "no-such-page"));
            assert!(matches!(export_document_pdf(&doc, "no-such-page"), Err(crate::LayoutError::PageNotFound(_))));
            assert!(matches!(export_document_png_cpu(&doc, "no-such-page"), Err(crate::LayoutError::PageNotFound(_))));
        }

        #[test]
        fn package_zip_rejects_invalid_document_json() {
            let error = export_package_zip("not json", "[]").expect_err("invalid json must fail");
            assert!(matches!(error, crate::LayoutError::Json(_)));
        }

        #[test]
        fn scene_png_from_display_list_writes_a_valid_png() {
            let doc = sample_document();
            let page = doc.pages.iter().find(|p| p.id == "page-1").expect("page-1");
            let list = build_display_list_for_page(&doc, page, "page-1", &[], None, false);
            let bytes = scene_png_from_display_list(&list).expect("scene png export succeeds");
            assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
        }
    }
    // #endregion export
}

mod operations {
    // #region operations
    //! 🧾 Typed VCS operation vocabulary for the layout document — the operations the layout program emits
    //! (page/story/link collections, per-page frame add/remove/patch, and camera). Each operation computes a
    //! true pre-state inverse so undo/redo round-trips exactly. See {@link store::Operation}.

    use serde::{Deserialize, Serialize};
    use protocol::{apply_collection_operation, invert_collection_operation, CollectionOperation, Identified, Operation, OperationDiff, Patchable};

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
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
        fn apply_patch(&mut self, patch: &PagePatch) {
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
        }

        fn diff_patch(&self, other: &Self) -> Option<PagePatch> {
            let mut patch = PagePatch::default();
            let mut changed = false;
            if self.name != other.name {
                patch.name = Some(other.name.clone());
                changed = true;
            }
            if self.width != other.width {
                patch.width = Some(other.width);
                changed = true;
            }
            if self.height != other.height {
                patch.height = Some(other.height);
                changed = true;
            }
            if self.margins.top != other.margins.top {
                patch.margin_top = Some(other.margins.top);
                changed = true;
            }
            if self.margins.right != other.margins.right {
                patch.margin_right = Some(other.margins.right);
                changed = true;
            }
            if self.margins.bottom != other.margins.bottom {
                patch.margin_bottom = Some(other.margins.bottom);
                changed = true;
            }
            if self.margins.left != other.margins.left {
                patch.margin_left = Some(other.margins.left);
                changed = true;
            }
            if self.columns.count != other.columns.count {
                patch.columns_count = Some(other.columns.count);
                changed = true;
            }
            if self.columns.gutter != other.columns.gutter {
                patch.columns_gutter = Some(other.columns.gutter);
                changed = true;
            }
            changed.then_some(patch)
        }
    }

    /// 📝 Sparse patch for a {@link TextStory}'s body content.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct TextStoryPatch {
        pub content: Option<String>,
    }

    impl Patchable<TextStoryPatch> for TextStory {
        fn apply_patch(&mut self, patch: &TextStoryPatch) {
            if let Some(content) = &patch.content {
                self.content = content.clone();
            }
        }

        fn diff_patch(&self, other: &Self) -> Option<TextStoryPatch> {
            (self.content != other.content).then(|| TextStoryPatch { content: Some(other.content.clone()) })
        }
    }

    /// 🔗 Sparse patch for an {@link ImageLink}'s file path.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct ImageLinkPatch {
        pub path: Option<String>,
    }

    impl Patchable<ImageLinkPatch> for ImageLink {
        fn apply_patch(&mut self, patch: &ImageLinkPatch) {
            if let Some(path) = &patch.path {
                self.path = path.clone();
            }
        }

        fn diff_patch(&self, other: &Self) -> Option<ImageLinkPatch> {
            (self.path != other.path).then(|| ImageLinkPatch { path: Some(other.path.clone()) })
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
            let add = LayoutOperation::Pages(CollectionOperation::Add { id: page_2.id.clone(), item: page_2, at: 1 });
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

        fn new_text(id: &str) -> Frame {
            Frame::Text { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 20.0, height: 20.0, rotation: 0.0 }, locked: None, visible: None, story_id: "story-1".into(), thread_next: None, columns: 1, inset: crate::document::LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, wrap_mode: "box".into() }
        }

        #[test]
        fn patch_frame_updates_text_fields_and_ignores_fill_on_image_frames() {
            let doc = sample_doc();
            let with_text = vcs::apply_operation(&doc, &LayoutOperation::AddFrame { page_id: "page-1".into(), index: 0, frame: new_text("frame-text"), layer_id: None });
            let patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-text".into(), patch: FramePatch { wrap_mode: Some("column".into()), columns: Some(2), ..Default::default() } };
            let patched = round_trip(&with_text, &patch);
            let Frame::Text { wrap_mode, columns, .. } = patched.pages[0].frames.iter().find(|frame| frame.id() == "frame-text").unwrap() else { panic!("expected text frame") };
            assert_eq!(wrap_mode, "column");
            assert_eq!(*columns, 2);

            let image_frame = Frame::Image { id: "frame-img".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 5.0, height: 5.0, rotation: 0.0 }, locked: None, visible: None, link_id: "link-1".into() };
            let with_image = vcs::apply_operation(&doc, &LayoutOperation::AddFrame { page_id: "page-1".into(), index: 0, frame: image_frame, layer_id: None });
            let image_patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-img".into(), patch: FramePatch { x: Some(3.0), fill: Some(Some([1.0, 0.0, 0.0, 1.0])), ..Default::default() } };
            let patched_image = round_trip(&with_image, &image_patch);
            let patched_frame = patched_image.pages[0].frames.iter().find(|frame| frame.id() == "frame-img").unwrap();
            assert_eq!(patched_frame.bounds().x, 3.0, "bounds still patch on an image frame");
        }

        #[test]
        fn add_remove_patch_frame_are_no_ops_when_target_missing() {
            let doc = sample_doc();

            let missing_page_add = LayoutOperation::AddFrame { page_id: "no-page".into(), index: 0, frame: new_rect("frame-x"), layer_id: None };
            assert_eq!(vcs::apply_operation(&doc, &missing_page_add), doc, "adding to a missing page must be a no-op");

            let unmatched_layer = LayoutOperation::AddFrame { page_id: "page-1".into(), index: 0, frame: new_rect("frame-y"), layer_id: Some("no-layer".into()) };
            let result = vcs::apply_operation(&doc, &unmatched_layer);
            assert!(result.pages[0].frames.iter().any(|frame| frame.id() == "frame-y"));
            assert!(result.pages[0].layers[0].object_ids.iter().all(|id| id != "frame-y"), "unmatched layer id must not be populated");

            let missing_page_remove = LayoutOperation::RemoveFrame { page_id: "no-page".into(), frame_id: "frame-1".into() };
            assert_eq!(vcs::apply_operation(&doc, &missing_page_remove), doc);
            assert!(missing_page_remove.backwards(&doc).is_empty());

            let missing_frame_remove = LayoutOperation::RemoveFrame { page_id: "page-1".into(), frame_id: "no-frame".into() };
            assert_eq!(vcs::apply_operation(&doc, &missing_frame_remove), doc);
            assert!(missing_frame_remove.backwards(&doc).is_empty());

            let missing_page_patch = LayoutOperation::PatchFrame { page_id: "no-page".into(), frame_id: "frame-1".into(), patch: FramePatch { x: Some(1.0), ..Default::default() } };
            assert_eq!(vcs::apply_operation(&doc, &missing_page_patch), doc);
            assert!(missing_page_patch.backwards(&doc).is_empty());

            let missing_frame_patch = LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "no-frame".into(), patch: FramePatch { x: Some(1.0), ..Default::default() } };
            assert_eq!(vcs::apply_operation(&doc, &missing_frame_patch), doc);
            assert!(missing_frame_patch.backwards(&doc).is_empty());
        }
    }
    // #endregion operations
}

mod dsl {
    //#region 🔖Dsl
    //! 🔤 `LayoutDocument`'s `store::DocumentDsl` and `LayoutOperation`'s `store::OpText` are now generated
    //! by the `dsl::` derive engine (see `dsl_derive`/`dsl_schema`) instead of a hand-rolled
    //! tokenizer/parser: every nested type in `crate::document` derives `dsl::DslRecord`/`dsl::DslEnum`
    //! directly, and `LayoutDocument` itself derives `dsl::DslDocument` (see `mod document`). Only
    //! `LayoutOperation` needs anything in this module: `vcs::CollectionOperation<K,V,P>` (used by its
    //! `Pages`/`Stories`/`Links` variants) is declared in the `vcs` crate, so it can't gain a
    //! `dsl::DslField`/`dsl::DslVariants` binding here (orphan rule) — `LayoutOperationDsl` is a local,
    //! DSL-only mirror that flattens each collection wrapper into its own keyworded variants, mirroring
    //! `process_3d::Process3dOperationDsl`'s identical fix for the same foreign-type problem.
    //! `FramePatch.fill`/`.stroke` (`Option<Option<[f32;4]>>`) has the same "no direct binding" issue
    //! one level down — `ColorPatch`/`FramePatchDsl` fix that the same way, converting only at the
    //! `patchFrame` op boundary; `FramePatch` itself is untouched so `layout/plugin` keeps compiling.

    use crate::document::*;
    use crate::operations::*;
    use protocol::{CollectionOperation, OpText};
    use store::TextError;

    //#region 🔖FramePatchDsl
    /// 🎨 3-state tag standing in for `FramePatch.fill`/`.stroke`'s `Option<Option<[f32;4]>>` — the DSL
    /// engine's plain `Option<T>` can only express "untouched vs present"; `Clear` carries the doubly-
    /// optional field's "explicitly cleared to none" state that a single `Option` can't.
    #[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
    enum ColorPatch {
        Clear,
        Set { color: [f32; 4] },
    }

    /// 🩹 DSL-only mirror of `FramePatch` — only `fill`/`stroke` differ from the real type (see
    /// `ColorPatch`); every other field passes through unchanged. Never fixture-visible (only ever
    /// appears inside a `patchFrame` op line), so its own shape has no compatibility obligation beyond
    /// its own round trip.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
    struct FramePatchDsl {
        x: Option<f64>,
        y: Option<f64>,
        width: Option<f64>,
        height: Option<f64>,
        #[dsl(statements, block)]
        fill: Option<ColorPatch>,
        #[dsl(statements, block)]
        stroke: Option<ColorPatch>,
        wrap_mode: Option<String>,
        columns: Option<u32>,
    }

    fn frame_patch_to_dsl(patch: &FramePatch) -> FramePatchDsl {
        FramePatchDsl {
            x: patch.x,
            y: patch.y,
            width: patch.width,
            height: patch.height,
            fill: match patch.fill {
                None => None,
                Some(None) => Some(ColorPatch::Clear),
                Some(Some(color)) => Some(ColorPatch::Set { color }),
            },
            stroke: match patch.stroke {
                None => None,
                Some(None) => Some(ColorPatch::Clear),
                Some(Some(color)) => Some(ColorPatch::Set { color }),
            },
            wrap_mode: patch.wrap_mode.clone(),
            columns: patch.columns,
        }
    }

    fn frame_patch_from_dsl(patch: FramePatchDsl) -> FramePatch {
        FramePatch {
            x: patch.x,
            y: patch.y,
            width: patch.width,
            height: patch.height,
            fill: match patch.fill {
                None => None,
                Some(ColorPatch::Clear) => Some(None),
                Some(ColorPatch::Set { color }) => Some(Some(color)),
            },
            stroke: match patch.stroke {
                None => None,
                Some(ColorPatch::Clear) => Some(None),
                Some(ColorPatch::Set { color }) => Some(Some(color)),
            },
            wrap_mode: patch.wrap_mode,
            columns: patch.columns,
        }
    }
    //#endregion 🔖FramePatchDsl
    //#endregion 🔖Dsl

    //#region 🔖OpText
    /// ⚡ DSL-only mirror of `LayoutOperation` — see this module's opening doc comment. Converts at the
    /// `store::OpText` boundary only; `LayoutOperation` itself (and every consumer matching on it, e.g.
    /// `layout/plugin`) is completely untouched.
    #[derive(Clone, Debug, PartialEq, dsl::DslOps)]
    enum LayoutOperationDsl {
        PagesAdd {
            index: usize,
            #[dsl(block)]
            item: Page,
        },
        PagesRemove { id: String },
        PagesMove {
            id: String,
            to_index: usize,
        },
        PagesPatch {
            id: String,
            #[dsl(block)]
            patch: PagePatch,
        },
        StoriesAdd {
            index: usize,
            #[dsl(block)]
            item: TextStory,
        },
        StoriesRemove { id: String },
        StoriesMove {
            id: String,
            to_index: usize,
        },
        StoriesPatch {
            id: String,
            #[dsl(block)]
            patch: TextStoryPatch,
        },
        LinksAdd {
            index: usize,
            #[dsl(block)]
            item: ImageLink,
        },
        LinksRemove { id: String },
        LinksMove {
            id: String,
            to_index: usize,
        },
        LinksPatch {
            id: String,
            #[dsl(block)]
            patch: ImageLinkPatch,
        },
        AddFrame {
            page_id: String,
            index: usize,
            #[dsl(statements)]
            frame: Box<Frame>,
            layer_id: Option<String>,
        },
        RemoveFrame {
            page_id: String,
            frame_id: String,
        },
        PatchFrame {
            page_id: String,
            frame_id: String,
            #[dsl(block)]
            patch: FramePatchDsl,
        },
        SetCamera {
            blueprint: bool,
            #[dsl(block)]
            camera: LayoutCamera,
        },
    }

    fn layout_operation_to_dsl(operation: &LayoutOperation) -> LayoutOperationDsl {
        match operation {
            LayoutOperation::Pages(CollectionOperation::Add { id: _id, item, at }) => LayoutOperationDsl::PagesAdd { index: *at, item: item.clone() },
            LayoutOperation::Pages(CollectionOperation::Remove { id }) => LayoutOperationDsl::PagesRemove { id: id.clone() },
            LayoutOperation::Pages(CollectionOperation::Move { id, to }) => LayoutOperationDsl::PagesMove { id: id.clone(), to_index: *to },
            LayoutOperation::Pages(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::PagesPatch { id: id.clone(), patch: patch.clone() },
            LayoutOperation::Stories(CollectionOperation::Add { id: _id, item, at }) => LayoutOperationDsl::StoriesAdd { index: *at, item: item.clone() },
            LayoutOperation::Stories(CollectionOperation::Remove { id }) => LayoutOperationDsl::StoriesRemove { id: id.clone() },
            LayoutOperation::Stories(CollectionOperation::Move { id, to }) => LayoutOperationDsl::StoriesMove { id: id.clone(), to_index: *to },
            LayoutOperation::Stories(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::StoriesPatch { id: id.clone(), patch: patch.clone() },
            LayoutOperation::Links(CollectionOperation::Add { id: _id, item, at }) => LayoutOperationDsl::LinksAdd { index: *at, item: item.clone() },
            LayoutOperation::Links(CollectionOperation::Remove { id }) => LayoutOperationDsl::LinksRemove { id: id.clone() },
            LayoutOperation::Links(CollectionOperation::Move { id, to }) => LayoutOperationDsl::LinksMove { id: id.clone(), to_index: *to },
            LayoutOperation::Links(CollectionOperation::Patch { id, patch }) => LayoutOperationDsl::LinksPatch { id: id.clone(), patch: patch.clone() },
            LayoutOperation::AddFrame { page_id, index, frame, layer_id } => {
                LayoutOperationDsl::AddFrame { page_id: page_id.clone(), index: *index, frame: Box::new(frame.clone()), layer_id: layer_id.clone() }
            }
            LayoutOperation::RemoveFrame { page_id, frame_id } => LayoutOperationDsl::RemoveFrame { page_id: page_id.clone(), frame_id: frame_id.clone() },
            LayoutOperation::PatchFrame { page_id, frame_id, patch } => {
                LayoutOperationDsl::PatchFrame { page_id: page_id.clone(), frame_id: frame_id.clone(), patch: frame_patch_to_dsl(patch) }
            }
            LayoutOperation::SetCamera { blueprint, camera } => LayoutOperationDsl::SetCamera { blueprint: *blueprint, camera: camera.clone() },
        }
    }

    fn layout_operation_from_dsl(operation: LayoutOperationDsl) -> LayoutOperation {
        match operation {
            LayoutOperationDsl::PagesAdd { index, item } => LayoutOperation::Pages(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
            LayoutOperationDsl::PagesRemove { id } => LayoutOperation::Pages(CollectionOperation::Remove { id }),
            LayoutOperationDsl::PagesMove { id, to_index } => LayoutOperation::Pages(CollectionOperation::Move { id, to: to_index }),
            LayoutOperationDsl::PagesPatch { id, patch } => LayoutOperation::Pages(CollectionOperation::Patch { id, patch }),
            LayoutOperationDsl::StoriesAdd { index, item } => LayoutOperation::Stories(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
            LayoutOperationDsl::StoriesRemove { id } => LayoutOperation::Stories(CollectionOperation::Remove { id }),
            LayoutOperationDsl::StoriesMove { id, to_index } => LayoutOperation::Stories(CollectionOperation::Move { id, to: to_index }),
            LayoutOperationDsl::StoriesPatch { id, patch } => LayoutOperation::Stories(CollectionOperation::Patch { id, patch }),
            LayoutOperationDsl::LinksAdd { index, item } => LayoutOperation::Links(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
            LayoutOperationDsl::LinksRemove { id } => LayoutOperation::Links(CollectionOperation::Remove { id }),
            LayoutOperationDsl::LinksMove { id, to_index } => LayoutOperation::Links(CollectionOperation::Move { id, to: to_index }),
            LayoutOperationDsl::LinksPatch { id, patch } => LayoutOperation::Links(CollectionOperation::Patch { id, patch }),
            LayoutOperationDsl::AddFrame { page_id, index, frame, layer_id } => LayoutOperation::AddFrame { page_id, index, frame: *frame, layer_id },
            LayoutOperationDsl::RemoveFrame { page_id, frame_id } => LayoutOperation::RemoveFrame { page_id, frame_id },
            LayoutOperationDsl::PatchFrame { page_id, frame_id, patch } => LayoutOperation::PatchFrame { page_id, frame_id, patch: frame_patch_from_dsl(patch) },
            LayoutOperationDsl::SetCamera { blueprint, camera } => LayoutOperation::SetCamera { blueprint, camera },
        }
    }

    impl OpText for LayoutOperation {
        fn parse_op(line: &str) -> Result<Self, TextError> {
            Ok(layout_operation_from_dsl(<LayoutOperationDsl as OpText>::parse_op(line)?))
        }

        fn print_op(&self) -> String {
            <LayoutOperationDsl as OpText>::print_op(&layout_operation_to_dsl(self))
        }
    }

    /// ⚡ Binary mirror of the `OpText` impl above — `LayoutOperationDsl` already derives
    /// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
    impl protocol::OpBinary for LayoutOperation {
        fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
            layout_operation_to_dsl(self).encode_op()
        }

        fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
            Ok(layout_operation_from_dsl(LayoutOperationDsl::decode_op(bytes)?))
        }
    }
    //#endregion 🔖OpText

    #[cfg(test)]
    mod tests {
        use super::*;
        use store::{create_document_envelope, test_support, DocumentDsl, DocumentCommand, DocumentStore};

        fn sample_document() -> LayoutDocument {
            LayoutDocument::parse_dsl(include_str!("../example/sample.layout")).expect("sample fixture parses")
        }

        #[test]
        fn dsl_round_trips_sample_fixture() {
            let doc = sample_document();
            assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
            assert_eq!(doc.pages.len(), 2);
            test_support::assert_dsl_round_trip(&doc);
            test_support::assert_dsl_pack_equivalence(&doc);
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
            test_support::assert_dsl_pack_equivalence(&doc);
        }

        #[test]
        fn op_text_round_trips_every_layout_operation_variant() {
            let doc = sample_document();

            let mut page_2 = doc.pages[0].clone();
            page_2.id = "page-3".into();
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Add { id: page_2.id.clone(), item: page_2, at: 1 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Remove { id: "page-1".into() }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Move { id: "page-1".into(), to: 1 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch {
                id: "page-1".into(),
                patch: PagePatch { name: Some("Renamed".into()), width: Some(300.0), columns_count: Some(3), ..Default::default() },
            }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch::default() }));

            let mut story_2 = doc.stories[0].clone();
            story_2.id = "story-2".into();
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Add { id: story_2.id.clone(), item: story_2, at: 1 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Remove { id: "story-1".into() }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Move { id: "story-1".into(), to: 0 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: Some("Edited".into()) } }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Stories(CollectionOperation::Patch { id: "story-1".into(), patch: TextStoryPatch { content: None } }));

            let mut link_2 = doc.links[0].clone();
            link_2.id = "link-2".into();
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Add { id: link_2.id.clone(), item: link_2, at: 1 }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Remove { id: "link-missing".into() }));
            test_support::assert_op_line_round_trip(&LayoutOperation::Links(CollectionOperation::Move { id: "link-missing".into(), to: 0 }));
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
            let envelope = create_document_envelope(LAYOUT_FIXTURE_SCHEMA, "layout-doc-text-test", initial, None);
            let mut store: DocumentStore<LayoutDocument, LayoutOperation> = DocumentStore::new(envelope);
            store
                .dispatch(DocumentCommand::Apply { operations: vec![LayoutOperation::SetCamera { blueprint: true, camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 } }], description: Some("pan camera".into()) })
                .expect("apply set camera");
            store
                .dispatch(DocumentCommand::Apply {
                    operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } })],
                    description: Some("rename page".into()),
                })
                .expect("apply patch page");
            test_support::assert_document_text_round_trip(&store);
            test_support::assert_document_pack_round_trip(&store);
            test_support::assert_live_equals_replay(&store);
        }

        #[test]
        fn dsl_round_trips_overrides_frame_flags_and_absent_print_target() {
            let doc = LayoutDocument {
                schema: LAYOUT_FIXTURE_SCHEMA.into(),
                name: "Flags".into(),
                camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
                preview_camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
                grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
                paragraph_styles: Vec::new(),
                character_styles: Vec::new(),
                stories: Vec::new(),
                links: Vec::new(),
                parent_pages: Vec::new(),
                spreads: Vec::new(),
                pages: vec![Page {
                    id: "page-1".into(),
                    name: "Page".into(),
                    spread_id: "spread-1".into(),
                    parent_page_id: None,
                    width: 100.0,
                    height: 100.0,
                    margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                    columns: PageColumns { count: 1, gutter: 0.0 },
                    guides: Vec::new(),
                    layer_ids: vec!["layer-1".into()],
                    layers: vec![Layer { id: "layer-1".into(), name: "Content".into(), visible: true, locked: false, object_ids: vec!["frame-locked".into(), "frame-unlocked".into()] }],
                    frames: vec![
                        Frame::Rect { id: "frame-locked".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: Some(true), visible: Some(false), fill: None, stroke: None },
                        Frame::Rect { id: "frame-unlocked".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: Some(false), visible: Some(true), fill: None, stroke: None },
                    ],
                    overrides: vec![
                        PageOverride { object_id: "frame-locked".into(), bounds: Some(LayoutBounds { x: 1.0, y: 2.0, width: 3.0, height: 4.0, rotation: 5.0 }), visible: Some(true), locked: Some(false) },
                        PageOverride { object_id: "frame-unlocked".into(), bounds: None, visible: None, locked: None },
                    ],
                }],
                print_target: None,
            };
            test_support::assert_dsl_round_trip(&doc);
            test_support::assert_dsl_pack_equivalence(&doc);
        }

        #[test]
        fn op_text_round_trips_full_page_and_frame_patch_fields() {
            let full_page_patch = PagePatch { name: Some("Renamed".into()), width: Some(300.0), height: Some(400.0), margin_top: Some(1.0), margin_right: Some(2.0), margin_bottom: Some(3.0), margin_left: Some(4.0), columns_count: Some(5), columns_gutter: Some(6.0) };
            test_support::assert_op_line_round_trip(&LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: full_page_patch }));

            let full_frame_patch = FramePatch { x: Some(1.0), y: Some(2.0), width: Some(3.0), height: Some(4.0), fill: Some(Some([0.1, 0.2, 0.3, 0.4])), stroke: Some(None), wrap_mode: Some("column".into()), columns: Some(3) };
            test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), patch: full_frame_patch });

            let clearing_frame_patch = FramePatch { fill: Some(None), stroke: Some(Some([0.5, 0.5, 0.5, 1.0])), ..Default::default() };
            test_support::assert_op_line_round_trip(&LayoutOperation::PatchFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), patch: clearing_frame_patch });
        }

        #[test]
        fn parse_dsl_reports_engine_parser_errors() {
            // The hand-rolled lexer/parser (and its bespoke error messages) is gone — parsing now goes
            // through the `dsl::` derive engine directly, so these assert only on the public
            // `store::DocumentDsl`/`store::OpText` surface, generically on failure rather than on exact
            // internal wording that no longer exists.
            assert!(LayoutDocument::parse_dsl("").is_err(), "empty text must fail: a document has required fields");
            assert!(LayoutDocument::parse_dsl("not a document at all").is_err(), "unrecognized leading token must fail");
            assert!(LayoutDocument::parse_dsl("schema=\"layout.fixture\" name=\"t\"").is_err(), "quoted schema must fail: schema is a bare ident");
            assert!(LayoutDocument::parse_dsl("schema=layout.fixture name=unquoted").is_err(), "unquoted name must fail: name is a quoted string");
            assert!(
                LayoutDocument::parse_dsl("schema=layout.fixture name=\"t\" camera { x=notanumber y=0 zoom=1 }").is_err(),
                "non-numeric camera field must fail"
            );
            let bad_bool = "schema=layout.fixture name=\"t\" camera { x=0 y=0 zoom=1 } previewCamera { x=0 y=0 zoom=1 } grid { baselineGrid=12 baselineOffset=0 snapToBaseline=maybe }";
            assert!(LayoutDocument::parse_dsl(bad_bool).is_err(), "non-boolean grid flag must fail");
            assert!(LayoutOperation::parse_op("setCamera blueprint=true camera=1,2,3").is_err(), "camera must be a block, not a bare tuple attribute");
        }

        #[test]
        fn op_text_rejects_unknown_operation_name() {
            assert!(LayoutOperation::parse_op("bogusOp id=x").is_err(), "unknown op keyword must fail");
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
