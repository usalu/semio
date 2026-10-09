//! 📊️ `s.bim.model@1/*` → `s.stdio.csv@rfc4180/*`: the schedules and the diagnostics of a [`ModelSnapshot`] as RFC 4180 tables. A schedule table has one header record (the heading of each column, the token of
//! its key where it has none) and one record per row of the inferred table, group and total rows included; numbers are written with at most six decimals, texts and enumerations as their stable
//! tokens, so the file is the same in every language. The diagnostics table has one record per finding (severity, code, storey, elements, missing ids, English and German message).
//! 🔖 `IoFidelity::Lossy`: the table keeps neither units nor structure beyond its rows. The serializer writes one report: a section per schedule in id order, then the diagnostics, separated by an empty record;
//! [`schedule_csv`] and [`diagnostics_csv`] write one table each.
//! 📎 https://www.rfc-editor.org/rfc/rfc4180

use crate::standards::v1::subsets::any::schema::inferences::diagnostics::Diagnostic;
use crate::standards::v1::subsets::any::schema::inferences::schedules::{RowKind, ScheduleCell, ScheduleTable};
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::{ModelSnapshot, Schedule};
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};

#[path = "🧱️codec/🦀️.rs"]
pub mod codec;

/// 🪪️ The RFC 4180 dialect this leaf writes.
pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

/// 🔢️ A number with at most six decimals and no trailing zeros.
pub fn number_text(value: f64) -> String {
    let fixed = format!("{value:.6}");
    let trimmed = fixed.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" || trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

fn cell_text(cell: &ScheduleCell) -> String {
    match cell {
        ScheduleCell::Empty => String::new(),
        ScheduleCell::Text { value } => value.clone(),
        ScheduleCell::Number { value } => number_text(*value),
    }
}

/// 🧾️ The records of one schedule table: the header, then a record per row; the label `total` fills the first empty cell of the grand total row.
pub fn schedule_records(schedule: &Schedule, table: &ScheduleTable) -> Vec<Vec<String>> {
    let header: Vec<String> = schedule.columns.iter().map(|column| column.heading.clone().unwrap_or_else(|| column.key.token())).collect();
    let mut records = vec![header];
    for row in &table.rows {
        let mut cells: Vec<String> = row.cells.iter().map(cell_text).collect();
        if row.kind == RowKind::Total {
            if let Some(first) = cells.iter_mut().find(|cell| cell.is_empty()) {
                *first = "total".to_string();
            }
        }
        records.push(cells);
    }
    records
}

/// 🚦️ The records of a diagnostics table: the header, then a record per finding.
pub fn diagnostics_records(found: &[Diagnostic]) -> Vec<Vec<String>> {
    let mut records = vec![["severity", "code", "storey", "elements", "missing", "message_en", "message_de"].map(String::from).to_vec()];
    for finding in found {
        records.push(vec![
            format!("{:?}", finding.severity).to_lowercase(),
            finding.code.slug().to_string(),
            finding.storey.clone().unwrap_or_default(),
            finding.elements.join("+"),
            finding.missing.join("+"),
            finding.text("en").unwrap_or_default(),
            finding.text("de").unwrap_or_default(),
        ]);
    }
    records
}

fn document(records: Vec<Vec<String>>) -> String {
    codec::document_text(records.into_iter().map(codec::record).collect())
}

/// 📊️ The CSV table of schedule `id` from an inference of `model`; `None` when the model has no such schedule.
pub fn schedule_csv(model: &ModelSnapshot, inferred: &ModelInference, id: &str) -> Option<String> {
    let schedule = model.schedules.get(id)?;
    Some(document(schedule_records(schedule, inferred.schedules.get(id).unwrap_or(&ScheduleTable::default()))))
}

/// 🚦️ The CSV table of the diagnostics of an inference.
pub fn diagnostics_csv(inferred: &ModelInference) -> String {
    document(diagnostics_records(&inferred.diagnostics))
}

/// 🚦️ The CSV table of the diagnostics of `model`, inferred through the model graph.
pub fn export_diagnostics(model: &ModelSnapshot) -> Result<String, semio_framework_value::ValueError> {
    use protocol::Inference;
    ModelInference::infer(model).map(|inferred| diagnostics_csv(&inferred))
}

/// ⚖️ The canonical JSON table `"<position>|<slug>|<elements>" → { severity, storey, missing, message_en, message_de }` of a diagnostics table read back with the decoder: the table the third-party oracle reads from the same file.
pub fn findings_json(text: &str) -> String {
    use std::collections::BTreeMap;
    let records = codec::read_records(text);
    let table: BTreeMap<String, BTreeMap<String, String>> = records
        .iter()
        .skip(1)
        .enumerate()
        .map(|(position, record)| (format!("{position:04}|{}|{}", record[1], record[3]), ["severity", "storey", "missing", "message_en", "message_de"].into_iter().zip([&record[0], &record[2], &record[4], &record[5], &record[6]]).map(|(name, value)| (name.to_string(), value.clone())).collect()))
        .collect();
    semio_framework_pack_json::to_json_string(&table)
}

/// 📋️ The report of a model: a section per schedule (a record `schedule,<id>,<name>,<category>`, then its table), then the diagnostics (a record `diagnostics,<count>`, then its table), sections separated by an empty record.
pub fn report_csv(model: &ModelSnapshot, inferred: &ModelInference) -> String {
    let mut records: Vec<Vec<String>> = Vec::new();
    for (id, schedule) in &model.schedules {
        records.push(vec!["schedule".to_string(), id.clone(), schedule.name.clone(), schedule.category.token().to_string()]);
        records.extend(schedule_records(schedule, inferred.schedules.get(id).unwrap_or(&ScheduleTable::default())));
        records.push(vec![String::new()]);
    }
    records.push(vec!["diagnostics".to_string(), inferred.diagnostics.len().to_string()]);
    records.extend(diagnostics_records(&inferred.diagnostics));
    document(records)
}

//#region 🔖️Serializer
/// 📊️ The CSV serializer of the BIM model: the report of its schedules and diagnostics.
pub struct ModelIntoCsv;

impl Serializer<ModelSnapshot> for ModelIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let report = crate::standards::v1::subsets::any::io::with_inferred("ModelIntoCsv", from, |inferred| report_csv(from, inferred))?;
        Ok(IoOutcome::clean(IoPayload::Text(report)))
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
