//! ⚙️ Layout artifact — headless compute over the `LayoutSnapshot` projection (constitutional: engine).
//!
//! This node (plus its `🦀️scene.rs` sibling) is pure over `crate::artifacts::layout` types. The rule for
//! what lands here rather than next to a single caller: a helper with MORE THAN ONE consumer across the
//! taxonomy tree lives here; a helper with exactly one consumer lives in that consumer's component file.
//! View state (`LayoutConfig`) is an APP concern — see `crate::apps::layout::config`.

use crate::artifacts::layout::{
    Frame, GridSettings, ImageLink, Layer, LayoutBounds, LayoutRect, LayoutSnapshot, Page, PageColumns,
    PageMargins, ParagraphStyle, ParentPage, Spread, TextStory, LAYOUT_DOCUMENT_SCHEMA,
};
use serde_json::Value;

//#region ⚠️Errors
/// 🚧️ All fallible layout-engine operations funnel through this — document parsing, scene/hit-test
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
//#endregion ⚠️Errors

//#region 🔖️Register
/// 🗂️ Registers `LayoutSnapshot`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse layout
/// documents without depending on this crate's concrete `Projection`/`Mutation` types. Also registers
/// the 2D export handler and the DWG import handler. Called from the plugin root's `semio_plugin!{
/// setup: … }`.
pub fn register() {
    register_artifact_schema();
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::layout::LayoutPlayApp>(LAYOUT_DOCUMENT_SCHEMA);
    semio_framework_os::register_2d_export_handlers("2d.layout", "layout", layout_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.layout", layout_document_json_from_dwg);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "layout.document",
        extension: Some("layout"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::layout::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::layout::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("layout.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "layout.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::layout::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::layout::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("layout.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "layout.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::layout::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::layout::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("layout.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "layout.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::layout::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("layout.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "layout.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::layout::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("layout.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ Layout's typed media I/O surface (`AppDefinition.io`) — the implicit `document:in`/`document:out`
/// pair (keyed by the `2d.layout` artifact kind `create_layout_app` already declares) plus the two
/// WORKFLOWS-END-TO-END-TYPED-PORTS ports: `fields:in` (a `form.dictionary` this layout binds as a new
/// named data source — see `crate::artifacts::layout::LayoutSnapshot::data_fields_json`) and `layout:out`
/// (the current layout re-exported as `2d.layout` vector/SVG for a downstream consumer).
pub fn layout_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "layout.layout".into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "fields:in".into(),
                label: "Fields".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
                kind_id: Some("form.dictionary".into()),
                required: false,
                multiplicity: semio_framework::PortMultiplicity::One,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "layout:out".into(),
                label: "Layout".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
                kind_id: Some("2d.layout".into()),
                required: false,
                multiplicity: semio_framework::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.layout".into(), name: "2D Layout".into(), dimension: "2d".into(), component_kind: "layout".into() },
    }
}
//#endregion 🔖️Io

//#region 📄️Document
pub fn parse_layout_document(json: &str) -> Result<LayoutSnapshot, LayoutError> {
    let doc: LayoutSnapshot = serde_json::from_str(json)?;
    if doc.schema != LAYOUT_DOCUMENT_SCHEMA {
        return Err(LayoutError::UnexpectedSchema(doc.schema));
    }
    Ok(doc)
}

pub struct ResolvedFrame {
    pub frame: Frame,
    pub inherited: bool,
}

pub fn resolve_page<'a>(doc: &'a LayoutSnapshot, page: &'a Page) -> Vec<ResolvedFrame> {
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
//#endregion 📄️Document

//#region 🔖️DocumentHelpers
/// 📄️ The bundled sample fixture, parsed once — the source of truth for `LayoutPlayApp::initial_snapshot`
/// and the app manifest's `.example(...)` document.
pub fn default_document() -> LayoutSnapshot {
    build_demo_layout_snapshot()
}

fn build_demo_layout_snapshot() -> LayoutSnapshot {
    LayoutSnapshot {
        schema: LAYOUT_DOCUMENT_SCHEMA.into(),
        name: "Demo".into(),
        grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: true },
        paragraph_styles: vec![ParagraphStyle {
            id: "paragraph.body".into(),
            name: "Body".into(),
            font_family: "Layout Sans".into(),
            font_size: 12.0,
            font_weight: 400,
            leading: 14.4,
            tracking: 0.0,
            alignment: "left".into(),
        }],
        character_styles: Vec::new(),
        stories: vec![TextStory {
            id: "story-1".into(),
            content: "Hello layout".into(),
            style_runs: Vec::new(),
        }],
        links: vec![ImageLink {
            id: "link-missing".into(),
            path: "assets/missing.png".into(),
            hash: "sha256:missing".into(),
            width: 100,
            height: 100,
            dpi: 300,
            color_profile: None,
            state: Some("missing".into()),
            proxy_data_url: None,
        }],
        parent_pages: vec![ParentPage {
            id: "parent-1".into(),
            name: "Master".into(),
            width: 400.0,
            height: 500.0,
            layer_ids: vec!["layer-parent".into()],
            layers: vec![Layer {
                id: "layer-parent".into(),
                name: "Master".into(),
                visible: true,
                locked: false,
                object_ids: vec!["frame-inherited".into()],
            }],
            frames: vec![Frame::Rect {
                id: "frame-inherited".into(),
                layer_id: "layer-parent".into(),
                bounds: LayoutBounds { x: 50.0, y: 50.0, width: 100.0, height: 80.0, rotation: 0.0 },
                locked: None,
                visible: None,
                fill: None,
                stroke: Some([0.4, 0.5, 0.7, 0.8]),
            }],
        }],
        spreads: vec![Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids: vec!["page-1".into(), "page-2".into()] }],
        pages: vec![
            Page {
                id: "page-1".into(),
                name: "Page 1".into(),
                spread_id: "spread-1".into(),
                parent_page_id: Some("parent-1".into()),
                width: 400.0,
                height: 500.0,
                margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: vec!["layer-1".into()],
                layers: vec![Layer {
                    id: "layer-1".into(),
                    name: "Content".into(),
                    visible: true,
                    locked: false,
                    object_ids: vec!["frame-text-1".into(), "frame-image-1".into(), "frame-1".into()],
                }],
                frames: vec![
                    Frame::Text {
                        id: "frame-text-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: LayoutBounds { x: 156.0, y: 220.0, width: 80.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        story_id: "story-1".into(),
                        thread_next: None,
                        columns: 1,
                        inset: LayoutRect { x: 0.0, y: 0.0, width: 80.0, height: 40.0 },
                        wrap_mode: "box".into(),
                    },
                    Frame::Image {
                        id: "frame-image-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: LayoutBounds { x: 136.0, y: 435.0, width: 60.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        link_id: "link-missing".into(),
                    },
                    Frame::Rect {
                        id: "frame-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: LayoutBounds { x: 10.0, y: 10.0, width: 40.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        fill: Some([1.0, 1.0, 1.0, 1.0]),
                        stroke: None,
                    },
                ],
                overrides: Vec::new(),
            },
            Page {
                id: "page-2".into(),
                name: "Page 2".into(),
                spread_id: "spread-1".into(),
                parent_page_id: None,
                width: 400.0,
                height: 500.0,
                margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: Vec::new(),
                layers: Vec::new(),
                frames: Vec::new(),
                overrides: Vec::new(),
            },
        ],
        print_target: None,
        data_fields_json: None,
    }
}

/// 🌉️ JSON bridge for `semio_framework_plugin::App::example`, which hardcodes `serde_json::from_str`
/// on its `document_json` parameter (shared framework machinery, out of scope for this DSL migration) —
/// derives the JSON from the DSL fixture rather than keeping a second, redundant JSON copy of it on disk.
pub fn layout_sample_document_json() -> String {
    serde_json::to_string(&default_document()).unwrap_or_default()
}

/// 🎨️ Formats an optional RGBA color as a comma-separated text field value; two consumers
/// (`📌️panels/🔍️inspection` reads it, `🎮️commands/✏️author` parses it back via `text_to_rgba`).
pub fn rgba_to_text(color: &Option<[f32; 4]>) -> String {
    color.map(|channels| channels.iter().map(|channel| channel.to_string()).collect::<Vec<_>>().join(", ")).unwrap_or_default()
}

/// 🎨️ Parses a comma-separated `r, g, b, a` text field value back into an RGBA color, or `None` if it
/// does not have exactly four numeric components.
pub fn text_to_rgba(text: &str) -> Option<[f32; 4]> {
    let parts: Vec<f32> = text.split(',').filter_map(|part| part.trim().parse::<f32>().ok()).collect();
    (parts.len() == 4).then(|| [parts[0], parts[1], parts[2], parts[3]])
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️MediaImportExport
pub fn layout_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::pages_rects_svg(value, "Layout")
}

/// 📥️ Extracts axis-aligned rectangular boundaries from closed 4-vertex `LwPolyline`s and frames one page per rectangle, falling back to a single page framed to the drawing extents.
fn dwg_rect_pages(drawing: &semio_framework_os::DwgDrawing) -> Vec<(f64, f64, f64, f64)> {
    let mut rects = Vec::new();
    for entity in &drawing.entities {
        let semio_framework_os::DwgGeometry::LwPolyline { closed: true, vertices, .. } = &entity.geometry else { continue };
        if vertices.len() != 4 {
            continue;
        }
        let (min_x, max_x) = vertices.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), v| (min.min(v[0]), max.max(v[0])));
        let (min_y, max_y) = vertices.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), v| (min.min(v[1]), max.max(v[1])));
        let is_axis_aligned = vertices.iter().all(|v| ((v[0] - min_x).abs() < 1e-6 || (v[0] - max_x).abs() < 1e-6) && ((v[1] - min_y).abs() < 1e-6 || (v[1] - max_y).abs() < 1e-6));
        if is_axis_aligned && max_x > min_x && max_y > min_y {
            rects.push((min_x, min_y, max_x - min_x, max_y - min_y));
        }
    }
    if rects.is_empty() {
        rects.push((drawing.extmin[0], drawing.extmin[1], (drawing.extmax[0] - drawing.extmin[0]).max(1.0), (drawing.extmax[1] - drawing.extmin[1]).max(1.0)));
    }
    rects
}

