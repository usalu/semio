//! 🚪️ IO s.layout (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.svg", "stdio.pdf", "stdio.step", "stdio.idml"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.png", "stdio.svg", "stdio.pdf", "stdio.step", "stdio.idml"]
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

    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_STEP: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };

    const DEP_IDML: Dialect = Dialect { artifact_kind: "s.stdio.idml", standard: StandardId("8.0"), subset: SubsetId("*") };

    pub struct LayoutComposerComposition;

    impl ArtifactComposition for LayoutComposerComposition {
        type Snapshot = LayoutSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_DXF, DEP_JSON, DEP_SVG, DEP_PDF, DEP_STEP, DEP_IDML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if [DEP_PDF,DEP_STEP,DEP_IDML,DEP_SVG,DEP_DXF,DEP_DWG].contains(&source.dialect){return Err(ComposeError{message:"layout.import.retained-preparation-required".into(),diagnostics:Vec::new()})}
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

fn rgba_from_channels(channels: [f32; 4]) -> VectorRgba {
    VectorRgba { r: channels[0], g: channels[1], b: channels[2], a: channels[3] }
}

fn drawing_codec() -> Result<&'static dyn semio_framework_io_ports::EmbeddedChildCodecV1, String> {
    semio_s_artifact_layout_layout_model::drawing_codec().map_err(|error| error.to_string())
}

