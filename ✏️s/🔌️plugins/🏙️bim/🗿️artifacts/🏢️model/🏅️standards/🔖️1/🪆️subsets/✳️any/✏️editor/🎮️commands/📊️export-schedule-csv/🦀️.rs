//! 📊️ `exportScheduleCsv`: writes the table of one schedule as an RFC 4180 file through the `s.stdio.csv` dialect and hands the shell one download. The schedule is the payload's, else the one the addressed
//! schedule window shows. The table is read from the instance's inference session, so the file shows exactly what the window shows. The expensive half is a job: [`CsvJob`] brings the session up to the
//! document in bounded steps (a progress stage per step, the nodes finished so far stay cached when the user cancels) and then writes the records in chunks; [`ScheduleCsvWork`] is its retained-command
//! shell. The dispatch path [`handle`] answers the same file in one go for callers that do not run jobs.
//!
//! @see ../../../🚪️io/📤️export/📊️csv/🦀️.rs — the dialect writer this command speaks.

use crate::editor::bim::kit::fault;
use crate::editor::bim::modes::edit::windows::schedule;
use crate::editor::bim::{BimCommand, BimDispatchCtx, BimModelApp};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::standards::v1::subsets::any::io::export::csv::{codec, schedule_records};
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

pub const MIME_TYPE: &str = "text/csv";
/// ⏩️ Graph nodes one job step may finish.
pub const NODES_PER_STEP: usize = 256;
/// ⏩️ Records one job step may write.
pub const RECORDS_PER_STEP: usize = 512;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-schedule-csv")]
pub struct ExportScheduleCsv {
    /// The schedule to export; empty means the one the addressed schedule window shows.
    pub id: String,
    /// The state a toggle that triggers the export reports; the export ignores it.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
}

/// 🏷️ The file name of a schedule: its name with every run of characters other than letters and digits turned into one dash, plus `.csv`.
pub fn file_name(name: &str) -> String {
    let mut stem = String::new();
    for ch in name.chars() {
        if ch.is_alphanumeric() {
            stem.push(ch);
        } else if !stem.ends_with('-') && !stem.is_empty() {
            stem.push('-');
        }
    }
    let stem = stem.trim_end_matches('-');
    format!("{}.csv", if stem.is_empty() { "schedule" } else { stem })
}

/// 🎯️ The id of the schedule to export: the payload's, else the shown one; a fault when the model has no such schedule.
pub fn resolve(requested: &str, shown: &str, snapshot: &ModelSnapshot) -> Result<String, Fault> {
    let id = if requested.is_empty() { shown } else { requested };
    if snapshot.schedules.contains_key(id) {
        Ok(id.to_string())
    } else {
        Err(fault("app.schedule.missing", format!("the model has no schedule '{id}' to export")))
    }
}

fn download(snapshot: &ModelSnapshot, id: &str, text: String) -> Emit<ModelMutation, NoConfigMutation> {
    Emit::effect(Effect::DownloadMediaExport { filename: file_name(&snapshot.schedules[id].name), mime_type: MIME_TYPE.into(), data: text, encoding: None })
}

//#region 🔖️Job
/// 🧵️ Where a job stands.
enum Stage {
    Open,
    Infer,
    Encode,
}

/// ⏳️ What one step of a job did.
#[derive(Debug, PartialEq)]
pub enum Advance {
    Progress(&'static str),
    Done(String),
}

/// 🧵️ The export of one schedule as a stepped, cancellable job: infer, then encode. Dropping it half-way is the cancellation: the session keeps every node it finished.
pub struct CsvJob {
    id: String,
    instance: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    analysis: inference::Analysis,
    records_per_step: usize,
    stage: Stage,
    records: Vec<Vec<String>>,
    written: usize,
    text: String,
}

impl CsvJob {
    /// 🏗️ A job exporting schedule `id` of the document that instance `instance` shows.
    pub fn new(id: String, instance: inference::Instance<'_>) -> Self {
        Self::with_steps(id, instance, NODES_PER_STEP, RECORDS_PER_STEP)
    }

    /// 🏗️ A job with explicit step sizes (at least one node and one record per step).
    pub fn with_steps(id: String, instance: inference::Instance<'_>, nodes: usize, records_per_step: usize) -> Self {
        Self { id, instance: instance.cloned(), analysis: inference::Analysis::new(instance, nodes), records_per_step: records_per_step.max(1), stage: Stage::Open, records: Vec::new(), written: 0, text: String::new() }
    }

    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
        self.analysis.cancel(snapshot);
        self.stage = Stage::Open;
    }

