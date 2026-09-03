//! 🚪️ sequence -> json. The exact carrier is `{schema, steps, edges}` and requires the composed
//! child scene to be materialized before serialization.

use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct SequenceIntoJson;

impl Serializer<SequenceSnapshot> for SequenceIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &SequenceSnapshot) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let fixture = from.try_to_fixture().map_err(|error| IoError { message: format!("SequenceIntoJson: {error}"), diagnostics: Vec::new() })?;
        let raw = dsl::ToValue::to_value(&fixture);
        let bytes = dsl::json::to_string_pretty(&dsl::json::from_dsl_value(&raw)).into_bytes();
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
