//! lowpoly <- txt
//!
//! 📜️ Exact inverse of the export leaf: the txt body IS lowpoly's own `.lowpoly` DSL text
//! verbatim (CARRIER_TEXT law, see the export leaf's doc comment), so import is just
//! `store::ArtifactDsl::parse_dsl` on the body -- no second bespoke grammar to maintain.
use crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl;
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize(&TxtSnapshot::from_body(text))
}