    /// 📈️ How much of the encoding is written, in records.
    pub fn written(&self) -> (usize, usize) {
        (self.written, self.records.len())
    }

    /// ⏩️ Does one bounded step against `snapshot`.
    pub fn advance(&mut self, snapshot: &ModelSnapshot) -> Result<Advance, Fault> {
        if !snapshot.schedules.contains_key(&self.id) {
            return Err(fault("app.schedule.missing", format!("the model has no schedule '{}' to export", self.id)));
        }
        match std::mem::replace(&mut self.stage, Stage::Open) {
            Stage::Open => {
                self.stage = Stage::Infer;
                Ok(Advance::Progress("bim-schedule-csv-infer"))
            }
            Stage::Infer => {
                let progress = self.analysis.advance(snapshot).map_err(|error| fault("app.schedule.inference", format!("the schedule table could not be inferred: {error:?}")))?;
                if progress.done {
                    let schedule = &snapshot.schedules[&self.id];
                    self.records = inference::with_inference(self.instance.as_ref(), snapshot, |found| schedule_records(schedule, found.schedules.get(&self.id).unwrap_or(&Default::default())));
                    self.stage = Stage::Encode;
                    Ok(Advance::Progress("bim-schedule-csv-encode"))
                } else {
                    self.stage = Stage::Infer;
                    Ok(Advance::Progress("bim-schedule-csv-infer"))
                }
            }
            Stage::Encode => {
                let end = (self.written + self.records_per_step).min(self.records.len());
                let chunk: Vec<_> = self.records[self.written..end].iter().cloned().map(codec::record).collect();
                self.text.push_str(&codec::document_text(chunk));
                self.written = end;
                if end == self.records.len() {
                    Ok(Advance::Done(std::mem::take(&mut self.text)))
                } else {
                    self.stage = Stage::Encode;
                    Ok(Advance::Progress("bim-schedule-csv-encode"))
                }
            }
        }
    }
}
//#endregion 🔖️Job

//#region 🔖️Work
/// 🧵️ The retained-command shell of a [`CsvJob`]: one step per call, the framework yields between steps and drops the work on cancellation.
pub struct ScheduleCsvWork {
    tool_id: &'static str,
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    job: Option<CsvJob>,
}

impl ScheduleCsvWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, job: None }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for ScheduleCsvWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.job.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, Fault> {
        let BimCommand::ExportScheduleCsv(payload) = input.command else { return Err(Fault::from("bim-schedule-csv-work-mismatch")) };
        if self.job.is_none() {
            let shown = schedule::config::from_snapshot(input.context.and_then(|context| context.window_config.as_ref())).schedule;
            self.job = Some(CsvJob::new(resolve(&payload.id, &shown, input.snapshot)?, Some(&self.owner)));
        }
        let job = self.job.as_mut().ok_or_else(|| Fault::from("bim-schedule-csv-work-terminal"))?;
        if cx.is_cancelled(){job.cancel(input.snapshot);return Err(fault("bim.export.cancelled", "the schedule export was cancelled"))}
        cx.consume_fuel(1);
        Ok(match job.advance(input.snapshot)? {
            Advance::Progress(stage) => {
                cx.set_stage(stage);
                ArtifactCommandWorkStep::Progress { stage, preview: b"{}" }
            }
            Advance::Done(text) => {
                let id = job.id.clone();
                ArtifactCommandWorkStep::Complete(download(input.snapshot, &id, text))
            }
        })
    }
}
//#endregion 🔖️Work

/// 📊️ The synchronous path: the table of the instance's inference session as one download.
pub fn handle(payload: &ExportScheduleCsv, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let id = resolve(&payload.id, &ctx.schedule.schedule, doc.snapshot)?;
    let schedule = &doc.snapshot.schedules[&id];
    let instance = ctx.gestures.as_ref();
    let records = inference::with_inference(instance, doc.snapshot, |found| schedule_records(schedule, found.schedules.get(&id).unwrap_or(&Default::default())));
    Ok(download(doc.snapshot, &id, codec::document_text(records.into_iter().map(codec::record).collect())))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
