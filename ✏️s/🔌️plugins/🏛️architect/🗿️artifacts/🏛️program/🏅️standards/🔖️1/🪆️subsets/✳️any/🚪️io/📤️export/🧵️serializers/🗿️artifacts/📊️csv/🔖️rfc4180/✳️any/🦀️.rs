//! 🏛️ program → csv — every register flattened to one RFC 4180 table
//! (`register,id,name,status,priority,tags,source`), written by stdio's own csv codec through
//! `export_registers_csv`, the same table the editor's "Export registers" action writes.
//!
//! 🔖 `IoFidelity::Lossy`: register rows only — relationships, adjacencies, quantities and project
//! metadata have no column in this table.
use crate::schema::snapshot::ProgramSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::export_registers_csv;

pub fn register() {}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    export_registers_csv(snapshot).map(String::into_bytes).map_err(|error| store::TextError::new(format!("program→csv: {error}"), dsl::TextSpan::at(1, 1)))
}
