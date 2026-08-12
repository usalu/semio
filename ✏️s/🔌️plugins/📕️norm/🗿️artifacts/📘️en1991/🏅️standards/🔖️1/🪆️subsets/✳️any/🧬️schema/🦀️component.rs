//! 🧬️ En1991 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use crate::artifacts::en1991::part_1_2::FireCurve;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1991 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Artifact {
    #[state(persistent)] pub area_m2: f64,
    #[state(persistent)] pub category: crate::document::ImposedCategory,
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub self_weight_material: String,
    #[state(persistent)] pub self_weight_thickness_m: f64,
    #[state(persistent)] pub assumed_g_k_kn_m2: f64,
    #[state(persistent)] pub fire_curve: crate::artifacts::en1991::part_1_2::FireCurve,
    #[state(persistent)] pub fire_resistance_min: f64,
    #[state(persistent)] pub fire_member_capacity_c: f64,
    #[state(persistent)] pub snow_zone: u8,
    #[state(persistent)] pub snow_altitude_m: f64,
    #[state(persistent)] pub en_s_k_kn_m2: f64,
    #[state(persistent)] pub wind_zone: u8,
    #[state(persistent)] pub en_v_b_m_s: f64,
    #[state(persistent)] pub delta_t_k: f64,
    #[state(persistent)] pub construction_activity: String,
    #[state(persistent)] pub accidental_mass_t: f64,
    #[state(persistent)] pub accidental_speed_km_h: f64,
    #[state(persistent)] pub bridge_lane: u8,
    #[state(persistent)] pub bridge_span_m: f64,
    #[state(persistent)] pub bridge_lane_width_m: f64,
    #[state(persistent)] pub bridge_moment_resistance_knm: f64,
    #[state(persistent)] pub crane_class: String,
    #[state(persistent)] pub hoist_class: String,
    #[state(persistent)] pub hoisting_speed_m_s: f64,
    #[state(persistent)] pub silo_bulk_density_kn_m3: f64,
    #[state(persistent)] pub silo_height_m: f64,
    #[state(persistent)] pub silo_hydraulic_radius_m: f64,
    #[state(persistent)] pub silo_mu: f64,
    #[state(persistent)] pub silo_k: f64,
    #[state(persistent)] pub c_s: f64,
    #[state(persistent)] pub c_d: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1991Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1991::En1991Snapshot {
        crate::artifacts::en1991::En1991Snapshot {
            area_m2: self.area_m2,
            category: self.category,
            annex: self.annex,
            self_weight_material: self.self_weight_material.clone(),
            self_weight_thickness_m: self.self_weight_thickness_m,
            assumed_g_k_kn_m2: self.assumed_g_k_kn_m2,
            fire_curve: self.fire_curve,
            fire_resistance_min: self.fire_resistance_min,
            fire_member_capacity_c: self.fire_member_capacity_c,
            snow_zone: self.snow_zone,
            snow_altitude_m: self.snow_altitude_m,
            en_s_k_kn_m2: self.en_s_k_kn_m2,
            wind_zone: self.wind_zone,
            en_v_b_m_s: self.en_v_b_m_s,
            delta_t_k: self.delta_t_k,
            construction_activity: self.construction_activity.clone(),
            accidental_mass_t: self.accidental_mass_t,
            accidental_speed_km_h: self.accidental_speed_km_h,
            bridge_lane: self.bridge_lane,
            bridge_span_m: self.bridge_span_m,
            bridge_lane_width_m: self.bridge_lane_width_m,
            bridge_moment_resistance_knm: self.bridge_moment_resistance_knm,
            crane_class: self.crane_class.clone(),
            hoist_class: self.hoist_class.clone(),
            hoisting_speed_m_s: self.hoisting_speed_m_s,
            silo_bulk_density_kn_m3: self.silo_bulk_density_kn_m3,
            silo_height_m: self.silo_height_m,
            silo_hydraulic_radius_m: self.silo_hydraulic_radius_m,
            silo_mu: self.silo_mu,
            silo_k: self.silo_k,
            c_s: self.c_s,
            c_d: self.c_d,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1991::En1991Snapshot) -> Self {
        Self {
            area_m2: snapshot.area_m2,
            category: snapshot.category,
            annex: snapshot.annex,
            self_weight_material: snapshot.self_weight_material.clone(),
            self_weight_thickness_m: snapshot.self_weight_thickness_m,
            assumed_g_k_kn_m2: snapshot.assumed_g_k_kn_m2,
            fire_curve: snapshot.fire_curve,
            fire_resistance_min: snapshot.fire_resistance_min,
            fire_member_capacity_c: snapshot.fire_member_capacity_c,
            snow_zone: snapshot.snow_zone,
            snow_altitude_m: snapshot.snow_altitude_m,
            en_s_k_kn_m2: snapshot.en_s_k_kn_m2,
            wind_zone: snapshot.wind_zone,
            en_v_b_m_s: snapshot.en_v_b_m_s,
            delta_t_k: snapshot.delta_t_k,
            construction_activity: snapshot.construction_activity.clone(),
            accidental_mass_t: snapshot.accidental_mass_t,
            accidental_speed_km_h: snapshot.accidental_speed_km_h,
            bridge_lane: snapshot.bridge_lane,
            bridge_span_m: snapshot.bridge_span_m,
            bridge_lane_width_m: snapshot.bridge_lane_width_m,
            bridge_moment_resistance_knm: snapshot.bridge_moment_resistance_knm,
            crane_class: snapshot.crane_class.clone(),
            hoist_class: snapshot.hoist_class.clone(),
            hoisting_speed_m_s: snapshot.hoisting_speed_m_s,
            silo_bulk_density_kn_m3: snapshot.silo_bulk_density_kn_m3,
            silo_height_m: snapshot.silo_height_m,
            silo_hydraulic_radius_m: snapshot.silo_hydraulic_radius_m,
            silo_mu: snapshot.silo_mu,
            silo_k: snapshot.silo_k,
            c_s: snapshot.c_s,
            c_d: snapshot.c_d,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1991::En1991Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1991` — twenty handcrafted schema leaves.
pub fn en1991_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1991",
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
    use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1991BuilderConstruction {
        snapshot: En1991Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1991BuilderConstruction {
        type Snapshot = En1991Snapshot;
        type Mutation = En1991Mutation;
        type Diff = En1991Diff;
        fn empty() -> Self { Self { snapshot: En1991Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1991Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1991Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1991Mutation as protocol::Mutation<En1991Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1991Diff as protocol::MutationDiff<En1991Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1991Diff as protocol::MutationDiff<En1991Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::en1991::En1991Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1991Parts {
        pub snapshot: Option<En1991Snapshot>,
    }

    pub struct En1991AnalyzerAnalysis;

    impl ArtifactAnalysis for En1991AnalyzerAnalysis {
        type Parts = En1991Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1991", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1991Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1991Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1991Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1991BuilderFacets {
        construction: derived_construction::En1991BuilderConstruction,
        analysis: derived_analysis::En1991AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1991ComposerComposition,
    }
    builder: En1991Builder,
    analyzer: En1991Analyzer,
    composer: En1991Composer,
);
//#endregion 🧬️DerivedArtifactFacets
