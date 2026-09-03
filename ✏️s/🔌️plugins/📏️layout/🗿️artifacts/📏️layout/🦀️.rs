//! 📄️ Layout artifact — the document entity the layout app edits (constitutional: general).

use protocol::{Identified, Patchable};
use semio_framework::{Dialect, StandardId, SubsetId};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
pub const LAYOUT_DOCUMENT_SCHEMA: &str = "layout.layout";

/// 🪪️ This artifact's compile-time surface coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level (not
/// under `✏️editor`/`👁️viewer`) specifically so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind = "s.layout.layout"` matches
/// `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`'s own `#[artifact_schema(id =
/// "s.layout.layout")]`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is `s.layout.layout@1/*#editor`
/// / `s.layout.layout@1/*#viewer`, exactly the contract §1 grammar. NOT the same type as
/// `store::os_io::ArtifactDialect` used a few lines below by `background_drawing_child_handle` (a
/// wire `ArtifactDialect` describing STDIO's drawing subset for composition purposes, unrelated to
/// this artifact's own surface identity) — this is the SDK's compile-time `Dialect`.
pub const LAYOUT_DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Constants

//#region 🔖️ComposedTypes
/// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4 (design map row `layout | C:drawing
/// R:model`): layout composes stdio's real `s.stdio.semio/v1/drawing` subset as a genuine persisted
/// child (`LayoutSnapshot::background_drawing`) instead of only ever computing one ephemerally at
/// SVG-export time (`io::layout_snapshot_to_semio_drawing`, unchanged, still the authored-frames→
/// drawing direction) — DWG/DXF/SVG import used to discard everything but page-boundary rectangles
/// (`io::dwg_drawing_to_semio_drawing`/`layout_document_json_from_dwg` only ever read `path_bounds`
/// back out of the drawing they built, then threw it away); it now mints a real content-addressed
/// child from the FULL decoded drawing instead — see `io::background_drawing_child_from_import`.
/// `LayoutSnapshot::referenced_model` is a forward `ArtifactLink` reference slot for the same design
/// map row's `R:model` half — layout pages can reference an architecture `model` artifact (e.g. a
/// floor plan a sheet is traced from) without owning/duplicating its content. This is genuinely new
/// capability (no prior inline `model`-shaped duplication existed anywhere in this plugin to remove —
/// confirmed by grep before this migration started); left schema/codec-complete but otherwise inert
/// (no `LinkResolver` seam, no mutation dispatch) — same documented-gap posture the migration recipe
/// sanctions for any composed slot a plugin agent can't wire a live resolver into yet.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutDrawingChild {
    pub handle: store::ArtifactChild<SemioDrawingSnapshot>,
    pub content: SemioDrawingSnapshot,
}

