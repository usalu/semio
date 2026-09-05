//! ⚙️ GIS terrain mutation application, store aliases, codec bridge, and behavior tests.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("../🧬️mutations/📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::gisterrain::schema::mutations::{ChangeExaggeration, ChangeImportedFeatures, GisTerrainMutation};
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use dsl::ToValue;
use protocol::Mutation;
use store::{ArtifactEnvelope, ArtifactStore};

pub type GisTerrainEnvelope = ArtifactEnvelope<GisTerrainSnapshot, GisTerrainMutation>;
pub type GisTerrainStore = ArtifactStore<GisTerrainSnapshot, GisTerrainMutation>;

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[semio_framework_async_macros::async_test]
    async fn change_exaggeration_and_change_imported_features_invert_to_the_prior_field_value() {
        let snapshot = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into(), ..Default::default() };
        assert_eq!(GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 9.0 }).inverse(&snapshot), vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 1.5 })]);
        assert_eq!(
            GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "{}".into() }).inverse(&snapshot),
            vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "null".into() })]
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn change_exaggeration_obeys_the_inverse_and_diff_absorb_laws() {
        let base = crate::artifacts::gisterrain::gis_terrain_snapshot_with_derived_mesh(GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into(), ..Default::default() });
        let mutation = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 4.0 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 8.0 }).diff(&base).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn change_imported_features_obeys_the_inverse_law() {
        let base = crate::artifacts::gisterrain::gis_terrain_snapshot_with_derived_mesh(GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: "null".into(), ..Default::default() });
        let mutation = GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "{}".into() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    }
}
//#endregion 🔹Tests

pub fn apply_gis_terrain_mutation(snapshot: &mut GisTerrainSnapshot, mutation: &GisTerrainMutation) -> protocol::MutationApplyResult<()> {
    let (next, _messages) = vcs::apply_mutation(snapshot, mutation)?;
    // 🕸️ `mesh` is a pure function of `(exaggeration, imported_features_json)` — re-derive it after
    // every mutation so the composed child handle never drifts from what
    // `gis_terrain_mesh_from_snapshot` would actually build (see `GisTerrainSnapshot.mesh`'s doc).
    *snapshot = crate::artifacts::gisterrain::gis_terrain_snapshot_with_derived_mesh(next);
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
/// Every committed snapshot is funnelled through `gis_terrain_snapshot_with_derived_mesh` on the way
/// in — the same call `Default`, `apply_gis_terrain_mutation` and `GisTerrainDiff::apply` each make —
/// because the committed vectors carry a placeholder mesh handle rather than a frozen digest.
/// Funnelling BOTH the base and the expected after-snapshot keeps the comparison exact.
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
        let decoded: GisTerrainSnapshot = dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(crate::artifacts::gisterrain::gis_terrain_snapshot_with_derived_mesh(decoded))
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
mod kinds_conformance {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
    /// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
    /// honest in both directions at once.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <GisTerrainMutation as protocol::SemanticMutation<GisTerrainSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🔮️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance
