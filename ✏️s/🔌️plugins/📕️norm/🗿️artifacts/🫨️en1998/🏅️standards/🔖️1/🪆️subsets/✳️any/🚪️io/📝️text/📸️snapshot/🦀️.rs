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
pub fn parse_dsl(text: &str) -> Result<En1998Snapshot, semio_framework_diagnostic::TextError> {
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

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{
    DeGroundCombo, DeSeismicZone, En1998Assessment, En1998Bridge, En1998Building, En1998Foundation, En1998RetainingWall, En1998Site, En1998Silo,
    En1998Storey, En1998VariableAction, En1998System, En1998Tank, En1998Tower, En1998Member,
};
use framework_schema::ArtifactSchema;

pub fn encode_en1998_snapshot_json(snapshot: &En1998Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

pub fn decode_en1998_snapshot_json(text: &str) -> Result<En1998Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub fn decode_en1998_dsl(text: &str) -> Result<En1998Snapshot, String> {
    <En1998Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

pub fn encode_en1998_dsl(snapshot: &En1998Snapshot) -> String {
    <En1998Snapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{
    DeGroundCombo, DeSeismicZone, En1998Assessment, En1998Bridge, En1998Building, En1998Foundation, En1998RetainingWall, En1998Site, En1998Silo,
    En1998Storey, En1998VariableAction, En1998System, En1998Tank, En1998Tower, En1998Member,
};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;

crate::impl_norm_artifact_record!(@text crate::En1998Snapshot, extension="en1998", envelope_id="norm.en1998");