/// 🧪️ Content-addressed child-handle mint (mirrors cad's `cad_model_child_handle` exactly) — hashes
/// the drawing content being wrapped so peers converge on replay instead of minting a random id.
/// `source_tag` disambiguates which import path produced the content (`"dwg"`/`"dxf"`/`"svg"`) so two
/// different-format imports of otherwise-identical geometry don't collide on the same child id.
pub async fn background_drawing_child_handle(source_tag: &str, content: &SemioDrawingSnapshot) -> LayoutDrawingChild {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(content).expect("SemioDrawingSnapshot is always JSON-serializable");
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("background-drawing-{source_tag}-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("layout-background-drawing-{source_tag}"), dialect };
    LayoutDrawingChild { handle: store::ArtifactChild::new(child_id, target), content: content.clone() }
}
//#endregion 🔖️ComposedTypes

/// 🔎️ The one accessor every render/export call site funnels through; `None` only when the
/// document has no snapshot-owned `background_drawing` record.
pub async fn background_drawing_content(snapshot: &LayoutSnapshot) -> Option<SemioDrawingSnapshot> {
    snapshot.background_drawing.as_ref().map(|child| child.content.clone())
}

//#region 🔖️DropPreview
/// 👻️ Ephemeral catalogue drag-ghost state (layout app config / artifact local-ui).
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutDropPreviewState {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}
//#endregion 🔖️DropPreview

//#region 🔖️Types
/// 📷️ Ephemeral per-surface camera pose (blueprint/preview). Never part of `LayoutSnapshot` — lives
/// in the layout app's `LayoutConfig` instead, so it stays out of undo history and off the wire.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
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

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct LayoutRect {
    pub x: f64,
    pub y: f64,
    #[cfg_attr(test, serde(rename = "w"))]
    #[value(rename = "w")]
    pub width: f64,
    #[cfg_attr(test, serde(rename = "h"))]
    #[value(rename = "h")]
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct LayoutBounds {
    pub x: f64,
    pub y: f64,
    #[cfg_attr(test, serde(rename = "w"))]
    #[value(rename = "w")]
    pub width: f64,
    #[cfg_attr(test, serde(rename = "h"))]
    #[value(rename = "h")]
    pub height: f64,
    pub rotation: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct PageMargins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct PageColumns {
    pub count: u32,
    pub gutter: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Layer {
    #[dsl(defines = "layer")]
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    #[cfg_attr(test, serde(rename = "objectIds"))]
    #[value(rename = "objectIds")]
    pub object_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslEnum, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind"))]
#[value(tag = "kind")]
pub enum Frame {
    #[cfg_attr(test, serde(rename = "rect"))]
    #[value(rename = "rect")]
    Rect {
        #[dsl(defines = "frame")]
        id: String,
        #[cfg_attr(test, serde(rename = "layerId"))]
        #[value(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        fill: Option<[f32; 4]>,
        stroke: Option<[f32; 4]>,
    },
    #[cfg_attr(test, serde(rename = "text"))]
    #[value(rename = "text")]
    Text {
        #[dsl(defines = "frame")]
        id: String,
        #[cfg_attr(test, serde(rename = "layerId"))]
        #[value(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[cfg_attr(test, serde(rename = "storyId"))]
        #[value(rename = "storyId")]
        #[dsl(refs = "story")]
        story_id: String,
        #[cfg_attr(test, serde(rename = "threadNext"))]
        #[value(rename = "threadNext")]
        #[dsl(refs = "frame")]
        thread_next: Option<String>,
        columns: u32,
        #[dsl(block)]
        inset: LayoutRect,
        #[cfg_attr(test, serde(rename = "wrapMode"))]
        #[value(rename = "wrapMode")]
        wrap_mode: String,
    },
    #[cfg_attr(test, serde(rename = "image"))]
    #[value(rename = "image")]
    Image {
        #[dsl(defines = "frame")]
        id: String,
        #[cfg_attr(test, serde(rename = "layerId"))]
        #[value(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[cfg_attr(test, serde(rename = "linkId"))]
        #[value(rename = "linkId")]
        #[dsl(refs = "link")]
        link_id: String,
    },
}

impl Frame {
    pub async fn id(&self) -> &str {
        match self {
            Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. } => id,
        }
    }

    pub async fn bounds(&self) -> &LayoutBounds {
        match self {
            Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
        }
    }

    pub async fn kind_str(&self) -> &str {
        match self {
            Frame::Rect { .. } => "rect",
            Frame::Text { .. } => "text",
            Frame::Image { .. } => "image",
        }
    }

    pub async fn visible(&self) -> bool {
        match self {
            Frame::Rect { visible, .. } | Frame::Text { visible, .. } | Frame::Image { visible, .. } => visible.unwrap_or(true),
        }
    }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyleRun {
    pub start: usize,
    pub end: usize,
    #[cfg_attr(test, serde(rename = "paragraphStyleId"))]
    #[value(rename = "paragraphStyleId")]
    #[dsl(refs = "paragraph-style")]
    pub paragraph_style_id: Option<String>,
    #[cfg_attr(test, serde(rename = "characterStyleId"))]
    #[value(rename = "characterStyleId")]
    #[dsl(refs = "character-style")]
    pub character_style_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct TextStory {
    #[dsl(defines = "story")]
    pub id: String,
    pub content: String,
    #[cfg_attr(test, serde(rename = "styleRuns"))]
    #[value(rename = "styleRuns")]
    #[dsl(table)]
    pub style_runs: Vec<TextStyleRun>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphStyle {
    #[dsl(defines = "paragraph-style")]
    pub id: String,
    pub name: String,
    #[cfg_attr(test, serde(rename = "fontFamily"))]
    #[value(rename = "fontFamily")]
    pub font_family: String,
    #[cfg_attr(test, serde(rename = "fontSize"))]
    #[value(rename = "fontSize")]
    pub font_size: f64,
    #[cfg_attr(test, serde(rename = "fontWeight"))]
    #[value(rename = "fontWeight")]
    pub font_weight: u32,
    pub leading: f64,
    pub tracking: f64,
    pub alignment: String,
}

/// 🔤️ A named run-level style override (bold/italic/color emphasis) applied on top of a
/// {@link ParagraphStyle} via {@link TextStyleRun.character_style_id}. Unlike `ParagraphStyle`,
/// every field besides `id` is optional: a character style typically overrides only one or two
/// attributes and inherits the rest from the paragraph it's layered onto.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CharacterStyle {
    #[dsl(defines = "character-style")]
    pub id: String,
    pub name: Option<String>,
    #[cfg_attr(test, serde(rename = "fontFamily"))]
    #[value(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[cfg_attr(test, serde(rename = "fontSize"))]
    #[value(rename = "fontSize")]
    pub font_size: Option<f64>,
    #[cfg_attr(test, serde(rename = "fontWeight"))]
    #[value(rename = "fontWeight")]
    pub font_weight: Option<u32>,
    pub italic: Option<bool>,
    pub color: Option<[f32; 4]>,
    pub tracking: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ImageLink {
    #[dsl(defines = "link")]
    pub id: String,
    pub path: String,
    pub hash: String,
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
    #[cfg_attr(test, serde(rename = "colorProfile"))]
    #[value(rename = "colorProfile")]
    pub color_profile: Option<String>,
    pub state: Option<String>,
    #[cfg_attr(test, serde(rename = "proxyDataUrl"))]
    #[value(rename = "proxyDataUrl")]
    pub proxy_data_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct PageOverride {
    #[cfg_attr(test, serde(rename = "objectId"))]
    #[value(rename = "objectId")]
    #[dsl(refs = "frame")]
    pub object_id: String,
    #[dsl(block)]
    pub bounds: Option<LayoutBounds>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ParentPage {
    #[dsl(defines = "parent-page")]
    pub id: String,
    pub name: String,
    pub width: f64,
    pub height: f64,
    #[cfg_attr(test, serde(rename = "layerIds"))]
    #[value(rename = "layerIds")]
    pub layer_ids: Vec<String>,
    #[dsl(table)]
    pub layers: Vec<Layer>,
    #[dsl(statements, block)]
    pub frames: Vec<Frame>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Page {
    pub id: String,
    pub name: String,
    #[cfg_attr(test, serde(rename = "spreadId"))]
    #[value(rename = "spreadId")]
    #[dsl(refs = "spread")]
    pub spread_id: String,
    #[cfg_attr(test, serde(rename = "parentPageId"))]
    #[value(rename = "parentPageId")]
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
    #[cfg_attr(test, serde(rename = "layerIds"))]
    #[value(rename = "layerIds")]
    pub layer_ids: Vec<String>,
    #[dsl(table)]
    pub layers: Vec<Layer>,
    #[dsl(statements, block)]
    pub frames: Vec<Frame>,
    #[dsl(table)]
    pub overrides: Vec<PageOverride>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Spread {
    #[dsl(defines = "spread")]
    pub id: String,
    pub name: String,
    #[cfg_attr(test, serde(rename = "pageIds"))]
    #[value(rename = "pageIds")]
    pub page_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct GridSettings {
    #[cfg_attr(test, serde(rename = "baselineGrid"))]
    #[value(rename = "baselineGrid")]
    pub baseline_grid: f64,
    #[cfg_attr(test, serde(rename = "baselineOffset"))]
    #[value(rename = "baselineOffset")]
    pub baseline_offset: f64,
    #[cfg_attr(test, serde(rename = "snapToBaseline"))]
    #[value(rename = "snapToBaseline")]
    pub snap_to_baseline: bool,
}

//#endregion 🔖️Types

pub use crate::artifacts::layout::schema::diff::LayoutDiff;
pub use crate::artifacts::layout::schema::mutations::LayoutMutation;
pub use crate::artifacts::layout::schema::snapshot::LayoutSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::layout::create_layout_app`'s `🔖️Manifest` region.
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
/// `OnceLock`-backed `io_registry::entries()` convention used by `standards::v1::subsets::any::io::io_registry`.
/// Relocated from `⚙️engine/🦀️.rs` alongside `declaration()` (ticket
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
/// DUPLICATE registration of the exact `standards::v1::subsets::any::io::io_registry::entries()` slice the
/// `.composers(…)` call below now registers once (that top-level `artifacts::layout::io_registry`
/// module has no other caller in the repo — deleting its call here rather than keeping it, per the
/// W1b duplicate-IO-registration finding; the module itself is left in place as inert dead code,
/// matching `🗒️note`'s own unreferenced sibling module). `crate::editor::layout::config::schema::
/// register_app_schema()` is the one exception, still called from `📏️layout/🦀️.rs`'s own
/// `.setup()`: it registers the `LayoutPlayApp` CONFIG/PRESENCE schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in §6's artifact-scoped function set.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.layout.standard.v1", "standard", "1", &[], None),
        ("s.layout.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.layout.schema.artifact", "schema", "s.layout.layout", &[("schema", "s.layout.layout")], None),
        ("s.layout.inference.artifact", "inference", "s.layout.layout.inference", &[("schema", "s.layout.layout.inference")], None),
        // 🐛️ D2-capability-claim-repairs: `io_registry::entries()` registers THREE composer rows, not
        // two — the two below plus `composer_entry_of::<LayoutAnyComposer>()` (`🚪️io/🦀️.rs`),
        // whose `writes` is this artifact's own native dialect (`LAYOUT_DIALECT`, `s.layout@1/*`), the
        // same gap class `🗒️note` hit first (see that file's own `definition()` doc comment).
        ("s.layout.composer.layout", "composer", "s.layout@1/*", &[("dialect", "s.layout@1/*")], None),
        ("s.layout.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.layout.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.layout.grammar.document", "grammar", "layout.document", &[("grammar", "layout.document")], None),
        ("s.layout.grammar.op", "grammar", "layout.op", &[("grammar", "layout.op")], None),
        ("s.layout.grammar.diff", "grammar", "layout.diff", &[("grammar", "layout.diff")], None),
        ("s.layout.grammar.pack", "grammar", "layout.pack", &[("grammar", "layout.pack")], None),
        ("s.layout.grammar.spr", "grammar", "layout.spr", &[("grammar", "layout.spr")], None),
        ("s.layout.codec.document.v1", "codec", "layout.layout:layout", &[("codec", "layout.layout"), ("extension", "layout")], None),
        ("s.layout.localization.en", "localization", "Layout", &[], Some(("en", "Layout"))),
        ("s.layout.localization.de", "localization", "Layout", &[], Some(("de", "Layout"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.layout")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::layout::schema::layout_artifact_schema_descriptor())
        .inferences([crate::artifacts::layout::standards::v1::subsets::any::schema::inferences::layout_artifact_inference_descriptor()])
        .composers(crate::artifacts::layout::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::layout::LayoutPlayApp>>()
        .try_build()
}
//#endregion 🔖️ArtifactKind

//#region 🔖️CollectionSupport
impl Identified<String> for Page {
    async fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for TextStory {
    async fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for ImageLink {
    async fn id(&self) -> &String {
        &self.id
    }
}

/// 🌱️ Sparse "one frame was inserted into this page" fragment of a {@link PagePatch} — carries the
/// `create-frame` semantic mutation's payload verbatim plus the FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct PageFrameAdded {
    pub frame: Frame,
    pub index: Option<usize>,
    pub layer_id: Option<String>,
}

/// 🩹️ Sparse "one frame inside this page was field-patched" fragment of a {@link PagePatch} — carries
/// the `move-frame`/`resize-frame`/`change-frame-*` semantic mutations' shared payload shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct PageFramePatched {
    pub frame_id: String,
    pub patch: FramePatch,
}

/// 📄️ Sparse scalar patch for a {@link Page} (name, size, margins, columns, one nested frame
/// add/remove/field-patch). Never derives `dsl::DslRecord` — `frame_patched.patch` nests a
/// {@link FramePatch}, which itself can't bind (its doubly-optional `fill`/`stroke` fields have no
/// direct DSL-field mapping; see `🧬️mutations/📝️text/🦀️.rs`'s doc comment), so this type is
/// JSON-only like `FramePatch` itself.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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
async fn apply_frame_field_patch(frame: &mut Frame, patch: &FramePatch) {
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
    async fn apply_patch(&mut self, patch: &PagePatch) {
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

    async fn diff_patch(&self, other: &Self) -> Option<PagePatch> {
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
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct TextStoryPatch {
    pub content: Option<String>,
}

impl Patchable<TextStoryPatch> for TextStory {
    async fn apply_patch(&mut self, patch: &TextStoryPatch) {
        if let Some(content) = &patch.content {
            self.content = content.clone();
        }
    }

    async fn diff_patch(&self, other: &Self) -> Option<TextStoryPatch> {
        (self.content != other.content).then(|| TextStoryPatch { content: Some(other.content.clone()) })
    }
}

/// 🔗️ Sparse patch for an {@link ImageLink}'s file path.
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ImageLinkPatch {
    pub path: Option<String>,
}

impl Patchable<ImageLinkPatch> for ImageLink {
    async fn apply_patch(&mut self, patch: &ImageLinkPatch) {
        if let Some(path) = &patch.path {
            self.path = path.clone();
        }
    }

    async fn diff_patch(&self, other: &Self) -> Option<ImageLinkPatch> {
        (self.path != other.path).then(|| ImageLinkPatch { path: Some(other.path.clone()) })
    }
}

/// 🖼️ Sparse patch for a {@link Frame}: bounds for any kind, fill/stroke for rects, wrap-mode/columns
/// for text. The doubly-optional `fill`/`stroke` distinguishes "unchanged" (outer `None`) from
/// "cleared" (inner `None`). Needed both by `op`'s `PatchFrame` operation and by the DSL/spr mirror in
/// `spr` (`FramePatchDsl`), so it lives here alongside the other `*Patch` records rather than in `op`
/// itself. Frame patching is per-page nested rather than a flat collection-wide op, so unlike the
/// patches above it has no `Patchable` impl.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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

    async fn rect_frame(id: &str, visible: Option<bool>) -> Frame {
        Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible, fill: None, stroke: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn frame_helpers_report_id_bounds_kind_and_visibility() {
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
    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_uses_the_fixture_schema() {
        assert_eq!(artifact_kind().schema, LAYOUT_DOCUMENT_SCHEMA);
        assert_eq!(artifact_kind().source_format, LAYOUT_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
