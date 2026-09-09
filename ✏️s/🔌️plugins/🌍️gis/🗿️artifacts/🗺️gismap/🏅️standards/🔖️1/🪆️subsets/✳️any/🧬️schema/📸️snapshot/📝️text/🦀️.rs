//! 📜️ GIS map artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::GisMapSnapshot;

/// 🗺️ The bundled "reuse map" example document, handcrafted in the `.gismap` DSL.
pub const REUSE_MAP_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.gismap` DSL text into a `GisMapSnapshot`.
pub fn parse_dsl(text: &str) -> Result<GisMapSnapshot, store::TextError> {
    <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `GisMapSnapshot` back to `.gismap` DSL text.
pub fn print_dsl(document: &GisMapSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GisMapSnapshotText = String;
//#endregion 🚚️Carrier
