//! 📜️ GIS terrain artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::GisTerrainSnapshot;

/// 🏔️ The bundled "reuse terrain" example document, handcrafted in the `.gisterrain` DSL.
pub const REUSE_TERRAIN_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.gisterrain` DSL text into a `GisTerrainSnapshot`.
pub fn parse_dsl(text: &str) -> Result<GisTerrainSnapshot, store::TextError> {
    <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `GisTerrainSnapshot` back to `.gisterrain` DSL text.
pub fn print_dsl(document: &GisTerrainSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GisTerrainSnapshotText = String;
//#endregion 🚚️Carrier
