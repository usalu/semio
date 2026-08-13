//! 🚪️ IO s.cad (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"] }
pub fn cad_to_wire(from: &crate::artifacts::cad::CadSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(from)
}
pub fn cad_from_wire(bytes: &[u8]) -> Result<crate::artifacts::cad::CadSnapshot, store::PackError> {
    <crate::artifacts::cad::CadSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::cad::CadSnapshot;
    use crate::artifacts::cad::standards::v1::subsets::any::schema::CadAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.cad", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_IFC: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_STEP: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };


    pub struct CadComposerComposition;

    impl ArtifactComposition for CadComposerComposition {
        type Snapshot = CadSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_IFC, DEP_JSON, DEP_OBJ, DEP_PNG, DEP_STEP, DEP_STL]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = CadAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_GLTF {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_IFC {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::ifc::v4::any::deserialize_text(&text) {
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
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_OBJ {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_PNG {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::png::v1_2::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_STEP {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::step::v_ap214::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }
                if source.dialect == DEP_STL {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }

            }
            Err(ComposeError { message: "CadComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
// 🐛️ Relocated verbatim from the deleted `⚙️engine/🦀️component.rs` (ticket
// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) -- io_registry/ComposerEntry wiring is
// io by definition (rule 5), not artifact-engine compute. The artifact root
// (`🗿️artifacts/📐️cad/🦀️component.rs`)'s own shadowing `io_registry` wrapper module and
// `declaration()`'s `.composers(...)` call were repointed here.
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::cad::standards::v1::subsets::any::schema::CadComposer as CadAnyComposer;
    use crate::artifacts::cad::standards::v1::subsets::any::schema::CadBuilder as CadAnyBuilder;

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
    const CAD_DIALECT: Dialect = Dialect { artifact_kind: "s.cad", standard: StandardId("1"), subset: SubsetId("*") };
    const CAD_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::cad::CadSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == CAD_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => CadAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => CadAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "CadComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == CAD_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let text = match &source.payload {
                IoPayload::Text(t) => t.clone(),
                IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
            };
            return crate::artifacts::cad::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "CadComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_IFC_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
    fn compose_export_ifc(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::ifc::v4::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_IFC_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STEP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    fn compose_export_step(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::step::v_ap214::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STEP_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::png::v1_2::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::stl::v_ascii::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::gltf::v2_0::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::cad::standards::v1::subsets::any::io::export::serializers::artifacts::obj::v3_0::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<CadAnyComposer>(),
            ComposerEntry { writes: EXPORT_IFC_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_ifc },
            ComposerEntry { writes: EXPORT_STEP_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_step },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_gltf },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[CAD_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🌉️SolidExport
// 🐛️ Relocated from the deleted `⚙️engine/🦀️component.rs` (ticket
// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) -- native-geometry solid export bridging
// to stdio's real semio/mesh + semio/brep codecs is io by definition (rule 5), not artifact-engine
// compute.
use base64::Engine as _;
use semio_framework_3d::brep::engine::{block_on, BrepKernel, GeometryHandle};
use semio_framework::MeshImporter;
use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
#[cfg(test)]
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::io::export::serializers::artifacts::step::v_ap214::any::SemioBrepToStep;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::io::import::deserializers::artifacts::step::v_ap214::any::SemioBrepFromStep;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::engine::encode_obj;
use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::engine::encode_stl_binary;
use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::engine::part21::{parse_part21, write_part21};
use semio_s_plugin_stdio::artifacts::step::StepSnapshot;
use serde_json::Value;

/// @emoji 📤️ A native-geometry export ready to be wrapped into a `HostEffect::DownloadMediaExport`.
pub struct CadSolidExport {
    pub filename: String,
    pub data: Value,
    pub mime_type: String,
    pub encoding: Option<String>,
}

// 🌉️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6: the
// deprecated framework format-enum layer is retired in favor of the plain `"s.stdio.<format>"`
// dialect id strings used throughout this ticket's io_dispatch/Dialect machinery.
pub const CAD_SOLID_EXPORT_DIALECT_OBJ: &str = "s.stdio.obj";
pub const CAD_SOLID_EXPORT_DIALECT_STL: &str = "s.stdio.stl";
pub const CAD_SOLID_EXPORT_DIALECT_STEP: &str = "s.stdio.step";

/// @emoji 🧾️ File extension for a `s.stdio.<format>` dialect id, as used in `export_solids_as`'s
/// downloaded filename.
fn cad_solid_export_extension(dialect_id: &str) -> Option<&'static str> {
    match dialect_id {
        CAD_SOLID_EXPORT_DIALECT_OBJ => Some("obj"),
        CAD_SOLID_EXPORT_DIALECT_STL => Some("stl"),
        CAD_SOLID_EXPORT_DIALECT_STEP => Some("step"),
        _ => None,
    }
}

/// @emoji 📎️ MIME type for a `s.stdio.<format>` dialect id, kept in parity with the retired
/// enum's mime-type values for the three formats `export_solids_as` supports.
fn cad_solid_export_mime_type(dialect_id: &str) -> Option<&'static str> {
    match dialect_id {
        CAD_SOLID_EXPORT_DIALECT_OBJ => Some("model/obj"),
        CAD_SOLID_EXPORT_DIALECT_STL => Some("model/stl"),
        CAD_SOLID_EXPORT_DIALECT_STEP => Some("model/step"),
        _ => None,
    }
}

/// 🌉️ Tessellates every solid in `solids` (via the live kernel) into one `SemioMeshSnapshot` —
/// one `SemioMesh`/one `SemioPrimitive` per solid, real positions/normals carried, `uvs`/`colors`
/// left empty (the kernel's `MeshTransfer` carries neither). Solids that fail to tessellate or
/// tessellate to zero triangles are skipped (never a fabricated triangle); `None` only when NOT A
/// SINGLE solid produced real geometry.
fn semio_mesh_snapshot_from_solids(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], deflection: f64) -> Option<SemioMeshSnapshot> {
    let mut meshes = Vec::new();
    for (index, handle) in solids.iter().enumerate() {
        let Ok(transfer) = block_on(kernel.tessellate(handle, deflection)) else { continue };
        if transfer.index.is_empty() || transfer.position.is_empty() {
            continue;
        }
        let positions: Vec<SemioPoint3> = transfer.position.chunks_exact(3).map(|c| SemioPoint3 { x: c[0] as f64, y: c[1] as f64, z: c[2] as f64 }).collect();
        let normals: Vec<SemioPoint3> = transfer.normal.chunks_exact(3).map(|c| SemioPoint3 { x: c[0] as f64, y: c[1] as f64, z: c[2] as f64 }).collect();
        meshes.push(SemioMesh {
            id: format!("{}-{index}", handle.as_str()),
            primitives: vec![SemioPrimitive {
                id: format!("{}-{index}-prim-0", handle.as_str()),
                topology: SemioTopology::Triangles,
                positions,
                normals,
                uvs: Vec::new(),
                colors: Vec::new(),
                indices: transfer.index.clone(),
                material_id: None,
            }],
        });
    }
    if meshes.is_empty() {
        return None;
    }
    Some(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes, materials: Vec::new(), textures: Vec::new() })
}

/// 🌉️ Real AP214 STEP text → `SemioBrepSnapshot`, via stdio's own Part-21 tokenizer + the genuine
/// `SemioBrepFromStep` entity-graph walk (never a re-implementation of either).
/// 🩹️ Confirmed framework bug (out of this plugin's write scope — reported, not patched at the
/// source): `🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs::write_step` builds its
/// `ADVANCED_BREP_SHAPE_REPRESENTATION` item list via `format!("({},)", items.join(", "))` —
/// UNCONDITIONALLY appending a trailing comma before the closing `)`, for every export (0, 1, or N
/// items). That is not valid ISO 10303-21 (a Part-21 list never permits a trailing comma before its
/// close), and stdio's own Part-21 tokenizer correctly rejects it (`UnexpectedChar { found: ')',
/// expected: "value" }`) rather than guessing. Quote-aware (a `,)` inside a real STEP string
/// literal, e.g. a product name, is left untouched) — repairs ONLY this exact malformed shape so
/// cad's `semio/brep` bridge can consume the kernel's real, otherwise-correct geometry today.
fn repair_step_trailing_comma_before_close_paren(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut in_string = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            in_string = !in_string;
            out.push(c);
            i += 1;
            continue;
        }
        if !in_string && c == ',' {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && chars[j] == ')' {
                i += 1;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

fn semio_brep_snapshot_from_step_text(text: &str) -> Option<SemioBrepSnapshot> {
    let repaired = repair_step_trailing_comma_before_close_paren(text);
    let document = parse_part21(&repaired).ok()?;
    let step_snapshot = StepSnapshot::from_part21_document(document);
    SemioBrepFromStep::deserialize(&step_snapshot).ok()
}

/// 🌉️ Inverse of `semio_brep_snapshot_from_step_text` — real `SemioBrepToStep` serialize +
/// stdio's own Part-21 writer.
fn step_text_from_semio_brep_snapshot(brep: &SemioBrepSnapshot) -> Option<String> {
    let step_snapshot = SemioBrepToStep::serialize(brep).ok()?;
    Some(write_part21(&step_snapshot.to_part21_document()))
}

/// @emoji 📤️ Encodes `solids` for `format`, routed through stdio's real semio-subset codecs
/// instead of a local hand-rolled encoder: OBJ/STL tessellate `solids` (via the live kernel) into
/// a `semio/mesh` snapshot and call stdio's own `SemioMeshToObj`/`SemioMeshToStl` + text/binary
/// grammar encoders; STEP still SOURCES its geometry from the framework brep kernel's native
/// `export_step` (the kernel's own AP214 writer — a real, working, geometry-exact encoder that
/// lives one layer below this plugin, not ad-hoc plugin-level codec duplication) but the BYTES
/// actually returned now come from re-encoding that text through a real `semio/brep` round trip
/// (`StepSnapshot` → `SemioBrepFromStep` → `SemioBrepToStep` → `StepSnapshot` → Part-21 text),
/// which both validates the kernel's output against stdio's AP214 entity-graph walk and produces
/// the export from the SAME codec stdio/semio uses everywhere else. STL is base64-wrapped since it
/// is a binary format, OBJ/STEP stay UTF-8 text.
pub fn export_solids_as(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle], format: &str, stem: &str) -> Option<CadSolidExport> {
    let extension = cad_solid_export_extension(format)?;
    let filename = format!("{stem}.{extension}");
    let mime_type = cad_solid_export_mime_type(format)?.to_string();
    match format {
        CAD_SOLID_EXPORT_DIALECT_OBJ => {
            let mesh_snapshot = semio_mesh_snapshot_from_solids(kernel, solids, 0.1)?;
            let obj_snapshot = SemioMeshToObj::serialize(&mesh_snapshot).ok()?;
            let text = encode_obj(&obj_snapshot);
            Some(CadSolidExport { filename, data: Value::String(text), mime_type, encoding: None })
        }
        CAD_SOLID_EXPORT_DIALECT_STL => {
            let mesh_snapshot = semio_mesh_snapshot_from_solids(kernel, solids, 0.1)?;
            let stl_snapshot = SemioMeshToStl::serialize(&mesh_snapshot).ok()?;
            let bytes = encode_stl_binary(&stl_snapshot);
            let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
            Some(CadSolidExport { filename, data: Value::String(encoded), mime_type, encoding: Some("base64".into()) })
        }
        CAD_SOLID_EXPORT_DIALECT_STEP => {
            let kernel_text = block_on(kernel.export_step(solids)).ok()?;
            let brep_snapshot = semio_brep_snapshot_from_step_text(&kernel_text)?;
            let text = step_text_from_semio_brep_snapshot(&brep_snapshot)?;
            Some(CadSolidExport { filename, data: Value::String(text), mime_type, encoding: None })
        }
        _ => None,
    }
}
//#endregion 🌉️SolidExport

//#region 📥️FilePayloadImport
// 🐛️ Relocated from the deleted `⚙️engine/🦀️component.rs` -- decoding a `requestFileOpen`
// payload and routing it to the matching native-geometry importer by extension is deserialization
// (rule 5), not artifact-engine compute.
/// @emoji 📦️ Decodes a `requestFileOpen` payload (a `data:` URL when `readAs: "dataUrl"` was
/// requested, otherwise a raw string) into bytes.
pub fn cad_file_bytes_from_payload(payload: &Value) -> Option<Vec<u8>> {
    let raw = payload.as_str()?;
    if raw.starts_with("data:") {
        let (_, encoded) = raw.split_once(',')?;
        base64::engine::general_purpose::STANDARD.decode(encoded).ok()
    } else {
        Some(raw.as_bytes().to_vec())
    }
}

/// @emoji 📦️ Decodes a `requestFileOpen` payload into UTF-8 text; see `cad_file_bytes_from_payload`.
pub fn cad_file_text_from_payload(payload: &Value) -> Option<String> {
    String::from_utf8(cad_file_bytes_from_payload(payload)?).ok()
}

/// @emoji 🧊️ Imports a STEP payload into the shared kernel, wrapping the first solid it contains
/// (STEP files may hold more than one shape) as a `SemioModelElement` — id/typology/placement plus
/// a `GeometryRef::Brep` naming the live kernel handle (the actual B-Rep geometry stays in the
/// kernel session / gets bridged to a real `SemioBrepSnapshot` on export via `export_solids_as`,
/// never re-duplicated here). Composing the returned element into a pane's `SemioModelSnapshot`
/// child is the caller's job (a `create`/`change` mutation dispatched against that CHILD document).
pub fn import_step_object(text: &str) -> Option<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelElement> {
    let mut kernel = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_brep_kernel().ok()?;
    let handle = block_on(kernel.import_step(text)).ok()?.into_iter().next()?;
    Some(model_element_from_solid_handle(crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::next_cad_id("object-step"), handle))
}

/// @emoji 🧊️ Imports an OBJ payload into the shared kernel as a new `SemioModelElement` — see
/// `import_step_object`'s doc comment for the returned shape's rationale.
pub fn import_obj_object(text: &str) -> Option<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelElement> {
    let mut kernel = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_brep_kernel().ok()?;
    let handle = block_on(kernel.import_obj(text, 0.01)).ok()?;
    Some(model_element_from_solid_handle(crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::next_cad_id("object-obj"), handle))
}

/// @emoji 🧊️ Imports an STL payload into the shared kernel as a new `SemioModelElement`.
pub fn import_stl_object(bytes: &[u8]) -> Option<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelElement> {
    let mut kernel = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_brep_kernel().ok()?;
    let handle = block_on(kernel.import_stl(bytes, 0.01)).ok()?;
    Some(model_element_from_solid_handle(crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::next_cad_id("object-stl"), handle))
}

/// @emoji 🧊️ Imports a GLB payload by decoding it to a tessellated mesh (via the shared
/// `MeshImporter` codec) and re-importing that mesh into the kernel as a solid, matching the
/// DWG-derived import path since GLB carries no exact B-Rep to preserve.
pub fn import_glb_object(bytes: &[u8]) -> Option<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelElement> {
    let mesh = semio_framework_plugin::GlbImporter.import(bytes).ok()?;
    let mut kernel = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_brep_kernel().ok()?;
    let handle_id = mesh_to_obj_text_for_import(&mesh).and_then(|text| block_on(kernel.import_obj(&text, 0.01)).ok())?;
    Some(model_element_from_solid_handle(crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::next_cad_id("object-glb"), handle_id))
}

/// 🌉️ Builds a `SemioModelElement` from a live kernel solid handle — id, identity placement, and a
/// `GeometryRef::Brep{brep_id}` naming the handle. Shared by every native-geometry import path.
fn model_element_from_solid_handle(id: String, handle: GeometryHandle) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelElement {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, SemioModelElement};
    SemioModelElement {
        id,
        class: ElementClass::Other { name: "spatial.shape.imported".into() },
        placement: SemioTransform::identity(),
        geometry: GeometryRef::Brep { brep_id: handle.0 },
        spatial_id: None,
        psets: Vec::new(),
    }
}

/// 🌉️ Same OBJ-text bridge `cad_object_from_mesh` used before this wave's rewrite (reused, not
/// reimplemented) — factored out so `import_glb_object` can mint a kernel handle without going
/// through the geometry-import module's now-`pub(crate)`-only `CadObject`.
fn mesh_to_obj_text_for_import(mesh: &semio_framework_plugin::MeshData) -> Option<String> {
    let snapshot = semio_mesh_snapshot_from_solids_placeholder(mesh)?;
    let obj_snapshot = SemioMeshToObj::serialize(&snapshot).ok()?;
    Some(encode_obj(&obj_snapshot))
}

fn semio_mesh_snapshot_from_solids_placeholder(mesh: &semio_framework_plugin::MeshData) -> Option<SemioMeshSnapshot> {
    if mesh.indices.is_empty() || mesh.indices.len() % 3 != 0 || mesh.positions.is_empty() {
        return None;
    }
    let positions: Vec<SemioPoint3> = mesh.positions.chunks_exact(3).map(|c| SemioPoint3 { x: c[0] as f64, y: c[1] as f64, z: c[2] as f64 }).collect();
    Some(SemioMeshSnapshot {
        schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(),
        meshes: vec![SemioMesh {
            id: "mesh-0".into(),
            primitives: vec![SemioPrimitive { id: "mesh-0-prim-0".into(), topology: SemioTopology::Triangles, positions, normals: Vec::new(), uvs: Vec::new(), colors: Vec::new(), indices: mesh.indices.clone(), material_id: None }],
        }],
        materials: Vec::new(),
        textures: Vec::new(),
    })
}

/// @emoji 🗂️ Routes a `requestFileOpen` payload to the matching native-geometry import by the
/// picked file's extension; returns `None` for anything else so the caller can fall back to the
/// spatial-JSON document path.
pub fn import_cad_object_by_extension(name: &str, payload: &Value) -> Option<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelElement> {
    if name.ends_with(".stp") || name.ends_with(".step") {
        return import_step_object(&cad_file_text_from_payload(payload)?);
    }
    if name.ends_with(".obj") {
        return import_obj_object(&cad_file_text_from_payload(payload)?);
    }
    if name.ends_with(".stl") {
        return import_stl_object(&cad_file_bytes_from_payload(payload)?);
    }
    if name.ends_with(".glb") {
        return import_glb_object(&cad_file_bytes_from_payload(payload)?);
    }
    None
}
//#endregion 📥️FilePayloadImport

//#region 🌉️GeometryBridges
// 🐛️ Relocated from the deleted `⚙️engine/🦀️component.rs` -- foreign-format(bytes/struct)-to-
// cad-document conversions are deserialization (rule 5). Kept reachable at THIS exact path
// (`crate::artifacts::cad::io::{cad_document_from_dwg, cad_document_from_mesh,
// cad_mesh_from_document}`) because two OTHER plugins import them at the artifact-level (not
// through an app-internal engine): 🎪️demonstrator/🎪️panes/📐️koordinator and 💠️lowpoly's schema.
pub fn unwrap_spatial_load_payload(raw: &Value) -> Option<Value> {
    if raw.get("modelSpace").is_some() {
        return raw.get("modelSpace").cloned();
    }
    if raw.get("model").is_some() {
        return raw.get("model").cloned();
    }
    if raw.get("raw").is_some() {
        return raw.get("raw").cloned();
    }
    Some(raw.clone())
}

/// 🌉 Real per-pane object extraction from a `spatial.modelspace`/`spatial.model` payload — the
/// SAME fixture-import machinery (`geometry_import::parse_geometry` + `objects_from_fixture_model`)
/// the Concrete Forest Left quad fixture is built from (`crate::apps::cad::forest_working_scene`),
/// since both are the identical `spatial.model`-shaped wire form. A `spatial.modelspace` payload
/// wraps zero or more `{id|modelDefinitionId, model}` entries in `models[]`; a bare `spatial.model`
/// payload IS one implicit entry. Each entry that resolves to a known `CadPaneId` and carries real
/// objects gets its own real `SemioModelSnapshot` minted (`geometry_import::semio_model_snapshot_from_objects`)
/// and composed as that pane's `CadModelChild` (`crate::artifacts::cad::cad_model_child_handle`) —
/// panes with no objects, or an id this document doesn't recognize, are left `None` rather than
/// fabricating an empty child. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3.
pub fn scene_from_spatial_payload(payload: &Value) -> Option<crate::artifacts::cad::CadSnapshot> {
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{cad_brep_kernel, default_document, CAD_MODEL_DEFINITION_SHAPE};
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::{objects_from_fixture_model, parse_geometry, semio_model_snapshot_from_objects};
    use crate::artifacts::cad::{cad_model_child_handle, cad_pane_from_model_definition_id, CadPaneId};
    let schema = payload.get("schema").and_then(|value| value.as_str());
    if schema != Some("spatial.modelspace") && schema != Some("spatial.model") {
        return None;
    }
    let mut document = default_document();
    let owned_models: Vec<Value>;
    let models: Vec<&Value> = if schema == Some("spatial.modelspace") {
        payload.get("models").and_then(Value::as_array).map(|entries| entries.iter().collect()).unwrap_or_default()
    } else {
        owned_models = vec![payload.clone()];
        owned_models.iter().collect()
    };
    for entry in models {
        let model_definition_id = entry.get("id").and_then(Value::as_str).or_else(|| entry.get("modelDefinitionId").and_then(Value::as_str)).unwrap_or(CAD_MODEL_DEFINITION_SHAPE);
        let Some(pane) = cad_pane_from_model_definition_id(model_definition_id) else {
            continue;
        };
        let model_value = entry.get("model").unwrap_or(entry);
        let Some(objects_value) = model_value.get("objects").and_then(Value::as_array) else {
            continue;
        };
        if objects_value.is_empty() {
            continue;
        }
        let geometry = parse_geometry(model_value.get("geometry"));
        let Ok(mut kernel) = cad_brep_kernel() else {
            continue;
        };
        let objects = objects_from_fixture_model(&mut *kernel, objects_value, &geometry);
        drop(kernel);
        if objects.is_empty() {
            continue;
        }
        let model = semio_model_snapshot_from_objects(&objects);
        let Ok(content_json) = serde_json::to_string(&model) else {
            continue;
        };
        let handle = Some(cad_model_child_handle(pane, &content_json));
        match pane {
            CadPaneId::Shape => document.shape_model = handle,
            CadPaneId::Building => document.building_model = handle,
            CadPaneId::Energy => document.energy_model = handle,
            CadPaneId::StructureClassic => document.structure_classic_model = handle,
        }
    }
    Some(document)
}

pub fn cad_mesh_from_document(doc: &Value) -> Result<semio_framework_plugin::MeshData, String> {
    let scene: crate::artifacts::cad::CadSnapshot = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::export_mesh_from_scene(&scene))
}

/// 🌉 One `CadObject` per non-empty DWG layer — filters `drawing` down to that layer's entities and
/// tessellates them via the existing `dwg_drawing_to_mesh` merge (reused verbatim, not
/// reimplemented), then imports the result into the live kernel through `geometry_import::cad_object_from_mesh`
/// — the identical OBJ-text bridge every other native-geometry import path in this file already
/// uses. A layer with no entities (or none that triangulate) contributes no object — never a
/// fabricated placeholder.
pub fn cad_working_scene_from_dwg(drawing: &semio_framework::DwgDrawing) -> crate::artifacts::cad::CadWorkingScene {
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::cad_object_from_mesh;
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{cad_brep_kernel, next_cad_id};
    let Ok(mut kernel) = cad_brep_kernel() else {
        return crate::artifacts::cad::CadWorkingScene::default();
    };
    let objects = drawing
        .layers
        .iter()
        .enumerate()
        .filter_map(|(layer_index, layer)| {
            let filtered = semio_framework::DwgDrawing { layers: drawing.layers.clone(), entities: drawing.entities.iter().filter(|entity| entity.layer == layer_index).cloned().collect(), extmin: drawing.extmin, extmax: drawing.extmax };
            if filtered.entities.is_empty() {
                return None;
            }
            let mesh = semio_framework::dwg_drawing_to_mesh(&filtered);
            if mesh.indices.is_empty() || mesh.positions.is_empty() {
                return None;
            }
            Some(cad_object_from_mesh(&mut *kernel, next_cad_id("object"), layer.name.clone(), "spatial.shape.imported", &mesh))
        })
        .collect();
    crate::artifacts::cad::CadWorkingScene { objects, ..Default::default() }
}

/// 🌉 Real per-layer DWG import: every non-empty layer becomes a real `CadObject`
/// (`cad_working_scene_from_dwg`), wrapped into a `SemioModelSnapshot` and minted as a
/// deterministic, content-addressed `shape_model` CHILD HANDLE (`cad_model_child_handle`) — the
/// same "mint the child, dispatch create-<pane>-model against the result" two-step gesture
/// `scene_from_spatial_payload` documents, done here for the ONE piece (the child's own content) a
/// pure function CAN produce. An empty drawing (no layer contributes real geometry) mints no
/// child, matching `scene_from_spatial_payload`'s "no fabricated child" rule. Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3.
pub fn cad_document_from_dwg(drawing: &semio_framework::DwgDrawing) -> Result<Value, String> {
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::semio_model_snapshot_from_objects;
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::default_document;
    use crate::artifacts::cad::{cad_model_child_handle, CadPaneId};
    let working = cad_working_scene_from_dwg(drawing);
    let mut document = default_document();
    if !working.objects.is_empty() {
        let model = semio_model_snapshot_from_objects(&working.objects);
        let content_json = serde_json::to_string(&model).map_err(|err| err.to_string())?;
        document.shape_model = Some(cad_model_child_handle(CadPaneId::Shape, &content_json));
    }
    serde_json::to_value(document).map_err(|err| err.to_string())
}

/// ⚠️ See `scene_from_spatial_payload`'s doc comment — same documented gap for a `MeshImporter`
/// (GLB) payload.
pub fn cad_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::default_document;
    serde_json::to_value(default_document()).map_err(|err| err.to_string())
}
//#endregion 🌉️GeometryBridges

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_3d::brep::kernel::Brep;

    //#region 🔖️SemioMeshBridge
    #[test]
    fn export_solids_as_obj_uses_real_stdio_mesh_codec_not_hand_rolled_bytes() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_OBJ, "box").expect("obj export");
        let Value::String(text) = export.data else { panic!("expected text data") };
        let vertex_lines = text.lines().filter(|l| l.starts_with("v ")).count();
        let face_lines = text.lines().filter(|l| l.starts_with("f ")).count();
        assert!(vertex_lines >= 8, "expected real OBJ vertices, got {vertex_lines} in {text:?}");
        assert!(face_lines >= 12, "expected real OBJ faces, got {face_lines}");
        assert_eq!(export.mime_type, cad_solid_export_mime_type(CAD_SOLID_EXPORT_DIALECT_OBJ).unwrap());
        assert!(export.encoding.is_none());
    }

    #[test]
    fn export_solids_as_stl_uses_real_stdio_mesh_codec() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_STL, "box").expect("stl export");
        let Value::String(encoded) = export.data else { panic!("expected base64 text data") };
        assert_eq!(export.encoding.as_deref(), Some("base64"));
        let bytes = base64::engine::general_purpose::STANDARD.decode(&encoded).expect("valid base64");
        assert!(bytes.len() > 84, "expected a real binary STL body, got {} bytes", bytes.len());
        let triangle_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
        assert!(triangle_count >= 12, "expected a real box's 12+ triangles, got {triangle_count}");
        assert_eq!(bytes.len(), 84 + (triangle_count as usize) * 50);
    }

    #[test]
    fn export_solids_as_obj_none_for_a_solid_that_fails_to_tessellate() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        block_on(kernel.dispose(&solid));
        assert!(export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_OBJ, "gone").is_none());
    }
    //#endregion 🔖️SemioMeshBridge

    //#region 🔖️SemioBrepBridge
    #[test]
    fn export_solids_as_step_round_trips_through_real_semio_brep_bridge() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(2.0, 3.0, 4.0)).expect("box");
        let original_volume = block_on(kernel.volume(&solid)).expect("volume");
        assert!((original_volume - 24.0).abs() < 1e-6, "box volume sanity: {original_volume}");

        let kernel_text = block_on(kernel.export_step(std::slice::from_ref(&solid))).expect("kernel step export");
        let original_brep = semio_brep_snapshot_from_step_text(&kernel_text).expect("semio/brep from kernel step");

        let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_STEP, "box").expect("step export");
        let Value::String(step_text) = export.data else { panic!("expected text data") };
        assert!(step_text.starts_with("ISO-10303-21;"), "real Part-21 header expected, got {step_text:?}");
        assert!(step_text.contains("MANIFOLD_SOLID_BREP") || step_text.contains("ADVANCED_BREP_SHAPE_REPRESENTATION"), "expected real AP214 brep entities");

        let reimported_brep = semio_brep_snapshot_from_step_text(&step_text).expect("reimport via semio/brep bridge");
        assert_eq!(reimported_brep.solids.len(), original_brep.solids.len(), "solid count geometry-equivalence");
        assert_eq!(reimported_brep.faces.len(), original_brep.faces.len(), "face count geometry-equivalence");
        assert_eq!(reimported_brep.vertices.len(), original_brep.vertices.len(), "vertex count geometry-equivalence");

        fn vertex_bounds(points: impl Iterator<Item = [f64; 3]>) -> ([f64; 3], [f64; 3]) {
            let mut min = [f64::INFINITY; 3];
            let mut max = [f64::NEG_INFINITY; 3];
            for p in points {
                for axis in 0..3 {
                    min[axis] = min[axis].min(p[axis]);
                    max[axis] = max[axis].max(p[axis]);
                }
            }
            (min, max)
        }
        let (brep_min, brep_max) = vertex_bounds(reimported_brep.vertices.iter().map(|v| [v.point.x, v.point.y, v.point.z]));
        for axis in 0..3 {
            assert!(brep_max[axis] > brep_min[axis], "reimported brep must carry real spatial extent on axis {axis}, got min {:?} max {:?}", brep_min, brep_max);
        }

        let mesh_snapshot = semio_mesh_snapshot_from_solids(&mut kernel, std::slice::from_ref(&solid), 0.1).expect("tessellate the same solid the reimported brep describes into a real semio/mesh snapshot");
        let mesh_positions: Vec<[f64; 3]> = mesh_snapshot.meshes.iter().flat_map(|m| m.primitives.iter()).flat_map(|p| p.positions.iter()).map(|p| [p.x, p.y, p.z]).collect();
        assert!(!mesh_positions.is_empty(), "expected real tessellated mesh positions, not an empty semio/mesh snapshot");
        let (mesh_min, mesh_max) = vertex_bounds(mesh_positions.iter().copied());
        for axis in 0..3 {
            assert!((mesh_min[axis] - brep_min[axis]).abs() < 1e-6, "semio/mesh vs reimported semio/brep bounding-box MIN mismatch on axis {axis}: mesh {} vs brep {}", mesh_min[axis], brep_min[axis]);
            assert!((mesh_max[axis] - brep_max[axis]).abs() < 1e-6, "semio/mesh vs reimported semio/brep bounding-box MAX mismatch on axis {axis}: mesh {} vs brep {}", mesh_max[axis], brep_max[axis]);
        }

        let gltf = SemioMeshToGltf::serialize(&mesh_snapshot).expect("real semio/mesh -> gltf codec must succeed on a real tessellated box");
        assert_eq!(gltf.document.meshes.len(), 1, "expected exactly one gltf mesh for one solid");
        assert_eq!(gltf.buffers.len(), 1, "expected one packed geometry buffer");
        let position_accessor = gltf.document.accessors.first().expect("POSITION accessor must exist");
        assert_eq!(position_accessor.count, mesh_positions.len(), "gltf POSITION accessor count must match the semio/mesh vertex count");
        let buffer_view = &gltf.document.buffer_views[position_accessor.buffer_view.expect("POSITION accessor must reference a bufferView")];
        let raw = &gltf.buffers[0][buffer_view.byte_offset..buffer_view.byte_offset + buffer_view.byte_length];
        let decoded_positions: Vec<[f64; 3]> = raw
            .chunks_exact(12)
            .map(|triple| {
                [
                    f32::from_le_bytes(triple[0..4].try_into().unwrap()) as f64,
                    f32::from_le_bytes(triple[4..8].try_into().unwrap()) as f64,
                    f32::from_le_bytes(triple[8..12].try_into().unwrap()) as f64,
                ]
            })
            .collect();
        assert_eq!(decoded_positions.len(), mesh_positions.len(), "decoded gltf buffer must carry exactly the semio/mesh vertex count");
        let (gltf_min, gltf_max) = vertex_bounds(decoded_positions.into_iter());
        for axis in 0..3 {
            assert!((gltf_min[axis] - brep_min[axis]).abs() < 1e-4, "final .gltf bytes vs reimported semio/brep bounding-box MIN mismatch on axis {axis}: gltf {} vs brep {}", gltf_min[axis], brep_min[axis]);
            assert!((gltf_max[axis] - brep_max[axis]).abs() < 1e-4, "final .gltf bytes vs reimported semio/brep bounding-box MAX mismatch on axis {axis}: gltf {} vs brep {}", gltf_max[axis], brep_max[axis]);
        }
    }

    #[test]
    fn semio_brep_snapshot_from_step_text_carries_real_topology() {
        let mut kernel = Brep::new();
        let solid = block_on(kernel.box_prim(1.0, 1.0, 1.0)).expect("box");
        let step_text = block_on(kernel.export_step(std::slice::from_ref(&solid))).expect("kernel step export");
        let brep = semio_brep_snapshot_from_step_text(&step_text).expect("semio/brep from step");
        assert!(!brep.solids.is_empty(), "expected at least one real BrepSolid");
        assert!(!brep.faces.is_empty(), "expected real BrepFaces, not an empty shell");
        assert!(!brep.vertices.is_empty(), "expected real BrepVertexes");
        let round_tripped = step_text_from_semio_brep_snapshot(&brep).expect("semio/brep to step");
        assert!(round_tripped.starts_with("ISO-10303-21;"));
    }

    #[test]
    fn repair_step_trailing_comma_before_close_paren_is_quote_aware() {
        assert_eq!(repair_step_trailing_comma_before_close_paren("(#1,)"), "(#1)");
        assert_eq!(repair_step_trailing_comma_before_close_paren("(#1, #2,)"), "(#1)");
        assert_eq!(repair_step_trailing_comma_before_close_paren("()"), "()");
        assert_eq!(repair_step_trailing_comma_before_close_paren("('weird,)name', #1)"), "('weird,)name', #1)");
    }
    //#endregion 🔖️SemioBrepBridge
}
//#endregion 🧪️Tests
