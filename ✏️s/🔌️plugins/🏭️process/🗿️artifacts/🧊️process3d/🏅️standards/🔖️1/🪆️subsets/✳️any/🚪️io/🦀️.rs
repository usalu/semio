//! 🚪️ IO s.process3d (1/✳️any) — the artifact declaration owns this composer table.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

//#region 🧩️Modules
#[path = "."]
pub mod import {
    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod txt {
                #[path = "."]
                pub mod v_utf_8 {
                    #[path = "."]
                    pub mod any {
                        #[path = "📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "."]
                    pub mod any {
                        #[path = "📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod export {
    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod txt {
                #[path = "."]
                pub mod v_utf_8 {
                    #[path = "."]
                    pub mod any {
                        #[path = "📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "."]
                    pub mod any {
                        #[path = "📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_stdio_step::geometry::StepMetresSolidExporter;
use semio_s_artifact_stdio_step::geometry::StepSolidImporter;
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::Process3dAnalyzer;
    use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
    use {semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactComposition,semio_plugin::io::ComposeError,semio_plugin::io::ComposeSource,semio_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.process.process3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Process3dComposerComposition;

    impl ArtifactComposition for Process3dComposerComposition {
        type Snapshot = Process3dSnapshot;
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
                    let analysis = Process3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
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
use semio_s_artifact_process_process3d_model::{snapshot::Pose, snapshot::Process3dSnapshot, snapshot::ProcessWorkingScene, snapshot::Stock, snapshot::WorkingSolid};
use semio_framework::DslValue;
use semio_framework::mesh_io::{MeshExporter, MeshImporter};
use semio_framework_3d_brep_engine::{Brep, BrepError, GeometryHandle, GlbSolidImporter, ObjSolidExporter, ObjSolidImporter, SolidExporter, SolidImporter, StlSolidExporter, StlSolidImporter};

/// 📤️ A pending native-geometry export ready to become a `Effect::DownloadMediaExport`.
pub struct Process3dModelExport {
    pub filename: String,
    pub data: DslValue,
    pub mime_type: String,
    pub encoding: Option<String>,
}

/// 🕳️ Tessellation tolerance for STEP/OBJ/STL import — mirrors the inference family's private
/// kernel-replay constant of the same value (`schema::inferences::PROCESS3D_TESSELLATION_TOLERANCE`).
const PROCESS3D_TESSELLATION_TOLERANCE: f64 = 0.05;

//#region 🔖️SolidCodecs
/// 🗃️ The closed set of `SolidExporter` implementors `export_process3d_model` ever selects among.
/// `SolidExporter` is declared in `🗄️stdio`, outside this packet's path scope, so `#[dyn_enum]` cannot
/// annotate its declaration (R11: "cannot annotate a trait you do not own") — hand-written here in the
/// same shape `dyn_enum_close!` would generate (O1: no `Box<dyn SolidExporter>`).
enum ProcessSolidExporter {
    Obj(ObjSolidExporter),
    Step(StepMetresSolidExporter),
    Stl(StlSolidExporter),
}

impl SolidExporter for ProcessSolidExporter {
    async fn format_kind(&self) -> &'static str {
        match self {
            Self::Obj(exporter) => exporter.format_kind().await,
            Self::Step(exporter) => exporter.format_kind().await,
            Self::Stl(exporter) => exporter.format_kind().await,
        }
    }

    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        match self {
            Self::Obj(exporter) => exporter.export(kernel, shapes, deflection).await,
            Self::Step(exporter) => exporter.export(kernel, shapes, deflection).await,
            Self::Stl(exporter) => exporter.export(kernel, shapes, deflection).await,
        }
    }
}

/// 🗃️ Mirror of `ProcessSolidExporter` for the import direction — same rationale (`SolidImporter` is
/// likewise a foreign, un-owned trait).
enum ProcessSolidImporter {
    Step(StepSolidImporter),
    Obj(ObjSolidImporter),
    Stl(StlSolidImporter),
    Glb(GlbSolidImporter),
}

impl SolidImporter for ProcessSolidImporter {
    async fn format_kind(&self) -> &'static str {
        match self {
            Self::Step(importer) => importer.format_kind().await,
            Self::Obj(importer) => importer.format_kind().await,
            Self::Stl(importer) => importer.format_kind().await,
            Self::Glb(importer) => importer.format_kind().await,
        }
    }

    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        match self {
            Self::Step(importer) => importer.import(kernel, bytes, tolerance).await,
            Self::Obj(importer) => importer.import(kernel, bytes, tolerance).await,
            Self::Stl(importer) => importer.import(kernel, bytes, tolerance).await,
            Self::Glb(importer) => importer.import(kernel, bytes, tolerance).await,
        }
    }
}
//#endregion 🔖️SolidCodecs

/// 🗂️ The format row of one process export, read from the stdio codec crate that owns the format — never from the
/// process-global format catalog, which only holds rows the plugins of the SAME guest registered (this guest registers
/// none, so every export used to be refused as an unknown format kind).
fn process_export_format(format_kind: &str) -> Result<semio_plugin::io::FormatDescriptor, String> {
    for rows in [semio_s_artifact_stdio_step::formats(), semio_s_artifact_stdio_obj::formats(), semio_s_artifact_stdio_stl::formats(), semio_s_artifact_stdio_gltf::formats()] {
        let rows = rows.map_err(|error| format!("process export format rows: {error:?}"))?;
        if let Some(row) = rows.into_iter().find(|row| row.kind_id == format_kind || row.short_id == format_kind || row.aliases.iter().any(|alias| alias == format_kind)) {
            return Ok(row);
        }
    }
    Err(format!("unknown process export format kind `{format_kind}`"))
}

/// 📤️ Encodes the replayed stock through `format`'s codec. STEP/OBJ/STL go through the
/// `ProcessSolidExporter`-dispatched `SolidExporter` codecs (real B-Rep, exact where the format
/// allows it); GLB goes through
/// the evaluated mesh and `GlbExporter`. Imported solids replay from the same persisted exact
/// B-Rep used by the preview and the composed stock child.

pub fn export_process3d_model(scene: &ProcessWorkingScene, resolved_up_to: Option<usize>, format: &str) -> Result<Option<Process3dModelExport>, String> {
    if format == "glb" {
        let Some(mesh) = semio_s_artifact_process_process3d_inferences::processed_mesh(scene, resolved_up_to).map_err(|error| error.to_string())? else {
            return Ok(None);
        };
        let bytes = semio_framework::mesh_io::GlbExporter.export(&mesh)?;
        let descriptor = process_export_format("glb")?;
        let extension = descriptor.extensions.first().ok_or_else(|| "process export format kind `glb` has no extension claim".to_string())?;
        let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| "process export format kind `glb` has no MIME claim".to_string())?;
        return Ok(Some(Process3dModelExport { filename: format!("process3d{extension}"), data: DslValue::String(base64_codec::base64_standard_encode(bytes)), mime_type, encoding: descriptor.is_binary.then(|| "base64".into()) }));
    }
    let exporter: ProcessSolidExporter = match format {
        "obj" => ProcessSolidExporter::Obj(ObjSolidExporter),
        "stl" => ProcessSolidExporter::Stl(StlSolidExporter),
        _ => ProcessSolidExporter::Step(StepMetresSolidExporter),
    };
    let mut session = semio_s_artifact_process_process3d_inferences::ProcessKernelReplay::new();
    let Some(handle) = semio_s_artifact_process_process3d_inferences::replay_process(&mut session, scene, resolved_up_to).map_err(|error| error.to_string())? else {
        return Ok(None);
    };
    let bytes = ::semio_framework_async::poll::resolve_ready(exporter.export(session.kernel(), &[handle], PROCESS3D_TESSELLATION_TOLERANCE)).map_err(|error| error.to_string())?;
    let format_kind = ::semio_framework_async::poll::resolve_ready(exporter.format_kind());
    let descriptor = process_export_format(format_kind)?;
    let extension = descriptor.extensions.first().ok_or_else(|| format!("process export format kind `{format_kind}` has no extension claim"))?;
    let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| format!("process export format kind `{format_kind}` has no MIME claim"))?;
    let data = if descriptor.is_binary { DslValue::String(base64_codec::base64_standard_encode(&bytes)) } else { DslValue::String(String::from_utf8(bytes).map_err(|error| error.to_string())?) };
    Ok(Some(Process3dModelExport { filename: format!("process3d{extension}"), data, mime_type, encoding: descriptor.is_binary.then(|| "base64".into()) }))
}

pub use semio_s_artifact_process_process3d_model::snapshot::{PROCESS3D_MODEL_DATA_URL_BYTES, PROCESS3D_MODEL_FILE_BYTES};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Process3dImportError {
    InputTooLarge,
    InvalidData(String),
    UnsupportedFormat,
    UnsupportedStepUnits,
    MissingSolid,
    MultipleSolids,
    SolidTooLarge,
}

impl std::fmt::Display for Process3dImportError {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputTooLarge => write!(output, "Select a model file up to {PROCESS3D_MODEL_FILE_BYTES} bytes."),
            Self::InvalidData(cause) => write!(output, "The model file could not be decoded: {cause}"),
            Self::UnsupportedFormat => output.write_str("Select a STEP, OBJ, STL, or GLB model."),
            Self::UnsupportedStepUnits => output.write_str("The STEP model must declare one supported metric length unit."),
            Self::MissingSolid => output.write_str("The model does not contain a closed solid."),
            Self::MultipleSolids => output.write_str("Select a model file with one closed solid for the workpiece."),
            Self::SolidTooLarge => output.write_str("The exact imported geometry exceeds the Process3d document's solid budget."),
        }
    }
}

/// 📥️ Imports exact solid geometry or keeps an open GLB as display-only stock.
pub fn import_process3d_model(name: &str, data_url: &str) -> Result<Process3dSnapshot, Process3dImportError> {
    use semio_s_artifact_process_process3d_model::snapshot::{capture_imported_brep, decode_process3d_model_data_url};

    if data_url.len() > PROCESS3D_MODEL_DATA_URL_BYTES {
        return Err(Process3dImportError::InputTooLarge);
    }
    let bytes = decode_process3d_model_data_url(data_url).ok_or_else(|| Process3dImportError::InvalidData("the data URL carries no base64 payload".into()))?;
    if bytes.len() > PROCESS3D_MODEL_FILE_BYTES as usize {
        return Err(Process3dImportError::InputTooLarge);
    }
    let extension = name.rsplit_once('.').map(|(_, extension)| extension.to_ascii_lowercase());
    let (importer, label) = match extension.as_deref() {
        Some("stp" | "step") => (ProcessSolidImporter::Step(StepSolidImporter), "Imported STEP"),
        Some("obj") => (ProcessSolidImporter::Obj(ObjSolidImporter), "Imported OBJ"),
        Some("stl") => (ProcessSolidImporter::Stl(StlSolidImporter), "Imported STL"),
        Some("glb") => (ProcessSolidImporter::Glb(GlbSolidImporter), "Imported GLB"),
        _ => return Err(Process3dImportError::UnsupportedFormat),
    };

    let scale = if matches!(&importer, ProcessSolidImporter::Step(_)) {
        let source = std::str::from_utf8(&bytes).map_err(|error| Process3dImportError::InvalidData(error.to_string()))?;
        semio_s_artifact_stdio_step::step_length_in_metres(source).ok().flatten().ok_or(Process3dImportError::UnsupportedStepUnits)?
    } else {
        1.0
    };
    let mut kernel = Brep::new();
    let handles = match ::semio_framework_async::poll::resolve_ready(importer.import(&mut kernel, &bytes, PROCESS3D_TESSELLATION_TOLERANCE)) {
        Ok(handles) => handles,
        Err(_) if matches!(importer, ProcessSolidImporter::Glb(_)) => Vec::new(),
        Err(error) => return Err(Process3dImportError::InvalidData(error.to_string())),
    };
    let handle = {
        let mut solids = handles.into_iter().filter(|handle| kernel.geometry_solid(handle).is_ok());
        let first = solids.next();
        if solids.next().is_some() {
            return Err(Process3dImportError::MultipleSolids);
        }
        first
    };
    let solid = if let Some(handle) = handle {
        let handle = if scale == 1.0 { handle } else { kernel.scale_sync(&handle, scale, [0.0; 3]).map_err(|error| Process3dImportError::InvalidData(error.to_string()))? };
        let shape = kernel.export_shape(&handle).map_err(|error| Process3dImportError::InvalidData(error.to_string()))?;
        capture_imported_brep(&shape).ok_or(Process3dImportError::SolidTooLarge)?
    } else if matches!(importer, ProcessSolidImporter::Glb(_)) {
        semio_framework::mesh_io::GlbImporter.import(&bytes).map_err(Process3dImportError::InvalidData)?;
        WorkingSolid::ImportedMesh { mesh_url: data_url.into() }
    } else {
        return Err(Process3dImportError::MissingSolid);
    };
    let stock = Stock { id: "stock".into(), label: label.into(), solid, pose: Pose::default() };
    Ok(semio_s_artifact_process_process3d_model::snapshot::process_working_scene_to_snapshot(&ProcessWorkingScene { stock, steps: Vec::new() }, Default::default()))
}

//#endregion 🔖️MediaImportExport

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::Process3dBuilder as Process3dAnyBuilder;
    use crate::Process3dComposer as Process3dAnyComposer;
    use {semio_plugin::composer_entry_of,semio_plugin::ArtifactBuilder,semio_plugin::io::ComposeError,semio_plugin::io::ComposedArtifact,semio_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_plugin::io::ErasedComposeSource,semio_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
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
    const PROCESS3D_DIALECT: Dialect = Dialect { artifact_kind: "s.process.process3d", standard: StandardId("1"), subset: SubsetId("*") };
    const PROCESS3D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PROCESS3D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Process3dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Process3dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Process3dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PROCESS3D_JSON_BRIDGE_DIALECT) {
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Process3dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_plugin::io::Confidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<Process3dAnyComposer>(),
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PROCESS3D_DIALECT], compose: compose_export_json },
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


pub mod derived_construction {
    use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
    use semio_s_artifact_process_process3d_mutations::Process3dMutation;
    use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
    use semio_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Process3dBuilderConstruction {
        snapshot: Process3dSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Process3dBuilderConstruction {
        type Snapshot = Process3dSnapshot;
        type Mutation = Process3dMutation;
        type Diff = Process3dDiff;
        fn empty() -> Self {
            Self { snapshot: Process3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Process3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Process3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Process3dMutation as protocol::ColdMutation<Process3dSnapshot>>::diff(&mutation, &self.snapshot);
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
    use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
    use {semio_plugin::io::Analysis,semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Process3dParts {
        pub snapshot: Option<Process3dSnapshot>,
    }

    pub struct Process3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Process3dAnalyzerAnalysis {
        type Parts = Process3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.process.process3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_plugin::io::Confidence {
            semio_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Process3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Process3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Process3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
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

semio_plugin::derive_artifact_facets!(
    pub spec Process3dBuilderFacets {
        construction: Process3dBuilderConstruction,
        analysis: Process3dAnalyzerAnalysis,
        composition: Process3dComposerComposition,
    }
    builder: Process3dBuilder,
    analyzer: Process3dAnalyzer,
    composer: Process3dComposer,
);
