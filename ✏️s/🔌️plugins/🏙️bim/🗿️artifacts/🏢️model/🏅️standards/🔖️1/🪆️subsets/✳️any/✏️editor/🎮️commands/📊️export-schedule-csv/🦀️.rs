//! 📊️ `exportScheduleCsv`: writes the table of one schedule as an RFC 4180 file through the `s.stdio.csv` dialect and hands the shell one download. The schedule is the payload's, else the one the addressed
//! schedule window shows. [`ScheduleCsvWork`] reads the rendered revision of the mounted inference session and writes the table through the host's segmented output. The reference [`CsvJob`] steps its
//! own inference and record encoding. The synchronous [`handle`] dispatch produces the same CSV.
//!
//! @see ../../../🚪️io/📤️export/📊️csv/🦀️.rs — the dialect writer this command speaks.

use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::csv::codec;
use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::schedule_records;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[path = "🧵️.rs"]
mod retained;
pub use retained::ScheduleCsvWork;

pub const MIME_TYPE: &str = "text/csv";
/// ⏩️ Graph nodes one job step may finish.
pub const NODES_PER_STEP: usize = 256;
/// ⏩️ Records one job step may write.
pub const RECORDS_PER_STEP: usize = 512;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-schedule-csv")]
pub struct ExportScheduleCsv {
    /// ⏱️ The schedule to export; empty means the one the addressed schedule window shows.
    pub id: String,
    /// 🔘️ The state a toggle that triggers the export reports; the export ignores it.
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
    analysis: semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis,
    stage: Stage,
    records: Vec<Vec<String>>,
    written: usize,
    text: String,
}

impl CsvJob {
    /// 🏗️ A job exporting schedule `id` of the document that instance `instance` shows.
    pub fn new(id: String, instance: semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>) -> Self {
        Self{id,analysis:semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis::new(instance),stage:Stage::Open,records:Vec::new(),written:0,text:String::new()}
    }

    /// 📈️ How much of the encoding is written, in records.
    pub fn written(&self) -> (usize, usize) {
        (self.written, self.records.len())
    }

    /// 📊️ Advances a reference export by bounded inference or CSV records.
    pub fn advance_cold(&mut self, snapshot: &ModelSnapshot, nodes: usize, records: usize) -> Result<Advance, Fault> {
        let schedule = snapshot.schedules.get(&self.id).ok_or_else(|| fault("app.schedule.missing", format!("the model has no schedule '{}' to export", self.id)))?;
        let ended = |error: protocol::InferenceError| fault("app.schedule.inference", format!("the schedule table could not be inferred: {error:?}"));
        match self.stage {
            Stage::Open => {
                self.stage = Stage::Infer;
                Ok(Advance::Progress("bim-schedule-csv-infer"))
            }
            Stage::Infer => {
                if !self.analysis.advance_cold(snapshot, nodes).map_err(ended)?.done {
                    return Ok(Advance::Progress("bim-schedule-csv-infer"));
                }
                self.records = self.analysis.read_cold(snapshot, |found| schedule_records(schedule, found.schedules.get(&self.id).unwrap_or(&Default::default()))).map_err(ended)?;
                self.stage = Stage::Encode;
                Ok(Advance::Progress("bim-schedule-csv-encode"))
            }
            Stage::Encode => {
                let end = self.written.saturating_add(records.max(1)).min(self.records.len());
                let chunk: Vec<_> = self.records[self.written..end].iter().cloned().map(codec::record).collect();
                self.text.push_str(&codec::document_text(chunk));
                self.written = end;
                if end == self.records.len() {
                    Ok(Advance::Done(std::mem::take(&mut self.text)))
                } else {
                    Ok(Advance::Progress("bim-schedule-csv-encode"))
                }
            }
        }
    }

    /// 🛑️ Cancels a reference export and releases the unpublished CSV.
    pub fn cancel_cold(&mut self, snapshot: &ModelSnapshot) {
        self.analysis.cancel_cold(snapshot);
        self.records.clear();
        self.text.clear();
        self.written = 0;
        self.stage = Stage::Open;
    }
}
//#endregion 🔖️Job

/// 📊️ The synchronous path: the table of the instance's inference session as one download.
pub fn handle(payload: &ExportScheduleCsv, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let id = resolve(&payload.id, &ctx.schedule.schedule, doc.snapshot)?;
    let schedule = &doc.snapshot.schedules[&id];
    let instance = ctx.gestures.as_ref();
    let records = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(instance, doc.snapshot, |found| schedule_records(schedule, found.schedules.get(&id).unwrap_or(&Default::default())));
    Ok(download(doc.snapshot, &id, semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::csv::codec::document_text(records.into_iter().map(semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::csv::codec::record).collect())))
}

