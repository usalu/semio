//! 🧬️ Sequence artifact schema — every field of the artifact with its state class.

use crate::artifacts::sequence::{default_snapshot, SequenceCamera, SequenceContentChild, SequenceSnapshot, SEQUENCE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use store::ArtifactDsl;

//#region 🔖️Artifact
/// 🧬️ Full sequence artifact state across the artifact, presence and config lanes. Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges` are replaced
/// by the same composed `content` CHILD slot `SequenceSnapshot` carries, mirroring `WriterArtifact`/
/// `FlowArtifact`'s precedent so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub content: SequenceContentChild,
    #[state(config)]
    pub last_run_json: String,
    #[state(config)]
    pub orientation: String,
    #[state(config)]
    pub camera: SequenceCamera,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SequenceArtifact {
    fn default() -> Self {
        Self {
            schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
            content: crate::artifacts::sequence::sequence_content_child_handle_and_cache(Vec::new(), Vec::new()),
            last_run_json: String::new(),
            orientation: "leftRight".into(),
            camera: SequenceCamera::default(),
            locale: "en-US".into(),
        }
    }
}

impl SequenceArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::sequence::SequenceSnapshot {
        crate::artifacts::sequence::SequenceSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::sequence::SequenceSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            content: snapshot.content,
            last_run_json: String::new(),
            orientation: "leftRight".into(),
            camera: SequenceCamera::default(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::sequence::SequenceSnapshot) {
        self.schema = snapshot.schema;
        self.content = snapshot.content;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sequence.sequence` — twenty handcrafted schema leaves.
pub fn sequence_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.sequence.sequence",
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

//#region 🔖️Example
/// 📄️ JSON re-serialization of `default_snapshot()`, round-tripped through its own `.sequence` DSL
/// first (see `crate::artifacts::sequence::dsl`), to prove the fixture is fully expressible in text —
/// for the framework-generic call site that contractually requires JSON (`App::example`'s manifest
/// `document_json` is loaded via `serde_json::from_str` by `ArtifactApp::load_document`'s default impl)
/// — out of scope to change, since both are defined in `framework/plugin`.
pub fn sequence_example_json() -> String {
    let fixture = <SequenceSnapshot as ArtifactDsl>::parse_dsl(&default_snapshot().print_dsl()).expect("default_snapshot round-trips through its own DSL");
    serde_json::to_string(&fixture).expect("default_snapshot is a static, hand-built value with no non-finite floats or non-UTF8 keys")
}
//#endregion 🔖️Example

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::sequence::schema::diff::SequenceDiff;
    use crate::artifacts::sequence::schema::mutations::SequenceMutation;
    use crate::artifacts::sequence::schema::snapshot::SequenceSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SequenceBuilderConstruction {
        snapshot: SequenceSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for SequenceBuilderConstruction {
        type Snapshot = SequenceSnapshot;
        type Mutation = SequenceMutation;
        type Diff = SequenceDiff;
        fn empty() -> Self { Self { snapshot: SequenceSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SequenceSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SequenceSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation, &self.snapshot).into_parts().0;
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SequenceDiff as protocol::MutationDiff<SequenceSnapshot>>::apply(&diff, &self.snapshot);
            self
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
    use crate::artifacts::sequence::SequenceSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SequenceParts {
        pub snapshot: Option<SequenceSnapshot>,
    }

    pub struct SequenceAnalyzerAnalysis;

    impl ArtifactAnalysis for SequenceAnalyzerAnalysis {
        type Parts = SequenceParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.sequence", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SequenceParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SequenceSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SequenceSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SequenceBuilderFacets {
        construction: derived_construction::SequenceBuilderConstruction,
        analysis: derived_analysis::SequenceAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SequenceComposerComposition,
    }
    builder: SequenceBuilder,
    analyzer: SequenceAnalyzer,
    composer: SequenceComposer,
);
//#endregion 🧬️DerivedArtifactFacets
