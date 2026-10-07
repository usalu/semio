//! 📜️ EN 1991 actions on structures — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1991Snapshot;

/// 🏬️ The de-office-compliant example fixture, handcrafted in `en1991`'s DSL
/// (`store::ArtifactDsl`): a DE office (design-load assumptions) evaluated under the EN annex with a
/// DE annex snow/wind/imposed and a full set of the other action sub-scenarios (snow, wind, thermal,
/// construction, accidental impact, bridge, crane, silo) at plausible non-zero values — distinct
/// from `En1991Snapshot::default()`'s category-B/DE-annex/standard-fire-curve values so the grammar's
/// non-default branches (category, annex, fire curve) are exercised too.
pub const EN1991_DE_OFFICE_COMPLIANT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio");

/// 📖️ Parses `.en1991` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1991Snapshot, semio_framework_diagnostic::TextError> {
    <En1991Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1991` DSL text.
pub fn print_dsl(document: &En1991Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1991SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{AccidentalCase, FloorArea, RoofArea, SelfWeightElement, WindFace};
use framework_schema::ArtifactSchema;

/// 📤️ Canonical JSON projection of [`En1991Snapshot`].
pub fn encode_en1991_snapshot_json(snapshot: &En1991Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_en1991_snapshot_json`].
pub fn decode_en1991_snapshot_json(text: &str) -> Result<En1991Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📖️ Parse committed `.dsl.semio` into [`En1991Snapshot`].
pub fn decode_en1991_dsl(text: &str) -> Result<En1991Snapshot, String> {
    <En1991Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Print [`En1991Snapshot`] to canonical `.dsl.semio`.
pub fn encode_en1991_dsl(snapshot: &En1991Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{AccidentalCase, FloorArea, RoofArea, SelfWeightElement, WindFace};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;

crate::impl_norm_artifact_record!(@text crate::En1991Snapshot, extension="en1991", envelope_id="norm.en1991");
