//! 📄 Layout app — document entities (constitutional: general).

use protocol::{Identified, Patchable};
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

//#region 🔖CollectionSupport
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

/// 📄 Sparse scalar patch for a {@link Page} (name, size, margins, columns).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
/// "cleared" (inner `None`). Needed both by `op`'s `PatchFrame` operation and by the DSL/OpText mirror
/// in `op` (`FramePatchDsl`), so it lives here alongside the other `*Patch` records rather than in `op`
/// itself. Frame patching is per-page nested rather than a flat `CollectionOperation`, so unlike the
/// patches above it has no `Patchable` impl.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
//#endregion 🔖CollectionSupport

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
