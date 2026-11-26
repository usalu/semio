//! 📄️ `exportSheets`: writes one sheet as a paper-size SVG 1.1 file, or the sheet set as one PDF 1.7 file with a page per sheet, and hands the shell one download. The drawings come from the instance's inference session (the `view-linework` of the views
//! and the `sheet-layout` of the sheets), so the file shows exactly what the sheet window shows. The expensive half is a job: [`SheetsJob`] brings the session up to the document in bounded steps, then writes one sheet per step (the PDF page by page),
//! with a progress stage per step; when the user cancels, the nodes finished so far stay cached and the pages written so far are dropped. [`SheetsExportWork`] is its retained-command shell, which honours `StepContext::is_cancelled`.
//! The headings of the title block are printed in the language the command names (`locale`, the language of the window that asked), English when it names none.
//!
//! @see ../../../🚪️io/📤️export/📄️sheets — the writers this command speaks.

use crate::editor::bim::commands::export_model::{close_export_writer, stem, Output, INFER_PREVIEW, NODES_PER_STEP};
use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::title_labels;
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use crate::editor::bim::{BimCommand, BimModelApp};
use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets;
use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::pdf::SheetsPdf;
use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::sheets::ink::TitleLabels;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::{model_graph::instance::{Analysis, MountedAnalysisAccess}, ModelInference};
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retirement::controlled::ControlledRetirement;

/// 🧾️ The formats of a sheet export: one sheet as SVG, the set as PDF.
pub const FORMATS: &[&str] = &["svg", "pdf"];
/// 🗣️ The progress text of the writing stage, English and German.
pub const PAGE_PREVIEW: &[u8] = r#"{"en":"Writing the sheets","de":"Blätter werden geschrieben"}"#.as_bytes();

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-sheets")]
pub struct ExportSheets {
    /// 📰️ The file format: `svg` (one sheet) or `pdf` (the set, or one sheet when `sheet` names it).
    pub format: String,
    /// 📃️ The sheet to export; the first sheet for an SVG, the whole set for a PDF when absent.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    /// 🌐️ The language of the headings: `en` or `de`; English when absent.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 🔘️ The state a toggle that triggers the export reports; the export ignores it.
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
    analysis: Analysis,
    pages: SheetPages,
}

#[derive(semio_framework_value::RetireOwned)]
struct SheetPages {
    format: String,
    only: Option<String>,
    labels: Option<TitleLabels>,
    targets: Option<Vec<String>>,
    locale:Option<String>,
    writer: Option<SheetsPdf>,
    written: usize,
}

impl SheetsJob {
    /// 🏗️ A job writing `format` of the sheet `only` (else the first for an SVG, all for a PDF) of the document that instance `instance` shows, headed in `locale`.
    pub fn new(original:ExportSheets,instance:semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>)->Self{
        Self { analysis: Analysis::new(instance), pages: SheetPages { format: original.format, only: original.sheet, locale: original.locale, labels: None, targets: None, writer: None, written: 0 } }
    }

    /// 📈️ How far the job is, as a fraction: the analysis counts for the first half, the sheets written for the second.
    pub fn fraction(&self, snapshot: &ModelSnapshot) -> f32 {
        let total = self.pages.targets.as_ref().map_or_else(|| {
            if self.pages.only.is_some() || self.pages.format == "svg" { 1 } else { snapshot.sheets.len() }
        }, Vec::len).max(1) as f32;
        if self.analysis.is_done() { 0.5 + 0.5 * self.pages.written as f32 / total } else { 0.5 * self.analysis.fraction() }
    }

    /// 🛑️ Cancels the job: the analysis is closed as cancelled, the pages written so far are dropped.
    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
        self.pages.writer = None;
        self.analysis.cancel(snapshot);
    }

    /// 📄️ Advances a reference sheet job by bounded inference work or one page.
    pub fn advance_cold(&mut self, snapshot: &ModelSnapshot, nodes: usize) -> Result<Advance, Fault> {
        if !self.analysis.is_done() {
            self.pages.prepare_targets(snapshot)?;
            self.analysis.advance_cold(snapshot, nodes).map_err(|error| fault("bim.export.inference", format!("the model could not be analysed: {error}")))?;
            return Ok(Advance::Progress("bim-export-infer"));
        }
        self.analysis.read_cold(snapshot, |inferred| self.pages.write(snapshot, inferred))
            .map_err(|error| fault("bim.export.inference", format!("the model could not be analysed: {error}")))?
    }

    /// 🛑️ Cancels reference inference and drops any unpublished sheet pages.
    pub fn cancel_cold(&mut self, snapshot: &ModelSnapshot) {
        self.pages.writer = None;
        self.pages.written = 0;
        self.analysis.cancel_cold(snapshot);
    }

}

