//! 🚪️ IO s.gis.gismap (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::gismap::standards::v1::subsets::any::schema::GisMapAnalyzer;
    use crate::artifacts::gismap::GisMapSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.gis.gismap", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_DXF: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct GisMapComposerComposition;

    impl ArtifactComposition for GisMapComposerComposition {
        type Snapshot = GisMapSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_DXF, DEP_JSON, DEP_PDF, DEP_PNG, DEP_SVG, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = GisMapAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gismap::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DXF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gismap::io::import::deserializers::artifacts::dxf::v_r12::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gismap::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PDF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gismap::io::import::deserializers::artifacts::pdf::v1_4::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gismap::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_SVG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gismap::io::import::deserializers::artifacts::svg::v1_1::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gismap::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "GisMapComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — export-direction `ComposerEntry` rows
/// wrapping this artifact's own `📤️export/🧵️serializers` leaves. Callers MUST reach `entries()`
/// through this fully-qualified path (`…standards::v1::subsets::any::io::io_registry`), never the
/// bare `io_registry` name: both artifact-root `🦀️.rs` files also define their own
/// `io_registry` wrapper (a `&[&ComposerEntry]` view, different type) which shadows a bare call.
pub mod io_registry {
    use crate::artifacts::gismap::standards::v1::subsets::any::schema::GisMapComposer as GisMapAnyComposer;
    use crate::artifacts::gismap::standards::v1::subsets::any::schema::GismapBuilder as GisMapAnyBuilder;
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
    const GISMAP_DIALECT: Dialect = Dialect { artifact_kind: "s.gis.gismap", standard: StandardId("1"), subset: SubsetId("*") };
    const GISMAP_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::gismap::GisMapSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == GISMAP_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => GisMapAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => GisMapAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "GisMapComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == GISMAP_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::gismap::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "GisMapComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gismap::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gismap::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gismap::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gismap::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gismap::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gismap::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<GisMapAnyComposer>(),
                    ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[GISMAP_DIALECT], compose: compose_export_svg },
                    ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[GISMAP_DIALECT], compose: compose_export_pdf },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[GISMAP_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[GISMAP_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[GISMAP_DIALECT], compose: compose_export_dwg },
                    ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[GISMAP_DIALECT], compose: compose_export_dxf },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️IoRegistry

/// 🗺️ Projects DWG geometry into GIS map positions.
pub mod dwg_projection {
    use crate::artifacts::gismap::standards::v1::subsets::any::schema::{default_document, dsl_to_value, value_to_dsl};
    use crate::artifacts::gismap::{gis_map_snapshot_with_derived_children, GisMapSnapshot, MapFeature};
    use dsl::ToValue;
    use semio_s_plugin_stdio::artifacts::dwg::{DwgDrawing, DwgGeometry};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot};
    use serde_json::{json, Value};
//#region 🔖️MediaImport
/// ✏️ Projects supported DWG geometry into drawing path segments.
fn dwg_geometry_to_draw_node(geometry: &DwgGeometry) -> Option<DrawNode> {
    let vertices: Vec<[f64; 2]> = match geometry {
        DwgGeometry::Point { at } => vec![[at[0], at[1]]],
        DwgGeometry::Line { start, end } => vec![[start[0], start[1]], [end[0], end[1]]],
        DwgGeometry::LwPolyline { vertices, .. } => vertices.clone(),
        DwgGeometry::Polyline3d { vertices, .. } => vertices.iter().map(|v| [v[0], v[1]]).collect(),
        _ => return None,
    };
    if vertices.is_empty() {
        return None;
    }
    let closed = matches!(geometry, DwgGeometry::LwPolyline { closed: true, .. } | DwgGeometry::Polyline3d { closed: true, .. });
    let mut segments: Vec<PathSegment> = vertices
        .iter()
        .enumerate()
        .map(|(index, v)| {
            let to = SemioPoint2 { x: v[0], y: v[1] };
            if index == 0 {
                PathSegment::MoveTo { to }
            } else {
                PathSegment::LineTo { to }
            }
        })
        .collect();
    if closed {
        segments.push(PathSegment::Close);
    }
    Some(DrawNode::Path { segments, style: None })
}

/// 🌉️ Builds a `SemioDrawingSnapshot` from a `DwgDrawing`'s entities — one `DrawNode::Path`
/// per real (non-degenerate) entity, all under one layer.
fn dwg_drawing_to_semio_drawing(drawing: &DwgDrawing) -> SemioDrawingSnapshot {
    let children: Vec<DrawNode> = drawing.entities.iter().filter_map(|entity| dwg_geometry_to_draw_node(&entity.geometry)).collect();
    SemioDrawingSnapshot { layers: vec![DrawLayer { id: "dwg-import".into(), name: "DWG Import".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }], ..SemioDrawingSnapshot::default() }
}

/// 📍️ Walks a `DrawNode` tree collecting every `MoveTo`/`LineTo` endpoint — the vertex set the
/// import path turns into position features (mirrors the old direct `DwgGeometry` vertex walk, now
/// over the semio/drawing shape instead).
fn collect_draw_node_points(node: &DrawNode, out: &mut Vec<SemioPoint2>) {
    match node {
        DrawNode::Path { segments, .. } => {
            for segment in segments {
                match segment {
                    PathSegment::MoveTo { to } | PathSegment::LineTo { to } => out.push(*to),
                    _ => {}
                }
            }
        }
        DrawNode::Group { children, .. } => children.iter().for_each(|child| collect_draw_node_points(child, out)),
        _ => {}
    }
}

/// 🗺️ Imports a DWG drawing into a bare gis map document: DWG entities lower to `DrawNode::Path`
/// geometry (`dwg_drawing_to_semio_drawing`), whose vertices become position features. Falls back
/// to the default reuse-map document when the DWG carries no point-like geometry.
pub fn gis2d_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let scene = dwg_drawing_to_semio_drawing(drawing);
    let mut points = Vec::new();
    for layer in &scene.layers {
        collect_draw_node_points(&layer.root, &mut points);
    }
    if points.is_empty() {
        return Ok(dsl_to_value(&default_document().to_value()));
    }
    let positions: Vec<MapFeature> = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let id = format!("dwg-{index}");
            MapFeature { id: id.clone(), data: value_to_dsl(&json!({ "id": id, "lon": point.x, "lat": point.y })) }
        })
        .collect();
    let document = gis_map_snapshot_with_derived_children(GisMapSnapshot { positions, routes: Vec::new(), regions: Vec::new(), ..Default::default() });
    Ok(dsl_to_value(&document.to_value()))
}
//#endregion 🔖️MediaImport
#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_plugin_stdio::artifacts::dwg::{DwgColor, DwgEntity};
    #[semio_framework_async_macros::async_test]
    async fn dwg_import_collects_point_and_line_vertices() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [3.0, 4.0, 0.0] } });
        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn dwg_import_falls_back_to_default_document_when_empty() {
        let drawing = DwgDrawing::default();
        let value = gis2d_document_json_from_dwg(&drawing).expect("import empty dwg");
        let snapshot: GisMapSnapshot = dsl::json::from_json_str(&value.to_string()).expect("document");
        assert!(!snapshot.positions.is_empty(), "fallback seeds the reuse-map document");
    }

    #[semio_framework_async_macros::async_test]
    async fn dwg_import_lowers_a_closed_polyline_through_a_draw_node_and_carries_the_close_segment() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], bulges: vec![0.0, 0.0, 0.0] } });
        let scene = dwg_drawing_to_semio_drawing(&drawing);
        let DrawNode::Group { children, .. } = &scene.layers[0].root else { panic!("expected a group root") };
        let DrawNode::Path { segments, .. } = &children[0] else { panic!("expected a path node") };
        assert!(matches!(segments.first(), Some(PathSegment::MoveTo { .. })));
        assert!(matches!(segments.last(), Some(PathSegment::Close)));
        assert_eq!(segments.len(), 4, "3 vertices + Close");

        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3, "one position feature per polyline vertex");
    }

}
}
