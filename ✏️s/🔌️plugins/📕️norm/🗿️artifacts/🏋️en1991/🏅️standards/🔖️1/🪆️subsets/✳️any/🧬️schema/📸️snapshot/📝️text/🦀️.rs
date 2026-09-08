//! 📜️ EN 1991 actions on structures — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1991Snapshot;

/// 🏬️ The retail-hydrocarbon-fire example fixture, handcrafted in `en1991`'s DSL
/// (`store::ArtifactDsl`): a retail unit (imposed category D) evaluated under the EN annex with a
/// hydrocarbon fire curve and a full set of the other action sub-scenarios (snow, wind, thermal,
/// construction, accidental impact, bridge, crane, silo) at plausible non-zero values — distinct
/// from `En1991Snapshot::default()`'s category-B/DE-annex/standard-fire-curve values so the grammar's
/// non-default branches (category, annex, fire curve) are exercised too.
pub const EN1991_RETAIL_HYDROCARBON_FIRE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🔥️retail-hydrocarbon-fire/🖼️assets/🔥️retail-hydrocarbon-fire/🗣️.dsl.semio");

/// 📖️ Parses `.en1991` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1991Snapshot, store::TextError> {
    <En1991Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1991` DSL text.
pub fn print_dsl(document: &En1991Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
