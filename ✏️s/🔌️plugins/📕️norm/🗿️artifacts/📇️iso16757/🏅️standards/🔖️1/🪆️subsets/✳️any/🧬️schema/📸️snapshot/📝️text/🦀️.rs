//! 📜️ ISO 16757 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Iso16757Snapshot;

/// 📄️ The `default` example document, handcrafted in the `.iso16757` DSL — a demo HVAC catalogue
/// worked example (control valve product group/class/series/product/variant, ISO 16757-4 dictionary
/// subject/property/controlled list, a box-primitive geometry with an inlet port, a selection
/// request, and a scripted part-number rule), mirroring `Document::reference_fixture()`.
pub const ISO16757_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.iso16757` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Iso16757Snapshot, store::TextError> {
    <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.iso16757` DSL text.
pub fn print_dsl(document: &Iso16757Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Iso16757SnapshotText = String;
//#endregion 🚚️Carrier
