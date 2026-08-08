//! ⚡️ GIS terrain artifact — the mutation enum, its `Mutation` law and the store aliases
//! (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gisterrain::diff::Gis3dTerrainDiff;
use crate::artifacts::gisterrain::Gis3dTerrainDocument;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Operation
/// 🗺️ Typed, invertible terrain operation. `SetExaggeration` is the one interactively-edited field
/// (the transform-gumball-equivalent slider control); `SetImportedFeatures` writes `map:in`'s overlay
/// pin layer (see `Gis3dTerrainDocument::imported_features_json`); `SetDocument` replaces the whole
/// document (`whole_document_operation`/`document:in` import, example load, reset) — mirrors
/// `crate::artifacts::gismap::op::GisMapMutation::SetDocument`'s identical "whole-projection replace"
/// shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Gis3dTerrainMutation {
    SetExaggeration {
        exaggeration: f64,
    },
    SetImportedFeatures {
        #[dsl(key = "features-json")]
        features_json: String,
    },
    SetDocument {
        #[dsl(block)]
        document: Gis3dTerrainDocument,
    },
}





impl Mutation<Gis3dTerrainDocument> for Gis3dTerrainMutation {
    type Diff = Gis3dTerrainDiff;

    fn diff(&self, _projection: &Gis3dTerrainDocument) -> Gis3dTerrainDiff {
        match self {
            Gis3dTerrainMutation::SetExaggeration { exaggeration } => Gis3dTerrainDiff { exaggeration: Some(*exaggeration), ..Default::default() },
            Gis3dTerrainMutation::SetImportedFeatures { features_json } => Gis3dTerrainDiff { imported_features_json: Some(features_json.clone()), ..Default::default() },
            Gis3dTerrainMutation::SetDocument { document } => Gis3dTerrainDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &Gis3dTerrainDocument) -> Vec<Self> {
        match self {
            Gis3dTerrainMutation::SetExaggeration { .. } => vec![Gis3dTerrainMutation::SetExaggeration { exaggeration: projection.exaggeration }],
            Gis3dTerrainMutation::SetImportedFeatures { .. } => vec![Gis3dTerrainMutation::SetImportedFeatures { features_json: projection.imported_features_json.clone() }],
            Gis3dTerrainMutation::SetDocument { .. } => vec![Gis3dTerrainMutation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Gis3dTerrainEnvelope = DocumentEnvelope<Gis3dTerrainDocument, Gis3dTerrainMutation>;
pub type Gis3dTerrainStore = DocumentStore<Gis3dTerrainDocument, Gis3dTerrainMutation>;
//#endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn gis3d_terrain_set_document_backwards_restores_the_exact_prior_projection() {
        let projection = Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: "null".into() };
        let operation = Gis3dTerrainMutation::SetDocument { document: Gis3dTerrainDocument { exaggeration: 4.0, imported_features_json: r#"{"positions":[]}"#.into() } };
        let next = operation.diff(&projection).apply(&projection);
        assert_eq!(next.exaggeration, 4.0);
        let backwards = operation.inverse(&projection);
        let restored = backwards[0].diff(&next).apply(&next);
        assert_eq!(restored, projection);
    }

    #[test]
    fn set_exaggeration_and_set_imported_features_invert_to_the_prior_field_value() {
        let projection = Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: "null".into() };
        assert_eq!(Gis3dTerrainMutation::SetExaggeration { exaggeration: 9.0 }.inverse(&projection), vec![Gis3dTerrainMutation::SetExaggeration { exaggeration: 1.5 }]);
        assert_eq!(
            Gis3dTerrainMutation::SetImportedFeatures { features_json: "{}".into() }.inverse(&projection),
            vec![Gis3dTerrainMutation::SetImportedFeatures { features_json: "null".into() }]
        );
    }
}
//#endregion 🧪️Tests


pub fn apply_gis_3d_terrain_mutation(projection: &mut Gis3dTerrainDocument, mutation: &Gis3dTerrainMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_gis_3d_terrain_mutation(projection: &Gis3dTerrainDocument, mutation: &Gis3dTerrainMutation) -> Vec<Gis3dTerrainMutation> {
    mutation.inverse(projection)
}
