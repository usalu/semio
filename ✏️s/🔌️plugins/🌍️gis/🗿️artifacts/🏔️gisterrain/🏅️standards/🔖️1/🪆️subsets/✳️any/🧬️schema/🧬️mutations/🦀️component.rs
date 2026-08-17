//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::schema::mutations::{change_exaggeration, change_imported_features};
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔹Operation
/// 🗺️ Typed, invertible, semantic terrain mutation vocabulary — every variant wraps exactly one
/// `protocol::MutationKind` payload struct declared in its own `🧬️mutations/<kind>/🦠️mutation`
/// triad leaf; `#[derive(dsl::Mutations)]` wires `Mutation`/`SemanticMutation` from those leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[mutations(snapshot = GisTerrainSnapshot, diff = GisTerrainDiff, schema = "gis.gisterrain")]
pub enum GisTerrainMutation {
    ChangeExaggeration(change_exaggeration::mutation::ChangeExaggeration),
    ChangeImportedFeatures(change_imported_features::mutation::ChangeImportedFeatures),
}

pub type GisTerrainEnvelope = ArtifactEnvelope<GisTerrainSnapshot, GisTerrainMutation>;
pub type GisTerrainStore = ArtifactStore<GisTerrainSnapshot, GisTerrainMutation>;
//#endregion 🔹Operation

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;
    use change_exaggeration::mutation::ChangeExaggeration;
    use change_imported_features::mutation::ChangeImportedFeatures;
    use protocol::MutationDiff;

    #[test]
    fn change_exaggeration_and_change_imported_features_invert_to_the_prior_field_value() {
        let snapshot = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into(), ..Default::default() };
        assert_eq!(
            GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 9.0 }).inverse(&snapshot),
            vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 1.5 })]
        );
        assert_eq!(
            GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "{}".into() }).inverse(&snapshot),
            vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "null".into() })]
        );
    }

    #[test]
    fn change_exaggeration_obeys_the_inverse_and_diff_absorb_laws() {
        let base = crate::artifacts::gisterrain::gis_terrain_snapshot_with_derived_mesh(GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into(), ..Default::default() });
        let mutation = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 4.0 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 8.0 }).diff(&base).into_parts().0;
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn change_imported_features_obeys_the_inverse_law() {
        let base = crate::artifacts::gisterrain::gis_terrain_snapshot_with_derived_mesh(GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: "null".into(), ..Default::default() });
        let mutation = GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: "{}".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
    }
}
//#endregion 🔹Tests

pub fn apply_gis_terrain_mutation(
    snapshot: &mut GisTerrainSnapshot,
    mutation: &GisTerrainMutation,
) -> protocol::MutationApplyResult<()> {
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
