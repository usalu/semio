//! 📜️ EN 1993 design of steel structures — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::En1993Snapshot;

/// 🔩️ The high-strength-connection example fixture, handcrafted in `en1993`'s DSL
/// (`store::ArtifactDsl`): an S460 high-strength steel member and bolted/welded connection
/// worked example (4×M24 grade 10.9 bolts, safe-life fatigue assessment, subgrade K2 toughness)
/// under the EN annex — distinct from `En1993Snapshot::default()`'s DE-annex/S355/2-bolt/damage-tolerant
/// values so the grammar's non-default branches (annex, bolt count, fatigue method, HSS section
/// class) are exercised too.
pub const EN1993_HIGH_STRENGTH_CONNECTION_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio");

/// 📖️ Parses `.en1993` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1993Snapshot, semio_framework_diagnostic::TextError> {
    <En1993Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1993` DSL text.
pub fn print_dsl(document: &En1993Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1993SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{
    BridgeFatigue, ColdFormedMember, CraneRunway, DesignAction, FatigueBand, FatigueDetail, FireExposure, ForceAction, JointForceAction, LoadCase,
    MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial, SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg,
};
use framework_schema::ArtifactSchema;

/// 📤️ The canonical JSON projection of a [`En1993Snapshot`] — the surface
/// `../../../../../🧪️tests/🔩️mutate-en1993-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_snapshot_json(snapshot: &En1993Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1993_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1993Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_snapshot_json(text: &str) -> Result<En1993Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1993Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_dsl(text: &str) -> Result<En1993Snapshot, String> {
    <En1993Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1993Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_dsl(snapshot: &En1993Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{
    BridgeFatigue, ColdFormedMember, CraneRunway, DesignAction, FatigueBand, FatigueDetail, FireExposure, ForceAction, JointForceAction, LoadCase,
    MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial, SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg,
};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;

crate::impl_norm_artifact_record!(@text crate::En1993Snapshot, extension="en1993", envelope_id="norm.en1993");
