//! 📜️ `s.wfc.grid3d` snapshot — textual document grammar surface + laws (constitutional: dsl).
//! Every record of this document is a local type that already derives `dsl::DslRecord`, so there is
//! no twin mirror here: the codecs live on `Grid3dSnapshot` itself (sibling `🦀️.rs`) and this leaf
//! carries the normative grammar plus the facet's parse/print entry points.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::snapshot::Grid3dSnapshot;

//#region 🔖️Examples
/// 📄️ The two authored tile sets this subset ships, in their own `.wfcgrid3d` DSL.
pub const GRID3D_EXAMPLE_BLOCKS_TEXT: &str = include_str!("../../../📚️examples/🧱️blocks/🖼️assets/🧱️blocks/🗣️.dsl.semio");
pub const GRID3D_EXAMPLE_PIPES_TEXT: &str = include_str!("../../../📚️examples/🪠️pipes-3d/🖼️assets/🪠️pipes-3d/🗣️.dsl.semio");
//#endregion 🔖️Examples

/// 📖️ Parses `.wfcgrid3d` DSL text into a `Grid3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Grid3dSnapshot, store::TextError> {
    <Grid3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Grid3dSnapshot` back to `.wfcgrid3d` DSL text.
pub fn print_dsl(document: &Grid3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_dsl`/`print_dsl` speak.
pub type Grid3dSnapshotTextCarrier = String;
//#endregion 🚚️Carrier
