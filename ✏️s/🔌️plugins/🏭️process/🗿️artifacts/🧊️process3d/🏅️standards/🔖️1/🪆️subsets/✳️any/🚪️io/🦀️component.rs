//! 🚪️ IO s.process3d (1/✳️any) — the artifact declaration owns this composer table.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl", "stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::process3d::standards::v1::subsets::any::schema::Process3dAnalyzer;
    use crate::artifacts::process3d::Process3dSnapshot;
    use semio_framework_plugin::ArtifactAnalyzer as _;
    use semio_framework_plugin::{AnalyzeSource, ArtifactBuilder, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.process3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_IFC: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_STEP: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Process3dComposerComposition;

    impl ArtifactComposition for Process3dComposerComposition {
        type Snapshot = Process3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_IFC, DEP_JSON, DEP_OBJ, DEP_PNG, DEP_STEP, DEP_STL, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = Process3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_IFC {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::ifc::v4::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STEP {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::step::v_ap214::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::process3d::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Process3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️MediaImportExport
use crate::artifacts::process3d::{Pose, Process3dSnapshot, ProcessWorkingScene, Stock, WorkingSolid};
use base64::Engine as _;
use semio_framework_plugin::{MeshExporter, MeshImporter};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{ObjSolidExporter, ObjSolidImporter, SolidExporter, SolidImporter, StepSolidExporter, StepSolidImporter, StlSolidExporter, StlSolidImporter};
use serde_json::Value;

/// 📤️ A pending native-geometry export ready to become a `HostEffect::DownloadMediaExport`.
pub struct Process3dModelExport {
    pub filename: String,
    pub data: Value,
    pub mime_type: String,
    pub encoding: Option<String>,
}

/// 🕳️ Tessellation tolerance for STEP/OBJ/STL import — mirrors the inference family's private
/// kernel-replay constant of the same value (`schema::inferences::PROCESS3D_TESSELLATION_TOLERANCE`).
const PROCESS3D_TESSELLATION_TOLERANCE: f64 = 0.05;

/// 📤️ Encodes the replayed stock through `format`'s codec. STEP/OBJ/STL go through the
/// `SolidExporter` trait objects (real B-Rep, exact where the format allows it); GLB goes through
/// the mesh tessellation bridge (`schema::inferences::processed_mesh` → `GlbExporter`), matching how
/// it is already rendered/exported elsewhere in this app.
///
/// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: takes a real, literal
/// `ProcessWorkingScene` now (never a bare `Process3dSnapshot` — its composed `stock_solid`/`steps`
/// children carry no resolvable content without a `LinkResolver` this ticket doesn't add; see
/// `ProcessWorkingScene`'s own doc comment in the artifact root file). Callers that only have a
/// persisted snapshot get the honest empty scene from `process_working_scene_from_snapshot` — this
/// export then legitimately returns `None` (no stock to replay), the same documented gap.
pub fn export_process3d_model(scene: &ProcessWorkingScene, resolved_up_to: Option<usize>, format: &str) -> Result<Option<Process3dModelExport>, String> {
    if format == "glb" {
        let Some(mesh) = crate::artifacts::process3d::schema::inferences::processed_mesh(scene, resolved_up_to) else {
            return Ok(None);
        };
        let bytes = semio_framework_plugin::GlbExporter.export(&mesh)?;
        let descriptor = semio_framework::format_descriptor("glb").map_err(|error| error.to_string())?.ok_or_else(|| "unknown process export format kind `glb`".to_string())?;
        let extension = descriptor.extensions.first().ok_or_else(|| "process export format kind `glb` has no extension claim".to_string())?;
        let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| "process export format kind `glb` has no MIME claim".to_string())?;
        return Ok(Some(Process3dModelExport { filename: format!("process3d{extension}"), data: Value::String(base64::engine::general_purpose::STANDARD.encode(bytes)), mime_type, encoding: descriptor.is_binary.then(|| "base64".into()) }));
    }
    let exporter: Box<dyn SolidExporter> = match format {
        "obj" => Box::new(ObjSolidExporter),
        "stl" => Box::new(StlSolidExporter),
        _ => Box::new(StepSolidExporter),
    };
    let mut session = crate::artifacts::process3d::schema::inferences::ProcessKernelReplay::new();
    let Some(handle) = crate::artifacts::process3d::schema::inferences::replay_process(&mut session, scene, resolved_up_to) else {
        return Ok(None);
    };
    let bytes = exporter.export(session.kernel(), &[handle], PROCESS3D_TESSELLATION_TOLERANCE).map_err(|error| error.to_string())?;
    let format_kind = exporter.format_kind();
    let descriptor = semio_framework::format_descriptor(format_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown process export format kind `{format_kind}`"))?;
    let extension = descriptor.extensions.first().ok_or_else(|| format!("process export format kind `{format_kind}` has no extension claim"))?;
    let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| format!("process export format kind `{format_kind}` has no MIME claim"))?;
    let data = if descriptor.is_binary { Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes)) } else { Value::String(String::from_utf8(bytes).map_err(|error| error.to_string())?) };
    Ok(Some(Process3dModelExport { filename: format!("process3d{extension}"), data, mime_type, encoding: descriptor.is_binary.then(|| "base64".into()) }))
}

/// 📦️ Decodes a `requestFileOpen(readAs: "dataUrl")` payload into raw bytes.
fn process3d_bytes_from_data_url(data_url: &str) -> Option<Vec<u8>> {
    if let Some((header, encoded)) = data_url.split_once(',') {
        if header.starts_with("data:") {
            return base64::engine::general_purpose::STANDARD.decode(encoded).ok();
        }
    }
    Some(data_url.as_bytes().to_vec())
}

/// 📥️ Imports a picked file into a brand-new stock-only fixture (steps cleared): STEP/OBJ/STL go
/// through the `SolidImporter` trait objects and land as `WorkingSolid::ImportedSolid` (real B-Rep,
/// reusable as a Cut/Drill/Attach operand); GLB is decoded once (via the mesh tessellation bridge,
/// `GlbImporter`) purely to validate it, then kept as `WorkingSolid::ImportedMesh` referencing the
/// original data url directly — it carries no exact B-Rep, so it is never re-imported into the
/// kernel. Real WRITE-side construction (mints real composed children from literal content via
/// `process_working_scene_to_snapshot` — the only place this migration can do so; see
/// `ProcessWorkingScene`'s own doc comment).
pub fn import_process3d_model(name: &str, data_url: &str) -> Option<Process3dSnapshot> {
    let bytes = process3d_bytes_from_data_url(data_url)?;
    if name.ends_with(".glb") {
        semio_framework_plugin::GlbImporter.import(&bytes).ok()?;
        let stock = Stock { id: "stock".into(), label: "Imported GLB".into(), solid: WorkingSolid::ImportedMesh { mesh_url: data_url.into() }, pose: Pose::default() };
        return Some(crate::artifacts::process3d::process_working_scene_to_snapshot(&ProcessWorkingScene { stock, steps: Vec::new() }, Default::default(), None));
    }
    let (importer, label): (Box<dyn SolidImporter>, &str) = if name.ends_with(".stp") || name.ends_with(".step") {
        (Box::new(StepSolidImporter), "Imported STEP")
    } else if name.ends_with(".obj") {
        (Box::new(ObjSolidImporter), "Imported OBJ")
    } else if name.ends_with(".stl") {
        (Box::new(StlSolidImporter), "Imported STL")
    } else {
        return None;
    };
    let mut session = crate::artifacts::process3d::schema::inferences::ProcessKernelReplay::new();
    let handle = importer.import(session.kernel_mut(), &bytes, PROCESS3D_TESSELLATION_TOLERANCE).ok()?.into_iter().next()?;
    let stock = Stock { id: "stock".into(), label: label.into(), solid: WorkingSolid::ImportedSolid { solid_handle: handle.0 }, pose: Pose::default() };
    Some(crate::artifacts::process3d::process_working_scene_to_snapshot(&ProcessWorkingScene { stock, steps: Vec::new() }, Default::default(), None))
}
//#endregion 🔖️MediaImportExport

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::process3d::standards::v1::subsets::any::schema::Process3dBuilder as Process3dAnyBuilder;
    use crate::artifacts::process3d::standards::v1::subsets::any::schema::Process3dComposer as Process3dAnyComposer;
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
    const PROCESS3D_DIALECT: Dialect = Dialect { artifact_kind: "s.process3d", standard: StandardId("1"), subset: SubsetId("*") };
    const PROCESS3D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::process3d::Process3dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PROCESS3D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Process3dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Process3dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Process3dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PROCESS3D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::process3d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Process3dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_IFC_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
    fn compose_export_ifc(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::ifc::v4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_IFC_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STEP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    fn compose_export_step(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::step::v_ap214::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STEP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::process3d::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<Process3dAnyComposer>(),
                    ComposerEntry { writes: EXPORT_IFC_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_ifc },
                    ComposerEntry { writes: EXPORT_STEP_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_step },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_dwg },
                    ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_stl },
                    ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_gltf },
                    ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_obj },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
