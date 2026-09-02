//! 🚪️ mathematical <- json. The exact carrier reconstructs composed children with a local owner.

use crate::artifacts::mathematical::{mathematical_snapshot_from_fixture, MathematicalFixture, MathematicalSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoMathematical;

impl Deserializer<MathematicalSnapshot> for JsonIntoMathematical {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<MathematicalSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "JsonIntoMathematical: expected a binary json payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = std::str::from_utf8(bytes).map_err(|error| IoError { message: format!("JsonIntoMathematical: not valid utf-8: {error}"), diagnostics: Vec::new() })?;
        let fixture: MathematicalFixture = pack::json::from_json_str(text).map_err(|error| IoError { message: format!("JsonIntoMathematical: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(mathematical_snapshot_from_fixture(fixture)))
    }
}
