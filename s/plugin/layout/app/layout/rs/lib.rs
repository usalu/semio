//! 📄 Layout app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖Constants
pub const LAYOUT_FIXTURE_SCHEMA: &str = "layout.fixture";
//#endregion 🔖Constants

//#region 🔖Types
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
//#endregion 🔖Types

//#region 🧪Tests
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
}
//#endregion 🧪Tests
