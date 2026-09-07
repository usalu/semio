//! 🚪️ fem2d → csv — foreign `Serializer<Fem2dSnapshot>` on the framework's `io_mechanism` channel.
//! A fem2d document is eight heterogeneous tables (nodes/elements/regions/materials/sections/
//! supports/load-cases/combinations) plus one settings block; RFC 4180 has exactly one table per
//! document and no nesting, so a column-per-field projection would have to pick ONE of the eight and
//! silently drop the rest. Instead this is a single-column ENVELOPE: header row `payload`, one data
//! row carrying this subset's own `.semio` DSL text as a quoted field. RFC 4180 §2 rule 6 quoting is
//! exactly reversible (stdio's tokenizer consumes a quoted field's embedded newlines as data), so
//! nothing about the snapshot is lost and the sibling `📥️import` leaf reconstructs it exactly:
//! `IoFidelity::Exact`.
//!
//! 🐛️ Repaired here (ticket 26/09/06/FEM-PLUGIN-END-TO-END, W4): the previous leaf built the same
//! envelope but reached the wire through `<CsvSnapshot as store::ArtifactPack>::encode_pack` — a
//! `.spk` binary container mislabelled as `s.stdio.csv` text. It now writes real RFC 4180 text
//! through stdio's own `encode_csv`.

use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{encode_csv, CsvField, CsvRecord};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

/// 🎯️ The foreign dialect this leaf writes.
pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

/// 🏷️ The single column this envelope declares — the anchor the sibling importer sniffs on.
pub const PAYLOAD_COLUMN: &str = "payload";

/// 📊️ The envelope as a `CsvSnapshot`: `records[0]` is the header, `records[1]` the DSL payload.
pub fn csv_snapshot(from: &Fem2dSnapshot) -> CsvSnapshot {
    CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        has_header: true,
        records: vec![CsvRecord { fields: vec![CsvField { value: PAYLOAD_COLUMN.into(), quoted: false }] }, CsvRecord { fields: vec![CsvField { value: <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(from), quoted: true }] }],
    }
}

/// 📊️ The envelope as real RFC 4180 text.
pub fn csv_text(from: &Fem2dSnapshot) -> String {
    encode_csv(&csv_snapshot(from))
}

/// 🧵️ `s.fem.fem2d@1/*` → `s.stdio.csv@rfc4180/*`.
pub struct Fem2dIntoCsv;

impl Serializer<Fem2dSnapshot> for Fem2dIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem2dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(csv_text(from))))
    }
}
