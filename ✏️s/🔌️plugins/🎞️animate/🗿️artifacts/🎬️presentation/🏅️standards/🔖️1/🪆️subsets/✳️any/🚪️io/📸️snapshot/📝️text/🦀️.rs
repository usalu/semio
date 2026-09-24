//! 🗣️ Animate presentation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};

/// 📄️ The handcrafted `.presentation` DSL-text fixture — a multi-tile deck exercising every field
/// (including the optional `source-aspect`), embedded at compile time as the permanent proof that
/// the checked-in fixture still parses and round trips.
pub const PRESENTATION_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.presentation` DSL text into a `PresentationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PresentationSnapshot, store::TextError> {
    <PresentationSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PresentationSnapshot` back to `.presentation` DSL text.
pub fn print_dsl(deck: &PresentationSnapshot) -> String {
    store::ArtifactDsl::print_dsl(deck)
}

//#region 🔖️HandcraftedArtifactDsl
/// 🚪️ Moved from `../../../🧬️schema/📸️snapshot/🦀️.rs` per design.md's CORRECTION — the
/// native codec sits directly under `🚪️io/<facet>/<representation>/`, unsplit (one bidirectional
/// trait impl, not an import/export mirror).
impl store::ArtifactDsl for PresentationSnapshot {
    const EXTENSION: &'static str = "presentation";
    fn envelope_id() -> &'static str {
        PRESENTATION_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

