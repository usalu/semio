//! 🧬️ Cad artifact schema — every field of the artifact with its state class.

use crate::{CadDrawingChild, CadModelChild, CadSnapshot};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

#[cfg(test)]
#[path = "🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract_tests;

//#region 🔖️Artifact
/// 🧬️ cad document artifact state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub building_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub energy_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub drawings: Vec<CadDrawingChild>,
    #[state(artifact)]
    #[value(default)]
    pub references_by_model_definition_id: BTreeMap<String, CadReferenceList>,
    #[state(artifact)]
    #[value(default)]
    pub nodes: Vec<CadNode>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for CadArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::empty_cad_snapshot())
    }
}

impl CadArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> CadSnapshot {
        CadSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            shape_model: self.shape_model.clone(),
            building_model: self.building_model.clone(),
            energy_model: self.energy_model.clone(),
            structure_classic_model: self.structure_classic_model.clone(),
            drawings: self.drawings.clone(),
            references_by_model_definition_id: self.references_by_model_definition_id.clone(),
            nodes: self.nodes.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: CadSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            shape_model: snapshot.shape_model,
            building_model: snapshot.building_model,
            energy_model: snapshot.energy_model,
            structure_classic_model: snapshot.structure_classic_model,
            drawings: snapshot.drawings,
            references_by_model_definition_id: snapshot.references_by_model_definition_id,
            nodes: snapshot.nodes,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: CadSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.shape_model = snapshot.shape_model;
        self.building_model = snapshot.building_model;
        self.energy_model = snapshot.energy_model;
        self.structure_classic_model = snapshot.structure_classic_model;
        self.drawings = snapshot.drawings;
        self.references_by_model_definition_id = snapshot.references_by_model_definition_id;
        self.nodes = snapshot.nodes;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.cad.cad` — twenty handcrafted schema leaves.
pub fn cad_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.cad.cad",
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
    use crate::diff::schema::CadDiff;
    use crate::mutations::CadMutation;
    use crate::{CadSnapshot, CAD_PLAY_DOCUMENT_SCHEMA};
    use semio_framework_plugin::ArtifactBuilder;
    use std::collections::BTreeMap;

    //#region Builder
    fn empty_snapshot() -> CadSnapshot {
        CadSnapshot {
            schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            shape_model: None,
            building_model: None,
            energy_model: None,
            structure_classic_model: None,
            drawings: Vec::new(),
            references_by_model_definition_id: BTreeMap::new(),
            nodes: Vec::new(),
        }
    }

    /// Builds a `cad` snapshot.
    #[derive(Clone, Debug)]
    pub struct CadBuilderConstruction {
        snapshot: CadSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for CadBuilderConstruction {
        type Snapshot = CadSnapshot;
        type Mutation = CadMutation;
        type Diff = CadDiff;

        fn empty() -> Self {
            Self { snapshot: empty_snapshot(), diagnostics: Vec::new() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<CadSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<CadSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, &self.snapshot)?;
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
    //#endregion Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::CadSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct CadParts {
        pub snapshot: Option<CadSnapshot>,
    }

    pub struct CadAnalyzerAnalysis;

    impl ArtifactAnalysis for CadAnalyzerAnalysis {
        type Parts = CadParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.cad.cad", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = CadParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <CadSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <CadSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec CadBuilderFacets {
        construction: CadBuilderConstruction,
        analysis: CadAnalyzerAnalysis,
        composition: super::super::io::derived_composition::CadComposerComposition,
    }
    builder: CadBuilder,
    analyzer: CadAnalyzer,
    composer: CadComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
pub use crate::CadCamera;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::CadNode;
pub use crate::CadReferenceList;
//#endregion 🔁️Re-exports
