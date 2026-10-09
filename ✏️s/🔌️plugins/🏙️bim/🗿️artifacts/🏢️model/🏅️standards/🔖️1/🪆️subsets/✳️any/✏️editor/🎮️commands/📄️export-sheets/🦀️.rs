//! 📄️ `exportSheets`: writes one sheet as a paper-size SVG 1.1 file, or the sheet set as one PDF 1.7 file with a page per sheet, and hands the shell one download. The drawings come from the instance's inference session (the `view-linework` of the views
//! and the `sheet-layout` of the sheets), so the file shows exactly what the sheet window shows. The expensive half is a job: [`SheetsJob`] brings the session up to the document in bounded steps, then writes one sheet per step (the PDF page by page),
//! with a progress stage per step; when the user cancels, the nodes finished so far stay cached and the pages written so far are dropped. [`SheetsExportWork`] is its retained-command shell, which honours `StepContext::is_cancelled`.
//! The headings of the title block are printed in the language the command names (`locale`, the language of the window that asked), English when it names none.
//!
//! @see ../../../🚪️io/📤️export/📄️sheets — the writers this command speaks.

use crate::editor::bim::commands::export_model::{stem, Output, INFER_PREVIEW, NODES_PER_STEP};
use crate::editor::bim::kit::fault;
use crate::editor::bim::modes::edit::windows::sheet::title_labels;
use crate::editor::bim::terminology::BimLabels;
use crate::editor::bim::{BimCommand, BimDispatchCtx, BimModelApp};
use crate::standards::v1::subsets::any::io::export::sheets::{self, pdf::SheetsPdf, TitleLabels};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry as inference;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::Analysis;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🧾️ The formats of a sheet export: one sheet as SVG, the set as PDF.
pub const FORMATS: &[&str] = &["svg", "pdf"];
/// 🗣️ The progress text of the writing stage, English and German.
pub const PAGE_PREVIEW: &[u8] = br#"{"en":"Writing the sheets","de":"Blätter werden geschrieben"}"#;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-sheets")]
pub struct ExportSheets {
    /// The file format: `svg` (one sheet) or `pdf` (the set, or one sheet when `sheet` names it).
    pub format: String,
    /// The sheet to export; the first sheet for an SVG, the whole set for a PDF when absent.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    /// The language of the headings: `en` or `de`; English when absent.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// The state a toggle that triggers the export reports; the export ignores it.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
}

fn refusal(format: &str) -> Fault {
    fault("bim.export.format-unknown", format!("'{format}' is not a sheet format (one of {})", FORMATS.join(", ")))
}

fn failed(format: &str, message: impl std::fmt::Display) -> Fault {
    fault("bim.export.failed", format!("the {format} file could not be written: {message}"))
}

fn binary(bytes: &[u8]) -> (String, Option<&'static str>) {
    (semio_framework_io_base64::base64_standard_encode(bytes), Some(MEDIA_EXPORT_BASE64_ENCODING))
}

/// 🗣️ The label set of a locale: German for `de`, else English.
pub fn labels_of(locale: Option<&str>) -> &'static BimLabels {
    if locale == Some("de") { &BimLabels::NATIVE_DE } else { &BimLabels::NATIVE_EN }
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

/// 🧵️ The sheet export as a stepped, cancellable job: analyse, then write one sheet per step. Cancelling it keeps every node the session finished.
pub struct SheetsJob {
    format: String,
    only: Option<String>,
    instance: Option<u32>,
    labels: TitleLabels,
    analysis: Analysis,
    writer: Option<SheetsPdf>,
    written: usize,
}

impl SheetsJob {
    /// 🏗️ A job writing `format` of the sheet `only` (else the first for an SVG, all for a PDF) of the document that instance `instance` shows, headed in `locale`.
    pub fn new(format: &str, only: Option<&str>, locale: Option<&str>, instance: Option<u32>) -> Self {
        Self::with_steps(format, only, locale, instance, NODES_PER_STEP)
    }

    /// 🏗️ A job that finishes at most `nodes` graph nodes per analysis step (at least one).
    pub fn with_steps(format: &str, only: Option<&str>, locale: Option<&str>, instance: Option<u32>, nodes: usize) -> Self {
        Self { format: format.to_string(), only: only.map(str::to_string), instance, labels: title_labels(labels_of(locale)), analysis: Analysis::new(instance, nodes), writer: None, written: 0 }
    }

    /// 📈️ How far the job is, as a fraction: the analysis counts for the first half, the sheets written for the second.
    pub fn fraction(&self, snapshot: &ModelSnapshot) -> f32 {
        let total = self.targets(snapshot).len().max(1) as f32;
        if self.analysis.is_done() { 0.5 + 0.5 * self.written as f32 / total } else { 0.5 * self.analysis.fraction() }
    }

