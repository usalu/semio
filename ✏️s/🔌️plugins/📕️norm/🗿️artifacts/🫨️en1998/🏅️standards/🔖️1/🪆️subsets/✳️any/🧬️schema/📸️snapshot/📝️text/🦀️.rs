//! 📜️ EN 1998 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1998Snapshot;

/// 🗄️ The seismic-rc-frame example fixture, handcrafted in `en1998`'s DSL (`store::ArtifactDsl`): a
/// high-importance dual-system RC building in seismic zone 3 on ground type D, resolved under the EN
/// annex's Type 2 spectrum on EN ground type C, with an isolated-bridge bearing check, a near-collapse
/// KL3 retrofit assessment, and companion silo/tank/tower/foundation/retaining-wall subsystem checks —
/// distinct from `En1998Snapshot::default()`'s DE-annex/CC2/moment-frame/KL2/significant-damage values so the
/// grammar's non-default branches (annex, importance class, structural system, ground types, spectrum
/// type, retrofit knowledge level and limit state, redundancy and chimney booleans) are exercised too.
pub const EN1998_SEISMIC_RC_FRAME_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio");

/// 📖️ Parses `.en1998` DSL text into a `En1998Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1998Snapshot, store::TextError> {
    <En1998Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1998Snapshot` back to `.en1998` DSL text.
pub fn print_dsl(document: &En1998Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1998SnapshotText = String;
//#endregion 🚚️Carrier
