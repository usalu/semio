//! 📜️ Playground artifact — textual document grammar surface + laws.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;

/// 📄️ The `demo` example checkpoint.
pub const PLAYGROUND_DEMO_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses playground DSL text into a `PlaygroundSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PlaygroundSnapshot, store::TextError> {
    <PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PlaygroundSnapshot` back to DSL text.
pub fn print_dsl(snapshot: &PlaygroundSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PlaygroundSnapshotText = String;
//#endregion 🚚️Carrier