/// 🖍️ Printable page content becomes translated scene groups with actual text, image bytes and center rotations.
/// Parent-page overrides and layer visibility are applied; imported background layers remain behind native content.
pub fn layout_snapshot_to_scene(doc: &LayoutSnapshot) -> Result<VectorScene, String> {
    const PAGE_GAP: f64 = 24.0;
    let mut styles = Vec::new();
    let mut layers = Vec::new();
    if let Some(background) = semio_s_artifact_layout_layout_model::background_drawing_scene(doc).map_err(|error| error.to_string())? {
        for mut layer in background.layers { layer.id = format!("background-{}", layer.id); layers.push(layer); }
        styles.extend(background.styles);
    }
    let mut x_offset = 0.0f64;
    let mut canvas_width = 0.0f64;
    let mut canvas_height = 0.0f64;
    for page in &doc.pages {
        let mut children = Vec::new();
        let parent = page.parent_page_id.as_ref().and_then(|id| doc.parent_pages.iter().find(|parent| &parent.id == id));
        let frames = parent.into_iter().flat_map(|parent| &parent.frames).chain(&page.frames);
        for frame in frames {
            let inherited = parent.is_some_and(|parent| parent.frames.iter().any(|candidate| std::ptr::eq(candidate, frame)));
            let override_value = inherited.then(|| page.overrides.iter().find(|value| value.object_id == frame.id())).flatten();
            if !override_value.and_then(|value| value.visible).unwrap_or_else(|| frame.visible()) { continue; }
            let visible_layer = page.layers.iter().chain(parent.into_iter().flat_map(|parent| &parent.layers)).find(|layer| layer.id == frame.layer_id()).is_none_or(|layer| layer.visible);
            if !visible_layer { continue; }
            let bounds = override_value.and_then(|value| value.bounds.as_ref()).unwrap_or_else(|| frame.bounds());
            let style_name = format!("frame-{}-{}", page.id, frame.id());
            let mut nodes = Vec::new();
            match frame {
                Frame::Rect { fill, stroke, .. } => {
                    styles.push(VectorStyle { name: style_name.clone(), fill: fill.map(rgba_from_channels), stroke: stroke.map(rgba_from_channels), stroke_width: stroke.map(|_| 1.0), opacity: None });
                    nodes.push(VectorNode::Path { segments: rect_path_segments(-bounds.width * 0.5, -bounds.height * 0.5, bounds.width, bounds.height), style: Some(style_name) });
                },
                Frame::Text { story_id, inset, .. } => {
                    let story = doc.stories.iter().find(|story| &story.id == story_id).ok_or_else(|| format!("layout export: missing story {story_id}"))?;
                    let paragraph = story.style_runs.first().and_then(|run| run.paragraph_style_id.as_ref()).and_then(|id| doc.paragraph_styles.iter().find(|style| &style.id == id));
                    let size = paragraph.map_or(12.0, |style| style.font_size);
                    let leading = paragraph.map_or(size * 1.2, |style| style.leading);
                    let color = story.style_runs.first().and_then(|run| run.character_style_id.as_ref()).and_then(|id| doc.character_styles.iter().find(|style| &style.id == id)).and_then(|style| style.color).unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    styles.push(VectorStyle { name: style_name.clone(), fill: Some(rgba_from_channels(color)), stroke: None, stroke_width: None, opacity: None });
                    for (index, value) in story.content.split('\n').enumerate() {
                        nodes.push(VectorNode::Text { value: value.into(), at: VectorPoint2 { x: -bounds.width * 0.5 + inset.x, y: -bounds.height * 0.5 + inset.y + size + index as f64 * leading }, style: Some(style_name.clone()) });
                    }
                },
                Frame::Image { link_id, .. } => {
                    let link = doc.links.iter().find(|link| &link.id == link_id).ok_or_else(|| format!("layout export: missing link {link_id}"))?;
                    let data = link.proxy_data_url.as_deref().ok_or_else(|| format!("layout export: unresolved image {link_id}"))?;
                    let (header, data) = data.split_once(",").ok_or("layout export: image data URL")?;
                    let mime = header.strip_prefix("data:").and_then(|header| header.strip_suffix(";base64")).filter(|mime| matches!(*mime, "image/png" | "image/jpeg" | "image/gif" | "image/svg+xml")).ok_or("layout export: unsupported image data URL")?;
                    let bytes = store::pack_rt::pack_value_from_base64(data).map_err(|error| error.to_string())?;
                    nodes.push(VectorNode::Image { at: VectorPoint2 { x: -bounds.width * 0.5, y: -bounds.height * 0.5 }, width: bounds.width, height: bounds.height, mime: mime.into(), bytes });
                },
            }
            let half = bounds.rotation * 0.5;
            children.push(VectorNode::Group { transform: VectorTransform { translation: VectorPoint3 { x: bounds.x + bounds.width * 0.5, y: bounds.y + bounds.height * 0.5, z: 0.0 }, rotation: semio_framework_io_ports::VectorQuaternion { x: 0.0, y: 0.0, z: half.sin(), w: half.cos() }, scale: VectorPoint3 { x: 1.0, y: 1.0, z: 1.0 } }, children: nodes });
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

/// 🧭️ Cold external analysis preserves original encoded bytes before identity, native decoding and publication.
pub fn layout_import_sources_from_format(format:&str,bytes:&[u8],name:&str,control:&mut crate::interchange::InterchangeControl<'_>)->Result<crate::interchange::LayoutImportSourcePlan,String>{
    use crate::interchange::{LayoutExternalDrawingFormat,LayoutImportSourcePlan,InterchangeProgress,MAX_INTERCHANGE_BYTES};
    let format=match format{"svg"=>LayoutExternalDrawingFormat::Svg,"dxf"=>LayoutExternalDrawingFormat::Dxf,"dwg"=>LayoutExternalDrawingFormat::Dwg,_=>return Err("layout.import.drawing-format".into())};
    if bytes.len()>MAX_INTERCHANGE_BYTES{return Err("layout.interchange.byte-limit".into())}
    if !control(InterchangeProgress{completed:0,total:bytes.len()}){return Err("layout.interchange.cancelled".into())}
    let mut source=Vec::new();source.try_reserve_exact(bytes.len()).map_err(|_|"layout.import.source-allocation")?;
    for(at,chunk)in bytes.chunks(4096).enumerate(){if !control(InterchangeProgress{completed:at*4096,total:bytes.len()}){return Err("layout.interchange.cancelled".into())}source.extend_from_slice(chunk)}
    if !control(InterchangeProgress{completed:bytes.len(),total:bytes.len()}){return Err("layout.interchange.cancelled".into())}
    Ok(LayoutImportSourcePlan::External{format,source,name:name.into()})
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
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let result = crate::interchange::export_document("pdf", &snapshot, &mut |_| true).map_err(|message| ComposeError { message, diagnostics: Vec::new() })?;
            let diagnostics = result.diagnostics.into_iter().map(|diagnostic| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new(diagnostic.code), severity: semio_framework_diagnostic::Severity::Warning, span: semio_framework_diagnostic::TextSpan::at(1, 1), message: diagnostic.message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }).collect();
            Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(result.value), diagnostics, confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_STEP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    fn compose_export_step(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let result = crate::interchange::export_document("step", &snapshot, &mut |_| true).map_err(|message| ComposeError { message, diagnostics: Vec::new() })?;
            let diagnostics = result.diagnostics.into_iter().map(|diagnostic| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new(diagnostic.code), severity: semio_framework_diagnostic::Severity::Warning, span: semio_framework_diagnostic::TextSpan::at(1, 1), message: diagnostic.message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }).collect();
            Ok(ComposedArtifact { dialect: EXPORT_STEP_DIALECT, payload: IoPayload::Binary(result.value), diagnostics, confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_IDML_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.idml", standard: StandardId("8.0"), subset: SubsetId("*") };
    fn compose_export_idml(sources: &[ErasedComposeSource]) -> semio_framework_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let result = crate::interchange::export_document("idml", &snapshot, &mut |_| true).map_err(|message| ComposeError { message, diagnostics: Vec::new() })?;
            let diagnostics = result.diagnostics.into_iter().map(|diagnostic| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new(diagnostic.code), severity: semio_framework_diagnostic::Severity::Warning, span: semio_framework_diagnostic::TextSpan::at(1, 1), message: diagnostic.message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }).collect();
            Ok(ComposedArtifact { dialect: EXPORT_IDML_DIALECT, payload: IoPayload::Binary(result.value), diagnostics, confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<LayoutAnyComposer>(),
                    ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_pdf },
                    ComposerEntry { writes: EXPORT_STEP_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_step },
                    ComposerEntry { writes: EXPORT_IDML_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_idml },
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

#[path = "🔀️interchange/🦀️.rs"]
pub mod interchange;
