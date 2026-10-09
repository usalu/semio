//! 🚪️ fem3d ← json — foreign `Deserializer<Fem3dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf: `IoFidelity::Exact`. Parsing goes
//! through stdio's own real RFC 8259 codec (`parse_json_text`), never a re-derived parser.

use crate::Fem3dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot.
pub fn from_json_text(text: &str) -> Result<Fem3dSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json→fem3d: parse failed: {error}"))))?;
    let raw: semio_framework_value::DslValue = JsonSnapshot::from_value(value).to_serde_value().into();
    semio_framework_value::FromValue::from_value(raw).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json→fem3d: {error}"))))
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
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<Fem3dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "json→fem3d: expected a text json payload".to_string())));
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
