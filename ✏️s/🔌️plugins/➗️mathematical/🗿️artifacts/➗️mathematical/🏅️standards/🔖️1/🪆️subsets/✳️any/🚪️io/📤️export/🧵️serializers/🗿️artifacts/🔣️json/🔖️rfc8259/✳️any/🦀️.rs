//! 🚪️ mathematical -> json. The exact carrier is `{graph, geometry, equation}`; composed-child
//! handles are persistence references and never stand in for their materialized content.

use crate::artifacts::mathematical::{mathematical_fixture, MathematicalSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct MathematicalIntoJson;

impl Serializer<MathematicalSnapshot> for MathematicalIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &MathematicalSnapshot) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let fixture = mathematical_fixture(from).map_err(|error| IoError { message: format!("MathematicalIntoJson: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(pack::json::to_json_string(&fixture).into_bytes())))
    }
}