    /// 🛑️ Cancels the job: the analysis is closed as cancelled, the pages written so far are dropped.
    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
        self.writer = None;
        self.analysis.cancel(snapshot);
    }

    fn targets(&self, snapshot: &ModelSnapshot) -> Vec<String> {
        let all = sheets::ordered(snapshot);
        match (&self.only, self.format.as_str()) {
            (Some(only), _) => all.into_iter().filter(|id| id == only).collect(),
            (None, "svg") => all.into_iter().take(1).collect(),
            (None, _) => all,
        }
    }

    /// ⏩️ Does one bounded step against `snapshot`.
    pub fn advance(&mut self, snapshot: &ModelSnapshot) -> Result<Advance, Fault> {
        if !FORMATS.contains(&self.format.as_str()) {
            return Err(refusal(&self.format));
        }
        if let Some(only) = &self.only {
            if !snapshot.sheets.contains_key(only) {
                return Err(fault("bim.export.sheet-missing", format!("the model has no sheet '{only}'")));
            }
        }
        let targets = self.targets(snapshot);
        if targets.is_empty() {
            return Err(fault("bim.export.sheets-none", "the model has no sheet to export"));
        }
        let ended = |error: protocol::InferenceError| fault("bim.export.inference", format!("the model could not be analysed: {error}"));
        if !self.analysis.is_done() {
            self.analysis.advance(snapshot).map_err(ended)?;
            return Ok(Advance::Progress("bim-export-infer"));
        }
        let name = stem(&snapshot.project.name);
        if self.format == "svg" {
            let id = &targets[0];
            let labels = &self.labels;
            let document = inference::try_with_inference(self.instance, snapshot, |inferred| sheets::sheet_svg(snapshot, inferred, id, labels)).map_err(ended)?.ok_or_else(|| fault("bim.export.sheet-missing", format!("the sheet '{id}' has no layout")))?.map_err(|message| failed("svg", message))?;
            let number = snapshot.sheets.get(id).map_or("sheet", |row| row.number.as_str());
            return Ok(Advance::Done(Output { filename: format!("{name}-{}.svg", sheets::file_stem(number)), mime_type: "image/svg+xml", data: document, encoding: None }));
        }
        if self.writer.is_none() {
            let title = if snapshot.project.name.is_empty() { "BIM model" } else { snapshot.project.name.as_str() };
            self.writer = Some(SheetsPdf::begin(title, targets.len()).map_err(|message| failed("pdf", message))?);
        }
        if self.written < targets.len() {
            let id = &targets[self.written];
            let (labels, writer) = (&self.labels, self.writer.as_mut().expect("the writer was started"));
            inference::try_with_inference(self.instance, snapshot, |inferred| match inferred.sheet_layouts.get(id) {
                Some(layout) => writer.page(layout, &sheets::drawings(inferred), labels),
                None => Err(format!("the sheet '{id}' has no layout")),
            })
            .map_err(ended)?
            .map_err(|message| failed("pdf", message))?;
            self.written += 1;
            return Ok(Advance::Progress("bim-sheets-page"));
        }
        let bytes = self.writer.take().expect("the writer was started").finish().map_err(|message| failed("pdf", message))?;
        let (data, encoding) = binary(&bytes);
        let filename = match &self.only {
            Some(_) => format!("{name}-{}.pdf", sheets::file_stem(snapshot.sheets.get(&targets[0]).map_or("sheet", |row| row.number.as_str()))),
            None => format!("{name}-sheets.pdf"),
        };
        Ok(Advance::Done(Output { filename, mime_type: "application/pdf", data, encoding }))
    }
}
//#endregion 🔖️Job

//#region 🔖️Work
/// 🧵️ The retained-command shell of a [`SheetsJob`]: one step per call, the framework yields between steps and tells the work when the user cancelled.
pub struct SheetsExportWork {
    tool_id: &'static str,
    job: Option<SheetsJob>,
}

impl SheetsExportWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, job: None }
    }
}

fn preview(stage: &str) -> &'static [u8] {
    if stage == "bim-sheets-page" { PAGE_PREVIEW } else { INFER_PREVIEW }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for SheetsExportWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.job.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, Fault> {
        let BimCommand::ExportSheets(payload) = input.command else { return Err(Fault::from("bim-export-sheets-work-mismatch")) };
        let job = self.job.get_or_insert_with(|| SheetsJob::new(&payload.format, payload.sheet.as_deref(), payload.locale.as_deref(), Some(input.operation.app_instance_id)));
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

/// 📄️ The synchronous path: the file of the instance's inference session as one download.
pub fn handle(payload: &ExportSheets, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let instance = doc.operation_optional().map(|operation| operation.app_instance_id);
    let locale = payload.locale.clone().or_else(|| ctx.labels().map(|labels| labels.locale().as_str().to_string()));
    let mut job = SheetsJob::new(&payload.format, payload.sheet.as_deref(), locale.as_deref(), instance);
    loop {
        if let Advance::Done(output) = job.advance(doc.snapshot)? {
            return Ok(download(output));
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
