//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.


use std::collections::BTreeMap;

use schema::ArtifactSchema;
use crate::artifacts::vdi3805::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Artifact {
    #[state(persistent)] pub manufacturer_file: ManufacturerFile,
    #[state(persistent)] pub catalog: ManufacturerCatalog,
    #[state(persistent)] pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(persistent)] pub correction_as_of: EditionId,
    #[state(persistent)] pub strict_mode: bool,
    #[state(persistent)] pub index: CatalogIndex,
    #[state(persistent)] pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(persistent)] pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(persistent)] pub limits: SecurityLimits,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Vdi3805Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::vdi3805::Vdi3805Snapshot {
        crate::artifacts::vdi3805::Vdi3805Snapshot {
            manufacturer_file: self.manufacturer_file.clone(),
            catalog: self.catalog.clone(),
            edition_profile: self.edition_profile.clone(),
            correction_as_of: self.correction_as_of.clone(),
            strict_mode: self.strict_mode,
            index: self.index.clone(),
            geometry: self.geometry.clone(),
            curves: self.curves.clone(),
            limits: self.limits.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::vdi3805::Vdi3805Snapshot) -> Self {
        Self {
            manufacturer_file: snapshot.manufacturer_file,
            catalog: snapshot.catalog,
            edition_profile: snapshot.edition_profile,
            correction_as_of: snapshot.correction_as_of,
            strict_mode: snapshot.strict_mode,
            index: snapshot.index,
            geometry: snapshot.geometry,
            curves: snapshot.curves,
            limits: snapshot.limits,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::vdi3805::Vdi3805Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.vdi3805` — twenty handcrafted schema leaves.
pub fn vdi3805_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.vdi3805",
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
    use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805BuilderConstruction {
        snapshot: Vdi3805Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Vdi3805BuilderConstruction {
        type Snapshot = Vdi3805Snapshot;
        type Mutation = Vdi3805Mutation;
        type Diff = Vdi3805Diff;
        fn empty() -> Self { Self { snapshot: Vdi3805Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::vdi3805::Vdi3805Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805Parts {
        pub snapshot: Option<Vdi3805Snapshot>,
    }

    pub struct Vdi3805AnalyzerAnalysis;

    impl ArtifactAnalysis for Vdi3805AnalyzerAnalysis {
        type Parts = Vdi3805Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Vdi3805Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Vdi3805BuilderFacets {
        construction: derived_construction::Vdi3805BuilderConstruction,
        analysis: derived_analysis::Vdi3805AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Vdi3805ComposerComposition,
    }
    builder: Vdi3805Builder,
    analyzer: Vdi3805Analyzer,
    composer: Vdi3805Composer,
);
//#endregion 🧬️DerivedArtifactFacets
