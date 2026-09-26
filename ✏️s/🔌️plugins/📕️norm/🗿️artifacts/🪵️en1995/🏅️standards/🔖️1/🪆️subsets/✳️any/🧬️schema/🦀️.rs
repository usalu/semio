//! 🪵️ EN 1995 artifact schema — every field with its state class.

use crate::document::AnnexChoice;
use crate::En1995Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1995 document artifact state — mirrors the timber-structure snapshot.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub members: Vec<crate::TimberMember>,
    #[state(artifact)]
    pub connections: Vec<crate::TimberConnection>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1995Artifact {
    fn default() -> Self { Self::from_snapshot(En1995Snapshot::default()) }
}
impl From<En1995Snapshot> for En1995Artifact {
    fn from(snapshot: En1995Snapshot) -> Self { Self::from_snapshot(snapshot) }
}
impl En1995Artifact {
    pub fn to_snapshot(&self) -> En1995Snapshot {
        En1995Snapshot { annex: self.annex, members: self.members.clone(), connections: self.connections.clone() }
    }
    pub fn from_snapshot(snapshot: En1995Snapshot) -> Self {
        Self { annex: snapshot.annex, members: snapshot.members, connections: snapshot.connections }
    }
    pub fn set_snapshot(&mut self, snapshot: En1995Snapshot) { *self = Self::from_snapshot(snapshot); }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1995_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1995",
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
    use crate::{En1995Diff, En1995Mutation, En1995Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1995BuilderConstruction {
        snapshot: En1995Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1995BuilderConstruction {
        type Snapshot = En1995Snapshot;
        type Mutation = En1995Mutation;
        type Diff = En1995Diff;
        fn empty() -> Self {
            Self { snapshot: En1995Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1995Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1995Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1995Diff as protocol::MutationDiff<En1995Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1995Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1995Parts {
        pub snapshot: Option<En1995Snapshot>,
    }

    pub struct En1995AnalyzerAnalysis;

    impl ArtifactAnalysis for En1995AnalyzerAnalysis {
        type Parts = En1995Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1995", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1995Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1995Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1995Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1995BuilderFacets {
        construction: En1995BuilderConstruction,
        analysis: En1995AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1995ComposerComposition,
    }
    builder: En1995Builder,
    analyzer: En1995Analyzer,
    composer: En1995Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
#[path = "⚖️timber/🦀️.rs"]
mod timber;
pub use timber::*;

/// ♻️ Compatibility aliases for older test module paths.
pub mod part_1_1 {
    pub use super::timber::*;
}
pub mod part_1_2 {
    pub use super::timber::*;
}
pub mod part_2 {
    pub use super::timber::*;
}
pub mod na_de {}
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
