//! 📜️ EN 1999 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1999Snapshot;

/// 🗄️ The aluminium-roof-purlin example fixture, handcrafted in `en1999`'s DSL (`store::ArtifactDsl`):
/// a welded AW-6082-T6 aluminium roof purlin under the EN annex, exercising the higher-strength alloy's
/// cross-section, buckling, bending, fatigue, welded-joint, cold-formed-sheeting, and shell-buckling
/// checks together, distinct from `En1999Snapshot::default()`'s AW-6060-T6/DE-annex values so the grammar's
/// non-default branches (alloy, annex) are exercised too.
pub const EN1999_ALUMINIUM_ROOF_PURLIN_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio");

/// 📖️ Parses `.en1999` DSL text into a `En1999Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1999Snapshot, semio_framework_diagnostic::TextError> {
    <En1999Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1999Snapshot` back to `.en1999` DSL text.
pub fn print_dsl(document: &En1999Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1999SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use framework_schema::ArtifactSchema;

pub fn encode_en1999_snapshot_json(snapshot: &En1999Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

pub fn decode_en1999_snapshot_json(text: &str) -> Result<En1999Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub fn decode_en1999_dsl(text: &str) -> Result<En1999Snapshot, String> {
    <En1999Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

pub fn encode_en1999_dsl(snapshot: &En1999Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
