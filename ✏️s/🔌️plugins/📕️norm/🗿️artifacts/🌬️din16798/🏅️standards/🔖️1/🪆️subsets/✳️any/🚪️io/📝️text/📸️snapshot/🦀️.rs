//! 📜️ DIN EN 16798 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Din16798Snapshot;

/// 📜️ Bundled default example document (`.semio` envelope + DSL body).
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses DIN EN 16798 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Din16798Snapshot, semio_framework_diagnostic::TextError> {
    <Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.din16798` DSL text.
pub fn print_dsl(document: &Din16798Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Din16798SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;

pub fn encode_din16798_snapshot_json(snapshot: &Din16798Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

pub fn decode_din16798_snapshot_json(text: &str) -> Result<Din16798Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub fn decode_din16798_dsl(text: &str) -> Result<Din16798Snapshot, String> {
    <Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

pub fn encode_din16798_dsl(snapshot: &Din16798Snapshot) -> String {
    <Din16798Snapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
