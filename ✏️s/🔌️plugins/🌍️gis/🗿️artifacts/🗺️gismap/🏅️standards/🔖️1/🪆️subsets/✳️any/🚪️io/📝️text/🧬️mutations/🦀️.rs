//! ⚡️ GIS map artifact — OpText/OpBinary codecs + grammar for `GisMapMutation`.

use crate::schema::mutations::{inverse_gis_map_mutation, GisMapMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits). `GisMapMutation`
/// derives `dsl::DslEnum` directly (every variant wraps a local `dsl::DslRecord` payload, no more
/// foreign `protocol::CollectionMutation` in its shape), so this is a pure `DslVariants` pass-through
/// — no local DSL-mirror type needed, unlike the pre-taxonomy-overhaul version of this file.
impl protocol::OpText for GisMapMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}


//#endregion 🔖️HandcraftedOpCodecs

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::GisMapDiff;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data};
use crate::GisMapSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use protocol::Mutation;
use store::ArtifactEnvelope;

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `GisMapMutation` nor
/// `GisMapSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
/// 🧩️ Committed snapshots include the stable drawing and value child identities. The report
/// decodes them directly, so comparisons validate those identities alongside feature content.
///
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn gis_map_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<GisMapSnapshot, String> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: GisMapMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let forward = <GisMapMutation as Mutation<GisMapSnapshot>>::diff(&mutation, &base);
    let applied = protocol::apply_diff(forward.diff(), &base).map_err(|error| error.to_string())?;
    let inverse = <GisMapMutation as Mutation<GisMapSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in inverse.iter().rev() {
        let outcome = <GisMapMutation as Mutation<GisMapSnapshot>>::diff(step, &undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
        undone = protocol::apply_diff(outcome.diff(), &undone).map_err(|error| error.to_string())?;
    }
    let report = semio_framework_pack_json::object([
        ("base".to_string(), semio_framework_pack_json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), semio_framework_pack_json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), semio_framework_pack_json::from_dsl_value(&forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), semio_framework_pack_json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), semio_framework_pack_json::from_dsl_value(&inverse_messages.to_value())),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use mutations_codec::*;

//#region 🌉️Apply
/// 🕸️ Applies one parent mutation while preserving the stable drawing/value member coordinates — the central-apply entry
/// point of the text/native bridge, kept outside the schema tree: only editors, io and stores call the central applier.
pub fn apply_gis_map_mutation(snapshot: &mut crate::GisMapSnapshot, mutation: &GisMapMutation) -> protocol::MutationApplyResult<()> {
    let (next, _messages) = vcs::apply_mutation(snapshot, mutation)?;
    *snapshot = next;
    Ok(())
}
//#endregion 🌉️Apply
