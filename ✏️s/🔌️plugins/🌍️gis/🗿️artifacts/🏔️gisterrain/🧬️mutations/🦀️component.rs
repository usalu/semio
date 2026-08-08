//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gisterrain::diff::{diff_exaggeration, diff_imported_features_json, diff_set_snapshot, GisTerrainDiff};
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔹Operation
/// 🗺️ Typed, invertible terrain operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GisTerrainMutation {
    SetExaggeration {
        exaggeration: f64,
    },
    SetImportedFeatures {
        #[dsl(key = "features-json")]
        features_json: String,
    },
    SetSnapshot {
        #[dsl(block)]
        snapshot: GisTerrainSnapshot,
    },
}

impl Mutation<GisTerrainSnapshot> for GisTerrainMutation {
    type Diff = GisTerrainDiff;

    fn diff(&self, _snapshot: &GisTerrainSnapshot) -> GisTerrainDiff {
        match self {
            GisTerrainMutation::SetExaggeration { exaggeration } => diff_exaggeration(*exaggeration),
            GisTerrainMutation::SetImportedFeatures { features_json } => diff_imported_features_json(features_json.clone()),
            GisTerrainMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &GisTerrainSnapshot) -> Vec<Self> {
        match self {
            GisTerrainMutation::SetExaggeration { .. } => vec![GisTerrainMutation::SetExaggeration { exaggeration: snapshot.exaggeration }],
            GisTerrainMutation::SetImportedFeatures { .. } => vec![GisTerrainMutation::SetImportedFeatures { features_json: snapshot.imported_features_json.clone() }],
            GisTerrainMutation::SetSnapshot { .. } => vec![GisTerrainMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

pub type GisTerrainEnvelope = DocumentEnvelope<GisTerrainSnapshot, GisTerrainMutation>;
pub type GisTerrainStore = DocumentStore<GisTerrainSnapshot, GisTerrainMutation>;
//#endregion 🔹Operation

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn gis_terrain_set_snapshot_backwards_restores_the_exact_prior_snapshot() {
        let snapshot = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into() };
        let operation = GisTerrainMutation::SetSnapshot { snapshot: GisTerrainSnapshot { exaggeration: 4.0, imported_features_json: r#"{"positions":[]}"#.into() } };
        let next = operation.diff(&snapshot).apply(&snapshot);
        assert_eq!(next.exaggeration, 4.0);
        let backwards = operation.inverse(&snapshot);
        let restored = backwards[0].diff(&next).apply(&next);
        assert_eq!(restored, snapshot);
    }

    #[test]
    fn set_exaggeration_and_set_imported_features_invert_to_the_prior_field_value() {
        let snapshot = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: "null".into() };
        assert_eq!(GisTerrainMutation::SetExaggeration { exaggeration: 9.0 }.inverse(&snapshot), vec![GisTerrainMutation::SetExaggeration { exaggeration: 1.5 }]);
        assert_eq!(
            GisTerrainMutation::SetImportedFeatures { features_json: "{}".into() }.inverse(&snapshot),
            vec![GisTerrainMutation::SetImportedFeatures { features_json: "null".into() }]
        );
    }
}
//#endregion 🔹Tests

pub fn apply_gis_terrain_mutation(snapshot: &mut GisTerrainSnapshot, mutation: &GisTerrainMutation) {
    *snapshot = vcs::apply_mutation(snapshot, mutation);
}

pub fn inverse_gis_terrain_mutation(snapshot: &GisTerrainSnapshot, mutation: &GisTerrainMutation) -> Vec<GisTerrainMutation> {
    mutation.inverse(snapshot)
}

