//! 📜️ Shooting artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::ArtifactDsl for ShootingSnapshot` is implemented directly on the artifact type (see
//! `🗿️artifacts/🎥️shooting/🦀️.rs`'s doc comment for why). This component only adds the thin
//! artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture constant and its
//! round-trip law.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::ShootingSnapshot;

/// 🗄️ The base-icon example snapshot, handcrafted in `shooting`'s DSL (`store::ArtifactDsl`).
pub const SHOOTING_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.shooting` DSL text into a `ShootingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `ShootingSnapshot` back to `.shooting` DSL text.
pub fn print_dsl(snapshot: &ShootingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ShootingSnapshotText = String;
//#endregion 🚚️Carrier
