//! 🚪️ IO s.layout (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.svg"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.png", "stdio.svg"]
}
pub fn layout_to_wire(from: &LayoutSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(from)
}
pub fn layout_from_wire(bytes: &[u8]) -> Result<LayoutSnapshot, store::PackError> {
    <LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::LayoutAnalyzer;
    use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId("*") };
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

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
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
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DXF {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::import::deserializers::artifacts::dxf::v_r12::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_JSON {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_SVG {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::import::deserializers::artifacts::svg::v1_1::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
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


//#endregion ⚠️Errors

//#region 🔖️MediaImportExport
/// 🌉️ Layout's pages, frames and background trace become one neutral [`VectorScene`] (one translated group per page, each page boundary and each frame
/// nested inside as a rect-shaped path); the codec bound to `LAYOUT_BACKGROUND_DRAWING` writes that scene as an external format and reads external documents back
/// as the document's background child. Layout never names the owner of those formats.
use semio_s_artifact_layout_layout_model::{Frame, GridSettings, Layer, Page, PageColumns, PageMargins, Spread, LAYOUT_DOCUMENT_SCHEMA};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_io_ports::{VectorCanvas, VectorLayer, VectorNode, VectorPoint2, VectorPoint3, VectorRgba, VectorScene, VectorSegment, VectorStyle, VectorTransform};
use semio_framework_value::DslValue as Value;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

/// 📐️ A closed axis-aligned rectangle as `MoveTo` + three `LineTo`s + `Close`, the "rects-as-paths" primitive the page and frame rects build on.
pub fn rect_path_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<VectorSegment> {
    vec![
        VectorSegment::MoveTo { to: VectorPoint2 { x, y } },
        VectorSegment::LineTo { to: VectorPoint2 { x: x + width, y } },
        VectorSegment::LineTo { to: VectorPoint2 { x: x + width, y: y + height } },
        VectorSegment::LineTo { to: VectorPoint2 { x, y: y + height } },
        VectorSegment::Close,
    ]
}

/// ▭️ The bounds of a path that is a closed axis-aligned rectangle — the page frames of a trace.
fn rectangle_bounds(segments: &[VectorSegment]) -> Option<(f64, f64, f64, f64)> {
    let corners: Vec<_> = segments.iter().filter_map(|segment| match segment {
        VectorSegment::MoveTo { to } | VectorSegment::LineTo { to } => Some(*to),
        _ => None,
    }).collect();
    let only_lines = segments.iter().all(|segment| matches!(segment, VectorSegment::MoveTo { .. } | VectorSegment::LineTo { .. } | VectorSegment::Close));
    let closed = matches!(segments.last(), Some(VectorSegment::Close)) || (corners.len() == 5 && (corners[0].x - corners[4].x).abs() < 1e-6 && (corners[0].y - corners[4].y).abs() < 1e-6);
    if !only_lines || !closed || !(4..=5).contains(&corners.len()) {
        return None;
    }
    let (x, y, width, height) = path_bounds(segments)?;
    let on_edge = corners.iter().all(|c| ((c.x - x).abs() < 1e-6 || (c.x - x - width).abs() < 1e-6) && ((c.y - y).abs() < 1e-6 || (c.y - y - height).abs() < 1e-6));
    (on_edge && width > 0.0 && height > 0.0).then_some((x, y, width, height))
}

fn path_bounds(segments: &[VectorSegment]) -> Option<(f64, f64, f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut any = false;
    for segment in segments {
        let point = match segment {
            VectorSegment::MoveTo { to } | VectorSegment::LineTo { to } | VectorSegment::QuadTo { to, .. } | VectorSegment::CubicTo { to, .. } | VectorSegment::ArcTo { to, .. } => Some(*to),
            VectorSegment::Close => None,
        };
        if let Some(point) = point {
            any = true;
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
    }
    any.then_some((min_x, min_y, max_x - min_x, max_y - min_y))
}

fn rgba_from_channels(channels: [f32; 4]) -> VectorRgba {
    VectorRgba { r: channels[0], g: channels[1], b: channels[2], a: channels[3] }
}

fn drawing_codec() -> Result<&'static dyn semio_framework_io_ports::EmbeddedChildCodecV1, String> {
    semio_s_artifact_layout_layout_model::drawing_codec().map_err(|error| error.to_string())
}

/// 🖍️ Maps this document's pages onto one translated group per page, each nesting the page boundary plus every visible
/// frame as a rect-shaped path — `Frame::Rect` keeps its real fill/stroke, `Text`/`Image` frames get a neutral outline (mirrors the blueprint
/// chrome colors the scene module uses for the same frame kinds). When the document has a composed `background_drawing`, its layers are merged in
/// first (behind every page layer), so an imported trace an author draws pages on top of actually reaches the export.
pub fn layout_snapshot_to_scene(doc: &LayoutSnapshot) -> Result<VectorScene, String> {
    const PAGE_GAP: f64 = 24.0;
    let mut styles = vec![VectorStyle { name: "page".into(), fill: None, stroke: Some(VectorRgba { r: 0.58, g: 0.65, b: 0.72, a: 1.0 }), stroke_width: Some(2.0), opacity: None }];
    let mut layers = Vec::with_capacity(doc.pages.len());

    if let Some(background) = semio_s_artifact_layout_layout_model::background_drawing_scene(doc).map_err(|error| error.to_string())? {
        for mut layer in background.layers {
            layer.id = format!("background-{}", layer.id);
            layers.push(layer);
        }
        styles.extend(background.styles);
    }

    let mut x_offset = 0.0f64;
    let mut canvas_width = 0.0f64;
    let mut canvas_height = 0.0f64;

    for page in &doc.pages {
        let mut children = vec![VectorNode::Path { segments: rect_path_segments(0.0, 0.0, page.width, page.height), style: Some("page".into()) }];
        for frame in &page.frames {
            if !frame.visible() {
                continue;
            }
            let bounds = frame.bounds();
            let segments = rect_path_segments(bounds.x, bounds.y, bounds.width, bounds.height);
            let (fill, stroke) = match frame {
                Frame::Rect { fill, stroke, .. } => (fill.map(rgba_from_channels), stroke.map(rgba_from_channels)),
                Frame::Text { .. } => (None, Some(VectorRgba { r: 0.2, g: 0.55, b: 0.9, a: 0.9 })),
                Frame::Image { .. } => (None, Some(VectorRgba { r: 0.85, g: 0.45, b: 0.2, a: 0.9 })),
            };
            if fill.is_none() && stroke.is_none() {
                children.push(VectorNode::Path { segments, style: None });
                continue;
            }
            let style_name = format!("frame-{}", frame.id());
            styles.push(VectorStyle { name: style_name.clone(), fill, stroke, stroke_width: stroke.map(|_| 1.0), opacity: None });
            children.push(VectorNode::Path { segments, style: Some(style_name) });
        }
        layers.push(VectorLayer { id: page.id.clone(), name: page.name.clone(), visible: true, root: VectorNode::Group { transform: VectorTransform { translation: VectorPoint3 { x: x_offset, y: 0.0, z: 0.0 }, ..VectorTransform::identity() }, children } });
        canvas_width = (x_offset + page.width).max(canvas_width);
        canvas_height = page.height.max(canvas_height);
        x_offset += page.width + PAGE_GAP;
    }

    Ok(VectorScene { schema: String::new(), canvas: VectorCanvas { width: canvas_width.max(1.0), height: canvas_height.max(1.0), background: Some(VectorRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) }, styles, layers })
}

/// 📤️ One scene written as one external document of `format` by the bound codec.
pub fn export_scene(scene: &VectorScene, format: &str) -> Result<Vec<u8>, String> {
    let codec = drawing_codec()?;
    let content = codec.lift(scene).map_err(|error| error.to_string())?;
    codec.export(format, &content)
}

/// 📤️ The pages laid side by side, written as one external document of `format` by the bound codec.
pub fn layout_snapshot_to_format(doc: &LayoutSnapshot, format: &str) -> Result<Vec<u8>, String> {
    export_scene(&layout_snapshot_to_scene(doc)?, format)
}

pub fn layout_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let doc: LayoutSnapshot = LayoutSnapshot::from_value(value.clone()).map_err(|e| format!("layout document: {e}"))?;
    let scene = layout_snapshot_to_scene(&doc)?;
    let width = scene.canvas.width.round() as u32;
    let height = scene.canvas.height.round() as u32;
    let svg = String::from_utf8(export_scene(&scene, "svg")?).map_err(|error| format!("layout->svg: {error}"))?;
    Ok((svg, width, height))
}

/// 📥️ Reads one external document of `format` as a drawing and frames a layout document on it; see [`layout_document_json_from_drawing`].
pub fn layout_document_json_from_format(format: &str, bytes: &[u8], name: &str) -> Result<Value, String> {
    let content = drawing_codec()?.import(format, bytes)?;
    layout_document_json_from_drawing(&content, format, name)
}

/// 📥️ Frames one page per closed axis-aligned rectangle of an imported drawing (a dwg, dxf or svg
/// trace) and keeps the whole drawing as the document's background, so an author lays pages out on
/// top of the real plan.
pub fn layout_document_json_from_drawing(content: &Value, source: &str, name: &str) -> Result<Value, String> {
    let scene: VectorScene = drawing_codec()?.lower(content).map_err(|error| error.to_string())?;
    let background_child = semio_s_artifact_layout_layout_model::background_drawing_child_handle(source, content);
    let mut rects: Vec<(f64, f64, f64, f64)> = Vec::new();
    for layer in &scene.layers {
        let nodes: Vec<&VectorNode> = match &layer.root {
            VectorNode::Group { children, .. } => children.iter().collect(),
            other => vec![other],
        };
        for node in nodes {
            if let VectorNode::Path { segments, .. } = node {
                if let Some(bounds) = rectangle_bounds(segments) {
                    rects.push(bounds);
                }
            }
        }
    }
    if rects.is_empty() && scene.canvas.width > 0.0 && scene.canvas.height > 0.0 {
        rects.push((0.0, 0.0, scene.canvas.width, scene.canvas.height));
    }
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
        name: name.into(),
        grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: vec![Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids }],
        pages,
        print_target: None,
        data_fields: None,
        background_drawing: Some(background_child),
        referenced_model: None,
    };
    Ok(document.to_value())
}
//#endregion 🔖️MediaImportExport

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
/// The artifact root's own `io_registry` (a `&'static [&'static ComposerEntry]` view) wraps THIS
/// module's `entries()` (`&'static [ComposerEntry]`, owning storage) — deliberately different return
/// types; do not conflate them when qualifying paths.
pub mod io_registry {
    use crate::LayoutBuilder as LayoutAnyBuilder;
    use crate::LayoutComposer as LayoutAnyComposer;
    use {semio_framework_plugin::composer_entry_of,semio_framework_plugin::ArtifactBuilder,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use std::sync::OnceLock;

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
    const LAYOUT_DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId("*") };
    const LAYOUT_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == LAYOUT_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => LayoutAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => LayoutAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "LayoutComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == LAYOUT_JSON_BRIDGE_DIALECT) {
            let text = match &source.payload {
                IoPayload::Text(t) => t.clone(),
                IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
            };
            return crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "LayoutComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let text = crate::export::serializers::artifacts::svg::v1_1::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let text = crate::export::serializers::artifacts::json::v_rfc8259::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<LayoutAnyComposer>(),
                    ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_svg },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_dxf },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_dwg },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry


