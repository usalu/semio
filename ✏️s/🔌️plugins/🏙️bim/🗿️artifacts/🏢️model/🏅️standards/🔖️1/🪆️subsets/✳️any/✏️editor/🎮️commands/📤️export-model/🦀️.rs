//! 📤️ `exportModel`: writes the whole model as an IFC 2x3 or IFC4 STEP file, a binary glTF scene, an SVG sheet of its views or a CSV report and hands the shell one download. The derived values the file
//! shows come from the instance's inference session, so the file shows exactly what the windows show. The expensive half is a job: [`ExportJob`] brings the session up to the document in
//! bounded steps (a progress stage per step; when the user cancels, the nodes finished so far stay cached) and then writes the file, an IFC file one family per step ([`ifc::STAGES`]) so it reports progress and stops between two families; [`ModelExportWork`] is its retained-command shell, which
//! honours `StepContext::is_cancelled`. The dispatch path [`handle`] answers the same file in one go for callers that do not run jobs.
//!
//! @see ../../../🚪️io/📤️export — the dialect writers this command speaks.

use crate::editor::bim::kit::fault;
use crate::editor::bim::dispatch_context::BimDispatchCtx;
use crate::editor::bim::{BimCommand, BimModelApp};
use crate::standards::v1::subsets::any::io::export::ifc::{Schema, StagedExport};
use crate::standards::v1::subsets::any::io::export::{csv, gltf, ifc, svg};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
use crate::standards::v1::subsets::any::schema::{inferences::ModelInference, mutations::ModelMutation, snapshot::ModelSnapshot};
use semio_framework_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🧾️ The formats of an export, in the order the window chrome lists them.
pub const FORMATS: &[&str] = &["ifc2x3", "ifc4", "glb", "svg", "csv"];
/// ⏩️ Graph nodes one job step may finish.
pub const NODES_PER_STEP: usize = 256;
/// 🗣️ The progress text of the inference stage, English and German.
pub const INFER_PREVIEW: &[u8] = br#"{"en":"Analysing the model","de":"Modell wird analysiert"}"#;
/// 🗣️ The progress text of the writing stage, English and German.
pub const ENCODE_PREVIEW: &[u8] = br#"{"en":"Writing the file","de":"Datei wird geschrieben"}"#;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-model")]
pub struct ExportModel {
    /// The file format: `ifc2x3`, `ifc4`, `glb`, `svg` or `csv`.
    pub format: String,
    /// The state a toggle that triggers the export reports; the export ignores it.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
}

/// 📄️ One written file, ready for the shell.
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    pub filename: String,
    pub mime_type: &'static str,
    pub data: String,
    pub encoding: Option<&'static str>,
}

/// 🏷️ The stem of the file name: the project name with every run of characters other than letters and digits turned into one dash.
pub fn stem(name: &str) -> String {
    let mut stem = String::new();
    for ch in name.chars() {
        if ch.is_alphanumeric() {
            stem.push(ch);
        } else if !stem.ends_with('-') && !stem.is_empty() {
            stem.push('-');
        }
    }
    let stem = stem.trim_end_matches('-');
    if stem.is_empty() { "model".to_string() } else { stem.to_string() }
}

fn refusal(format: &str) -> Fault {
    fault("bim.export.format-unknown", format!("'{format}' is not an export format of the model (one of {})", FORMATS.join(", ")))
}

fn failed(format: &str, message: impl std::fmt::Display) -> Fault {
    fault("bim.export.failed", format!("the {format} file could not be written: {message}"))
}

fn binary(bytes: &[u8]) -> (String, Option<&'static str>) {
    match std::str::from_utf8(bytes) {
        Ok(text) => (text.to_string(), None),
        Err(_) => (semio_framework_io_base64::base64_standard_encode(bytes), Some(MEDIA_EXPORT_BASE64_ENCODING)),
    }
}

/// ✍️ The file of `format` for `model` from its inference.
pub fn encode(format: &str, model: &ModelSnapshot, inferred: &ModelInference) -> Result<Output, Fault> {
    let stem = stem(&model.project.name);
    let (extension, mime_type, data, encoding) = match format {
        "ifc2x3" | "ifc4" => {
            let (document, _) = ifc::inferred_to_part21(schema_of(format).unwrap_or(Schema::Ifc2x3), model, inferred);
            return document_output(format, &stem, document);
        }
        "glb" => {
            let (scene, _) = gltf::scene::build(model, inferred);
            let bytes = scene.to_glb().map_err(|error| failed(format, error))?;
            ("glb", "model/gltf-binary", semio_framework_io_base64::base64_standard_encode(&bytes), Some(MEDIA_EXPORT_BASE64_ENCODING))
        }
        "svg" => ("svg", "image/svg+xml", svg::views_to_svg(model, &inferred.view_linework).map_err(|error| failed(format, error))?, None),
        "csv" => ("csv", "text/csv", csv::report_csv(model, inferred), None),
        other => return Err(refusal(other)),
    };
    Ok(Output { filename: format!("{stem}.{extension}"), mime_type, data, encoding })
}

/// 🔖️ The IFC schema a format writes, `None` for the other formats.
pub fn schema_of(format: &str) -> Option<Schema> {
    match format {
        "ifc2x3" => Some(Schema::Ifc2x3),
        "ifc4" => Some(Schema::Ifc4),
        _ => None,
    }
}

