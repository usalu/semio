//! 🚪️ sequence -> json. The exact carrier is `{schema, steps, edges}` and requires the composed
//! child scene to be materialized before serialization.

use crate::SequenceSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct SequenceIntoJson;

impl Serializer<SequenceSnapshot> for SequenceIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &SequenceSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let fixture = from.try_to_host_snapshot().map_err(|error| IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner, format!("SequenceIntoJson: {error}"))))?;
        let raw = semio_framework_value::ToValue::to_value(&fixture);
        let bytes = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&raw)).into_bytes();
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
