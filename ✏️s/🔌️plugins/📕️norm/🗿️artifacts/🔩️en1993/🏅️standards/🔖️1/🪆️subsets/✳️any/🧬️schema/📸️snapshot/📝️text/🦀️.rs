//! 📜️ EN 1993 design of steel structures — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1993Snapshot;

/// 🔩️ The high-strength-connection example fixture, handcrafted in `en1993`'s DSL
/// (`store::ArtifactDsl`): an S460 high-strength steel member and bolted/welded connection
/// worked example (4×M24 grade 10.9 bolts, safe-life fatigue assessment, subgrade K2 toughness)
/// under the EN annex — distinct from `En1993Snapshot::default()`'s DE-annex/S355/2-bolt/damage-tolerant
/// values so the grammar's non-default branches (annex, bolt count, fatigue method, HSS section
/// class) are exercised too.
pub const EN1993_HIGH_STRENGTH_CONNECTION_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🔩️high-strength-connection/🖼️assets/🔩️high-strength-connection/🗣️.dsl.semio");

/// 📖️ Parses `.en1993` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1993Snapshot, store::TextError> {
    <En1993Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1993` DSL text.
pub fn print_dsl(document: &En1993Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1993SnapshotText = String;
//#endregion 🚚️Carrier
