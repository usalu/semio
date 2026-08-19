//! 🚪️ vcs <- xlsx — foreign `Deserializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Pre-migration behavior
//! preserved verbatim (a plain `serde_json` struct coercion), hence `IoFidelity::Lossy`. The old
//! hand-rolled channel took an already-typed `&XlsxSnapshot`; this leaf additionally decodes the
//! foreign payload's own pack bytes first, as the `FROM: XLSX_DIALECT` coordinate requires.

use crate::artifacts::vcs::VcsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct XlsxIntoVcs;

impl Deserializer<VcsSnapshot> for XlsxIntoVcs {
    const FROM: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<VcsSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "XlsxIntoVcs: expected a binary xlsx payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_XLSX_DOCUMENT_SCHEMA;
        let xlsx = <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("XlsxIntoVcs: xlsx decode failed: {error}"), diagnostics: Vec::new() })?;
        let value = serde_json::to_value(&xlsx).map_err(|error| IoError { message: format!("XlsxIntoVcs: {error}"), diagnostics: Vec::new() })?;
        let snapshot: VcsSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("XlsxIntoVcs: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
