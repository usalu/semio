//! 🚪️ vcs -> csv — foreign `Serializer<VcsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Pre-migration behavior
//! preserved verbatim (a plain `serde_json` struct coercion, kept as-is — reshaping the domain
//! mapping is out of this cutover's scope): `VcsSnapshot`'s fields have no counterpart in
//! `CsvSnapshot`'s `{schema,has_header,records}` shape, so only `schema` survives and every row is
//! dropped, hence `IoFidelity::Lossy`.

use crate::artifacts::vcs::VcsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::CsvSnapshot;

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct VcsIntoCsv;

impl Serializer<VcsSnapshot> for VcsIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &VcsSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("VcsIntoCsv: {error}"), diagnostics: Vec::new() })?;
        let csv: CsvSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("VcsIntoCsv: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(&csv))))
    }
}
