//! 🚪️ IO s.lowpoly (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub async fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"]
}
pub async fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::LowpolyAnalyzer;
    use crate::artifacts::lowpoly::LowpolySnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_LAS: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PLY: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct LowpolyComposerComposition;

    impl ArtifactComposition for LowpolyComposerComposition {
        type Snapshot = LowpolySnapshot;
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
                    let analysis = LowpolyAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_LAS {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::las::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PLY {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::lowpoly::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "LowpolyComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::LowpolyBuilder as LowpolyAnyBuilder;
    use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::LowpolyComposer as LowpolyAnyComposer;
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
    const LOWPOLY_DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };
    const LOWPOLY_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::lowpoly::LowpolySnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == LOWPOLY_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => LowpolyAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => LowpolyAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "LowpolyComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == LOWPOLY_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::lowpoly::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "LowpolyComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    async fn compose_export_las(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    async fn compose_export_ply(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    async fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    async fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    async fn compose_export_dwg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    async fn compose_export_stl(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    async fn compose_export_gltf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    async fn compose_export_obj(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<LowpolyAnyComposer>(),
                    ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_las },
                    ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_ply },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_dwg },
                    ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_stl },
                    ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_gltf },
                    ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_obj },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
