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
use crate::editor::bim::dispatch_context::BimDispatchCtx;
use crate::editor::bim::{BimCommand, BimModelApp};
use crate::standards::v1::subsets::any::io::export::sheets::{self, pdf::SheetsPdf, TitleLabels};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
use semio_framework_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🧾️ The formats of a sheet export: one sheet as SVG, the set as PDF.
pub const FORMATS: &[&str] = &["svg", "pdf"];
/// 🗣️ The progress text of the writing stage, English and German.
pub const PAGE_PREVIEW: &[u8] = r#"{"en":"Writing the sheets","de":"Blätter werden geschrieben"}"#.as_bytes();

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
    instance: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    labels:Option<TitleLabels>,
    locale:Option<String>,
    analysis: Analysis,
    writer: Option<SheetsPdf>,
    written: usize,
}

impl SheetsJob {
    /// 🏗️ A job writing `format` of the sheet `only` (else the first for an SVG, all for a PDF) of the document that instance `instance` shows, headed in `locale`.
    pub fn new(original:ExportSheets,instance:crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>)->Self{
        Self{format:original.format,only:original.sheet,locale:original.locale,instance:None,labels:None,analysis:Analysis::new(instance),writer:None,written:0}
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
    pub fn advance(&mut self,snapshot:&ModelSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<Advance,protocol::InferenceWorkFailure>{
        if !self.analysis.is_done(){self.analysis.advance(snapshot,cx)?;return Ok(Advance::Progress("bim-export-infer"))}
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original sheet writer and output have no funded producer").into())
    }

    fn write_cold(&mut self, snapshot: &ModelSnapshot) -> Result<Advance, Fault> {
        if self.labels.is_none(){self.labels=Some(title_labels(labels_of(self.locale.as_deref())))}
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
        let name = stem(&snapshot.project.name);
        if self.format == "svg" {
            let id = &targets[0];
            let labels=self.labels.as_ref().unwrap();
            let document = inference::try_with_inference(self.instance.as_ref(), snapshot, |inferred| sheets::sheet_svg(snapshot, inferred, id, labels)).map_err(ended)?.ok_or_else(|| fault("bim.export.sheet-missing", format!("the sheet '{id}' has no layout")))?.map_err(|message| failed("svg", message))?;
            let number = snapshot.sheets.get(id).map_or("sheet", |row| row.number.as_str());
            return Ok(Advance::Done(Output { filename: format!("{name}-{}.svg", sheets::file_stem(number)), mime_type: "image/svg+xml", data: document, encoding: None }));
        }
        if self.writer.is_none() {
            let title = if snapshot.project.name.is_empty() { "BIM model" } else { snapshot.project.name.as_str() };
            self.writer = Some(SheetsPdf::begin(title, targets.len()).map_err(|message| failed("pdf", message))?);
        }
        if self.written < targets.len() {
            let id = &targets[self.written];
            let (labels, writer) = (self.labels.as_ref().unwrap(), self.writer.as_mut().expect("the writer was started"));
            inference::try_with_inference(self.instance.as_ref(), snapshot, |inferred| match inferred.sheet_layouts.get(id) {
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
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    job: Option<SheetsJob>,
    failure:Option<protocol::InferenceWorkFailure>,
    original:Option<ExportSheets>,
    read:Option<store::os_store::snapshot_read::SnapshotRead<ModelSnapshot>>,
}

impl SheetsExportWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, job: None,failure:None,original:None,read:None }
    }
}

fn preview(stage: &str) -> &'static [u8] {
    if stage == "bim-sheets-page" { PAGE_PREVIEW } else { INFER_PREVIEW }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for SheetsExportWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn receive_original_command(&mut self,command:&mut BimCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.original.is_some(){return Ok(true)}let BimCommand::ExportSheets(payload)=command else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original sheets export command changed").into())};let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original=Some(std::mem::replace(payload,ExportSheets{format:String::new(),sheet:None,locale:None,pressed:None}));Ok(true)
    }

    fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.read.is_none()&&self.job.is_none()&&self.failure.is_none()}

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.job.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, semio_framework_job::OriginalTurnFailure> {
        if let Some(original)=protocol::admit_inference_failure(&mut self.failure,cx)?{return Err(original)}
        if self.failure.is_some(){return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}
        let BimCommand::ExportSheets(_payload) = input.command else { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original BIM sheets export command changed").into()) };
        if self.read.is_none(){self.read=input.snapshot_read.take_current(input.operation.generation,input.operation.canonical_base_revision,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}
        if self.job.is_none(){let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}let original=self.original.take().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original sheets export fields were not received"))?;self.job=Some(SheetsJob::new(original,Some(&self.owner)));cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}
        let snapshot=self.read.as_ref().unwrap().get();let job=self.job.as_mut().unwrap();if cx.is_cancelled(){job.cancel(snapshot)}
        match job.advance(snapshot,cx){Ok(Advance::Progress(stage))=>{cx.set_stage(stage);Ok(ArtifactCommandWorkStep::Progress{stage,preview:if stage=="bim-export-infer"{INFER_PREVIEW}else{PAGE_PREVIEW}})},Ok(Advance::Done(output))=>Ok(ArtifactCommandWorkStep::Complete(download(output))),Err(original)=>{self.failure=Some(original);Ok(ArtifactCommandWorkStep::Progress{stage:"bim-export-infer",preview:INFER_PREVIEW})}}

    }
}
//#endregion 🔖️Work

/// 📄️ The synchronous path: the file of the instance's inference session as one download.
pub fn handle(payload: &ExportSheets, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let instance = ctx.gestures.as_ref();
    let locale = payload.locale.clone().or_else(|| ctx.labels().map(|labels| labels.locale().as_str().to_string()));
    let mut job=SheetsJob::new(ExportSheets{format:payload.format.clone(),sheet:payload.sheet.clone(),locale,pressed:payload.pressed},instance);
    job.instance=instance.cloned();
    loop {
        if let Advance::Done(output) = job.write_cold(doc.snapshot)? {
            return Ok(download(output));
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