/// 📥️ Builds a schema-valid layout document from a parsed DWG drawing, framing one page per rectangular boundary found.
pub fn layout_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let pages: Vec<Page> = dwg_rect_pages(drawing)
        .into_iter()
        .enumerate()
        .map(|(index, (_x, _y, width, height))| {
            let id = format!("page-{}", index + 1);
            let layer_id = format!("layer-{id}");
            Page {
                id,
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
    let document = LayoutSnapshot {
        schema: LAYOUT_DOCUMENT_SCHEMA.into(),
        name: "Imported DWG".into(),
        grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: vec![Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids }],
        pages,
        print_target: None,
        data_fields_json: None,
    };
    serde_json::to_value(document).map_err(|e| e.to_string())
}
//#endregion 🔖️MediaImportExport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn base_doc() -> LayoutSnapshot {
        LayoutSnapshot {
            schema: LAYOUT_DOCUMENT_SCHEMA.into(),
            name: "t".into(),
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
        }
    }

    fn rect_frame(id: &str, visible: Option<bool>) -> Frame {
        Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: crate::artifacts::layout::LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible, fill: None, stroke: None }
    }

    #[test]
    fn resolve_page_marks_overridden_parent_frames_and_ignores_missing_parent() {
        let mut doc = base_doc();
        doc.parent_pages.push(crate::artifacts::layout::ParentPage {
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
            overrides: vec![crate::artifacts::layout::PageOverride { object_id: "frame-a".into(), bounds: None, visible: None, locked: None }],
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
        let wrong_schema = r#"{"schema":"other.schema","name":"t","grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},"paragraphStyles":[],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[]}"#;
        let error = parse_layout_document(wrong_schema).expect_err("wrong schema must fail");
        assert!(matches!(error, LayoutError::UnexpectedSchema(schema) if schema == "other.schema"));

        let invalid_json = "not json";
        let error = parse_layout_document(invalid_json).expect_err("invalid json must fail");
        assert!(matches!(error, LayoutError::Json(_)));
    }

    #[test]
    fn dwg_import_frames_page_to_rectangular_polyline() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer: 0,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[10.0, 20.0], [110.0, 20.0], [110.0, 70.0], [10.0, 70.0]], bulges: vec![0.0; 4] },
        });
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutSnapshot = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 100.0);
        assert_eq!(document.pages[0].height, 50.0);
    }

    #[test]
    fn dwg_import_without_rectangles_falls_back_to_extents() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        drawing.entities.push(semio_framework_os::DwgEntity { layer: 0, color: semio_framework_os::DwgColor::ByLayer, geometry: semio_framework_os::DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [200.0, 150.0, 0.0] } });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [200.0, 150.0, 0.0];
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutSnapshot = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 200.0);
        assert_eq!(document.pages[0].height, 150.0);
    }

    #[test]
    fn rgba_text_round_trips() {
        assert_eq!(rgba_to_text(&Some([0.1, 0.2, 0.3, 1.0])), "0.1, 0.2, 0.3, 1");
        assert_eq!(rgba_to_text(&None), "");
        assert_eq!(text_to_rgba("0.5, 0.4, 0.3, 1"), Some([0.5, 0.4, 0.3, 1.0]));
        assert_eq!(text_to_rgba("not, a, color"), None);
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent layout artifact engine — owns the full artifact; every transition is a mutation.
pub struct LayoutArtifactEngine {
    artifact: crate::artifacts::layout::schema::LayoutArtifact,
    snapshot: crate::artifacts::layout::LayoutSnapshot,
}

impl LayoutArtifactEngine {
    /// 🏗️ Seeds the engine from a persisted snapshot.
    pub fn new(snapshot: crate::artifacts::layout::LayoutSnapshot) -> Self {
        let artifact = crate::artifacts::layout::schema::LayoutArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    /// 📸️ Consumes the engine and returns its persisted snapshot.
    pub fn into_snapshot(self) -> crate::artifacts::layout::LayoutSnapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for LayoutArtifactEngine {
    type Artifact = crate::artifacts::layout::schema::LayoutArtifact;
    type Snapshot = crate::artifacts::layout::LayoutSnapshot;
    type Mutation = crate::artifacts::layout::mutations::LayoutMutation;
    type Diff = crate::artifacts::layout::diff::LayoutDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the fifteen handcrafted schema leaves for `s.layout.layout`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::layout::schema::layout_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry

