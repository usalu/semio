//! 🧬️ Fem2d artifact schema — every field of the artifact with its state class.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Artifact
/// 🧬️ fem2d document artifact state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.fem.fem2d")]
#[derive(Default)]
pub struct Fem2dArtifact {
    #[state(artifact)]
    pub nodes: Vec<FemNode>,
    #[state(artifact)]
    #[dsl(statements, block)]
    pub elements: Vec<FemElement>,
    #[state(artifact)]
    pub regions: Vec<FemRegion>,
    #[state(artifact)]
    pub materials: Vec<FemMaterial>,
    #[state(artifact)]
    pub sections: Vec<FemSection>,
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


impl Fem2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot {
        crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot {
            nodes: self.nodes.clone(),
            elements: self.elements.clone(),
            regions: self.regions.clone(),
            materials: self.materials.clone(),
            sections: self.sections.clone(),
            supports: self.supports.clone(),
            load_cases: self.load_cases.clone(),
            combinations: self.combinations.clone(),
            analysis: self.analysis.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, preserving the authored document fields.
    pub fn from_snapshot(snapshot: crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot) -> Self {
        Self {
            nodes: snapshot.nodes,
            elements: snapshot.elements,
            regions: snapshot.regions,
            materials: snapshot.materials,
            sections: snapshot.sections,
            supports: snapshot.supports,
            load_cases: snapshot.load_cases,
            combinations: snapshot.combinations,
            analysis: snapshot.analysis,

        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot) {
        self.nodes = snapshot.nodes;
        self.elements = snapshot.elements;
        self.regions = snapshot.regions;
        self.materials = snapshot.materials;
        self.sections = snapshot.sections;
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



//#endregion 🌱️DerivedEmpty

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🏗️Structure
use super::structure::{FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
//#endregion 🏗️Structure
