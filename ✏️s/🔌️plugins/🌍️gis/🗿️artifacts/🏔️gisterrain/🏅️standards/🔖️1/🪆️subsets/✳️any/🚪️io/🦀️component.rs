//! 🚪️ IO s.gisterrain (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"]
}
// 🗺️ No "stdio.png"/"stdio.json"/"stdio.dwg" here (export-only list): gismap owns those EXPORT
// claims, see `🚪️IoRegistry` region below. Import is unaffected — all three stay in `import_stdio_kinds` above.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.gltf", "stdio.las", "stdio.obj", "stdio.ply", "stdio.stl", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::GisTerrainAnalyzer;
    use crate::artifacts::gisterrain::GisTerrainSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.gisterrain", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_LAS: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PLY: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct GisTerrainComposerComposition;

    impl ArtifactComposition for GisTerrainComposerComposition {
        type Snapshot = GisTerrainSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_JSON, DEP_LAS, DEP_OBJ, DEP_PLY, DEP_PNG, DEP_STL, DEP_TXT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = GisTerrainAnalyzer::analyze(&[native]).await;
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_LAS {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::las::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PLY {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::gisterrain::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "GisTerrainComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
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
/// bare `io_registry` name: both artifact-root `🦀️component.rs` files also define their own
/// `io_registry` wrapper (a `&[&ComposerEntry]` view, different type) which shadows a bare call.
pub mod io_registry {
    use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::GisTerrainComposer as GisTerrainAnyComposer;
    use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::GisterrainBuilder as GisTerrainAnyBuilder;
    use semio_framework_plugin::{composer_entry_of, resolve_ready, ArtifactBuilder, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource, IoConfidence, IoPayload, StandardId, SubsetId};
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
    const GISTERRAIN_DIALECT: Dialect = Dialect { artifact_kind: "s.gisterrain", standard: StandardId("1"), subset: SubsetId("*") };
    const GISTERRAIN_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::gisterrain::GisTerrainSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == GISTERRAIN_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => resolve_ready(GisTerrainAnyBuilder::from_text(t)).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => resolve_ready(GisTerrainAnyBuilder::from_binary(b)).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return resolve_ready(builder.build()).map_err(|diagnostics| ComposeError { message: "GisTerrainComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == GISTERRAIN_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::gisterrain::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "GisTerrainComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_las(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gisterrain::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_ply(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gisterrain::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    // 🗺️ No `compose_export_png`/`compose_export_json`/`compose_export_dwg` here: gismap owns all
    // three EXPORT claims (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 — see
    // `../../🦀️component.rs`'s `definition()` docstring for the ownership rule). `derived_composition`'s
    // `reads()` above still lists `DEP_PNG`/`DEP_JSON`/`DEP_DWG`, so import is unaffected.
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gisterrain::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gisterrain::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::gisterrain::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<GisTerrainAnyComposer>(),
                    ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[GISTERRAIN_DIALECT], compose: compose_export_las },
                    ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[GISTERRAIN_DIALECT], compose: compose_export_ply },
                    ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[GISTERRAIN_DIALECT], compose: compose_export_stl },
                    ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[GISTERRAIN_DIALECT], compose: compose_export_gltf },
                    ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[GISTERRAIN_DIALECT], compose: compose_export_obj },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️IoRegistry
