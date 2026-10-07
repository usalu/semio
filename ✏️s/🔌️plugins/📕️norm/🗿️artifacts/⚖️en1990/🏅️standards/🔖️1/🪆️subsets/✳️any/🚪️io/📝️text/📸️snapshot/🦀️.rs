//! 📜️ EN 1990 basis of structural design — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1990Snapshot;

/// 🏢️ The high-consequence-office example fixture, handcrafted in `en1990`'s DSL
/// (`store::ArtifactDsl`): a CC3 (high-consequence) office building basis-of-design check with
/// three variable-action entries under the EN annex and the seismic accidental action disabled —
/// distinct from `En1990Snapshot::default()`'s CC2/DE-annex/seismic-enabled values so the grammar's
/// non-default branches (consequence class, annex, `q_k` table cardinality) are exercised too.
pub const EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🏢️high-consequence-office/🏢️high-consequence-office/🗣️.dsl.semio");

/// 📖️ Parses `.en1990` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1990Snapshot, semio_framework_diagnostic::TextError> {
    <En1990Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1990` DSL text.
pub fn print_dsl(document: &En1990Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1990SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{AccidentalAction, BridgeSls, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};
use framework_schema::ArtifactSchema;

/// 📤️ Canonical JSON projection of [`En1990Snapshot`].
pub fn encode_en1990_snapshot_json(snapshot: &En1990Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_en1990_snapshot_json`].
pub fn decode_en1990_snapshot_json(text: &str) -> Result<En1990Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📖️ Parses committed `.dsl.semio` into [`En1990Snapshot`].
pub fn decode_en1990_dsl(text: &str) -> Result<En1990Snapshot, String> {
    <En1990Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints [`En1990Snapshot`] to canonical `.dsl.semio`.
pub fn encode_en1990_dsl(snapshot: &En1990Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{AccidentalAction, BridgeSls, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;

crate::impl_norm_artifact_record!(@text crate::En1990Snapshot, extension="en1990", envelope_id="norm.en1990");
