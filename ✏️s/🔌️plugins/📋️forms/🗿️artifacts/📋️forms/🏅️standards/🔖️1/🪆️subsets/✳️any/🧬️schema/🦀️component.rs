//! 🧬️ Forms artifact schema — every field of the artifact with its state class.

use crate::artifacts::forms::{FormStep, FORMS_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full forms artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub id: String,
    #[state(persistent)]
    pub version: String,
    #[state(persistent)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(persistent)]
    pub steps: Vec<FormStep>,
    #[state(shared_ui)]
    pub selected_ids: Vec<String>,
    #[state(local_ui)]
    pub current_step_index: u32,
    #[state(local_ui)]
    pub try_values_json: String,
    #[state(local_ui)]
    pub locale: String,
    #[state(local_ui)]
    pub contributions_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for FormsArtifact {
    fn default() -> Self {
        Self {
            schema: FORMS_DOCUMENT_SCHEMA.into(),
            id: "forms".into(),
            version: "1".into(),
            title: None,
            steps: Vec::new(),
            selected_ids: Vec::new(),
            current_step_index: 0,
            try_values_json: "{}".into(),
            locale: "en-US".into(),
            contributions_json: "[]".into(),
        }
    }
}

impl FormsArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::forms::FormsSnapshot {
        crate::artifacts::forms::FormsSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            version: self.version.clone(),
            title: self.title.clone(),
            steps: self.steps.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::forms::FormsSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            version: snapshot.version,
            title: snapshot.title,
            steps: snapshot.steps,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::forms::FormsSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.version = snapshot.version;
        self.title = snapshot.title;
        self.steps = snapshot.steps;
    }

}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.forms.forms` — twenty handcrafted schema leaves.
pub fn forms_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.forms.forms",
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
    use crate::artifacts::forms::{FormsDiff, FormMutation, FormsSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct FormsBuilderConstruction {
        snapshot: FormsSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for FormsBuilderConstruction {
        type Snapshot = FormsSnapshot;
        type Mutation = FormMutation;
        type Diff = FormsDiff;
        fn empty() -> Self { Self { snapshot: FormsSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<FormsSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = crate::artifacts::forms::schema::mutations::apply_form_edit_mutation(&self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::forms::FormsSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct FormsParts {
        pub snapshot: Option<FormsSnapshot>,
    }

    pub struct FormsAnalyzerAnalysis;

    impl ArtifactAnalysis for FormsAnalyzerAnalysis {
        type Parts = FormsParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.forms", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = FormsParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <FormsSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec FormsBuilderFacets {
        construction: derived_construction::FormsBuilderConstruction,
        analysis: derived_analysis::FormsAnalyzerAnalysis,
        composition: super::super::io::derived_composition::FormsComposerComposition,
    }
    builder: FormsBuilder,
    analyzer: FormsAnalyzer,
    composer: FormsComposer,
);
//#endregion 🧬️DerivedArtifactFacets
