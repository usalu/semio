//! 🚪️ wires <- csv — foreign `Deserializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). A flat CSV grid has no
//! node/edge graph concept, so this hop stays the pre-migration honest no-op (a fresh empty
//! document) — `IoFidelity::Lossy`.
//!
//! 🐛️ Fixes a pre-migration bug (same class `📓️w4-sequence-report.md` fixed for `sequence`'s own
//! csv leaf): the old `deserialize_bytes` decoded the incoming bytes as a `WiresSnapshot` pack
//! directly instead of as a `CsvSnapshot` pack first. This impl decodes the foreign `CsvSnapshot`
//! first, as the `FROM: CSV_DIALECT` coordinate requires, proving the payload is real CSV before
//! discarding it.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoWires;

impl Deserializer<WiresSnapshot> for CsvIntoWires {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<WiresSnapshot> {
        let _ = STDIO_CSV_DOCUMENT_SCHEMA;
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "CsvIntoWires: expected a binary csv payload".to_string(), diagnostics: Vec::new() });
        };
        let _csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("CsvIntoWires: csv decode failed: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(crate::artifacts::wires::empty_wires_snapshot()))
    }
}
