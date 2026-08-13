//! 🚪️ IO s.present (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from `crate::apps::present::register`, relocated there by ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg", "stdio.txt"] }
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::present::PresentSnapshot;
    use crate::artifacts::present::standards::v1::subsets::any::schema::PresentAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.present", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_MD: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_PPTX: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };


    pub struct PresentComposerComposition;

    impl ArtifactComposition for PresentComposerComposition {
        type Snapshot = PresentSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON, DEP_MD, DEP_PDF, DEP_PNG, DEP_PPTX, DEP_SVG, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = PresentAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::present::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_MD {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::present::io::import::deserializers::artifacts::md::v_commonmark::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PDF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::present::io::import::deserializers::artifacts::pdf::v1_4::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::present::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PPTX {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::present::io::import::deserializers::artifacts::pptx::v_ecma_376::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_SVG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::present::io::import::deserializers::artifacts::svg::v1_1::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::present::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }

            }
            Err(ComposeError { message: "PresentComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🗄️ Relocated verbatim from the former artifact-tree `⚙️engine`'s root `component.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — this is the real, typed composer registry
/// (`ComposerEntry`/`entries()`), mirroring `🧱️block/🗿️artifacts/◻2d/🚪️io/🦀️component.rs`'s
/// `io_registry` shape exactly. The artifact root's OWN `io_registry` module is a DIFFERENT, thinner
/// wrapper (`&'static [&'static ComposerEntry]`) that calls this one fully-qualified — never confuse
/// the two, and never reference this module by a bare `io_registry::` path from outside `🚪️io`.
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::present::standards::v1::subsets::any::schema::PresentComposer as PresentAnyComposer;
    use crate::artifacts::present::standards::v1::subsets::any::schema::PresentBuilder as PresentAnyBuilder;

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
    const PRESENT_DIALECT: Dialect = Dialect { artifact_kind: "s.present", standard: StandardId("1"), subset: SubsetId("*") };
    const PRESENT_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::present::PresentSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PRESENT_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => PresentAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => PresentAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "PresentComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PRESENT_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::present::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "PresentComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_PPTX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    fn compose_export_pptx(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::present::io::export::serializers::artifacts::pptx::v_ecma_376::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PPTX_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::present::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::present::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    fn compose_export_md(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::present::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_MD_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::present::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::present::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<PresentAnyComposer>(),
            ComposerEntry { writes: EXPORT_PPTX_DIALECT, reads: &[PRESENT_DIALECT], compose: compose_export_pptx },
            ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[PRESENT_DIALECT], compose: compose_export_svg },
            ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[PRESENT_DIALECT], compose: compose_export_pdf },
            ComposerEntry { writes: EXPORT_MD_DIALECT, reads: &[PRESENT_DIALECT], compose: compose_export_md },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[PRESENT_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PRESENT_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🔖️MediaCodec
/// 🖼️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): title-card SVG export for the app catalogue/
/// thumbnail surface. There is no real drawing content to route through semio/drawing here (this is a
/// generic placeholder title card, not a geometry export), so the shared framework helper stays — but
/// its output is round-tripped through stdio's own real SVG codec (`parse_svg_xml`/`write_svg_xml`)
/// before being returned, which both validates it is genuinely spec-conformant SVG and exercises the
/// real stdio engine rather than returning the framework helper's raw string untouched.
pub fn animate_present_document_json_to_svg(value: &serde_json::Value) -> Result<(String, u32, u32), String> {
    use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, write_svg_xml};
    let (svg, width, height) = semio_framework_os::title_card_svg(value, "Animate Present", 1280, 720)?;
    let doc = parse_svg_xml(&svg)?;
    Ok((write_svg_xml(&doc), width, height))
}

/// 📥️ Builds a degenerate-but-valid one-slide deck from a rasterized DWG drawing, for the DWG import
/// path. `stdio_gap`: this plugin's write scope explicitly forbids inventing a converter inside
/// animate — there is no bridge anywhere in stdio/framework from the legacy
/// `semio_s_plugin_stdio::artifacts::dwg::DwgDrawing` (11 geometry variants: Line/Point/Circle/Arc/Ellipse/LwPolyline/
/// Spline/Text/Face3d/Polyline3d/PolyfaceMesh) to semio's `SemioDrawingSnapshot`/`DrawNode` tree.
/// Hand-rolling that conversion here would duplicate `semio_framework_os::dwg_drawing_to_svg`'s
/// existing, correct, shared geometry logic for a hand-rolled struct — reported in
/// `w5a--report.md`'s stdio_gaps rather than invented. The framework helpers stay (shared,
/// non-duplicative utilities, not local ad-hoc codec code); the SVG they produce is still round-
/// tripped through stdio's real SVG codec before rasterization, same as the title-card path.
pub fn animate_present_document_json_from_dwg(drawing: &semio_s_plugin_stdio::artifacts::dwg::DwgDrawing) -> Result<serde_json::Value, String> {
    use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, write_svg_xml};
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let validated_svg = write_svg_xml(&parse_svg_xml(&svg)?);
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&validated_svg, width, height)?;
    let frame = crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let source = crate::artifacts::present::FigureTileSource { src: format!("data:image/png;base64,{png_base64}"), kind: "image".into(), frame: frame.clone(), source_aspect: Some(width as f64 / height.max(1) as f64), pdf_page: None };
    let tiles = vec![crate::artifacts::present::FigureTileDraft { id: "imported-drawing".into(), name: "Imported Drawing".into(), crop: frame }];
    let deck = crate::artifacts::present::present_snapshot_with_tiles(&source, &tiles);
    serde_json::to_value(&deck).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn animate_present_document_json_to_svg_embeds_title() {
        let (svg, width, height) = animate_present_document_json_to_svg(&json!({ "title": "My Deck" })).expect("svg");
        assert!(svg.contains("My Deck"));
        assert_eq!((width, height), (1280, 720));
    }

    #[test]
    fn animate_present_document_json_to_svg_falls_back_to_app_label_without_title() {
        let (svg, _, _) = animate_present_document_json_to_svg(&json!({})).expect("svg fallback");
        assert!(svg.contains("Animate Present"));
    }

    #[test]
    fn from_dwg_builds_single_slide_deck_from_entity() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing {
            layers: vec![semio_s_plugin_stdio::artifacts::dwg::DwgLayer::default()],
            entities: vec![semio_s_plugin_stdio::artifacts::dwg::DwgEntity {
                layer: 0,
                color: semio_s_plugin_stdio::artifacts::dwg::DwgColor::ByLayer,
                geometry: semio_s_plugin_stdio::artifacts::dwg::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
            }],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let document = animate_present_document_json_from_dwg(&drawing).expect("from_dwg");
        let deck: crate::artifacts::present::PresentSnapshot = serde_json::from_value(document).expect("deck");
        assert_eq!(deck.schema, crate::artifacts::present::PRESENT_DOCUMENT_SCHEMA);
        let (source, tiles) = crate::artifacts::present::present_working_scene(&deck);
        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0].name, "Imported Drawing");
        assert!(source.src.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn from_dwg_never_errors_on_empty_drawing() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let document = animate_present_document_json_from_dwg(&drawing).expect("from_dwg on empty drawing");
        let deck: crate::artifacts::present::PresentSnapshot = serde_json::from_value(document).expect("deck");
        let (_, tiles) = crate::artifacts::present::present_working_scene(&deck);
        assert_eq!(tiles.len(), 1);
    }
}
//#endregion 🔖️MediaCodec
