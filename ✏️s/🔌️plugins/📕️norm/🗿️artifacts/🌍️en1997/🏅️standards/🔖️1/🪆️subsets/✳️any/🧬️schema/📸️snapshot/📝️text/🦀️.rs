//! 📜️ EN 1997 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1997Snapshot;

/// 📄️ The `default` example document, handcrafted in the `.en1997` DSL — a shallow footing +
/// pile worked example (bearing, sliding, settlement, pile axial, ground investigation depth)
/// under the DE national annex, DA1-C1 design approach.
pub const EN1997_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.en1997` DSL text into a `En1997Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1997Snapshot, store::TextError> {
    <En1997Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1997Snapshot` back to `.en1997` DSL text.
pub fn print_dsl(document: &En1997Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
