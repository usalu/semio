//! 🚪️ vcs <- zip — foreign `Deserializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Pre-migration behavior
//! preserved verbatim (a plain `serde_json` struct coercion), hence `IoFidelity::Lossy`. The old
//! hand-rolled channel took an already-typed `&ZipSnapshot`; this leaf additionally decodes the
//! foreign payload's own pack bytes first, as the `FROM: ZIP_DIALECT` coordinate requires.

use crate::artifacts::vcs::VcsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct ZipIntoVcs;

impl Deserializer<VcsSnapshot> for ZipIntoVcs {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<VcsSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "ZipIntoVcs: expected a binary zip payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
        let zip = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("ZipIntoVcs: zip decode failed: {error}"), diagnostics: Vec::new() })?;
        let value = serde_json::to_value(&zip).map_err(|error| IoError { message: format!("ZipIntoVcs: {error}"), diagnostics: Vec::new() })?;
        let snapshot: VcsSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("ZipIntoVcs: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
