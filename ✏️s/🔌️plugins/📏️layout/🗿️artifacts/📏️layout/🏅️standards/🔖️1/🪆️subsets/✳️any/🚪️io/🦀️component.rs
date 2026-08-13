//! 🚪️ IO s.layout (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }
pub fn layout_to_wire(from: &crate::artifacts::layout::LayoutSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(from)
}
pub fn layout_from_wire(bytes: &[u8]) -> Result<crate::artifacts::layout::LayoutSnapshot, store::PackError> {
    <crate::artifacts::layout::LayoutSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::layout::LayoutSnapshot;
    use crate::artifacts::layout::standards::v1::subsets::any::schema::LayoutAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.layout", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_DXF: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };


    pub struct LayoutComposerComposition;

    impl ArtifactComposition for LayoutComposerComposition {
        type Snapshot = LayoutSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_DXF, DEP_JSON, DEP_SVG]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = LayoutAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::layout::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DXF {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::layout::io::import::deserializers::artifacts::dxf::v_r12::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_JSON {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::layout::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_SVG {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::layout::io::import::deserializers::artifacts::svg::v1_1::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }

            }
            Err(ComposeError { message: "LayoutComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region ⚠️Errors
/// 🚧️ All fallible layout-engine operations funnel through this — document parsing, scene/hit-test
/// resolution, and export (SVG/PDF/PNG/zip package). Relocated from the deleted `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); the app engine's scene/export functions
/// (`🎛️apps/📏️layout/⚙️engine`) reach it by qualified path — an app depending on its artifact is normal
/// direction, not a layering violation.
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

//#region 🔖️MediaImportExport
/// 🌉️ Semio/drawing bridge (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT
/// W5b): every SVG this artifact emits is composed through stdio's real `s.stdio.semio/v1/drawing`
/// subset via `io_dispatch` — nothing in this region hand-rolls an SVG string anymore. Layout's own
/// page/rect model maps onto `DrawNode::Group` (one per page, translated) with each page boundary
/// and each `Frame::Rect`/`Text`/`Image` nested inside as a rect-shaped `DrawNode::Path`. Relocated
/// from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// codec/composer-dispatch territory per the region → destination map.
///
/// `rect_path_segments` and `compose_svg_from_drawing` are `pub` (were `pub(crate)`, widened because
/// the app engine's `export_display_list_svg` is now a cross-module SECOND consumer — see this
/// region's own header on the "more than one consumer" rule).
use crate::artifacts::layout::{Frame, GridSettings, Layer, LayoutSnapshot, Page, PageColumns, PageMargins, Spread, LAYOUT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId, io_dispatch};
use semio_s_plugin_stdio::artifacts::dwg::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{write_svg_xml, SvgSnapshot};
use serde_json::Value;

const DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const SVG_FORMAT_KIND: &str = "s.stdio.svg";
const SVG_FORMAT_STANDARD: &str = "1.1";

/// 📐️ A closed axis-aligned rectangle as `MoveTo` + three `LineTo`s + `Close` — the shared
/// "rects-as-paths" primitive both `layout_snapshot_to_semio_drawing` (page/frame rects) and the app
/// engine's `display_list_to_semio_drawing` (rendered display-list rects) build on.
pub fn rect_path_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<PathSegment> {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
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
pub fn compose_svg_from_drawing(drawing: &SemioDrawingSnapshot) -> Result<String, String> {
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
/// `Image` frames get a neutral outline (mirrors the blueprint chrome colors the app engine's scene
/// module already uses for the same frame kinds).
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
fn dwg_rect_pages(drawing: &DwgDrawing) -> Vec<(f64, f64, f64, f64)> {
    let mut rects = Vec::new();
    for entity in &drawing.entities {
        let DwgGeometry::LwPolyline { closed: true, vertices, .. } = &entity.geometry else { continue };
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
fn dwg_drawing_to_semio_drawing(drawing: &DwgDrawing) -> SemioDrawingSnapshot {
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
pub fn layout_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
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
/// file's own tests and the app engine's scene tests (both call through `compose_svg_from_drawing`).
/// Widened from `pub(crate)` to `pub`: the app engine is now a cross-module second caller.
#[cfg(test)]
pub fn ensure_stdio_semio_drawing_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::register);
}
//#endregion 🔖️TestSupport

//#region 🧪️Tests
#[cfg(test)]
mod media_import_export_tests {
    use super::*;

    #[test]
    fn dwg_import_frames_page_to_rectangular_polyline() {
        let mut drawing = DwgDrawing::default();
        drawing.entities.push(DwgEntity {
            layer: 0,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[10.0, 20.0], [110.0, 20.0], [110.0, 70.0], [10.0, 70.0]], bulges: vec![0.0; 4] },
        });
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutSnapshot = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 100.0);
        assert_eq!(document.pages[0].height, 50.0);
    }

    #[test]
    fn dwg_import_without_rectangles_falls_back_to_extents() {
        let mut drawing = DwgDrawing::default();
        drawing.entities.push(DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [200.0, 150.0, 0.0] } });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [200.0, 150.0, 0.0];
        let value = layout_document_json_from_dwg(&drawing).expect("import dwg");
        let document: LayoutSnapshot = serde_json::from_value(value).expect("valid layout document");
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].width, 200.0);
        assert_eq!(document.pages[0].height, 150.0);
    }

    /// 🌉️ Real end-to-end proof that `layout_document_json_to_svg` composes through stdio's actual
    /// `s.stdio.semio/v1/drawing`→svg bridge (`io_dispatch`) rather than hand-rolling SVG text — the
    /// two demo pages (400x500 each, 24px gap) lay out canvas-wide, and the resulting markup uses
    /// `<path>` (the drawing subset's SVG vocabulary has no `<rect>` element).
    #[test]
    fn svg_export_composes_through_semio_drawing_bridge() {
        ensure_stdio_semio_drawing_registered();
        let doc = crate::artifacts::layout::schema::default_document();
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

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
/// The artifact root's own `io_registry` (a `&'static [&'static ComposerEntry]` view) wraps THIS
/// module's `entries()` (`&'static [ComposerEntry]`, owning storage) — deliberately different return
/// types; do not conflate them when qualifying paths.
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
