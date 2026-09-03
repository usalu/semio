//! 🚪️ vcs <- csv — foreign `Deserializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Pre-migration behavior
//! preserved verbatim (a plain structural `DslValue` coercion): `CsvSnapshot`'s `records` have no
//! counterpart on `VcsSnapshot`, so only `schema` survives, hence `IoFidelity::Lossy`. The old
//! hand-rolled channel took an already-typed `&CsvSnapshot`; this leaf additionally decodes the
//! foreign payload's own pack bytes first, as the `FROM: CSV_DIALECT` coordinate requires.

use crate::artifacts::vcs::VcsSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::CsvSnapshot;

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoVcs;

impl Deserializer<VcsSnapshot> for CsvIntoVcs {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<VcsSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "CsvIntoVcs: expected a binary csv payload".to_string(), diagnostics: Vec::new() });
        };
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("CsvIntoVcs: csv decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = VcsSnapshot::from_value(csv.to_value()).map_err(|error| IoError { message: format!("CsvIntoVcs: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
