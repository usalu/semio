//! 🧬️ Cad artifact schema — every field of the artifact with its state class.

use crate::{CadDrawingChild, CadModelChild, CadSnapshot};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[path = "📎️references/🦀️.rs"]
pub mod references;
pub use references::CadReferenceIndex;

#[cfg(test)]
#[path = "🧪️tests/🪪️document/🦀️.rs"]
mod document_contract_tests;

//#region 🔖️Artifact
/// 🧬️ cad document artifact state.
///
/// 🛡️ `deny_unknown_fields`: a cad document is exactly these keys. The retired inline pane state
/// (`objects`, `shapeGeometry`, `activeModelDefinitionId` — replaced by the composed child slots and
/// by window config) must FAIL to decode instead of being silently dropped, which is the law
/// `🧫️fixtures/🪪️document`'s `invalidDocuments` rows state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub building_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub energy_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub drawings: Vec<CadDrawingChild>,
    #[state(artifact)]
    #[value(default)]
    pub references_by_model_definition_id: CadReferenceIndex,
    #[state(artifact)]
    #[value(default)]
    pub nodes: Vec<CadNode>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for CadArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::empty_cad_snapshot())
    }
}

impl CadArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> CadSnapshot {
        CadSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            shape_model: self.shape_model.clone(),
            building_model: self.building_model.clone(),
            energy_model: self.energy_model.clone(),
            structure_classic_model: self.structure_classic_model.clone(),
            drawings: self.drawings.clone(),
            references_by_model_definition_id: self.references_by_model_definition_id.clone(),
            nodes: self.nodes.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: CadSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            shape_model: snapshot.shape_model,
            building_model: snapshot.building_model,
            energy_model: snapshot.energy_model,
            structure_classic_model: snapshot.structure_classic_model,
            drawings: snapshot.drawings,
            references_by_model_definition_id: snapshot.references_by_model_definition_id,
            nodes: snapshot.nodes,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: CadSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.shape_model = snapshot.shape_model;
        self.building_model = snapshot.building_model;
        self.energy_model = snapshot.energy_model;
        self.structure_classic_model = snapshot.structure_classic_model;
        self.drawings = snapshot.drawings;
        self.references_by_model_definition_id = snapshot.references_by_model_definition_id;
        self.nodes = snapshot.nodes;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.cad.cad` — twenty handcrafted schema leaves.
pub fn cad_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.cad.cad",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
pub use crate::CadCamera;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::CadNode;
pub use crate::CadReferenceList;
//#endregion 🔁️Re-exports
