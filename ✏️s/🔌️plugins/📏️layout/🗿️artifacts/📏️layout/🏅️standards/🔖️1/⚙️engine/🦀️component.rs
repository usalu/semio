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
use semio_framework_plugin::{Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId, io_dispatch};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{write_svg_xml, SvgSnapshot};
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
    #[error("svg: {0}")]
    Svg(String),
}
//#endregion ⚠️Errors

//#region 🔖️Register
/// 🗂️ Registers `LayoutSnapshot`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse layout
/// documents without depending on this crate's concrete `Projection`/`Mutation` types. Also registers
/// the 2D export handler and the DWG import handler. Called from the plugin root's `semio_plugin!{
/// setup: … }`.
pub fn register() {
    crate::artifacts::layout::io_registry::register();

    register_artifact_schema();
    register_pilot_languages();
    crate::apps::layout::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::layout::LayoutPlayApp>(LAYOUT_DOCUMENT_SCHEMA);
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
        grammar: Some(crate::artifacts::layout::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::layout::schema::diff::text::COMPONENT_GRAMMAR_PATH),
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
        // 🗄️ `AppIo.export_formats`/`import_formats` stay enum-of-legacy-formats-typed in the framework
        // (no string-based sibling field exists here the way `ArtifactKindSpec::export_stdio_kinds`
        // does) and `negotiate_wire_format` never reads `AppIo`'s copies (only `ArtifactKindSpec`'s,
        // via `OsArtifactDescriptor`) — this is still-scaffolding per `AppIo`'s own doc comment
        // ("apps don't populate this yet"). Left empty rather than fabricating a legacy-enum value
        // for a field nothing consumes; see ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 report.
        export_formats: vec![],
        import_formats: vec![],
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
/// 🌉️ Semio/drawing bridge (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT
/// W5b): every SVG this artifact emits is composed through stdio's real `s.stdio.semio/v1/drawing`
/// subset via `io_dispatch` — nothing in this region hand-rolls an SVG string anymore. Layout's own
/// page/rect model maps onto `DrawNode::Group` (one per page, translated) with each page boundary
/// and each `Frame::Rect`/`Text`/`Image` nested inside as a rect-shaped `DrawNode::Path`.
///
/// `rect_path_segments` and `compose_svg_from_drawing` are `pub(crate)` (not private) because
/// `⚙️engine/🎬️scene/🦀️component.rs`'s own `export_display_list_svg` is a SECOND real consumer —
/// see this file's own header comment on the "more than one consumer lives here" rule.
const DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const SVG_FORMAT_KIND: &str = "s.stdio.svg";
const SVG_FORMAT_STANDARD: &str = "1.1";

/// 📐️ A closed axis-aligned rectangle as `MoveTo` + three `LineTo`s + `Close` — the shared
/// "rects-as-paths" primitive both `layout_snapshot_to_semio_drawing` (page/frame rects) and
/// `⚙️engine/🎬️scene`'s `display_list_to_semio_drawing` (rendered display-list rects) build on.
pub(crate) fn rect_path_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<PathSegment> {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
    vec![
        PathSegment::MoveTo { to: SemioPoint2 { x, y } },
        PathSegment::LineTo { to: SemioPoint2 { x: x + width, y } },
        PathSegment::LineTo { to: SemioPoint2 { x: x + width, y: y + height } },
        PathSegment::LineTo { to: SemioPoint2 { x, y: y + height } },
        PathSegment::Close,
    ]
}

/// 📐️ Recovers a rect's `(x, y, width, height)` from a `MoveTo`/`LineTo`×3/`Close` path — the exact
/// inverse of `rect_path_segments`, used to read `dwg_drawing_to_semio_drawing`'s output back into
/// `Page` boundaries.
fn path_bounds(segments: &[PathSegment]) -> Option<(f64, f64, f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut any = false;
    for segment in segments {
        let point = match segment {
            PathSegment::MoveTo { to } | PathSegment::LineTo { to } | PathSegment::QuadTo { to, .. } | PathSegment::CubicTo { to, .. } | PathSegment::ArcTo { to, .. } => Some(*to),
            PathSegment::Close => None,
        };
        if let Some(point) = point {
            any = true;
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
    }
    any.then(|| (min_x, min_y, max_x - min_x, max_y - min_y))
}

fn semio_rgba_from_channels(channels: [f32; 4]) -> SemioRgba {
    SemioRgba { r: channels[0], g: channels[1], b: channels[2], a: channels[3] }
}

/// 🌉️ Composes a `SemioDrawingSnapshot` into real SVG text through stdio's `drawing↔svg` bridge
/// (`io_dispatch`, never a hand-rolled string). `stdio_gaps`: this is the only DWG/SVG bridge stdio
/// registers for the `drawing` subset today (svg/dxf/pdf per the master plan's own lattice) — see
/// `layout_document_json_from_dwg` below for the DWG-import side of that gap.
pub(crate) fn compose_svg_from_drawing(drawing: &SemioDrawingSnapshot) -> Result<String, String> {
    let key = IoKey {
        artifact_kind: DRAWING_DIALECT.artifact_kind.to_string(),
        standard: DRAWING_DIALECT.standard.0.to_string(),
        subset: DRAWING_DIALECT.subset.0.to_string(),
        direction: IoDirection::Export,
        format_kind: SVG_FORMAT_KIND.to_string(),
        format_standard: SVG_FORMAT_STANDARD.to_string(),
        format_subset: "*".to_string(),
    };
    let bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(drawing);
    let source = ErasedComposeSource { dialect: DRAWING_DIALECT, payload: IoPayload::Binary(bytes) };
    let composed = io_dispatch(&key, std::slice::from_ref(&source)).map_err(|e| format!("layout->semio/drawing->svg: {}", e.message))?;
    let svg_bytes = match composed.payload {
        IoPayload::Binary(bytes) => bytes,
        IoPayload::Text(text) => text.into_bytes(),
    };
    let svg = <SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes).map_err(|e| format!("layout->semio/drawing->svg decode: {e:?}"))?;
    Ok(write_svg_xml(&svg.doc))
}

/// 🖍️ Maps this document's pages onto one translated `DrawNode::Group` per page (matching the
/// previous side-by-side thumbnail layout), each nesting the page boundary plus every visible
/// frame as a rect-shaped `DrawNode::Path` — `Frame::Rect` keeps its real fill/stroke, `Text`/
/// `Image` frames get a neutral outline (mirrors the blueprint chrome colors `⚙️engine/🎬️scene`
/// already uses for the same frame kinds).
fn layout_snapshot_to_semio_drawing(doc: &LayoutSnapshot) -> SemioDrawingSnapshot {
    const PAGE_GAP: f64 = 24.0;
    let mut styles = vec![DrawStyle { name: "page".into(), fill: None, stroke: Some(SemioRgba { r: 0.58, g: 0.65, b: 0.72, a: 1.0 }), stroke_width: Some(2.0), opacity: None }];
    let mut layers = Vec::with_capacity(doc.pages.len());
    let mut x_offset = 0.0f64;
    let mut canvas_width = 0.0f64;
    let mut canvas_height = 0.0f64;

    for page in &doc.pages {
        let mut children = vec![DrawNode::Path { segments: rect_path_segments(0.0, 0.0, page.width, page.height), style: Some("page".into()) }];
        for frame in &page.frames {
            if !frame.visible() {
                continue;
            }
            let bounds = frame.bounds();
            let segments = rect_path_segments(bounds.x, bounds.y, bounds.width, bounds.height);
            let (fill, stroke) = match frame {
                Frame::Rect { fill, stroke, .. } => (fill.map(semio_rgba_from_channels), stroke.map(semio_rgba_from_channels)),
                Frame::Text { .. } => (None, Some(SemioRgba { r: 0.2, g: 0.55, b: 0.9, a: 0.9 })),
                Frame::Image { .. } => (None, Some(SemioRgba { r: 0.85, g: 0.45, b: 0.2, a: 0.9 })),
            };
            if fill.is_none() && stroke.is_none() {
                children.push(DrawNode::Path { segments, style: None });
                continue;
            }
            let style_name = format!("frame-{}", frame.id());
            styles.push(DrawStyle { name: style_name.clone(), fill, stroke, stroke_width: stroke.map(|_| 1.0), opacity: None });
            children.push(DrawNode::Path { segments, style: Some(style_name) });
        }
        layers.push(DrawLayer {
            id: page.id.clone(),
            name: page.name.clone(),
            visible: true,
            root: DrawNode::Group { transform: SemioTransform { translation: SemioPoint3 { x: x_offset, y: 0.0, z: 0.0 }, ..SemioTransform::identity() }, children },
        });
        canvas_width = (x_offset + page.width).max(canvas_width);
        canvas_height = page.height.max(canvas_height);
        x_offset += page.width + PAGE_GAP;
    }

    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: canvas_width.max(1.0), height: canvas_height.max(1.0), background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles,
        layers,
    }
}

