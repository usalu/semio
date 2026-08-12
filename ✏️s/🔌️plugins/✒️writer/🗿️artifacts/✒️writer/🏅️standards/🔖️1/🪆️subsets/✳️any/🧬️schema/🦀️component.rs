//! 🧬️ Writer artifact schema — every field with its state class.

use crate::artifacts::writer::{WriterEditorSelection, WriterEditorSettings, WRITER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full writer artifact across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub language_id: String,
    #[state(persistent)] pub uri: String,
    #[state(persistent)] pub text: String,
    #[state(shared_ui)] pub selected_ast_ids: Vec<String>,
    #[state(shared_ui)] pub editor_selection: Option<WriterEditorSelection>,
    #[state(shared_ui)] pub editor_settings: WriterEditorSettings,
    #[state(local_ui)] pub format_signal: u32,
    #[state(local_ui)] pub lint_signal: u32,
    #[state(local_ui)] pub revision: u32,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub tree_hovered_ast_id: Option<String>,
    #[state(preview)] pub editor_hover_offset: Option<usize>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WriterArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::writer::WriterSnapshot::default())
    }
}

impl WriterArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::writer::WriterSnapshot {
        crate::artifacts::writer::WriterSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            language_id: self.language_id.clone(),
            uri: self.uri.clone(),
            text: self.text.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot with UI defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::writer::WriterSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            language_id: snapshot.language_id,
            uri: snapshot.uri,
            text: snapshot.text,
            ..Self::default_ui()
        }
    }

    fn default_ui() -> Self {
        Self {
            schema: WRITER_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            language_id: "plaintext".into(),
            uri: crate::artifacts::writer::default_uri(),
            text: String::new(),
            selected_ast_ids: Vec::new(),
            editor_selection: None,
            editor_settings: WriterEditorSettings::default(),
            format_signal: 0,
            lint_signal: 0,
            revision: 0,
            engagement_input: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
            tree_hovered_ast_id: None,
            editor_hover_offset: None,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::writer::WriterSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.language_id = snapshot.language_id;
        self.uri = snapshot.uri;
        self.text = snapshot.text;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.writer.writer` — twenty handcrafted schema leaves.
pub fn writer_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.writer.writer",
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
    use crate::artifacts::writer::{WriterDiff, WriterMutation, WriterSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct WriterBuilderConstruction {
        snapshot: WriterSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for WriterBuilderConstruction {
        type Snapshot = WriterSnapshot;
        type Mutation = WriterMutation;
        type Diff = WriterDiff;
        fn empty() -> Self { Self { snapshot: WriterSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<WriterSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WriterSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::writer::schema::mutations::apply_writer_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <WriterDiff as protocol::MutationDiff<WriterSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::writer::WriterSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct WriterParts {
        pub snapshot: Option<WriterSnapshot>,
    }

    pub struct WriterAnalyzerAnalysis;

    impl ArtifactAnalysis for WriterAnalyzerAnalysis {
        type Parts = WriterParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.writer", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = WriterParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <WriterSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec WriterBuilderFacets {
        construction: derived_construction::WriterBuilderConstruction,
        analysis: derived_analysis::WriterAnalyzerAnalysis,
        composition: super::super::io::derived_composition::WriterComposerComposition,
    }
    builder: WriterBuilder,
    analyzer: WriterAnalyzer,
    composer: WriterComposer,
);
//#endregion 🧬️DerivedArtifactFacets
