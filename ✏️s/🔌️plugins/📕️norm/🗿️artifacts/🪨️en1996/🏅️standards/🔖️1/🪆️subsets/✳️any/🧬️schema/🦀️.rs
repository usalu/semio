//! 🧱️ EN 1996 artifact schema — every field with its state class.

use crate::document::AnnexChoice;
use crate::En1996Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1996 document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub masonry_class: crate::MasonryClass,
    #[state(artifact)]
    pub design_situation: crate::document::DesignSituation,
    #[state(artifact)]
    pub storeys: u32,
    #[state(artifact)]
    pub walls: Vec<crate::MasonryWall>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1996Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1996Snapshot::default())
    }
}

impl From<En1996Snapshot> for En1996Artifact {
    fn from(snapshot: En1996Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1996Artifact {
    pub fn to_snapshot(&self) -> En1996Snapshot {
        En1996Snapshot {
            annex: self.annex,
            masonry_class: self.masonry_class,
            design_situation: self.design_situation,
            storeys: self.storeys,
            walls: self.walls.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1996Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            masonry_class: snapshot.masonry_class,
            design_situation: snapshot.design_situation,
            storeys: snapshot.storeys,
            walls: snapshot.walls,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: En1996Snapshot) { *self = Self::from_snapshot(snapshot); }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1996_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1996",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
    use crate::{En1996Diff, En1996Mutation, En1996Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1996BuilderConstruction {
        snapshot: En1996Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1996BuilderConstruction {
        type Snapshot = En1996Snapshot;
        type Mutation = En1996Mutation;
        type Diff = En1996Diff;
        fn empty() -> Self {
            Self { snapshot: En1996Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1996Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1996Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1996Diff as protocol::MutationDiff<En1996Snapshot>>::apply(&diff, &self.snapshot)?;
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

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::En1996Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1996Parts {
        pub snapshot: Option<En1996Snapshot>,
    }

    pub struct En1996AnalyzerAnalysis;

    impl ArtifactAnalysis for En1996AnalyzerAnalysis {
        type Parts = En1996Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1996", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1996Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1996Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1996Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1996BuilderFacets {
        construction: En1996BuilderConstruction,
        analysis: En1996AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1996ComposerComposition,
    }
    builder: En1996Builder,
    analyzer: En1996Analyzer,
    composer: En1996Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
#[path = "⚖️masonry/🦀️.rs"]
mod masonry;
pub use masonry::*;
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests

#[cfg(test)]
#[path = "🧪️tests/🔬️oracle/🦀️.rs"]
mod oracle_tests;

