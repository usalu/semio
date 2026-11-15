//! 📜️ DIN V 18599 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

/// 📜️ Bundled default example document (`.semio` envelope + DSL body).
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses DIN V 18599 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Din18599Snapshot, semio_framework_diagnostic::TextError> {
    <Din18599Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.din18599` DSL text.
pub fn print_dsl(document: &Din18599Snapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Din18599SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use crate::snapshot::*;
use semio_s_artifact_norm_din18599_model::*;
use semio_s_artifact_norm_din18599_model::{Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, ElementKind, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem};
use framework_schema::ArtifactSchema;

/// 📤️ The canonical JSON projection of a [`Din18599Snapshot`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_snapshot_json(snapshot: &Din18599Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_din18599_snapshot_json`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_snapshot_json(text: &str) -> Result<Din18599Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`Din18599Snapshot`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_dsl(text: &str) -> Result<Din18599Snapshot, String> {
    <Din18599Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Din18599Snapshot`] back to its canonical `.dsl.semio` body.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_dsl(snapshot: &Din18599Snapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(snapshot)
}
}
pub use crate::snapshot::snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::snapshot::*;
use semio_s_artifact_norm_din18599_model::*;
use semio_s_artifact_norm_din18599_model::{Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, ElementKind, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem};
use framework_schema::ArtifactSchema;

}
#[cfg(test)] pub use crate::snapshot::snapshot_wire_codec::*;