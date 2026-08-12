//! 🧬️ Din18599 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use crate::artifacts::din18599::{MonthlyClimate, UseClass};
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Din18599 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Artifact {
    #[state(persistent)] pub use_class: crate::artifacts::din18599::UseClass,
    #[state(persistent)] pub heated_area_m2: f64,
    #[state(persistent)] pub occupants: u32,
    #[state(persistent)] pub h_t: f64,
    #[state(persistent)] pub h_v: f64,
    #[state(persistent)] pub climate: crate::artifacts::din18599::MonthlyClimate,
    #[state(persistent)] pub internal_gains_w_m2: f64,
    #[state(persistent)] pub solar_gains_kwh: f64,
    #[state(persistent)] pub system_losses_kwh: f64,
    #[state(persistent)] pub renewable_kwh: f64,
    #[state(persistent)] pub annual_limit_kwh: f64,
    #[state(persistent)] pub energy_carrier: String,
    #[state(persistent)] pub reference_q_p_kwh: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din18599Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::din18599::Din18599Snapshot {
        crate::artifacts::din18599::Din18599Snapshot {
            use_class: self.use_class,
            heated_area_m2: self.heated_area_m2,
            occupants: self.occupants,
            h_t: self.h_t,
            h_v: self.h_v,
            climate: self.climate.clone(),
            internal_gains_w_m2: self.internal_gains_w_m2,
            solar_gains_kwh: self.solar_gains_kwh,
            system_losses_kwh: self.system_losses_kwh,
            renewable_kwh: self.renewable_kwh,
            annual_limit_kwh: self.annual_limit_kwh,
            energy_carrier: self.energy_carrier.clone(),
            reference_q_p_kwh: self.reference_q_p_kwh,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::din18599::Din18599Snapshot) -> Self {
        Self {
            use_class: snapshot.use_class,
            heated_area_m2: snapshot.heated_area_m2,
            occupants: snapshot.occupants,
            h_t: snapshot.h_t,
            h_v: snapshot.h_v,
            climate: snapshot.climate,
            internal_gains_w_m2: snapshot.internal_gains_w_m2,
            solar_gains_kwh: snapshot.solar_gains_kwh,
            system_losses_kwh: snapshot.system_losses_kwh,
            renewable_kwh: snapshot.renewable_kwh,
            annual_limit_kwh: snapshot.annual_limit_kwh,
            energy_carrier: snapshot.energy_carrier.clone(),
            reference_q_p_kwh: snapshot.reference_q_p_kwh,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::din18599::Din18599Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din18599` — twenty handcrafted schema leaves.
pub fn din18599_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.din18599",
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
    use crate::artifacts::din18599::{Din18599Diff, Din18599Mutation, Din18599Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Din18599BuilderConstruction {
        snapshot: Din18599Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Din18599BuilderConstruction {
        type Snapshot = Din18599Snapshot;
        type Mutation = Din18599Mutation;
        type Diff = Din18599Diff;
        fn empty() -> Self { Self { snapshot: Din18599Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::din18599::Din18599Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Din18599Parts {
        pub snapshot: Option<Din18599Snapshot>,
    }

    pub struct Din18599AnalyzerAnalysis;

    impl ArtifactAnalysis for Din18599AnalyzerAnalysis {
        type Parts = Din18599Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.din18599", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Din18599Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Din18599BuilderFacets {
        construction: derived_construction::Din18599BuilderConstruction,
        analysis: derived_analysis::Din18599AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Din18599ComposerComposition,
    }
    builder: Din18599Builder,
    analyzer: Din18599Analyzer,
    composer: Din18599Composer,
);
//#endregion 🧬️DerivedArtifactFacets
