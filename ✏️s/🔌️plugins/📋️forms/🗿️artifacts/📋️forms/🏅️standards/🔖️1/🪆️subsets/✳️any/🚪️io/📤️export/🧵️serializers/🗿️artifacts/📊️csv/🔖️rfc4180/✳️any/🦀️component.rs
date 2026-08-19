//! 🚪️ forms -> csv — foreign `Serializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Real, meaningful export: one
//! row per question, flattened in step order (`id`/`stepId`/`label`/`kind`/`required`) — the same
//! projection `crate::artifacts::forms::forms_results_from_steps` already derives for the
//! `results` composed child, built directly here to avoid a `s.stdio.semio.table` round trip.
//! `IoFidelity::Lossy`: drops `schema`/`id`/`version`/`title` and every per-question config field
//! (`options`/`condition`/`params`/`default`/…) — a flat grid has no place for them.

use crate::artifacts::forms::{forms_steps, FormsSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::{CsvField, CsvRecord, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

async fn field(value: String) -> CsvField {
    CsvField { value, quoted: false }
}

pub struct FormsIntoCsv;

impl Serializer<FormsSnapshot> for FormsIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &FormsSnapshot) -> IoResult<IoPayload> {
        let mut records = vec![CsvRecord { fields: ["id", "stepId", "label", "kind", "required"].into_iter().map(|header| field(header.to_string())).collect() }];
        for step in forms_steps(from) {
            for block in step.blocks {
                records.push(CsvRecord {
                    fields: vec![
                        field(block.id),
                        field(step.id.clone()),
                        field(block.label),
                        field(block.kind),
                        field(block.required.map(|value| value.to_string()).unwrap_or_default()),
                    ],
                });
            }
        }
        let csv = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records };
        Ok(IoOutcome::clean(IoPayload::Binary(store::ArtifactPack::encode_pack(&csv))))
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::schema::onboarding_example_spec;

    #[semio_framework_async_macros::async_test]
    async fn export_flattens_one_row_per_question_plus_header() {
        let spec = onboarding_example_spec();
        let question_count: usize = forms_steps(&spec).iter().map(|step| step.blocks.len()).sum();
        let IoOutcome { value: IoPayload::Binary(bytes), .. } = FormsIntoCsv::serialize(&spec).expect("serialize") else {
            panic!("expected a binary payload");
        };
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(csv.records.len(), question_count + 1, "header row + one row per question");
        assert_eq!(csv.records[0].fields[0].value, "id");
    }
}
//#endregion 🧪️Tests
