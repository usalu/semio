//! 📜️ Lowpoly artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::LowpolySnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📜️ The reuse example, handcrafted against `COMPONENT_GRAMMAR_SEMIO` — structured half-edge mesh
/// productions (no `mesh-json`). Derive-based `parse_dsl` does not yet consume this shape; the
/// recognizer / handcrafted codec will.
pub const LOWPOLY_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.lowpoly` DSL text into a `LowpolySnapshot`.
pub fn parse_dsl(text: &str) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    <LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `LowpolySnapshot` back to `.lowpoly` DSL text.
pub fn print_dsl(document: &LowpolySnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type LowpolySnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{LowpolyObject, LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

/// ✉️ Handcrafted `ArtifactDsl`; `ArtifactPack` over the derived record spec.
impl store::ArtifactDsl for LowpolySnapshot {
    const EXTENSION: &'static str = "lowpoly";
    fn envelope_id() -> &'static str {
        "lowpoly.lowpoly"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
