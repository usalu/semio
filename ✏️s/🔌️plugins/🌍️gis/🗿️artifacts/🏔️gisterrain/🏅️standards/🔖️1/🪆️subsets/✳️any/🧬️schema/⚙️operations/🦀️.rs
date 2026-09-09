//! ⚙️ GIS terrain mutation application, store aliases, codec bridge, and behavior tests.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("../🧬️mutations/📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use dsl::ToValue;
use protocol::Mutation;
use store::{ArtifactEnvelope, ArtifactStore};

pub type GisTerrainEnvelope = ArtifactEnvelope<GisTerrainSnapshot, GisTerrainMutation>;
pub type GisTerrainStore = ArtifactStore<GisTerrainSnapshot, GisTerrainMutation>;

//#region 🔹Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔹Tests

pub fn apply_gis_terrain_mutation(snapshot: &mut GisTerrainSnapshot, mutation: &GisTerrainMutation) -> protocol::MutationApplyResult<()> {
    let (next, _messages) = vcs::apply_mutation(snapshot, mutation)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_gis_terrain_mutation(snapshot: &GisTerrainSnapshot, mutation: &GisTerrainMutation) -> Vec<GisTerrainMutation> {
    mutation.inverse(snapshot)
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `GisTerrainMutation` variant, in declaration order — the vocabulary the `gisterrain-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `🏔️mutate-gisterrain-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &["change-exaggeration", "change-imported-features"];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `GisTerrainMutation` nor
/// `GisTerrainSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
/// Committed snapshots are decoded exactly, including their durable mesh handles.
///
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn gis_terrain_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<GisTerrainSnapshot, String> {
        dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: GisTerrainMutation = dsl::os_pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <GisTerrainMutation as Mutation<GisTerrainSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <GisTerrainMutation as Mutation<GisTerrainSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <GisTerrainMutation as Mutation<GisTerrainSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = dsl::os_pack::json::object([
        ("base".to_string(), dsl::os_pack::json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), dsl::os_pack::json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), dsl::os_pack::json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), dsl::os_pack::json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), dsl::os_pack::json::from_dsl_value(&forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), dsl::os_pack::json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), dsl::os_pack::json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), dsl::os_pack::json::from_dsl_value(&inverse_messages.to_value())),
    ]);
    Ok(dsl::os_pack::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
