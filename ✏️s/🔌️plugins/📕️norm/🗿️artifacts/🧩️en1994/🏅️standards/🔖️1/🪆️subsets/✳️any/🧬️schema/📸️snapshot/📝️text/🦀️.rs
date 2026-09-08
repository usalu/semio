//! 📜️ EN 1994 design of composite steel and concrete structures — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1994Snapshot;

/// 🗄️ The composite-bridge-girder example fixture, handcrafted in `en1994`'s DSL (`store::ArtifactDsl`):
/// an EN-annex EN 1994-2 composite bridge girder with a re-entrant deck under an R90 fire rating and a
/// shear-connector fatigue detail, distinct from `En1994Snapshot::default()`'s DE-annex/R60/trapezoidal-deck/
/// stud-welded values so the grammar's non-default branches (annex, fire rating, deck type, fatigue
/// detail) are exercised too.
pub const EN1994_COMPOSITE_BRIDGE_GIRDER_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌉️composite-bridge-girder/🖼️assets/🌉️composite-bridge-girder/🗣️.dsl.semio");

/// 📖️ Parses `.en1994` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1994Snapshot, store::TextError> {
    <En1994Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1994` DSL text.
pub fn print_dsl(document: &En1994Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1994SnapshotText = String;
//#endregion 🚚️Carrier
