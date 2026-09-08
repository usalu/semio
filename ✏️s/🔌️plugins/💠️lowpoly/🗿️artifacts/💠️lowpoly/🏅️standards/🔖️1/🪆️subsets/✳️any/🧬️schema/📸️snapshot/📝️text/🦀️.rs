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
pub const LOWPOLY_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.lowpoly` DSL text into a `LowpolySnapshot`.
pub fn parse_dsl(text: &str) -> Result<LowpolySnapshot, store::TextError> {
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
