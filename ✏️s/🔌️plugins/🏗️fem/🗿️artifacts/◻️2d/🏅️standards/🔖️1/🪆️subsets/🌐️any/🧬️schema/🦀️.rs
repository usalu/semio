//! 🧬️ Fem2d artifact schema — every field of the artifact with its state class.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Artifact
/// 🧬️ fem2d document artifact state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.fem.fem2d")]
#[derive(Default)]
pub struct Fem2dArtifact {
    #[state(artifact)]
    pub nodes: Vec<FemNode>,
    #[state(artifact)]
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
    pub fn to_snapshot(&self) -> crate::Fem2dSnapshot {
        crate::Fem2dSnapshot {
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
    pub fn from_snapshot(snapshot: crate::Fem2dSnapshot) -> Self {
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
    pub fn set_snapshot(&mut self, snapshot: crate::Fem2dSnapshot) {
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
pub fn fem2d_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.fem.fem2d",
        artifact: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema::FacetLeaves {
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
pub mod derived_construction {
    use crate::{Fem2dDiff, Fem2dMutation, Fem2dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Fem2dBuilderConstruction {
        snapshot: Fem2dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Fem2dBuilderConstruction {
        type Snapshot = Fem2dSnapshot;
        type Mutation = Fem2dMutation;
        type Diff = Fem2dDiff;
        fn empty() -> Self {
            Self { snapshot: Fem2dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self { snapshot: <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?, diagnostics: Vec::new() })
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <Fem2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?, diagnostics: Vec::new() })
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
            let snapshot = <Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&diff, &self.snapshot)?;
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
/// 🌱️ An empty `Fem2dSnapshot` — every test fixture's blank baseline and the fallback boot document.
pub fn empty_fem2d_snapshot() -> crate::Fem2dSnapshot {
    crate::Fem2dSnapshot::default()
}

/// 🌱️ The document every fresh fem2d surface boots on: the bundled `📚️examples/🎬️demo` DSL, so the
/// editor and the viewer both paint a real structure at first frame instead of an empty canvas. A
/// fixture that ever stops parsing degrades to `empty_fem2d_snapshot` rather than faulting the boot,
/// and says so on the console.
pub fn default_fem2d_snapshot() -> crate::Fem2dSnapshot {
    match <crate::Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM2D_EXAMPLE_TEXT) {
        Ok(snapshot) => {
            eprintln!(
                "[DEBUG] fem2d boot snapshot: loaded the bundled example — nodes={} elements={} regions={} materials={} sections={} supports={} loadCases={} combinations={}",
                snapshot.nodes.len(),
                snapshot.elements.len(),
                snapshot.regions.len(),
                snapshot.materials.len(),
                snapshot.sections.len(),
                snapshot.supports.len(),
                snapshot.load_cases.len(),
                snapshot.combinations.len()
            );
            snapshot
        }
        Err(error) => {
            eprintln!("[DEBUG] fem2d boot snapshot: the bundled example failed to parse, falling back to the empty document — {error}");
            empty_fem2d_snapshot()
        }
    }
}
//#endregion 🌱️DerivedEmpty

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::Fem2dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Fem2dParts {
        pub snapshot: Option<Fem2dSnapshot>,
    }

    pub struct Fem2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Fem2dAnalyzerAnalysis {
        type Parts = Fem2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.fem.fem2d", standard: StandardId("1"), subset: SubsetId("*") };

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
        construction: Fem2dBuilderConstruction,
        analysis: Fem2dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Fem2dComposerComposition,
    }
    builder: Fem2dBuilder,
    analyzer: Fem2dAnalyzer,
    composer: Fem2dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
pub use crate::FemAnalysisSettings;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::FemCombination;
pub use crate::FemElement;
pub use crate::FemLoadCase;
pub use crate::FemMaterial;
pub use crate::FemNode;
pub use crate::FemRegion;
pub use crate::FemSection;
pub use crate::FemSupport;
//#endregion 🔁️Re-exports
