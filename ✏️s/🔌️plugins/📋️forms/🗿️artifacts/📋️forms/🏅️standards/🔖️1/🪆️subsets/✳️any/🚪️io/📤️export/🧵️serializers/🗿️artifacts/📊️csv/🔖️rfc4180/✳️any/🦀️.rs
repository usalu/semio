//! 📋️ forms -> csv — the question grid: a header row, then one row per question
//! (`id`/`stepId`/`label`/`kind`/`required`) in step order, written by stdio's own RFC 4180 codec.
//!
//! 🔖 `IoFidelity::Lossy`: step titles, logic, options and results have no column, so there is no
//! csv import — the document itself travels as json, txt or zip.

use crate::{forms_steps, FormsSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_csv::{CsvField, CsvRecord, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

fn field(value: String) -> CsvField {
    CsvField { value, quoted: false }
}

/// 📋️ The question grid rows, header first — shared by the csv and xlsx exports.
pub fn question_grid(from: &FormsSnapshot) -> Vec<Vec<String>> {
    let mut rows = vec![["id", "stepId", "label", "kind", "required"].into_iter().map(String::from).collect()];
    for step in forms_steps(from) {
        for block in step.blocks {
            rows.push(vec![block.id, step.id.clone(), block.label, block.kind, block.required.map(|value| value.to_string()).unwrap_or_default()]);
        }
    }
    rows
}

pub struct FormsIntoCsv;

impl Serializer<FormsSnapshot> for FormsIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &FormsSnapshot) -> IoResult<IoPayload> {
        let records = question_grid(from).into_iter().map(|row| CsvRecord { fields: row.into_iter().map(field).collect() }).collect();
        let csv = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records };
        Ok(IoOutcome::clean(IoPayload::Binary(store::ArtifactPack::encode_pack(&csv))))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