impl SheetPages {
    fn targets(&self, snapshot: &ModelSnapshot) -> Vec<String> {
        let all = semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::ordered(snapshot);
        match (&self.only, self.format.as_str()) {
            (Some(only), _) => all.into_iter().filter(|id| id == only).collect(),
            (None, "svg") => all.into_iter().take(1).collect(),
            (None, _) => all,
        }
    }

    fn prepare_targets(&mut self, snapshot: &ModelSnapshot) -> Result<(), Fault> {
        if self.targets.is_some() { return Ok(()); }
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
        self.targets = Some(targets);
        Ok(())
    }

    fn write(&mut self, snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<Advance, Fault> {
        self.prepare_targets(snapshot)?;
        let targets = self.targets.as_ref().expect("validated sheet targets");
        if self.labels.is_none(){self.labels=Some(title_labels(labels_of(self.locale.as_deref())))}
        let name = stem(&snapshot.project.name);
        if self.format == "svg" {
            let id = &targets[0];
            let labels=self.labels.as_ref().unwrap();
            let document = sheets::sheet_svg(snapshot, inferred, id, labels).ok_or_else(|| fault("bim.export.sheet-missing", format!("the sheet '{id}' has no layout")))?.map_err(|message| failed("svg", message))?;
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
            match inferred.sheet_layouts.get(id) {
                Some(layout) => writer.page(layout, &semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::drawings(inferred), labels),
                None => Err(format!("the sheet '{id}' has no layout")),
            }
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
    owner: semio_plugin::ArtifactInstanceOperationOwnerHandle,
    access: MountedAnalysisAccess,
    writer: Option<ControlledRetirement<SheetPages>>,
    complete: bool,
}

impl SheetsExportWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, access: Default::default(), writer: None, complete: false }
    }
    fn close_demand(&self, bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.writer.as_ref().map_or_else(|| Ok(RetirementDemand::default()), |writer| writer.selected_demand(bytes))
    }

}

fn preview(stage: &str) -> &'static [u8] {
    if stage == "bim-sheets-page" { PAGE_PREVIEW } else { INFER_PREVIEW }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for SheetsExportWork {
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(std::mem::size_of::<Self>())
    }

    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.access.terminal_is_empty()
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (!self.complete && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, OriginalTurnFailure> {
        let BimCommand::ExportSheets(payload) = input.command else {
            return Err(ValueError::invariant("sheet export command changed").into());
        };
        if self.complete { return Err(ValueError::invariant("sheet export already completed").into()); }
        if cx.is_cancelled() { return Err(Fault::from("bim.export.cancelled").into()); }
        let page = Analysis::read_rendered(&self.owner, &mut self.access, input.operation.canonical_base_revision, cx, |inferred, cx| {
            if !cx.charge_work_unit() { return Ok(Advance::Progress("bim-sheets-page")); }
            cx.set_stage("bim-sheets-page");
            let writer = self.writer.get_or_insert_with(|| ControlledRetirement::retain(SheetPages {
                format: payload.format.clone(), only: payload.sheet.clone(), locale: payload.locale.clone(),
                labels: None, targets: None, writer: None, written: 0,
            }));
            writer.original_mut().expect("active sheet exporter retains its pages").write(input.snapshot, inferred)
        })?;
        match page.transpose()? {
            Some(Advance::Done(output)) => {
                self.complete = true;
                Ok(ArtifactCommandWorkStep::Complete(download(output)))
            }
            Some(Advance::Progress(stage)) => Ok(ArtifactCommandWorkStep::Progress { stage, preview: preview(stage) }),
            None => Ok(ArtifactCommandWorkStep::Progress { stage: "bim-export-infer", preview: INFER_PREVIEW }),
        }
    }

    close_export_writer!();
}
//#endregion 🔖️Work

/// 📄️ The synchronous path: the file of the instance's inference session as one download.
pub fn handle(payload: &ExportSheets, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let instance = ctx.gestures.as_ref();
    let locale = payload.locale.clone().or_else(|| ctx.labels().map(|labels| labels.locale().as_str().to_string()));
    let mut job=SheetsJob::new(ExportSheets{format:payload.format.clone(),sheet:payload.sheet.clone(),locale,pressed:payload.pressed},instance);
    loop {
        if let Advance::Done(output) = job.advance_cold(doc.snapshot, NODES_PER_STEP)? {
            return Ok(download(output));
        }
    }
}

