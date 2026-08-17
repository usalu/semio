//! 🧬️ VCS artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DocumentHelpers
/// 🌱️ The artifact's empty/default snapshot — used as `VcsPlayApp::initial_snapshot()` and by every
/// test fixture that needs a base document (was: `⚙️engine::empty_vcs_snapshot()`, dissolved per ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub fn empty_vcs_snapshot() -> crate::artifacts::vcs::VcsSnapshot {
    crate::artifacts::vcs::VcsSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Artifact
/// 🧬️ Full VCS demo artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.vcs.vcs")]
pub struct VcsArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub title: String,
    #[state(artifact)]
    pub counter: i64,
    #[state(artifact)]
    pub notes: String,
    #[state(artifact)]
    pub status: String,
    #[state(artifact)]
    #[serde(default)]
    pub tags: Vec<String>,
    #[state(presence)]
    #[serde(default)]
    pub selected_checkpoint_ids: Vec<String>,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for VcsArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA.into(),
            title: "VCS Demo".into(),
            counter: 0,
            notes: String::new(),
            status: "new".into(),
            tags: Vec::new(),
            selected_checkpoint_ids: Vec::new(),
            locale: "en-US".into(),
        }
    }
}

impl VcsArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::vcs::VcsSnapshot {
        crate::artifacts::vcs::VcsSnapshot {
            schema: self.schema.clone(),
            title: self.title.clone(),
            counter: self.counter,
            notes: self.notes.clone(),
            status: self.status.clone(),
            tags: self.tags.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::vcs::VcsSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            title: snapshot.title,
            counter: snapshot.counter,
            notes: snapshot.notes,
            status: snapshot.status,
            tags: snapshot.tags,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::vcs::VcsSnapshot) {
        self.schema = snapshot.schema;
        self.title = snapshot.title;
        self.counter = snapshot.counter;
        self.notes = snapshot.notes;
        self.status = snapshot.status;
        self.tags = snapshot.tags;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.vcs.vcs` — twenty handcrafted schema leaves.
pub fn vcs_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.vcs.vcs",
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
    use crate::artifacts::vcs::{VcsDiff, VcsDemoMutation, VcsSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct VcsBuilderConstruction {
        snapshot: VcsSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for VcsBuilderConstruction {
        type Snapshot = VcsSnapshot;
        type Mutation = VcsDemoMutation;
        type Diff = VcsDiff;
        fn empty() -> Self { Self { snapshot: VcsSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<VcsSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<VcsSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
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
            let snapshot = <VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::vcs::VcsSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct VcsParts {
        pub snapshot: Option<VcsSnapshot>,
    }

    pub struct VcsAnalyzerAnalysis;

    impl ArtifactAnalysis for VcsAnalyzerAnalysis {
        type Parts = VcsParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.vcs", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = VcsParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <VcsSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <VcsSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec VcsBuilderFacets {
        construction: VcsBuilderConstruction,
        analysis: VcsAnalyzerAnalysis,
        composition: super::super::io::derived_composition::VcsComposerComposition,
    }
    builder: VcsBuilder,
    analyzer: VcsAnalyzer,
    composer: VcsComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_vcs_snapshot();
        assert_eq!(snapshot.schema, crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.status, "new");
    }
}
//#endregion 🧪️Tests
