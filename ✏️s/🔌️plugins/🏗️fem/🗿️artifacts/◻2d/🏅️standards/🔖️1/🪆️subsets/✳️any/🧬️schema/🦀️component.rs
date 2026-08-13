//! 🧬️ Fem2d artifact schema — every field of the artifact with its state class.

use crate::artifacts::fem2d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full fem2d artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem2d")]
pub struct Fem2dArtifact {
    #[state(artifact)] pub nodes: Vec<FemNode>,
    #[state(artifact)] pub elements: Vec<FemElement>,
    #[state(artifact)] pub regions: Vec<FemRegion>,
    #[state(artifact)] pub materials: Vec<FemMaterial>,
    #[state(artifact)] pub sections: Vec<FemSection>,
    #[state(artifact)] pub supports: Vec<FemSupport>,
    #[state(artifact)] pub load_cases: Vec<FemLoadCase>,
    #[state(artifact)] pub combinations: Vec<FemCombination>,
    #[state(artifact)] pub analysis: FemAnalysisSettings,
    #[state(presence)] pub result_source_id: Option<String>,
    #[state(presence)] pub result_mode: String,
    #[state(presence)] pub result_mode_index: u32,
    #[state(config)] pub camera: FemCamera,
    #[state(config)] pub locale: String,
    #[state(artifact)] pub solver_results_json: String,
    #[state(artifact)] pub mesh_preview_json: String,
}
//#endregion 🔖️Artifact


//#region 🔖️Conversions
impl Default for Fem2dArtifact {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            elements: Default::default(),
            regions: Default::default(),
            materials: Default::default(),
            sections: Default::default(),
            supports: Default::default(),
            load_cases: Default::default(),
            combinations: Default::default(),
            analysis: Default::default(),
            result_source_id: None,
            result_mode: "static".into(),
            result_mode_index: 0,
            camera: FemCamera::default(),
            locale: "en-US".into(),
            solver_results_json: String::new(),
            mesh_preview_json: String::new(),
        }
    }
}

impl Fem2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::fem2d::Fem2dSnapshot {
        crate::artifacts::fem2d::Fem2dSnapshot {
            nodes: self.nodes.clone(), elements: self.elements.clone(), regions: self.regions.clone(), materials: self.materials.clone(), sections: self.sections.clone(), supports: self.supports.clone(), load_cases: self.load_cases.clone(), combinations: self.combinations.clone(), analysis: self.analysis.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI/preview fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::fem2d::Fem2dSnapshot) -> Self {
        Self {
            nodes: snapshot.nodes, elements: snapshot.elements, regions: snapshot.regions, materials: snapshot.materials, sections: snapshot.sections, supports: snapshot.supports, load_cases: snapshot.load_cases, combinations: snapshot.combinations, analysis: snapshot.analysis,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::fem2d::Fem2dSnapshot) {
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

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.fem.fem2d` — twenty handcrafted schema leaves.
pub fn fem2d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.fem.fem2d",
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
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::fem2d::{Fem2dDiff, Fem2dMutation, Fem2dSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Fem2dBuilderConstruction {
        snapshot: Fem2dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Fem2dBuilderConstruction {
        type Snapshot = Fem2dSnapshot;
        type Mutation = Fem2dMutation;
        type Diff = Fem2dDiff;
        fn empty() -> Self { Self { snapshot: Fem2dSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Fem2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::fem2d::schema::mutations::apply_fem2d_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🌱️DerivedEmpty
/// 🌱️ An empty `Fem2dSnapshot` — the app's genesis document and every test fixture's blank baseline.
pub fn empty_fem2d_snapshot() -> crate::artifacts::fem2d::Fem2dSnapshot {
    crate::artifacts::fem2d::Fem2dSnapshot::default()
}
//#endregion 🌱️DerivedEmpty

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::fem2d::Fem2dSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Fem2dParts {
        pub snapshot: Option<Fem2dSnapshot>,
    }

    pub struct Fem2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Fem2dAnalyzerAnalysis {
        type Parts = Fem2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.fem2d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Fem2dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Fem2dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Fem2dBuilderFacets {
        construction: derived_construction::Fem2dBuilderConstruction,
        analysis: derived_analysis::Fem2dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Fem2dComposerComposition,
    }
    builder: Fem2dBuilder,
    analyzer: Fem2dAnalyzer,
    composer: Fem2dComposer,
);
//#endregion 🧬️DerivedArtifactFacets
