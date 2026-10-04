//! 🚪️ vcs -> json — foreign `Serializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Bridges via json's own RFC8259
//! text codec (`write_json_pretty`), so this hop is `IoFidelity::Exact`.

use crate::VcsSnapshot;
use semio_framework_value::ToValue;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub fn serialize(snapshot: &VcsSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    Ok(JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&snapshot.to_value())))
}

pub fn serialize_bytes(snapshot: &VcsSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}

pub struct VcsIntoJson;

impl Serializer<VcsSnapshot> for VcsIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &VcsSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let bytes = serialize_bytes(from).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("VcsIntoJson: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
