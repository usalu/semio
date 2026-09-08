//! 📜️ EN 1992 design of concrete structures — textual document grammar surface + laws (constitutional: dsl).

use crate::En1992Snapshot;

/// 💧️ The liquid-retaining-fem-anchor example fixture, handcrafted in `en1992`'s DSL
/// (`store::ArtifactDsl`): a liquid-retaining structure (EN 1992-3 tightness class TC2) section
/// checked with a FEM-based analysis, an R90 fire rating, and a post-installed anchor in cracked
/// concrete, under the EN annex — distinct from `En1992Snapshot::default()`'s DE-annex/TC1/R60/uncracked
/// values so the grammar's non-default branches (annex, fire rating, tightness class, `use_fem`,
/// `anchor_cracked`) are exercised too.
pub const EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🛢️liquid-retaining-fem-anchor/🖼️assets/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio");

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📖️ Parses `.en1992` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1992Snapshot, store::TextError> {
    <En1992Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1992` DSL text.
pub fn print_dsl(document: &En1992Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1992SnapshotText = String;
//#endregion 🚚️Carrier
