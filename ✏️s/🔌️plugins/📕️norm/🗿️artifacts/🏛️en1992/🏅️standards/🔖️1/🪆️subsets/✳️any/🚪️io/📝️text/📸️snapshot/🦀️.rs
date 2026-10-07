//! 📜️ EN 1992 design of concrete structures — textual document grammar surface + laws (constitutional: dsl).

use crate::En1992Snapshot;

/// 💧️ The liquid-retaining-fem-anchor example fixture, handcrafted in `en1992`'s DSL
/// (`store::ArtifactDsl`): a liquid-retaining structure (EN 1992-3 tightness class TC2) section
/// checked with a FEM-based analysis, an R90 fire rating, and a post-installed anchor in cracked
/// concrete, under the EN annex — distinct from `En1992Snapshot::default()`'s DE-annex/TC1/R60/uncracked
/// values so the grammar's non-default branches (annex, fire rating, tightness class, `use_fem`,
/// `anchor_cracked`) are exercised too.
pub const EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio");

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📖️ Parses `.en1992` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1992Snapshot, semio_framework_diagnostic::TextError> {
    <En1992Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1992` DSL text.
pub fn print_dsl(document: &En1992Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1992SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{
    Anchor, BarLayer, ConcreteGrade, ExposureClass, FireRating, FireSpec, LoadCaseActions, MemberKind, PrestressSpec, PrestressSteel, PunchingSpec, RcMember, ReinforcementGrade, Stirrups, SupportCondition,
    TightnessClass,
};
use framework_schema::ArtifactSchema;

/// 📤️ Canonical JSON projection of [`En1992Snapshot`].
pub fn encode_en1992_snapshot_json(snapshot: &En1992Snapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_en1992_snapshot_json`].
pub fn decode_en1992_snapshot_json(text: &str) -> Result<En1992Snapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📖️ Parses committed `.dsl.semio` into [`En1992Snapshot`].
pub fn decode_en1992_dsl(text: &str) -> Result<En1992Snapshot, String> {
    <En1992Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints [`En1992Snapshot`] to canonical `.dsl.semio`.
pub fn encode_en1992_dsl(snapshot: &En1992Snapshot) -> String {
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
    Anchor, BarLayer, ConcreteGrade, ExposureClass, FireRating, FireSpec, LoadCaseActions, MemberKind, PrestressSpec, PrestressSteel, PunchingSpec, RcMember, ReinforcementGrade, Stirrups, SupportCondition,
    TightnessClass,
};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;

crate::impl_norm_artifact_record!(@text crate::En1992Snapshot, extension="en1992", envelope_id="norm.en1992");
