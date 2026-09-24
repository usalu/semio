//! 🚪️ fem3d → csv — the node coordinate table: header `id, x, y, z`, then one row per node in
//! document order, written as real RFC 4180 text by stdio's own `encode_csv`. Coordinates use Rust's
//! shortest round-trip float spelling, so a reader recovers every coordinate bit for bit.
//!
//! 🔖 `IoFidelity::Lossy`: elements, regions, materials, sections, supports, loads and analysis
//! settings have no column in one flat table, so there is no csv import.
use crate::Fem3dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_csv::schema::snapshot::{encode_csv, CsvField, CsvRecord};
use semio_s_artifact_stdio_csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

/// 🎯️ The foreign dialect this leaf writes.
pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

/// 📊️ The node table as real RFC 4180 text.
pub fn csv_text(from: &Fem3dSnapshot) -> String {
    let record = |values: Vec<String>| CsvRecord { fields: values.into_iter().map(|value| CsvField { value, quoted: false }).collect() };
    let mut records = vec![record(vec!["id".into(), "x".into(), "y".into(), "z".into()])];
    records.extend(from.nodes.iter().map(|node| record(vec![node.id.clone(), node.x.to_string(), node.y.to_string(), node.z.to_string()])));
    encode_csv(&CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records })
}

/// 🧵️ `s.fem.fem3d@1/*` → `s.stdio.csv@rfc4180/*`.
pub struct Fem3dIntoCsv;

impl Serializer<Fem3dSnapshot> for Fem3dIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &Fem3dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(csv_text(from))))
    }
}