fn document_output(format: &str, stem: &str, document: semio_s_artifact_stdio_contract::part21::Part21Document) -> Result<Output, Fault> {
    let bytes = ifc::encode(schema_of(format).unwrap_or(Schema::Ifc2x3), document).map_err(|error| failed(format, error))?;
    let (data, encoding) = binary(&bytes);
    Ok(Output { filename: format!("{stem}.ifc"), mime_type: "application/x-step", data, encoding })
}

fn download(output: Output) -> Emit<ModelMutation, NoConfigMutation> {
    Emit::effect(Effect::DownloadMediaExport { filename: output.filename, mime_type: output.mime_type.into(), data: output.data, encoding: output.encoding.map(str::to_string) })
}

//#region 🔖️Job
/// ⏳️ What one step of a job did.
#[derive(Debug, PartialEq)]
pub enum Advance {
    Progress(&'static str),
    Done(Output),
}

/// 🧵️ The export of the model as a stepped, cancellable job: analyse, then write. Cancelling it keeps every node the session finished.
pub struct ExportJob {
    format: String,
    analysis: Analysis,
    writer: Option<StagedExport>,
}

impl ExportJob {
    /// 🏗️ A job writing `format` of the document that instance `instance` shows.
    pub fn new(format:String, instance: crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>) -> Self {
        Self { format,analysis:Analysis::new(instance),writer:None }
    }

    /// 📈️ How far the job is: the analysis is the first half, the families of an IFC file the second half (one once the file is written).
    pub fn fraction(&self) -> f32 {
        match (&self.writer, schema_of(&self.format)) {
            (Some(writer), _) => 0.5 + 0.5 * writer.fraction(),
            (None, Some(_)) => 0.5 * self.analysis.fraction(),
            (None, None) => self.analysis.fraction(),
        }
    }

    /// 🏷️ The family the next step writes, `None` outside the writing of an IFC file.
    pub fn writing(&self) -> Option<&'static str> {
        self.writer.as_ref().and_then(StagedExport::stage)
    }

    /// 🛑️ Cancels the job: the analysis is closed as cancelled, so the session keeps the nodes it finished and the next read computes only the rest.
    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
        self.writer = None;
        self.analysis.cancel(snapshot);
    }

    /// ⏩️ Does one bounded step against `snapshot`.
    pub fn advance(&mut self,snapshot:&ModelSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<Advance,protocol::InferenceWorkFailure>{
        if !self.analysis.is_done(){let progress=self.analysis.advance(snapshot,cx)?;return Ok(Advance::Progress(if progress.done{"bim-export-encode"}else{"bim-export-infer"}))}
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original model export writer and output have no funded producer").into())
    }

}
//#endregion 🔖️Job

//#region 🔖️Work
/// 🧵️ The retained-command shell of an [`ExportJob`]: one step per call, the framework yields between steps and tells the work when the user cancelled.
pub struct ModelExportWork {
    tool_id: &'static str,
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    job: Option<ExportJob>,
    failure:Option<protocol::InferenceWorkFailure>,
    original:Option<ExportModel>,
    read:Option<store::os_store::snapshot_read::SnapshotRead<ModelSnapshot>>,
}

impl ModelExportWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, job: None,failure:None,original:None,read:None }
    }
}

fn preview(stage: &str) -> &'static [u8] {
    if stage == "bim-export-encode" { ENCODE_PREVIEW } else { INFER_PREVIEW }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for ModelExportWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn receive_original_command(&mut self,command:&mut BimCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.original.is_some(){return Ok(true)}let BimCommand::ExportModel(payload)=command else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original model export command changed").into())};let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original=Some(std::mem::replace(payload,ExportModel{format:String::new(),pressed:None}));Ok(true)
    }

    fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.read.is_none()&&self.job.is_none()&&self.failure.is_none()}

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.job.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, semio_framework_job::OriginalTurnFailure> {
        if let Some(original)=protocol::admit_inference_failure(&mut self.failure,cx)?{return Err(original)}
        if self.failure.is_some(){return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}
        let BimCommand::ExportModel(_payload)=input.command else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original BIM model export command changed").into())};
        if self.read.is_none(){self.read=input.snapshot_read.take_current(input.operation.generation,input.operation.canonical_base_revision,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}
        if self.job.is_none(){let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}let original=self.original.take().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original model export fields were not received"))?;self.job=Some(ExportJob::new(original.format,Some(&self.owner)));cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}
        let snapshot=self.read.as_ref().unwrap().get();let job=self.job.as_mut().unwrap();if cx.is_cancelled(){job.cancel(snapshot)}
        match job.advance(snapshot,cx){Ok(Advance::Progress(stage))=>{cx.set_stage(stage);Ok(ArtifactCommandWorkStep::Progress{stage,preview:preview(stage)})},Ok(Advance::Done(output))=>Ok(ArtifactCommandWorkStep::Complete(download(output))),Err(original)=>{self.failure=Some(original);Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}}

    }
}
//#endregion 🔖️Work

/// 📤️ The synchronous path: the file of the instance's inference session as one download.
pub fn handle(payload: &ExportModel, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let ended=|error:protocol::InferenceError|fault("bim.export.inference",format!("the model could not be analysed: {error}"));
    inference::try_with_inference(ctx.gestures.as_ref(),doc.snapshot,|inferred|encode(&payload.format,doc.snapshot,inferred)).map_err(ended)?.map(download)

}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
