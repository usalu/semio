//! 📜️ EN 1990 basis of structural design — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1990Snapshot;

/// 🏢️ The high-consequence-office example fixture, handcrafted in `en1990`'s DSL
/// (`store::ArtifactDsl`): a CC3 (high-consequence) office building basis-of-design check with
/// three variable-action entries under the EN annex and the seismic accidental action disabled —
/// distinct from `En1990Snapshot::default()`'s CC2/DE-annex/seismic-enabled values so the grammar's
/// non-default branches (consequence class, annex, `q_k` table cardinality) are exercised too.
pub const EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏢️high-consequence-office/🖼️assets/🏢️high-consequence-office/🗣️.dsl.semio");

/// 📖️ Parses `.en1990` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1990Snapshot, store::TextError> {
    <En1990Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1990` DSL text.
pub fn print_dsl(document: &En1990Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
