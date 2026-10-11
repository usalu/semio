//! 🧬️ Fem3d artifact schema — every field of the artifact with its state class.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Artifact
/// 🧬️ fem3d document artifact state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, Default, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dArtifact {
    #[state(artifact)]
    pub nodes: Vec<FemNode>,
    #[state(artifact)]
    #[dsl(statements, block)]
    pub elements: Vec<FemElement>,
    #[state(artifact)]
    pub materials: Vec<FemMaterial>,
    #[state(artifact)]
    pub sections: Vec<FemSection>,
    #[state(artifact)]
    pub solids: Vec<FemSolid>,
    #[state(artifact)]
    pub supports: Vec<FemSupport>,
    #[state(artifact)]
    pub load_cases: Vec<FemLoadCase>,
    #[state(artifact)]
    pub combinations: Vec<FemCombination>,
    #[state(artifact)]
    pub analysis: FemAnalysisSettings,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions

impl Fem3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::Fem3dSnapshot {
        crate::Fem3dSnapshot {
            nodes: self.nodes.clone(),
            elements: self.elements.clone(),
            materials: self.materials.clone(),
            sections: self.sections.clone(),
            solids: self.solids.clone(),
            supports: self.supports.clone(),
            load_cases: self.load_cases.clone(),
            combinations: self.combinations.clone(),
            analysis: self.analysis.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, preserving the authored document fields.
    pub fn from_snapshot(snapshot: crate::Fem3dSnapshot) -> Self {
        Self {
            nodes: snapshot.nodes,
            elements: snapshot.elements,
            materials: snapshot.materials,
            sections: snapshot.sections,
            solids: snapshot.solids,
            supports: snapshot.supports,
            load_cases: snapshot.load_cases,
            combinations: snapshot.combinations,
            analysis: snapshot.analysis,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::Fem3dSnapshot) {
        self.nodes = snapshot.nodes;
        self.elements = snapshot.elements;
        self.materials = snapshot.materials;
        self.sections = snapshot.sections;
        self.solids = snapshot.solids;
        self.supports = snapshot.supports;
        self.load_cases = snapshot.load_cases;
        self.combinations = snapshot.combinations;
        self.analysis = snapshot.analysis;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.fem.fem3d` — twenty handcrafted schema leaves.
pub fn fem3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.fem.fem3d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
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

//#region 🌱️DerivedEmpty
/// 🌱️ An empty `Fem3dSnapshot` — the app's genesis document and every test fixture's blank baseline.
pub fn empty_fem3d_snapshot() -> crate::Fem3dSnapshot {
    crate::Fem3dSnapshot::default()
}
//#endregion 🌱️DerivedEmpty

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
pub use crate::FemAnalysisSettings;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::FemCombination;
pub use crate::FemElement;
pub use crate::FemLoadCase;
pub use crate::FemMaterial;
pub use crate::FemNode;
pub use crate::FemSection;
pub use crate::FemSolid;
pub use crate::FemSupport;
//#endregion 🔁️Re-exports
