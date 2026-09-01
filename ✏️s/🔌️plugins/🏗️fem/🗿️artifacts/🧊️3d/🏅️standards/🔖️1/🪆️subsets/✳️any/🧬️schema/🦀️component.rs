//! 🧬️ Fem3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::fem3d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Artifact
/// 🧬️ Full fem3d artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dArtifact {
    #[state(artifact)]
    pub nodes: Vec<FemNode>,
    #[state(artifact)]
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
    #[state(presence)]
    pub result_source_id: Option<String>,
    #[state(presence)]
    pub result_mode: String,
    #[state(presence)]
    pub result_mode_index: u32,
    #[state(config)]
    pub camera: FemCamera,
    #[state(artifact)]
    pub solver_results_json: String,
    #[state(artifact)]
    pub mesh_preview_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Fem3dArtifact {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            elements: Default::default(),
            materials: Default::default(),
            sections: Default::default(),
            solids: Default::default(),
            supports: Default::default(),
            load_cases: Default::default(),
            combinations: Default::default(),
            analysis: Default::default(),
            result_source_id: None,
            result_mode: "static".into(),
            result_mode_index: 0,
            camera: FemCamera::default(),
            solver_results_json: String::new(),
            mesh_preview_json: String::new(),
        }
    }
}

impl Fem3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::fem3d::Fem3dSnapshot {
        crate::artifacts::fem3d::Fem3dSnapshot {
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

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI/preview fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::fem3d::Fem3dSnapshot) -> Self {
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
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::fem3d::Fem3dSnapshot) {
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
pub fn fem3d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.fem.fem3d",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::fem3d::{Fem3dDiff, Fem3dMutation, Fem3dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Fem3dBuilderConstruction {
        snapshot: Fem3dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Fem3dBuilderConstruction {
        type Snapshot = Fem3dSnapshot;
        type Mutation = Fem3dMutation;
        type Diff = Fem3dDiff;
        fn empty() -> Self {
            Self { snapshot: Fem3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self { snapshot: <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?, diagnostics: Vec::new() })
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <Fem3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?, diagnostics: Vec::new() })
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🌱️DerivedEmpty
/// 🌱️ An empty `Fem3dSnapshot` — the app's genesis document and every test fixture's blank baseline.
pub fn empty_fem3d_snapshot() -> crate::artifacts::fem3d::Fem3dSnapshot {
    crate::artifacts::fem3d::Fem3dSnapshot::default()
}
//#endregion 🌱️DerivedEmpty

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::fem3d::Fem3dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Fem3dParts {
        pub snapshot: Option<Fem3dSnapshot>,
    }

    pub struct Fem3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Fem3dAnalyzerAnalysis {
        type Parts = Fem3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.fem3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Fem3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Fem3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Fem3dBuilderFacets {
        construction: Fem3dBuilderConstruction,
        analysis: Fem3dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Fem3dComposerComposition,
    }
    builder: Fem3dBuilder,
    analyzer: Fem3dAnalyzer,
    composer: Fem3dComposer,
);
//#endregion 🧬️DerivedArtifactFacets
