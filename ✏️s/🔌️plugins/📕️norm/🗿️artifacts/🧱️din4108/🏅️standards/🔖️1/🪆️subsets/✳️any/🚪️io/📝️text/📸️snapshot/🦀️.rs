//! 📜️ DIN 4108 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

/// 📜️ Bundled default example document (`.semio` envelope + DSL body).
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses DIN 4108 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Din4108Snapshot, semio_framework_diagnostic::TextError> {
    <Din4108Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.din4108` DSL text.
pub fn print_dsl(document: &Din4108Snapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Din4108SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use crate::text::snapshot::*;
use semio_s_artifact_norm_din4108_model::*;
use semio_s_plugin_norm_compliance::ClimateZoneDe;
use semio_s_artifact_norm_din4108_model::{EnvelopeElement, LayerDocument, LayerSegment, ThermalBridge, ThermalZone, ZoneWindow};
use framework_schema::ArtifactSchema;

/// 📤️ Canonical JSON projection of a [`Din4108Snapshot`].
pub fn encode_din4108_snapshot_json(snapshot: &Din4108Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_din4108_snapshot_json`].
pub fn decode_din4108_snapshot_json(text: &str) -> Result<Din4108Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📖️ Parses a committed `.dsl.semio` artifact into a [`Din4108Snapshot`].
pub fn decode_din4108_dsl(text: &str) -> Result<Din4108Snapshot, String> {
    <Din4108Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Din4108Snapshot`] to its canonical `.dsl.semio` body.
pub fn encode_din4108_dsl(snapshot: &Din4108Snapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(snapshot)
}
}
pub use crate::text::snapshot::snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::text::snapshot::*;
use semio_s_artifact_norm_din4108_model::*;
use semio_s_plugin_norm_compliance::ClimateZoneDe;
use semio_s_artifact_norm_din4108_model::{EnvelopeElement, LayerDocument, LayerSegment, ThermalBridge, ThermalZone, ZoneWindow};
use framework_schema::ArtifactSchema;

}
#[cfg(test)] pub use crate::text::snapshot::snapshot_wire_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@text Din4108Snapshot, extension="din4108", envelope_id="norm.din4108");
