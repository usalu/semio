//! 📜️ FEM 2D artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::Fem2dSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📦️ The `fem2d-play` "default" example, embedded at compile time as handcrafted `.fem2d` DSL text —
/// shared by the manifest's `.example(...)` registration, the `setActiveExample` handler, and every
/// test fixture.
pub const FEM2D_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.fem2d` DSL text into a `Fem2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Fem2dSnapshot, store::TextError> {
    <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Fem2dSnapshot` back to `.fem2d` DSL text.
pub fn print_dsl(document: &Fem2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Fem2dSnapshotText = String;
//#endregion 🚚️Carrier
