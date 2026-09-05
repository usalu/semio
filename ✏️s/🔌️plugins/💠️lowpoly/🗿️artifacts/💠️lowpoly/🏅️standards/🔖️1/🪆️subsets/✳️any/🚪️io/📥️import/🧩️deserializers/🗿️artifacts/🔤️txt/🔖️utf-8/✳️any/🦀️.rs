//! lowpoly <- txt
//!
//! 📜️ Exact inverse of the export leaf: the txt body IS lowpoly's own `.lowpoly` DSL text
//! verbatim (CARRIER_TEXT law, see the export leaf's doc comment), so import is just
//! `store::ArtifactDsl::parse_dsl` on the body -- no second bespoke grammar to maintain.
use crate::artifacts::lowpoly::schema::snapshot::text::parse_dsl;
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&TxtSnapshot::from_body(text))
}
