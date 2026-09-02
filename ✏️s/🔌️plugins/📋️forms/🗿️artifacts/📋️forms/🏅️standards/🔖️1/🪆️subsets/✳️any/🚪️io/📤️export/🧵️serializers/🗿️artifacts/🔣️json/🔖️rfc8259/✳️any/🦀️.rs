//! 🚪️ forms -> json — foreign `Serializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct `serde_json`
//! serialization of every field, so this hop is `IoFidelity::Exact`.

use crate::artifacts::forms::FormsSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct FormsIntoJson;

impl Serializer<FormsSnapshot> for FormsIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn serialize(from: &FormsSnapshot) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = dsl::json::to_string_pretty(&dsl::json::from_dsl_value(&from.to_value()));
        Ok(IoOutcome::clean(IoPayload::Binary(text.into_bytes())))
    }
}