#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use semio_s_artifact_layout_layout_diff::LayoutDiff;
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug)]
    pub struct LayoutBuilderConstruction {
        snapshot: LayoutSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for LayoutBuilderConstruction {
        type Snapshot = LayoutSnapshot;
        type Mutation = LayoutMutation;
        type Diff = LayoutDiff;
        fn empty() -> Self {
            Self { snapshot: semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<LayoutSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::ColdMutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff_cold(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct LayoutParts {
        pub snapshot: Option<LayoutSnapshot>,
    }

    pub struct LayoutAnalyzerAnalysis;

    impl ArtifactAnalysis for LayoutAnalyzerAnalysis {
        type Parts = LayoutParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = LayoutParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <LayoutSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec LayoutBuilderFacets {
        construction: LayoutBuilderConstruction,
        analysis: LayoutAnalyzerAnalysis,
        composition: LayoutComposerComposition,
    }
    builder: LayoutBuilder,
    analyzer: LayoutAnalyzer,
    composer: LayoutComposer,
);

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod export {
    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod v_ac1018 {
                    #[path = "📤️export/🧵️serializers/🗿️artifacts/🏗️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod v_r12 {
                    #[path = "📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod v1_2 {
                    #[path = "📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod v1_1 {
                    #[path = "📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
        }
    }
}
#[path = "."]
pub mod import {
    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod v_ac1018 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/🏗️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod v_r12 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod v1_1 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
        }
    }
}
//#endregion 🪢️TaxonomyMounts
