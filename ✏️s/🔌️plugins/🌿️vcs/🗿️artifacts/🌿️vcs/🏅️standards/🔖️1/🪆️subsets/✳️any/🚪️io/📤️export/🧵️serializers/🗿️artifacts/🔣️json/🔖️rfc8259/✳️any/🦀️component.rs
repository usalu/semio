//! 🚪️ vcs -> json — foreign `Serializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct `serde_json`
//! serialization of every field, so this hop is `IoFidelity::Exact`.

use crate::artifacts::vcs::VcsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct VcsIntoJson;

impl Serializer<VcsSnapshot> for VcsIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn serialize(from: &VcsSnapshot) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("VcsIntoJson: {error}"), diagnostics: Vec::new() })?;
        let bytes = serde_json::to_vec_pretty(&value).map_err(|error| IoError { message: format!("VcsIntoJson: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
