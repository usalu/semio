//! 📄️ Layout artifact — the document entity the layout app edits (constitutional: general).

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const LAYOUT_DOCUMENT_SCHEMA: &str = "layout.layout";
//#endregion 🔖️Constants


//#region 🔖️DropPreview
/// 👻️ Ephemeral catalogue drag-ghost state (layout app config / artifact local-ui).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutDropPreviewState {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}
//#endregion 🔖️DropPreview

//#region 🔖️Types
/// 📷️ Ephemeral per-surface camera pose (blueprint/preview). Never part of `LayoutSnapshot` — lives
/// in the layout app's `LayoutConfig` instead, so it stays out of undo history and off the wire.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct LayoutCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for LayoutCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
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
    #[dsl(defines = "layer")]
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    #[serde(rename = "objectIds")]
    pub object_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind")]
pub enum Frame {
    #[serde(rename = "rect")]
    Rect {
        #[dsl(defines = "frame")]
        id: String,
        #[serde(rename = "layerId")]
        #[dsl(refs = "layer")]
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
        #[dsl(defines = "frame")]
        id: String,
        #[serde(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[serde(rename = "storyId")]
        #[dsl(refs = "story")]
        story_id: String,
        #[serde(rename = "threadNext")]
        #[dsl(refs = "frame")]
        thread_next: Option<String>,
        columns: u32,
        #[dsl(block)]
        inset: LayoutRect,
        #[serde(rename = "wrapMode")]
        wrap_mode: String,
    },
    #[serde(rename = "image")]
    Image {
        #[dsl(defines = "frame")]
        id: String,
        #[serde(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[serde(rename = "linkId")]
        #[dsl(refs = "link")]
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
    #[dsl(refs = "paragraph-style")]
    pub paragraph_style_id: Option<String>,
    #[serde(rename = "characterStyleId")]
    #[dsl(refs = "character-style")]
    pub character_style_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct TextStory {
    #[dsl(defines = "story")]
    pub id: String,
    pub content: String,
    #[serde(rename = "styleRuns")]
    #[dsl(table)]
    pub style_runs: Vec<TextStyleRun>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ParagraphStyle {
    #[dsl(defines = "paragraph-style")]
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

/// 🔤️ A named run-level style override (bold/italic/color emphasis) applied on top of a
/// {@link ParagraphStyle} via {@link TextStyleRun.character_style_id}. Unlike `ParagraphStyle`,
/// every field besides `id` is optional: a character style typically overrides only one or two
/// attributes and inherits the rest from the paragraph it's layered onto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CharacterStyle {
    #[dsl(defines = "character-style")]
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<f64>,
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<u32>,
    pub italic: Option<bool>,
    pub color: Option<[f32; 4]>,
    pub tracking: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ImageLink {
    #[dsl(defines = "link")]
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
    #[dsl(refs = "frame")]
    pub object_id: String,
    #[dsl(block)]
    pub bounds: Option<LayoutBounds>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ParentPage {
    #[dsl(defines = "parent-page")]
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
    #[dsl(refs = "spread")]
    pub spread_id: String,
    #[serde(rename = "parentPageId")]
    #[dsl(refs = "parent-page")]
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
    #[dsl(defines = "spread")]
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

//#endregion 🔖️Types

pub use crate::artifacts::layout::schema::snapshot::LayoutSnapshot;
pub use crate::artifacts::layout::schema::diff::LayoutDiff;
pub use crate::artifacts::layout::schema::mutations::LayoutMutation;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::layout::create_layout_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.layout".into(),
        name: "Layout".into(),
        source_format: LAYOUT_DOCUMENT_SCHEMA.into(),
        component_kind: "layout".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        schema: LAYOUT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.svg", "stdio.png"],
        import_stdio_kinds: vec!["stdio.svg", "stdio.png"],
    }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention used by `standards::v1::engine::io_registry`.
/// Relocated from `⚙️engine/🦀️component.rs` alongside `declaration()` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — `declaration()`'s only caller, kept private.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "layout.document",
                    extension: Some("layout"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::layout::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::layout::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("layout.document"),
                },
                dsl::LanguageSpec {
                    id: "layout.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::layout::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::layout::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("layout.op"),
                },
                dsl::LanguageSpec {
                    id: "layout.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::layout::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::layout::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("layout.diff"),
                },
                dsl::LanguageSpec {
                    id: "layout.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("layout.pack"),
                },
                dsl::LanguageSpec {
                    id: "layout.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("layout.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback, including `crate::artifacts::layout::io_registry::register()` — a
/// DUPLICATE registration of the exact `standards::v1::engine::io_registry::entries()` slice the
/// `.composers(…)` call below now registers once (that top-level `artifacts::layout::io_registry`
/// module has no other caller in the repo — deleting its call here rather than keeping it, per the
/// W1b duplicate-IO-registration finding; the module itself is left in place as inert dead code,
/// matching `🗒️note`'s own unreferenced sibling module). `crate::apps::layout::config::schema::
/// register_app_schema()` is the one exception, still called from `📏️layout/🦀️component.rs`'s own
/// `.setup()`: it registers the `LayoutPlayApp` CONFIG/PRESENCE schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in §6's artifact-scoped function set.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.layout")
        .schema(crate::artifacts::layout::schema::layout_artifact_schema_descriptor())
        .inferences([crate::artifacts::layout::standards::v1::subsets::any::schema::inferences::layout_artifact_inference_descriptor()])
        .composers(crate::artifacts::layout::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::layout::LayoutPlayApp>()
        .build()
}
//#endregion 🔖️ArtifactKind

//#region 🔖️CollectionSupport
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

/// 🌱️ Sparse "one frame was inserted into this page" fragment of a {@link PagePatch} — carries the
/// `create-frame` semantic mutation's payload verbatim plus the FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageFrameAdded {
    pub frame: Frame,
    pub index: Option<usize>,
    pub layer_id: Option<String>,
}

/// 🩹️ Sparse "one frame inside this page was field-patched" fragment of a {@link PagePatch} — carries
/// the `move-frame`/`resize-frame`/`change-frame-*` semantic mutations' shared payload shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageFramePatched {
    pub frame_id: String,
    pub patch: FramePatch,
}

/// 📄️ Sparse scalar patch for a {@link Page} (name, size, margins, columns, one nested frame
/// add/remove/field-patch). Never derives `dsl::DslRecord` — `frame_patched.patch` nests a
/// {@link FramePatch}, which itself can't bind (its doubly-optional `fill`/`stroke` fields have no
/// direct DSL-field mapping; see `🧬️mutations/📝️text/🦀️component.rs`'s doc comment), so this type is
/// JSON-only like `FramePatch` itself.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    pub frame_added: Option<PageFrameAdded>,
    pub frame_removed: Option<String>,
    pub frame_patched: Option<PageFramePatched>,
}

/// 🩹️ Pure field-apply for a {@link FramePatch} onto a {@link Frame} — no inverse capture (every
/// semantic mutation computes its own inverse from `base` directly; see `↩️inverse` triad leaves).
fn apply_frame_field_patch(frame: &mut Frame, patch: &FramePatch) {
    {
        let bounds = match frame {
            Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
        };
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
            if let Some(new) = patch.fill {
                *fill = new;
            }
            if let Some(new) = patch.stroke {
                *stroke = new;
            }
        }
        Frame::Text { wrap_mode, columns, .. } => {
            if let Some(new) = &patch.wrap_mode {
                *wrap_mode = new.clone();
            }
            if let Some(new) = patch.columns {
                *columns = new;
            }
        }
        Frame::Image { .. } => {}
    }
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
        if let Some(added) = &patch.frame_added {
            let at = added.index.unwrap_or(self.frames.len()).min(self.frames.len());
            self.frames.insert(at, added.frame.clone());
            if let Some(layer_id) = &added.layer_id {
                if let Some(layer) = self.layers.iter_mut().find(|layer| layer.id == *layer_id) {
                    layer.object_ids.push(added.frame.id().to_string());
                }
            }
        }
        if let Some(frame_id) = &patch.frame_removed {
            self.frames.retain(|frame| frame.id() != frame_id);
            for layer in &mut self.layers {
                layer.object_ids.retain(|id| id != frame_id);
            }
        }
        if let Some(entry) = &patch.frame_patched {
            if let Some(frame) = self.frames.iter_mut().find(|frame| frame.id() == entry.frame_id) {
                apply_frame_field_patch(frame, &entry.patch);
            }
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

/// 📝️ Sparse patch for a {@link TextStory}'s body content.
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

/// 🔗️ Sparse patch for an {@link ImageLink}'s file path.
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
/// "cleared" (inner `None`). Needed both by `op`'s `PatchFrame` operation and by the DSL/spr mirror in
/// `spr` (`FramePatchDsl`), so it lives here alongside the other `*Patch` records rather than in `op`
/// itself. Frame patching is per-page nested rather than a flat collection-wide op, so unlike the
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
//#endregion 🔖️CollectionSupport

//#region 🧪️Tests
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

        let text = Frame::Text {
            id: "frame-3".into(),
            layer_id: "layer-1".into(),
            bounds: LayoutBounds { x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0 },
            locked: None,
            visible: None,
            story_id: "story-1".into(),
            thread_next: None,
            columns: 1,
            inset: LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            wrap_mode: "box".into(),
        };
        assert_eq!(text.kind_str(), "text");

        let image = Frame::Image { id: "frame-4".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0 }, locked: None, visible: Some(true), link_id: "link-1".into() };
        assert_eq!(image.kind_str(), "image");
        assert!(image.visible());
    }

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` matches the store envelope schema for layout
    /// (unlike e.g. flow, layout uses the same string for both).
    #[test]
    fn artifact_kind_uses_the_fixture_schema() {
        assert_eq!(artifact_kind().schema, LAYOUT_DOCUMENT_SCHEMA);
        assert_eq!(artifact_kind().source_format, LAYOUT_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::layout::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("LayoutComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
