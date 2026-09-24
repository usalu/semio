//! 🚪️ IO s.generation2d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.txt"]
}
// 🖊️ No "stdio.dwg" here: generation3d owns the plugin's dwg EXPORT claim, see `🚪️IoRegistry` below.
// Imports are this document's own json and txt carriers: a drawing file is not a generative program.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::Generation2dAnalyzer;
    use crate::Generation2dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation2d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Generation2dComposerComposition;

    impl ArtifactComposition for Generation2dComposerComposition {
        type Snapshot = Generation2dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = Generation2dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Generation2dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
/// 🚪️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the composer/export-entry registry lives with the rest of `🚪️io`, not behind an engine facade.
pub mod io_registry {
    use crate::standards::v1::subsets::any::schema::Generation2dBuilder as Generation2dAnyBuilder;
    use crate::standards::v1::subsets::any::schema::Generation2dComposer as Generation2dAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ArtifactBuilder, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource, IoConfidence, IoPayload, StandardId, SubsetId};
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
    const GENERATION2D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation2d", standard: StandardId("1"), subset: SubsetId("*") };
    const GENERATION2D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::Generation2dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION2D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Generation2dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Generation2dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Generation2dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION2D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Generation2dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    // 🖊️ No `compose_export_dwg`/`EXPORT_DWG_DIALECT` here: generation3d owns the `s.stdio.dwg@ac1018/*`
    // EXPORT claim (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 — see `../../🦀️.rs`'s
    // `definition()` docstring for the ownership rule). `derived_composition`'s `reads()` above still
    // lists `DEP_DWG`, so import is unaffected.
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<Generation2dAnyComposer>(),
                    ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[GENERATION2D_DIALECT], compose: compose_export_svg },
                    ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[GENERATION2D_DIALECT], compose: compose_export_pdf },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[GENERATION2D_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[GENERATION2D_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[GENERATION2D_DIALECT], compose: compose_export_dxf },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️IoRegistry

//#region 🖍️EvaluatedDrawing
/// 🖍️ Flattened drawing scenes (the flow drawing kernel's `render_scene_json`: `nodes` of
/// `{transform, node, fill, stroke, opacity}` with rect/ellipse/circle/line/polygon/path/text nodes)
/// as one drawing, every node's transform applied to its own geometry — the picture every page export
/// writes. Text is placed at its baseline with the node's transform applied to the anchor only.
pub fn semio_drawing_from_scenes(scenes_json: &[String]) -> Result<semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot, String> {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioRgba, SemioTransform};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::{flatten_segments, transformed_segments};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
    let p = |x: f64, y: f64| SemioPoint2 { x, y };
    let point = |value: &dsl::json::Value| -> Option<SemioPoint2> {
        let pair = value.as_array()?;
        Some(p(pair.first()?.as_f64()?, pair.get(1)?.as_f64()?))
    };
    let number = |value: &dsl::json::Value, key: &str| value.get(key).and_then(|v| v.as_f64());
    let colour = |value: Option<&dsl::json::Value>, opacity: f64| -> Option<SemioRgba> {
        let channels = value?.as_array()?;
        let at = |i: usize| channels.get(i).and_then(|c| c.as_f64()).unwrap_or(1.0) as f32;
        Some(SemioRgba { r: at(0), g: at(1), b: at(2), a: at(3) * opacity as f32 })
    };
    let ellipse = |cx: f64, cy: f64, rx: f64, ry: f64| {
        vec![
            PathSegment::MoveTo { to: p(cx + rx, cy) },
            PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: false, sweep: true, to: p(cx - rx, cy) },
            PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: false, sweep: true, to: p(cx + rx, cy) },
            PathSegment::Close,
        ]
    };
    let mut styles = Vec::new();
    let mut children = Vec::new();
    for scene_json in scenes_json {
        let scene = dsl::json::parse(scene_json).map_err(|error| error.to_string())?;
        if let Some(error) = scene.get("error") {
            return Err(format!("the drawing kernel refused the scene: {error:?}"));
        }
        for item in scene.get("nodes").and_then(|n| n.as_array()).cloned().unwrap_or_default() {
            let Some(node) = item.get("node") else { continue };
            let matrix: Vec<f64> = item.get("transform").and_then(|t| t.as_array()).map(|t| t.iter().filter_map(|v| v.as_f64()).collect()).unwrap_or_default();
            let matrix = if matrix.len() == 6 { [matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5]] } else { [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] };
            let opacity = item.get("opacity").and_then(|o| o.as_f64()).unwrap_or(1.0);
            let fill = item.get("fill").filter(|f| f.get("kind").and_then(|k| k.as_str()) == Some("solid")).and_then(|f| colour(f.get("color"), opacity));
            let stroke = item.get("stroke").and_then(|s| colour(s.get("color"), opacity));
            let stroke_width = item.get("stroke").and_then(|s| number(s, "width"));
            let name = format!("node-{}", styles.len());
            let kind = node.get("kind").and_then(|k| k.as_str()).unwrap_or_default();
            let segments: Vec<PathSegment> = match kind {
                "rect" => {
                    let (x, y, w, h) = (number(node, "x").unwrap_or(0.0), number(node, "y").unwrap_or(0.0), number(node, "width").unwrap_or(0.0), number(node, "height").unwrap_or(0.0));
                    vec![PathSegment::MoveTo { to: p(x, y) }, PathSegment::LineTo { to: p(x + w, y) }, PathSegment::LineTo { to: p(x + w, y + h) }, PathSegment::LineTo { to: p(x, y + h) }, PathSegment::Close]
                }
                "ellipse" => ellipse(number(node, "cx").unwrap_or(0.0), number(node, "cy").unwrap_or(0.0), number(node, "rx").unwrap_or(0.0), number(node, "ry").unwrap_or(0.0)),
                "circle" => {
                    let r = number(node, "r").unwrap_or(0.0);
                    ellipse(number(node, "cx").unwrap_or(0.0), number(node, "cy").unwrap_or(0.0), r, r)
                }
                "line" => vec![PathSegment::MoveTo { to: p(number(node, "x1").unwrap_or(0.0), number(node, "y1").unwrap_or(0.0)) }, PathSegment::LineTo { to: p(number(node, "x2").unwrap_or(0.0), number(node, "y2").unwrap_or(0.0)) }],
                "polygon" => {
                    let points: Vec<SemioPoint2> = node.get("points").and_then(|v| v.as_array()).map(|list| list.iter().filter_map(point).collect()).unwrap_or_default();
                    points.iter().enumerate().map(|(i, q)| if i == 0 { PathSegment::MoveTo { to: *q } } else { PathSegment::LineTo { to: *q } }).chain((!points.is_empty()).then_some(PathSegment::Close)).collect()
                }
                "path" => node
                    .get("segments")
                    .and_then(|s| s.as_array())
                    .map(|list| {
                        list.iter()
                            .filter_map(|segment| match segment.get("kind")?.as_str()? {
                                "move" => Some(PathSegment::MoveTo { to: point(segment.get("to")?)? }),
                                "line" => Some(PathSegment::LineTo { to: point(segment.get("to")?)? }),
                                "quad" => Some(PathSegment::QuadTo { c: point(segment.get("ctrl")?)?, to: point(segment.get("to")?)? }),
                                "cubic" => Some(PathSegment::CubicTo { c1: point(segment.get("ctrl1")?)?, c2: point(segment.get("ctrl2")?)?, to: point(segment.get("to")?)? }),
                                "arc" => Some(PathSegment::ArcTo { rx: number(segment, "rx")?, ry: number(segment, "ry")?, x_rotation: number(segment, "rotation")?, large_arc: segment.get("largeArc")?.as_bool()?, sweep: segment.get("sweep")?.as_bool()?, to: point(segment.get("to")?)? }),
                                "close" => Some(PathSegment::Close),
                                _ => None,
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                "text" => {
                    let (x, y) = (number(node, "x").unwrap_or(0.0), number(node, "y").unwrap_or(0.0));
                    let at = p(matrix[0] * x + matrix[2] * y + matrix[4], matrix[1] * x + matrix[3] * y + matrix[5]);
                    styles.push(DrawStyle { name: name.clone(), fill: fill.or(Some(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: opacity as f32 })), stroke: None, stroke_width: None, opacity: None });
                    children.push(DrawNode::Text { value: node.get("content").and_then(|c| c.as_str()).unwrap_or_default().to_string(), at, style: Some(name) });
                    continue;
                }
                _ => continue,
            };
            if segments.is_empty() {
                continue;
            }
            styles.push(DrawStyle { name: name.clone(), fill, stroke, stroke_width, opacity: None });
            children.push(DrawNode::Path { segments: transformed_segments(&segments, &matrix), style: Some(name) });
        }
    }
    let points: Vec<[f64; 2]> = children
        .iter()
        .flat_map(|node| match node {
            DrawNode::Path { segments, .. } => flatten_segments(segments, 1.0).into_iter().flat_map(|(points, _)| points).collect(),
            DrawNode::Text { at, .. } => vec![[at.x, at.y]],
            _ => Vec::new(),
        })
        .collect();
    if points.is_empty() {
        return Err("the document evaluates to no drawing".into());
    }
    let (min, max) = points.iter().fold(([f64::MAX; 2], [f64::MIN; 2]), |(lo, hi), q| ([lo[0].min(q[0]), lo[1].min(q[1])], [hi[0].max(q[0]), hi[1].max(q[1])]));
    let pad = 16.0;
    let shift = SemioTransform { translation: SemioPoint3 { x: pad - min[0], y: pad - min[1], z: 0.0 }, ..SemioTransform::identity() };
    Ok(SemioDrawingSnapshot {
        canvas: DrawCanvas { width: (max[0] - min[0] + 2.0 * pad).max(1.0), height: (max[1] - min[1] + 2.0 * pad).max(1.0), background: None },
        styles,
        layers: vec![DrawLayer { id: "generation".into(), name: "generation".into(), visible: true, root: DrawNode::Group { transform: shift, children } }],
        ..SemioDrawingSnapshot::default()
    })
}

/// 🖍️ The document's evaluated drawing: its flow program run through the preview evaluator and
/// every drawing it outputs flattened by the flow drawing kernel — the picture the editor's
/// `drawing:out` port publishes.
#[cfg(feature = "component-app-assembly")]
pub fn generation2d_drawing(snapshot: &crate::Generation2dSnapshot) -> Result<semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot, String> {
    let eval_json = crate::standards::v1::subsets::any::schema::with_host(&snapshot.host_snapshot, |host| host.evaluate().unwrap_or_default());
    let outputs = dsl::json::parse(&eval_json).map_err(|error| error.to_string())?;
    let mut handles = Vec::new();
    crate::standards::v1::subsets::any::schema::collect_drawing_handles_from_eval(&outputs, &mut handles);
    handles.sort();
    handles.dedup();
    semio_drawing_from_scenes(&handles.iter().map(|handle| semio_framework_os_flow::render_scene_json(handle)).collect::<Vec<_>>())
}

/// 🖍️ Without the flow evaluator there is nothing to draw, and saying so beats an empty picture.
#[cfg(not(feature = "component-app-assembly"))]
pub fn generation2d_drawing(_snapshot: &crate::Generation2dSnapshot) -> Result<semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot, String> {
    Err("generation2d export needs the flow evaluator (build with the `component-app-assembly` feature)".into())
}
//#endregion 🖍️EvaluatedDrawing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🧪️tests/🔬️bundled-example-export/🦀️.rs"]
mod bundled_example_export_tests;
//#endregion 🧪️Tests
