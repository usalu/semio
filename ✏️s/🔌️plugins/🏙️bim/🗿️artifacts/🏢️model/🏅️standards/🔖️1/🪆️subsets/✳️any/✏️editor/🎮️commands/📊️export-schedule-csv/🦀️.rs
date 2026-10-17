//! 📊️ `exportScheduleCsv`: writes the table of one schedule as an RFC 4180 file through the `s.stdio.csv` dialect and hands the shell one download. The schedule is the payload's, else the one the addressed
//! schedule window shows. The table is read from the instance's inference session, so the file shows exactly what the window shows. The expensive half is a job: [`CsvJob`] brings the session up to the
//! document in bounded steps (a progress stage per step, the nodes finished so far stay cached when the user cancels) and then writes the records in chunks; [`ScheduleCsvWork`] is its retained-command
//! shell. The dispatch path [`handle`] answers the same file in one go for callers that do not run jobs.
//!
//! @see ../../../🚪️io/📤️export/📊️csv/🦀️.rs — the dialect writer this command speaks.

use crate::editor::bim::kit::fault;
use crate::editor::bim::modes::edit::windows::schedule;
use crate::editor::bim::dispatch_context::BimDispatchCtx;
use crate::editor::bim::{BimCommand, BimModelApp};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::standards::v1::subsets::any::io::export::csv::{codec, schedule_records};
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
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
        Self{id,analysis:inference::Analysis::new(instance),records_per_step:RECORDS_PER_STEP,stage:Stage::Open,records:Vec::new(),written:0,text:String::new()}
    }

    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
        self.analysis.cancel(snapshot);
    }

    /// 📈️ How much of the encoding is written, in records.
    pub fn written(&self) -> (usize, usize) {
        (self.written, self.records.len())
    }

    /// ⏩️ Does one bounded step against `snapshot`.
    pub fn advance(&mut self,snapshot:&ModelSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<Advance,protocol::InferenceWorkFailure>{
        if !self.analysis.is_done(){self.analysis.advance(snapshot,cx)?;return Ok(Advance::Progress("bim-schedule-csv-infer"))}
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original schedule projection and CSV output have no funded producer").into())
    }


}
//#endregion 🔖️Job

//#region 🔖️Work
/// 🧵️ The retained-command shell of a [`CsvJob`]: one step per call, the framework yields between steps and drops the work on cancellation.
pub struct ScheduleCsvWork {
    tool_id: &'static str,
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    job: Option<CsvJob>,
    failure:Option<protocol::InferenceWorkFailure>,
    original:Option<ExportScheduleCsv>,
    read:Option<store::os_store::snapshot_read::SnapshotRead<ModelSnapshot>>,
}

impl ScheduleCsvWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, job: None,failure:None,original:None,read:None }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for ScheduleCsvWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn receive_original_command(&mut self,command:&mut BimCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.original.is_some(){return Ok(true)}let BimCommand::ExportScheduleCsv(payload)=command else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original schedule export command changed").into())};let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original=Some(std::mem::replace(payload,ExportScheduleCsv{id:String::new(),pressed:None}));Ok(true)
    }

    fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.read.is_none()&&self.job.is_none()&&self.failure.is_none()}

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (self.job.is_none() && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, semio_framework_job::OriginalTurnFailure> {
        if let Some(original)=protocol::admit_inference_failure(&mut self.failure,cx)?{return Err(original)}
        if self.failure.is_some(){return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-schedule-csv-infer",preview:b"{}"})}
        let BimCommand::ExportScheduleCsv(_payload)=input.command else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original BIM schedule export command changed").into())};
        if self.read.is_none(){self.read=input.snapshot_read.take_current(input.operation.generation,input.operation.canonical_base_revision,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-schedule-csv-infer",preview:b"{}"})}
        if self.job.is_none(){let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-schedule-csv-infer",preview:b"{}"})}let original=self.original.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original schedule export fields were not received"))?;
            if original.id.is_empty(){self.failure=Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original selected schedule config field has no donating producer").into());return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-schedule-csv-infer",preview:b"{}"})}
            let original=self.original.take().unwrap();self.job=Some(CsvJob::new(original.id,Some(&self.owner)));cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::Progress{stage:"bim-schedule-csv-infer",preview:b"{}"})}
        let snapshot=self.read.as_ref().unwrap().get();let job=self.job.as_mut().unwrap();if cx.is_cancelled(){job.cancel(snapshot)}
        match job.advance(snapshot,cx){Ok(Advance::Progress(stage))=>{cx.set_stage(stage);Ok(ArtifactCommandWorkStep::Progress{stage,preview:b"{}"})},Ok(Advance::Done(text))=>Ok(ArtifactCommandWorkStep::Complete(download(snapshot,&job.id,text))),Err(original)=>{self.failure=Some(original);Ok(ArtifactCommandWorkStep::Progress{stage:"bim-schedule-csv-infer",preview:b"{}"})}}

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
