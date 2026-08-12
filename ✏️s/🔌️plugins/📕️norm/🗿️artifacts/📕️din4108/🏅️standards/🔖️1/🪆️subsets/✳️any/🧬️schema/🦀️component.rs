//! 🧬️ Din4108 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Din4108 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Artifact {
    #[state(persistent)] pub category: String,
    #[state(persistent)] pub layers: Vec<crate::artifacts::din4108::LayerDocument>,
    #[state(persistent)] pub climate: crate::document::ClimateZoneDe,
    #[state(persistent)] pub airtightness_n50: f64,
    #[state(persistent)] pub psi_times_l_sum: f64,
    #[state(persistent)] pub rh_int: f64,
    #[state(persistent)] pub catalog_id: String,
    #[state(persistent)] pub material_id: String,
    #[state(persistent)] pub airtightness_class: String,
    #[state(persistent)] pub t_int_c: f64,
    #[state(persistent)] pub solar_absorptance: f64,
    #[state(persistent)] pub irradiance_w_m2: f64,
    #[state(persistent)] pub moisture_mu_exterior: f64,
    #[state(persistent)] pub moisture_mu_interior: f64,
    #[state(persistent)] pub envelope_area_m2: f64,
    #[state(persistent)] pub bb2_details_conform: bool,
    #[state(persistent)] pub application_type: String,
    #[state(persistent)] pub declared_application_class: String,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din4108Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::din4108::Din4108Snapshot {
        crate::artifacts::din4108::Din4108Snapshot {
            category: self.category.clone(),
            layers: self.layers.clone(),
            climate: self.climate,
            airtightness_n50: self.airtightness_n50,
            psi_times_l_sum: self.psi_times_l_sum,
            rh_int: self.rh_int,
            catalog_id: self.catalog_id.clone(),
            material_id: self.material_id.clone(),
            airtightness_class: self.airtightness_class.clone(),
            t_int_c: self.t_int_c,
            solar_absorptance: self.solar_absorptance,
            irradiance_w_m2: self.irradiance_w_m2,
            moisture_mu_exterior: self.moisture_mu_exterior,
            moisture_mu_interior: self.moisture_mu_interior,
            envelope_area_m2: self.envelope_area_m2,
            bb2_details_conform: self.bb2_details_conform,
            application_type: self.application_type.clone(),
            declared_application_class: self.declared_application_class.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::din4108::Din4108Snapshot) -> Self {
        Self {
            category: snapshot.category.clone(),
            layers: snapshot.layers.clone(),
            climate: snapshot.climate,
            airtightness_n50: snapshot.airtightness_n50,
            psi_times_l_sum: snapshot.psi_times_l_sum,
            rh_int: snapshot.rh_int,
            catalog_id: snapshot.catalog_id.clone(),
            material_id: snapshot.material_id.clone(),
            airtightness_class: snapshot.airtightness_class.clone(),
            t_int_c: snapshot.t_int_c,
            solar_absorptance: snapshot.solar_absorptance,
            irradiance_w_m2: snapshot.irradiance_w_m2,
            moisture_mu_exterior: snapshot.moisture_mu_exterior,
            moisture_mu_interior: snapshot.moisture_mu_interior,
            envelope_area_m2: snapshot.envelope_area_m2,
            bb2_details_conform: snapshot.bb2_details_conform,
            application_type: snapshot.application_type.clone(),
            declared_application_class: snapshot.declared_application_class.clone(),
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::din4108::Din4108Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din4108` — twenty handcrafted schema leaves.
pub fn din4108_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.din4108",
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
    use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Din4108BuilderConstruction {
        snapshot: Din4108Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Din4108BuilderConstruction {
        type Snapshot = Din4108Snapshot;
        type Mutation = Din4108Mutation;
        type Diff = Din4108Diff;
        fn empty() -> Self { Self { snapshot: Din4108Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::din4108::Din4108Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Din4108Parts {
        pub snapshot: Option<Din4108Snapshot>,
    }

    pub struct Din4108AnalyzerAnalysis;

    impl ArtifactAnalysis for Din4108AnalyzerAnalysis {
        type Parts = Din4108Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.din4108", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Din4108Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Din4108BuilderFacets {
        construction: derived_construction::Din4108BuilderConstruction,
        analysis: derived_analysis::Din4108AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Din4108ComposerComposition,
    }
    builder: Din4108Builder,
    analyzer: Din4108Analyzer,
    composer: Din4108Composer,
);
//#endregion 🧬️DerivedArtifactFacets
