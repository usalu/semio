use serde::{Deserialize, Serialize};

pub const LAYOUT_FIXTURE_SCHEMA: &str = "layout.fixture";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutRect {
    pub x: f64,
    pub y: f64,
    #[serde(rename = "w")]
    pub width: f64,
    #[serde(rename = "h")]
    pub height: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutBounds {
    pub x: f64,
    pub y: f64,
    #[serde(rename = "w")]
    pub width: f64,
    #[serde(rename = "h")]
    pub height: f64,
    pub rotation: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageMargins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageColumns {
    pub count: u32,
    pub gutter: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    #[serde(rename = "objectIds")]
    pub object_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FrameKind {
    #[serde(rename = "rect")]
    Rect,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrameBase {
    pub id: String,
    #[serde(rename = "layerId")]
    pub layer_id: String,
    pub bounds: LayoutBounds,
    pub locked: Option<bool>,
    pub visible: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RectFrame {
    #[serde(flatten)]
    pub base: FrameBase,
    pub fill: Option<[f32; 4]>,
    pub stroke: Option<[f32; 4]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageFrame {
    #[serde(flatten)]
    pub base: FrameBase,
    #[serde(rename = "linkId")]
    pub link_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextStyleRun {
    pub start: usize,
    pub end: usize,
    #[serde(rename = "paragraphStyleId")]
    pub paragraph_style_id: Option<String>,
    #[serde(rename = "characterStyleId")]
    pub character_style_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextStory {
    pub id: String,
    pub content: String,
    #[serde(rename = "styleRuns")]
    pub style_runs: Vec<TextStyleRun>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageOverride {
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub bounds: Option<LayoutBounds>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spread {
    pub id: String,
    pub name: String,
    #[serde(rename = "pageIds")]
    pub page_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GridSettings {
    #[serde(rename = "baselineGrid")]
    pub baseline_grid: f64,
    #[serde(rename = "baselineOffset")]
    pub baseline_offset: f64,
    #[serde(rename = "snapToBaseline")]
    pub snap_to_baseline: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

pub fn parse_layout_document(json: &str) -> Result<LayoutDocument, String> {
    let doc: LayoutDocument = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if doc.schema != LAYOUT_FIXTURE_SCHEMA {
        return Err(format!("unexpected schema {}", doc.schema));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_document() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[]}"#;
        let doc = parse_layout_document(json).expect("parse");
        assert_eq!(doc.name, "t");
    }

    #[test]
    fn frame_kind_tag_discriminates_variant() {
        let json = r#"{"id":"frame-text-1","layerId":"layer-1","kind":"text","bounds":{"x":36,"y":120,"w":240,"h":200,"rotation":0},"storyId":"story-1","threadNext":"frame-text-2","columns":1,"inset":{"x":4,"y":4,"w":232,"h":192},"wrapMode":"box"}"#;
        let frame: Frame = serde_json::from_str(json).unwrap();
        assert!(matches!(frame, Frame::Text { .. }));
        assert_eq!(frame.kind_str(), "text");
    }
}
