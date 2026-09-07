//! 🚪️ fem2d ← csv — foreign `Deserializer<Fem2dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf's single-column `payload` envelope
//! (`IoFidelity::Exact`). A csv document that is NOT that envelope is a typed `Err` naming the
//! reason, never a default snapshot.
//!
//! 🐛️ Repaired here (ticket 26/09/06/FEM-PLUGIN-END-TO-END, W4): the previous leaf ignored `bytes`
//! entirely and returned `Ok(Fem2dSnapshot::default())` — silent total data loss on every import.

use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::decode_csv_with;

/// 🎯️ The foreign dialect this leaf reads.
pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

/// 🏷️ The single column the sibling exporter's envelope declares.
pub const PAYLOAD_COLUMN: &str = "payload";

/// 📊️ Reads the envelope's `payload` cell back into this subset's snapshot.
pub fn from_csv_text(text: &str) -> Result<Fem2dSnapshot, IoError> {
    let snapshot = decode_csv_with(text, true);
    let header = snapshot.records.first().ok_or_else(|| IoError { message: "csv→fem2d: empty document, expected a `payload` header row".to_string(), diagnostics: Vec::new() })?;
    let column = header.fields.iter().position(|field| field.value == PAYLOAD_COLUMN).ok_or_else(|| IoError { message: "csv→fem2d: not a fem2d envelope — no `payload` column in the header row".to_string(), diagnostics: Vec::new() })?;
    let cell = snapshot.records.get(1).and_then(|record| record.fields.get(column)).ok_or_else(|| IoError { message: "csv→fem2d: header row present but no data row carries the `payload` cell".to_string(), diagnostics: Vec::new() })?;
    <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(&cell.value).map_err(|error| IoError { message: format!("csv→fem2d: {error}"), diagnostics: Vec::new() })
}

/// 🧩️ `s.stdio.csv@rfc4180/*` → `s.fem.fem2d@1/*`.
pub struct CsvIntoFem2d;

impl Deserializer<Fem2dSnapshot> for CsvIntoFem2d {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.starts_with(PAYLOAD_COLUMN) => Confidence::Medium,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Fem2dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "csv→fem2d: expected a text rfc4180 payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_csv_text(text)?))
    }
}
