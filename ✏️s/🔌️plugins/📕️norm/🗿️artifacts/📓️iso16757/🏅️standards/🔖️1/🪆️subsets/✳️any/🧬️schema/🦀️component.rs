//! 🧬️ Iso16757 artifact schema — every field of the artifact with its state class.


use std::collections::BTreeMap;

use schema::ArtifactSchema;
use crate::artifacts::iso16757::CatalogueValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Iso16757 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Artifact {
    #[state(persistent)] pub catalogue: crate::artifacts::iso16757::part_1::Catalogue,
    #[state(persistent)] pub dictionary: crate::artifacts::iso16757::part_4::Dictionary,
    #[state(persistent)] pub geometry: crate::artifacts::iso16757::part_2::GeometryCatalogue,
    #[state(persistent)] pub selection: crate::artifacts::iso16757::part_1::SelectionRequest,
    #[state(persistent)] pub part_number_rule: crate::artifacts::iso16757::part_5::PartNumberRule,
    #[state(persistent)] pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(persistent)] pub script_limits: crate::artifacts::iso16757::part_5::ScriptLimits,
    #[state(persistent)] pub exchange_process: crate::artifacts::iso16757::part_5::ExchangeProcess,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Iso16757Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::iso16757::Iso16757Snapshot {
        crate::artifacts::iso16757::Iso16757Snapshot {
            catalogue: self.catalogue.clone(),
            dictionary: self.dictionary.clone(),
            geometry: self.geometry.clone(),
            selection: self.selection.clone(),
            part_number_rule: self.part_number_rule.clone(),
            part_number_inputs: self.part_number_inputs.clone(),
            script_limits: self.script_limits.clone(),
            exchange_process: self.exchange_process.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::iso16757::Iso16757Snapshot) -> Self {
        Self {
            catalogue: snapshot.catalogue,
            dictionary: snapshot.dictionary,
            geometry: snapshot.geometry,
            selection: snapshot.selection,
            part_number_rule: snapshot.part_number_rule,
            part_number_inputs: snapshot.part_number_inputs,
            script_limits: snapshot.script_limits,
            exchange_process: snapshot.exchange_process,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::iso16757::Iso16757Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.iso16757` — twenty handcrafted schema leaves.
pub fn iso16757_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.iso16757",
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
    use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Iso16757BuilderConstruction {
        snapshot: Iso16757Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Iso16757BuilderConstruction {
        type Snapshot = Iso16757Snapshot;
        type Mutation = Iso16757Mutation;
        type Diff = Iso16757Diff;
        fn empty() -> Self { Self { snapshot: Iso16757Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::iso16757::Iso16757Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Iso16757Parts {
        pub snapshot: Option<Iso16757Snapshot>,
    }

    pub struct Iso16757AnalyzerAnalysis;

    impl ArtifactAnalysis for Iso16757AnalyzerAnalysis {
        type Parts = Iso16757Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.iso16757", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Iso16757Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Iso16757BuilderFacets {
        construction: derived_construction::Iso16757BuilderConstruction,
        analysis: derived_analysis::Iso16757AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Iso16757ComposerComposition,
    }
    builder: Iso16757Builder,
    analyzer: Iso16757Analyzer,
    composer: Iso16757Composer,
);
//#endregion 🧬️DerivedArtifactFacets