pub fn layout_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let doc: LayoutSnapshot = serde_json::from_value(value.clone()).map_err(|e| format!("layout document: {e}"))?;
    let drawing = layout_snapshot_to_semio_drawing(&doc);
    let width = drawing.canvas.width.round() as u32;
    let height = drawing.canvas.height.round() as u32;
    let svg = compose_svg_from_drawing(&drawing)?;
    Ok((svg, width, height))
}

/// 📥️ Extracts axis-aligned rectangular boundaries from closed 4-vertex `LwPolyline`s and frames one page per rectangle, falling back to a single page framed to the drawing extents. Reads an already-decoded `DwgDrawing` (real geometry, not raw bytes) — see `dwg_drawing_to_semio_drawing` for how this feeds the shared `DrawNode` shape.
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

/// 🖍️ Builds a real `SemioDrawingSnapshot` from the rectangular page boundaries `dwg_rect_pages`
/// detects — one flat layer, one unstyled rect-shaped `Path` per detected page. `stdio_gaps`: stdio
/// registers no `drawing↔dwg` composer entry (the master plan's own lattice only wires
/// `drawing↔svg/dxf/pdf`), so this can't route through `io_dispatch` the way `compose_svg_from_drawing`
/// does — it still avoids hand-rolling anything by funneling the already-decoded `DwgDrawing`
/// geometry through the real, schema-owning `SemioDrawingSnapshot`/`DrawNode` shape instead of a
/// bespoke tuple list, symmetric with the export direction above.
fn dwg_drawing_to_semio_drawing(drawing: &semio_framework_os::DwgDrawing) -> SemioDrawingSnapshot {
    let children: Vec<DrawNode> = dwg_rect_pages(drawing)
        .into_iter()
        .map(|(x, y, width, height)| DrawNode::Path { segments: rect_path_segments(x, y, width, height), style: None })
        .collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer { id: "imported".into(), name: "Imported".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// 📥️ Builds a schema-valid layout document from a parsed DWG drawing, framing one page per rectangular boundary found — routed through the real `SemioDrawingSnapshot`/`DrawNode` shape (`dwg_drawing_to_semio_drawing`/`path_bounds`) rather than a bespoke tuple list.
pub fn layout_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let drawing_snapshot = dwg_drawing_to_semio_drawing(drawing);
    let root_children: &[DrawNode] = match drawing_snapshot.layers.first().map(|layer| &layer.root) {
        Some(DrawNode::Group { children, .. }) => children,
        _ => &[],
    };
    let rects: Vec<(f64, f64, f64, f64)> = root_children
        .iter()
        .filter_map(|child| match child {
            DrawNode::Path { segments, .. } => path_bounds(segments),
            _ => None,
        })
        .collect();
    let pages: Vec<Page> = rects
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

//#region 🔖️TestSupport
/// 🧪️ Registers stdio's real `s.stdio.semio/v1/drawing` composer (svg/dxf/pdf io entries incl.)
/// into the shared `io` registry exactly once per test binary — mirrors what the plugin host does
/// once at boot via `Plugin::builder(...).setup(...)`, which `cargo test` never runs. Shared by this
/// file's own tests and `⚙️engine/🎬️scene/🦀️component.rs`'s (both call through `compose_svg_from_drawing`).
#[cfg(test)]
pub(crate) fn ensure_stdio_semio_drawing_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::register);
}
//#endregion 🔖️TestSupport

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

    /// 🌉️ Real end-to-end proof that `layout_document_json_to_svg` composes through stdio's actual
    /// `s.stdio.semio/v1/drawing`→svg bridge (`io_dispatch`) rather than hand-rolling SVG text — the
    /// two demo pages (400x500 each, 24px gap) lay out canvas-wide, and the resulting markup uses
    /// `<path>` (the drawing subset's SVG vocabulary has no `<rect>` element).
    #[test]
    fn svg_export_composes_through_semio_drawing_bridge() {
        ensure_stdio_semio_drawing_registered();
        let doc = default_document();
        let value = serde_json::to_value(&doc).expect("doc to json");
        let (svg, width, height) = layout_document_json_to_svg(&value).expect("svg export succeeds");
        assert!(svg.starts_with("<svg"), "{svg}");
        assert!(svg.contains("<path"), "{svg}");
        assert!(svg.ends_with("</svg>"), "{svg}");
        assert_eq!(width, 824);
        assert_eq!(height, 500);
    }

    #[test]
    fn svg_export_rejects_invalid_document_json() {
        let value = serde_json::json!({ "not": "a layout document" });
        assert!(layout_document_json_to_svg(&value).is_err());
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
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the twenty handcrafted schema leaves for `s.layout.layout`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::layout::schema::layout_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::layout::standards::v1::subsets::any::schema::LayoutComposer as LayoutAnyComposer;
    use crate::artifacts::layout::standards::v1::subsets::any::schema::LayoutBuilder as LayoutAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const LAYOUT_DIALECT: Dialect = Dialect { artifact_kind: "s.layout", standard: StandardId("1"), subset: SubsetId("*") };
    const LAYOUT_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::layout::LayoutSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == LAYOUT_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => LayoutAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => LayoutAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "LayoutComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == LAYOUT_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let text = match &source.payload {
                IoPayload::Text(t) => t.clone(),
                IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
            };
            return crate::artifacts::layout::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "LayoutComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::layout::io::export::serializers::artifacts::svg::v1_1::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::layout::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<LayoutAnyComposer>(),
            ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_svg },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
