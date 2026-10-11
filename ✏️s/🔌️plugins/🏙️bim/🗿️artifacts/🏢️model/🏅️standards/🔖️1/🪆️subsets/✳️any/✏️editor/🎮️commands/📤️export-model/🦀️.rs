//! 📤️ `exportModel`: writes the whole model as an IFC 2x3 or IFC4 STEP file, a binary glTF scene, an SVG sheet of its views or a CSV report and hands the shell one download. The derived values the file
//! shows come from the instance's inference session, so the file shows exactly what the windows show. The expensive half is a job: [`ExportJob`] brings the session up to the document in
//! bounded steps (a progress stage per step; when the user cancels, the nodes finished so far stay cached) and then writes the file, an IFC file one family per step ([`ifc::STAGES`]) so it reports progress and stops between two families; [`ModelExportWork`] is its retained-command shell, which
//! honours `StepContext::is_cancelled`. The dispatch path [`handle`] answers the same file in one go for callers that do not run jobs.
//!
//! @see ../../../🚪️io/📤️export — the dialect writers this command speaks.

use crate::editor::bim::kit::fault;
use crate::editor::bim::{BimCommand, BimDispatchCtx, BimModelApp};
use crate::standards::v1::subsets::any::io::export::ifc::{Schema, StagedExport};
use crate::standards::v1::subsets::any::io::export::{csv, gltf, ifc, svg};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
use crate::{ModelInference, ModelMutation, ModelSnapshot};
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

fn document_output(format: &str, stem: &str, document: semio_s_artifact_stdio_ifc::part21::Part21Document) -> Result<Output, Fault> {
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
    instance: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    analysis: Analysis,
    writer: Option<StagedExport>,
}

impl ExportJob {
    /// 🏗️ A job writing `format` of the document that instance `instance` shows.
    pub fn new(format: &str, instance: crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>) -> Self {
        Self::with_steps(format, instance, NODES_PER_STEP)
    }

    /// 🏗️ A job that finishes at most `nodes` graph nodes per step (at least one).
    pub fn with_steps(format: &str, instance: crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>, nodes: usize) -> Self {
        Self { format: format.to_string(), instance: instance.cloned(), analysis: Analysis::new(instance, nodes), writer: None }
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
    pub fn advance(&mut self, snapshot: &ModelSnapshot) -> Result<Advance, Fault> {
        if !FORMATS.contains(&self.format.as_str()) {
            return Err(refusal(&self.format));
        }
        let ended = |error: protocol::InferenceError| fault("bim.export.inference", format!("the model could not be analysed: {error}"));
        if !self.analysis.is_done() {
            let progress = self.analysis.advance(snapshot).map_err(ended)?;
            return Ok(Advance::Progress(if progress.done { "bim-export-encode" } else { "bim-export-infer" }));
        }
        let Some(schema) = schema_of(&self.format) else {
            return inference::try_with_inference(self.instance.as_ref(), snapshot, |inferred| encode(&self.format, snapshot, inferred)).map_err(ended)?.map(Advance::Done);
        };
        let writer = self.writer.get_or_insert_with(|| StagedExport::new(schema, snapshot));
        match inference::try_with_inference(self.instance.as_ref(), snapshot, |inferred| writer.step(snapshot, inferred)).map_err(ended)? {
            Some((document, _)) => document_output(&self.format, &stem(&snapshot.project.name), document).map(Advance::Done),
            None => Ok(Advance::Progress("bim-export-encode")),
        }
    }
}
//#endregion 🔖️Job

//#region 🔖️Work
/// 🧵️ The retained-command shell of an [`ExportJob`]: one step per call, the framework yields between steps and tells the work when the user cancelled.
pub struct ModelExportWork {
    tool_id: &'static str,
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    job: Option<ExportJob>,
}

impl ModelExportWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, job: None }
    }
}

fn preview(stage: &str) -> &'static [u8] {
    if stage == "bim-export-encode" { ENCODE_PREVIEW } else { INFER_PREVIEW }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for ModelExportWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.job.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, Fault> {
        let BimCommand::ExportModel(payload) = input.command else { return Err(Fault::from("bim-export-work-mismatch")) };
        let job = self.job.get_or_insert_with(|| ExportJob::new(&payload.format, Some(&self.owner)));
        if cx.is_cancelled() {
            job.cancel(input.snapshot);
            return Err(fault("bim.export.cancelled", "the export was cancelled"));
        }
        cx.consume_fuel(1);
        Ok(match job.advance(input.snapshot)? {
            Advance::Progress(stage) => {
                cx.set_stage(stage);
                ArtifactCommandWorkStep::Progress { stage, preview: preview(stage) }
            }
            Advance::Done(output) => ArtifactCommandWorkStep::Complete(download(output)),
        })
    }
}
//#endregion 🔖️Work

/// 📤️ The synchronous path: the file of the instance's inference session as one download.
pub fn handle(payload: &ExportModel, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let instance = ctx.gestures.as_ref();
    let mut job = ExportJob::new(&payload.format, instance);
    loop {
        if let Advance::Done(output) = job.advance(doc.snapshot)? {
            return Ok(download(output));
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
