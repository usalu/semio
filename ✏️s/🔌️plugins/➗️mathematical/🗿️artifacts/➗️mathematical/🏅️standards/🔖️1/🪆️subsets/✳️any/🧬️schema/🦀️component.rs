//! 🧬️ Mathematical artifact schema — every field with its state class.

use crate::artifacts::mathematical::{MathematicalComputedChild, MathematicalGeometry, MathematicalGraph, MathematicalNotationChild, MathematicalResultsChild};
use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full mathematical artifact across the artifact and config lanes. `notation`/`results`/
/// `computed` mirror `MathematicalSnapshot`'s own composed-child slots (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, `mathematical→C:text,table,value`); `equation`
/// mirrors its plain (non-`#[child]`) persistent sibling added in wave M3a of
/// 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical")]
pub struct MathematicalArtifact {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub notation: MathematicalNotationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: MathematicalResultsChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub computed: MathematicalComputedChild,
    #[state(artifact)]
    pub equation: EquationSnapshot,
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for MathematicalArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::mathematical::MathematicalSnapshot::default())
    }
}

impl MathematicalArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::mathematical::MathematicalSnapshot {
        crate::artifacts::mathematical::MathematicalSnapshot { notation: self.notation.clone(), results: self.results.clone(), computed: self.computed.clone(), equation: self.equation.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::mathematical::MathematicalSnapshot) -> Self {
        Self { notation: snapshot.notation, results: snapshot.results, computed: snapshot.computed, equation: snapshot.equation, ..Self::default_ui() }
    }

    fn default_ui() -> Self {
        let default_snapshot = crate::artifacts::mathematical::mathematical_snapshot_with_state(MathematicalGraph::default(), MathematicalGeometry::default());
        Self {
            notation: default_snapshot.notation,
            results: default_snapshot.results,
            computed: default_snapshot.computed,
            equation: default_snapshot.equation,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::mathematical::MathematicalSnapshot) {
        self.notation = snapshot.notation;
        self.results = snapshot.results;
        self.computed = snapshot.computed;
        self.equation = snapshot.equation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.mathematical.mathematical` — twenty handcrafted schema leaves.
pub fn mathematical_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.mathematical.mathematical",
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
    use crate::artifacts::mathematical::{MathematicalDiff, MathematicalMutation, MathematicalSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct MathematicalBuilderConstruction {
        snapshot: MathematicalSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for MathematicalBuilderConstruction {
        type Snapshot = MathematicalSnapshot;
        type Mutation = MathematicalMutation;
        type Diff = MathematicalDiff;
        fn empty() -> Self { Self { snapshot: MathematicalSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<MathematicalSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }
        fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::mathematical::MathematicalSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct MathematicalParts {
        pub snapshot: Option<MathematicalSnapshot>,
    }

    pub struct MathematicalAnalyzerAnalysis;

    impl ArtifactAnalysis for MathematicalAnalyzerAnalysis {
        type Parts = MathematicalParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.mathematical", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = MathematicalParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <MathematicalSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec MathematicalBuilderFacets {
        construction: MathematicalBuilderConstruction,
        analysis: MathematicalAnalyzerAnalysis,
        composition: super::super::io::derived_composition::MathematicalComposerComposition,
    }
    builder: MathematicalBuilder,
    analyzer: MathematicalAnalyzer,
    composer: MathematicalComposer,
);
//#endregion 🧬️DerivedArtifactFacets
