//! 🚪️ equation <- json. The exact `{graph, geometry, equation}` carrier rebuilds the snapshot with its derived handles.

use crate::{equation_snapshot_from_host_snapshot, EquationCarrierSnapshot, EquationSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use crate::standards::v1::subsets::any::io::invalid_payload;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoEquation;

impl Deserializer<EquationSnapshot> for JsonIntoEquation {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<EquationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(invalid_payload("JsonIntoEquation", "expected a binary json payload"));
        };
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::from(error).under("JsonIntoEquation")))?;
        let fixture: EquationCarrierSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoError::from_value_error(error.under("JsonIntoEquation")))?;
        Ok(IoOutcome::clean(equation_snapshot_from_host_snapshot(fixture)))
    }
}
