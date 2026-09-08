//! 🚪️ fem3d ← json — foreign `Deserializer<Fem3dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf: `IoFidelity::Exact`. Parsing goes
//! through stdio's own real RFC 8259 codec (`parse_json_text`), never a re-derived parser.

use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot.
pub fn from_json_text(text: &str) -> Result<Fem3dSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError { message: format!("json→fem3d: parse failed: {error}"), diagnostics: Vec::new() })?;
    let raw: dsl::DslValue = JsonSnapshot::from_value(value).to_serde_value().into();
    dsl::FromValue::from_value(raw).map_err(|error| IoError { message: format!("json→fem3d: {error}"), diagnostics: Vec::new() })
}

/// 🧩️ `s.stdio.json@rfc8259/*` → `s.fem.fem3d@1/*`.
pub struct JsonIntoFem3d;

impl Deserializer<Fem3dSnapshot> for JsonIntoFem3d {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with('{') => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Fem3dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "json→fem3d: expected a text json payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
