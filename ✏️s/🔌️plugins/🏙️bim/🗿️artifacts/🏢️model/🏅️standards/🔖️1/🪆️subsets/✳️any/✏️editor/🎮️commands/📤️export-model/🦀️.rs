//! 📤️ `exportModel`: writes the whole model as an IFC 2x3 STEP file, a binary glTF scene, an SVG sheet of its views or a CSV report and hands the shell one download. The derived values the file
//! shows come from the instance's inference session, so the file shows exactly what the windows show. The expensive half is a job: [`ExportJob`] brings the session up to the document in
//! bounded steps (a progress stage per step; when the user cancels, the nodes finished so far stay cached) and then writes the file; [`ModelExportWork`] is its retained-command shell, which
//! honours `StepContext::is_cancelled`. The dispatch path [`handle`] answers the same file in one go for callers that do not run jobs.
//!
//! @see ../../../🚪️io/📤️export — the dialect writers this command speaks.

use crate::editor::bim::kit::fault;
use crate::editor::bim::{BimCommand, BimDispatchCtx, BimModelApp};
use crate::standards::v1::subsets::any::io::export::{csv, gltf, ifc, svg};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry as inference;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::Analysis;
use crate::{ModelInference, ModelMutation, ModelSnapshot};
use semio_framework_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🧾️ The formats of an export, in the order the window chrome lists them.
pub const FORMATS: &[&str] = &["ifc", "glb", "svg", "csv"];
/// ⏩️ Graph nodes one job step may finish.
pub const NODES_PER_STEP: usize = 256;
/// 🗣️ The progress text of the inference stage, English and German.
pub const INFER_PREVIEW: &[u8] = br#"{"en":"Analysing the model","de":"Modell wird analysiert"}"#;
/// 🗣️ The progress text of the writing stage, English and German.
pub const ENCODE_PREVIEW: &[u8] = br#"{"en":"Writing the file","de":"Datei wird geschrieben"}"#;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-model")]
pub struct ExportModel {
    /// The file format: `ifc`, `glb`, `svg` or `csv`.
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
        "ifc" => {
            let (document, _) = ifc::inferred_to_part21(model, inferred);
            let bytes = ifc::codec::encode_document(document).map_err(|error| failed(format, error))?;
            let (data, encoding) = binary(&bytes);
            ("ifc", "application/x-step", data, encoding)
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
    instance: Option<u32>,
    analysis: Analysis,
}

impl ExportJob {
    /// 🏗️ A job writing `format` of the document that instance `instance` shows.
    pub fn new(format: &str, instance: Option<u32>) -> Self {
        Self::with_steps(format, instance, NODES_PER_STEP)
    }

    /// 🏗️ A job that finishes at most `nodes` graph nodes per step (at least one).
    pub fn with_steps(format: &str, instance: Option<u32>, nodes: usize) -> Self {
        Self { format: format.to_string(), instance, analysis: Analysis::new(instance, nodes) }
    }

    /// 📈️ How far the analysis is, as a fraction of its plan (one once it is over).
    pub fn fraction(&self) -> f32 {
        self.analysis.fraction()
    }

    /// 🛑️ Cancels the job: the analysis is closed as cancelled, so the session keeps the nodes it finished and the next read computes only the rest.
    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
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
        inference::try_with_inference(self.instance, snapshot, |inferred| encode(&self.format, snapshot, inferred)).map_err(ended)?.map(Advance::Done)
    }
}
//#endregion 🔖️Job

//#region 🔖️Work
/// 🧵️ The retained-command shell of an [`ExportJob`]: one step per call, the framework yields between steps and tells the work when the user cancelled.
pub struct ModelExportWork {
    tool_id: &'static str,
    job: Option<ExportJob>,
}

impl ModelExportWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, job: None }
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
        let job = self.job.get_or_insert_with(|| ExportJob::new(&payload.format, Some(input.operation.app_instance_id)));
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
pub fn handle(payload: &ExportModel, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let instance = doc.operation_optional().map(|operation| operation.app_instance_id);
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
