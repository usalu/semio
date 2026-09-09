//! 🗣️ Architect program artifact — the textual document surface (constitutional: dsl).
//!
//! `ProgramSnapshot`'s `store::ArtifactDsl` impl is `#[derive(dsl::DslRecord)]`-generated on the document
//! type itself (see `🦀️.rs`); this node owns the named entry points every consumer calls and
//! the bundled `.architect` example the derive is validated against.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::ProgramSnapshot;

/// @emoji 📦️ The "Sample Clinic" default example, embedded at compile time as handcrafted
/// `.architect` DSL text — a static transcription of `sample_plugin()`, kept in sync with it by
/// `architect_example_text_parses_to_sample_plugin_and_round_trips`. The app manifest's
/// `.example("sample", ...)` still registers `sample_plugin()` serialized to JSON at runtime
/// (a separate, pre-existing concern) — this constant exists so a static `.architect` fixture
/// is available on disk for DSL-notation round-trip testing.
pub const ARCHITECT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 🗣️ Parses an Architect program from its textual DSL representation.
pub fn parse(text: &str) -> Result<ProgramSnapshot, store::TextError> {
    <ProgramSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an Architect program in its canonical textual DSL representation.
pub fn print(document: &ProgramSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ProgramSnapshotText = String;
//#endregion 🚚️Carrier
