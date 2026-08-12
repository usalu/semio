//! 🧬️ Present artifact schema — every field of the artifact with its state class.

use crate::artifacts::present::{FigureTileDraft, FigureTileSource, PRESENT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full present artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub source: FigureTileSource,
    #[state(persistent)]
    pub tiles: Vec<FigureTileDraft>,
    #[state(shared_ui)]
    pub selected_ids: Vec<String>,
    #[state(local_ui)]
    pub engagement_input: String,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PresentArtifact {
    fn default() -> Self {
        Self {
            schema: PRESENT_DOCUMENT_SCHEMA.into(),
            source: crate::artifacts::present::default_figure_tile_source(),
            tiles: Vec::new(),
            selected_ids: Vec::new(),
            engagement_input: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl PresentArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::present::PresentSnapshot {
        crate::artifacts::present::PresentSnapshot {
            schema: self.schema.clone(),
            source: self.source.clone(),
            tiles: self.tiles.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::present::PresentSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            source: snapshot.source,
            tiles: snapshot.tiles,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::present::PresentSnapshot) {
        self.schema = snapshot.schema;
        self.source = snapshot.source;
        self.tiles = snapshot.tiles;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.animate.present` — twenty handcrafted schema leaves.
pub fn present_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.animate.present",
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
    use crate::artifacts::present::schema::diff::PresentDiff;
    use crate::artifacts::present::schema::mutations::PresentMutation;
    use crate::artifacts::present::schema::snapshot::PresentSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct PresentBuilderConstruction {
        snapshot: PresentSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PresentBuilderConstruction {
        type Snapshot = PresentSnapshot;
        type Mutation = PresentMutation;
        type Diff = PresentDiff;
        fn empty() -> Self { Self { snapshot: PresentSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PresentSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PresentSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <PresentDiff as protocol::MutationDiff<PresentSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::present::PresentSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct PresentParts {
        pub snapshot: Option<PresentSnapshot>,
    }

    pub struct PresentAnalyzerAnalysis;

    impl ArtifactAnalysis for PresentAnalyzerAnalysis {
        type Parts = PresentParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.present", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PresentParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PresentSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PresentSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec PresentBuilderFacets {
        construction: derived_construction::PresentBuilderConstruction,
        analysis: derived_analysis::PresentAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PresentComposerComposition,
    }
    builder: PresentBuilder,
    analyzer: PresentAnalyzer,
    composer: PresentComposer,
);
//#endregion 🧬️DerivedArtifactFacets
