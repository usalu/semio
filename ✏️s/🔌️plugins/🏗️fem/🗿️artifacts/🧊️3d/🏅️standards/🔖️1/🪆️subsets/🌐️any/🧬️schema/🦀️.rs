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
    pub fn to_snapshot(&self) -> crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot {
        crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot {
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
    pub fn from_snapshot(snapshot: crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot) -> Self {
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
    pub fn set_snapshot(&mut self, snapshot: crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot) {
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

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🌱️DerivedEmpty
/// 🌱️ An empty `Fem3dSnapshot` — the app's genesis document and every test fixture's blank baseline.
pub fn empty_fem3d_snapshot() -> crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot::default()
}
//#endregion 🌱️DerivedEmpty

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
pub use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1::subsets::any::schema::structure::FemCombination;
pub use crate::standards::v1::subsets::any::schema::structure::FemElement;
pub use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
pub use crate::standards::v1::subsets::any::schema::structure::FemMaterial;
pub use crate::standards::v1::subsets::any::schema::structure::FemNode;
pub use crate::standards::v1::subsets::any::schema::structure::FemSection;
pub use crate::standards::v1::subsets::any::schema::structure::FemSolid;
pub use crate::standards::v1::subsets::any::schema::structure::FemSupport;
//#endregion 🔁️Re-exports
