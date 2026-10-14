//! 📜️ EN 1995 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_norm_en1995_model::En1995Snapshot;

/// 🗄️ The glulam-footbridge example fixture in `en1995`'s DSL (`pack::snapshot::ArtifactDsl`): a DE-annex EN 1995-2
/// glulam pedestrian footbridge girder (role `bridge`, service class 2, Annex A fatigue and Annex B comfort inputs,
/// no connections) — distinct from `En1995Snapshot::default()`'s floor beam so the bridge branches are exercised.
pub const EN1995_GLULAM_FOOTBRIDGE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🌉️glulam-footbridge/🌉️glulam-footbridge/🗣️.dsl.semio");

/// 📖️ Parses `.en1995` DSL text into a `En1995Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1995Snapshot, semio_framework_diagnostic::TextError> {
    <En1995Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1995Snapshot` back to `.en1995` DSL text.
pub fn print_dsl(document: &En1995Snapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
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

#[allow(unused_imports)]
mod snapshot_codec {
use crate::text::snapshot::*;
use semio_s_artifact_norm_en1995_model::*;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1995_model::{CharacteristicAction, ConnectionAction, MemberRole, SupportType, TimberConnection, TimberMember};
use framework_schema::ArtifactSchema;

pub fn encode_en1995_snapshot_json(snapshot: &En1995Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

pub fn decode_en1995_snapshot_json(text: &str) -> Result<En1995Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub fn decode_en1995_dsl(text: &str) -> Result<En1995Snapshot, String> {
    <En1995Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

pub fn encode_en1995_dsl(snapshot: &En1995Snapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(snapshot)
}
}
pub use crate::text::snapshot::snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::text::snapshot::*;
use semio_s_artifact_norm_en1995_model::*;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1995_model::{CharacteristicAction, ConnectionAction, MemberRole, SupportType, TimberConnection, TimberMember};
use framework_schema::ArtifactSchema;

}
#[cfg(test)] pub use crate::text::snapshot::snapshot_wire_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@text En1995Snapshot, extension="en1995", envelope_id="norm.en1995");
