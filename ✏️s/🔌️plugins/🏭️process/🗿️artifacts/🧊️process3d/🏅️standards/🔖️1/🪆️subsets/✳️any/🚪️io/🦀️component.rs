//! 🚪️ IO s.process3d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from `crate::apps::process3d::register`), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl", "stdio.txt"] }
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::process3d::Process3dSnapshot;
    use crate::artifacts::process3d::standards::v1::subsets::any::schema::Process3dAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

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
use base64::Engine as _;
use crate::artifacts::process3d::{Pose, Process3dSnapshot, SolidSpec, Stock};
use semio_framework_3d::brep::kernel::{ObjSolidExporter, ObjSolidImporter, SolidExporter, SolidImporter, StepSolidExporter, StepSolidImporter, StlSolidExporter, StlSolidImporter};
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
pub fn export_process3d_model(fixture: &Process3dSnapshot, format: &str) -> Option<Process3dModelExport> {
    if format == "glb" {
        let mesh = crate::artifacts::process3d::schema::inferences::processed_mesh(fixture)?;
        let bytes = semio_framework_plugin::GlbExporter.export(&mesh).ok()?;
        return Some(Process3dModelExport {
            filename: "process3d.glb".into(),
            data: Value::String(base64::engine::general_purpose::STANDARD.encode(bytes)),
            mime_type: "model/gltf-binary".into(),
            encoding: Some("base64".into()),
        });
    }
    let exporter: Box<dyn SolidExporter> = match format {
        "obj" => Box::new(ObjSolidExporter),
        "stl" => Box::new(StlSolidExporter),
        _ => Box::new(StepSolidExporter),
    };
    let mut session = crate::artifacts::process3d::schema::inferences::ProcessKernelReplay::new();
    let handle = crate::artifacts::process3d::schema::inferences::replay_process(&mut session, fixture)?;
    let bytes = exporter.export(&*session.kernel().lock().ok()?, &[handle], PROCESS3D_TESSELLATION_TOLERANCE).ok()?;
    let format_kind = exporter.format_kind();
    let descriptor = semio_framework::format_descriptor(format_kind);
    let binary = descriptor.as_ref().map(|d| d.is_binary).unwrap_or(true);
    let mime_type = descriptor.map(|d| d.mime).unwrap_or_else(|| "application/octet-stream".to_string());
    let data = if binary { Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes)) } else { Value::String(String::from_utf8(bytes).ok()?) };
    Some(Process3dModelExport { filename: format!("process3d.{}", format_kind), data, mime_type, encoding: if binary { Some("base64".into()) } else { None } })
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
/// through the `SolidImporter` trait objects and land as `SolidSpec::ImportedSolid` (real B-Rep,
/// reusable as a Cut/Drill/Attach operand); GLB is decoded once (via the mesh tessellation bridge,
/// `GlbImporter`) purely to validate it, then kept as `SolidSpec::ImportedMesh` referencing the
/// original data url directly — it carries no exact B-Rep, so it is never re-imported into the kernel.
pub fn import_process3d_model(name: &str, data_url: &str) -> Option<Process3dSnapshot> {
    let bytes = process3d_bytes_from_data_url(data_url)?;
    let mut fixture = Process3dSnapshot::default();
    if name.ends_with(".glb") {
        semio_framework_plugin::GlbImporter.import(&bytes).ok()?;
        fixture.stock = Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: data_url.into() }, pose: Pose::default() };
        return Some(fixture);
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
    let handle = importer.import(&mut *session.kernel().lock().ok()?, &bytes, PROCESS3D_TESSELLATION_TOLERANCE).ok()?.into_iter().next()?;
    fixture.stock = Stock { id: "stock".into(), label: label.into(), solid: SolidSpec::ImportedSolid { solid_handle: handle.0 }, pose: Pose::default() };
    Some(fixture)
}
//#endregion 🔖️MediaImportExport
