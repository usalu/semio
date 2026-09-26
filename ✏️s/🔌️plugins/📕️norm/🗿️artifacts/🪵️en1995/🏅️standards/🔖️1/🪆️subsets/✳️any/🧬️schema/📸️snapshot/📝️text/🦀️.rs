//! 📜️ EN 1995 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1995Snapshot;

/// 🗄️ The glulam-footbridge example fixture in `en1995`'s DSL (`store::ArtifactDsl`): a DE-annex EN 1995-2
/// glulam pedestrian footbridge girder (role `bridge`, service class 2, Annex A fatigue and Annex B comfort inputs,
/// no connections) — distinct from `En1995Snapshot::default()`'s floor beam so the bridge branches are exercised.
pub const EN1995_GLULAM_FOOTBRIDGE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🌉️glulam-footbridge/🌉️glulam-footbridge/🗣️.dsl.semio");

/// 📖️ Parses `.en1995` DSL text into a `En1995Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1995Snapshot, store::TextError> {
    <En1995Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1995Snapshot` back to `.en1995` DSL text.
pub fn print_dsl(document: &En1995Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1995SnapshotText = String;
//#endregion 🚚️Carrier
