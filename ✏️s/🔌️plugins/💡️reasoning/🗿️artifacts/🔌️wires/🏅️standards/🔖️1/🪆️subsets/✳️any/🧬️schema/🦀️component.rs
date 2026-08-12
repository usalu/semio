//! 🧬️ Wires artifact schema — every field of the artifact with its state class.

use dsl::DslValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full wires artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresArtifact {
    #[state(persistent)]
    pub wires_fixture: DslValue,
    #[state(persistent)]
    pub board_fixture: DslValue,
    #[state(shared_ui)]
    pub selected_ids: Vec<String>,
    #[state(preview)]
    pub drag_node_id: Option<String>,
    #[state(preview)]
    pub drag_last_x: f64,
    #[state(preview)]
    pub drag_last_y: f64,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WiresArtifact {
    fn default() -> Self {
        Self {
            wires_fixture: crate::artifacts::wires::empty_wires_fixture(),
            board_fixture: crate::artifacts::wires::empty_board_fixture(),
            selected_ids: Vec::new(),
            drag_node_id: None,
            drag_last_x: 0.0,
            drag_last_y: 0.0,
            locale: "en-US".into(),
        }
    }
}

impl WiresArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::wires::WiresSnapshot {
        crate::artifacts::wires::WiresSnapshot {
            wires_fixture: self.wires_fixture.clone(),
            board_fixture: self.board_fixture.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::wires::WiresSnapshot) -> Self {
        Self {
            wires_fixture: snapshot.wires_fixture,
            board_fixture: snapshot.board_fixture,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::wires::WiresSnapshot) {
        self.wires_fixture = snapshot.wires_fixture;
        self.board_fixture = snapshot.board_fixture;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.reasoning.wires` — twenty handcrafted schema leaves.
pub fn wires_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.reasoning.wires",
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
    use crate::artifacts::wires::schema::diff::WiresDiff;
    use crate::artifacts::wires::schema::mutations::WiresMutation;
    use crate::artifacts::wires::schema::snapshot::WiresSnapshot;

    #[derive(Clone, Debug)]
    pub struct WiresBuilderConstruction {
        snapshot: WiresSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for WiresBuilderConstruction {
        type Snapshot = WiresSnapshot;
        type Mutation = WiresMutation;
        type Diff = WiresDiff;
        fn empty() -> Self { Self { snapshot: crate::artifacts::wires::empty_wires_snapshot(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::wires::WiresSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct WiresParts {
        pub snapshot: Option<WiresSnapshot>,
    }

    pub struct WiresAnalyzerAnalysis;

    impl ArtifactAnalysis for WiresAnalyzerAnalysis {
        type Parts = WiresParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.wires", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = WiresParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <WiresSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <WiresSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec WiresBuilderFacets {
        construction: derived_construction::WiresBuilderConstruction,
        analysis: derived_analysis::WiresAnalyzerAnalysis,
        composition: super::super::io::derived_composition::WiresComposerComposition,
    }
    builder: WiresBuilder,
    analyzer: WiresAnalyzer,
    composer: WiresComposer,
);
//#endregion 🧬️DerivedArtifactFacets
