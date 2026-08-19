//! 🚪️ wires -> csv — foreign `Serializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). A flat CSV grid has no
//! node/edge graph concept, so this hop stays the pre-migration honest no-op (an empty CSV
//! document, no real tabular mapping) — `IoFidelity::Lossy`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct WiresIntoCsv;

impl Serializer<WiresSnapshot> for WiresIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &WiresSnapshot) -> IoResult<IoPayload> {
        let csv = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: Vec::new() };
        Ok(IoOutcome::clean(IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(&csv))))
    }
}
