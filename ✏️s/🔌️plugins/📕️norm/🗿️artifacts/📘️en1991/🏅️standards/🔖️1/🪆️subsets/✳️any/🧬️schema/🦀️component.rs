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

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1991 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. Every `part_1_N`/`part_N` module is a pure function
/// library; the snapshot-level composition (`evaluate`, `check_full_actions`, `check_floor_actions`)
/// lives in `💡️inferences`.
use crate::document::{AnnexChoice, CheckResult, ClauseId, ImposedCategory, NationalAnnex, Quantity};
use crate::artifacts::en1991::part_1_2::FireCurve;

// #region 🔖️NaDe
pub mod na_de {
    pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::na_de::NaDe;

    /// ❄️ German snow zone per DIN EN 1991-1-3/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SnowZone {
        Zone1,
        Zone2,
        Zone3,
    }

    impl SnowZone {
        pub fn as_u8(self) -> u8 {
            match self {
                Self::Zone1 => 1,
                Self::Zone2 => 2,
                Self::Zone3 => 3,
            }
        }

        pub fn s_k_kn_m2(self) -> f64 {
            match self {
                Self::Zone1 => 0.65,
                Self::Zone2 => 0.85,
                Self::Zone3 => 1.10,
            }
        }
    }

    /// 🌬️ German wind zone per DIN EN 1991-1-4/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WindZone {
        Zone1,
        Zone2,
        Zone3,
        Zone4,
    }

    impl WindZone {
        pub fn v_b_m_s(self) -> f64 {
            match self {
                Self::Zone1 => 22.5,
                Self::Zone2 => 25.0,
                Self::Zone3 => 27.5,
                Self::Zone4 => 30.0,
            }
        }
    }

    pub fn ground_snow_load(zone: SnowZone) -> f64 {
        zone.s_k_kn_m2()
    }

    pub fn basic_wind_velocity(zone: WindZone) -> f64 {
        zone.v_b_m_s()
    }
}
// #endregion 🔖️NaDe

// #region 🔖️Part1_1
pub mod part_1_1 {
    use super::*;

    /// 🧱️ Unit weight [kN/m³] per EN 1991-1-1 Annex A.
    pub fn self_weight_kn_m3(material: &str) -> f64 {
        match material {
            "concrete" => 25.0,
            "reinforced_concrete" => 25.0,
            "steel" => 78.5,
            "timber" => 5.0,
            "glulam" => 4.2,
            "masonry" => 18.0,
            "brick" => 20.0,
            "aluminium" => 27.0,
            "glass" => 25.0,
            "water" => 10.0,
            "sand" => 18.0,
            "gravel" => 20.0,
            "asphalt" => 23.0,
            _ => 20.0,
        }
    }

    /// 🧱️ Self-weight per unit area [kN/m²] of a layer of given thickness.
    pub fn self_weight_kn_m2(material: &str, thickness_m: f64) -> f64 {
        self_weight_kn_m3(material) * thickness_m
    }

    pub fn imposed_load_kn_m2(category: ImposedCategory) -> f64 {
        category.q_k_kn_m2()
    }

    pub fn check_imposed(area_m2: f64, category: ImposedCategory, annex: &dyn NationalAnnex) -> CheckResult {
        let q = imposed_load_kn_m2(category) * area_m2;
        let psi = annex.psi_0(category.label());
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-1", "Table 6.1", "q"), Quantity::force_kn(q * psi), Quantity::force_kn(q), "imposed load", annex.choice())
    }

    /// ✅️ Verify the assumed design dead load covers the material self-weight.
    pub fn check_self_weight(material: &str, thickness_m: f64, assumed_g_k_kn_m2: f64, annex: AnnexChoice) -> CheckResult {
        let g_k = self_weight_kn_m2(material, thickness_m);
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-1", "Annex A", "A.1"),
            Quantity::new(crate::document::QuantityKind::Pressure, g_k * 1000.0),
            Quantity::new(crate::document::QuantityKind::Pressure, assumed_g_k_kn_m2 * 1000.0),
            "self-weight vs assumed dead load",
            annex,
        )
    }
}
// #endregion 🔖️Part1_1

// #region 🔖️Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥️ ISO 834 standard temperature-time curve θ_g [°C], EN 1991-1-2 Eq. 3.4.
    pub fn standard_gas_temperature_c(t_min: f64) -> f64 {
        20.0 + 345.0 * (8.0 * t_min.max(0.0) + 1.0).log10()
    }

    /// 🔥️ External fire curve θ_g [°C], EN 1991-1-2 Annex B Eq. B.4.
    pub fn external_gas_temperature_c(t_min: f64) -> f64 {
        660.0 * (1.0 - 0.687 * (-0.32 * t_min).exp() - 0.313 * (-3.8 * t_min).exp()) + 20.0
    }

    /// 🔥️ Hydrocarbon fire curve θ_g [°C], EN 1991-1-2 Annex B Eq. B.5.
    pub fn hydrocarbon_gas_temperature_c(t_min: f64) -> f64 {
        1080.0 * (1.0 - 0.325 * (-0.167 * t_min).exp() - 0.675 * (-2.5 * t_min).exp()) + 20.0
    }

    pub fn gas_temperature_c(curve: FireCurve, t_min: f64) -> f64 {
        match curve {
            FireCurve::Standard => standard_gas_temperature_c(t_min),
            FireCurve::External => external_gas_temperature_c(t_min),
            FireCurve::Hydrocarbon => hydrocarbon_gas_temperature_c(t_min),
        }
    }

    /// ✅️ Verify the member's rated fire-resistance temperature capacity exceeds the gas temperature at t_min.
    pub fn check_fire_action(curve: FireCurve, t_min: f64, member_capacity_c: f64, annex: AnnexChoice) -> CheckResult {
        let theta_g = gas_temperature_c(curve, t_min);
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-2", "§3.2", "3.4"), Quantity::new(crate::document::QuantityKind::Temperature, theta_g), Quantity::new(crate::document::QuantityKind::Temperature, member_capacity_c), "fire gas temperature", annex)
    }
}
// #endregion 🔖️Part1_2

// #region 🔖️Part1_3
pub mod part_1_3 {
    use super::*;

    pub fn ground_snow_load_zone(zone: u8) -> f64 {
        match zone {
            1 => na_de::SnowZone::Zone1.s_k_kn_m2(),
            2 => na_de::SnowZone::Zone2.s_k_kn_m2(),
            3 => na_de::SnowZone::Zone3.s_k_kn_m2(),
            _ => na_de::SnowZone::Zone2.s_k_kn_m2(),
        }
    }

    pub fn roof_snow_load(s_k: f64, mu: f64) -> f64 {
        mu * s_k
    }

    pub fn altitude_correction(s_k: f64, altitude_m: f64, zone: u8) -> f64 {
        let delta_h = match zone {
            1 => 150.0,
            2 => 200.0,
            3 => 250.0,
            _ => 200.0,
        };
        if altitude_m <= delta_h {
            s_k
        } else {
            s_k * (1.0 + 0.001 * (altitude_m - delta_h).max(0.0))
        }
    }

    /// ❄️ Characteristic ground snow load: DE zone/altitude formula vs EN user-supplied s_k (NDP EN 1991-1-3/NA §4.1).
    pub fn design_ground_snow_load(annex: AnnexChoice, zone: u8, altitude_m: f64, en_s_k_kn_m2: f64) -> f64 {
        match annex {
            AnnexChoice::De => altitude_correction(ground_snow_load_zone(zone), altitude_m, zone),
            AnnexChoice::En => en_s_k_kn_m2,
        }
    }

    pub fn check_snow(s_kn_m2: f64, limit: f64, annex: &dyn NationalAnnex) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-3", "§5", "5.1"), Quantity::new(crate::document::QuantityKind::Pressure, s_kn_m2 * 1000.0), Quantity::new(crate::document::QuantityKind::Pressure, limit * 1000.0), "snow load", annex.choice())
    }
}
// #endregion 🔖️Part1_3

// #region 🔖️Part1_4
pub mod part_1_4 {
    use super::*;

    /// 🌬️ Terrain category per EN 1991-1-4 Table 4.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TerrainCategory {
        Zero,
        I,
        II,
        III,
        IV,
    }

    impl TerrainCategory {
        pub fn z_0_m(self) -> f64 {
            match self {
                Self::Zero => 0.003,
                Self::I => 0.01,
                Self::II => 0.05,
                Self::III => 0.3,
                Self::IV => 1.0,
            }
        }

        pub fn z_min_m(self) -> f64 {
            match self {
                Self::Zero => 1.0,
                Self::I => 1.0,
                Self::II => 2.0,
                Self::III => 5.0,
                Self::IV => 10.0,
            }
        }
    }

    /// 🌬️ Basic velocity pressure q_b [kN/m²] from v_b.
    pub fn basic_velocity_pressure(rho: f64, v_b_m_s: f64) -> f64 {
        0.5 * rho * v_b_m_s * v_b_m_s / 1000.0
    }

    /// 🌬️ Peak velocity pressure q_p [kN/m²] per EN 1991-1-4 Eq. (4.8).
    pub fn peak_velocity_pressure(rho: f64, v_b_m_s: f64, c_e: f64) -> f64 {
        c_e * basic_velocity_pressure(rho, v_b_m_s)
    }

    pub fn exposure_factor(z_m: f64, terrain: TerrainCategory) -> f64 {
        let z = z_m.max(terrain.z_min_m());
        let z_0 = terrain.z_0_m();
        let k_r = 0.19 * (z_0 / 0.05_f64).powf(0.07);
        let c_0 = k_r * (z / z_0).ln();
        c_0 * c_0
    }

    pub fn wind_pressure(q_p: f64, c_pe: f64, c_pi: f64) -> f64 {
        q_p * (c_pe - c_pi)
    }

    pub fn structural_factor(c_s: f64, c_d: f64) -> f64 {
        c_s * c_d
    }

    /// 🌬️ Basic wind velocity v_b: DE wind-zone table vs EN user-supplied value (NDP EN 1991-1-4/NA §4.2).
    pub fn design_basic_wind_velocity(annex: AnnexChoice, zone: u8, en_v_b_m_s: f64) -> f64 {
        match annex {
            AnnexChoice::De => match zone {
                1 => na_de::WindZone::Zone1.v_b_m_s(),
                2 => na_de::WindZone::Zone2.v_b_m_s(),
                3 => na_de::WindZone::Zone3.v_b_m_s(),
                _ => na_de::WindZone::Zone4.v_b_m_s(),
            },
            AnnexChoice::En => en_v_b_m_s,
        }
    }

    pub fn check_wind(w_p_kn_m2: f64, limit: f64, annex: &dyn NationalAnnex) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-4", "§5", "5.1"), Quantity::new(crate::document::QuantityKind::Pressure, w_p_kn_m2 * 1000.0), Quantity::new(crate::document::QuantityKind::Pressure, limit * 1000.0), "wind pressure", annex.choice())
    }
}
// #endregion 🔖️Part1_4

// #region 🔖️Part1_5
pub mod part_1_5 {
    use super::*;

    pub fn thermal_coefficient_alpha_k_inv() -> f64 {
        1.0e-5
    }

    pub fn temperature_difference_action(delta_t_k: f64, alpha: f64, e_modulus_gpa: f64) -> f64 {
        alpha * delta_t_k * e_modulus_gpa
    }

    pub fn check_temperature_action(delta_t_k: f64, limit_k: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-5", "§6", "6.1"), Quantity::new(crate::document::QuantityKind::Temperature, delta_t_k), Quantity::new(crate::document::QuantityKind::Temperature, limit_k), "thermal action", AnnexChoice::De)
    }

    pub fn check_fire_boundary_temperature(t_surface_k: f64, t_limit_k: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-5", "Annex A", "A.1"),
            Quantity::new(crate::document::QuantityKind::Temperature, t_surface_k),
            Quantity::new(crate::document::QuantityKind::Temperature, t_limit_k),
            "fire boundary temperature",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖️Part1_5

// #region 🔖️Part1_6
pub mod part_1_6 {
    use super::*;

    pub fn construction_load_kn_m2(activity: &str) -> f64 {
        match activity {
            "storage" => 2.0,
            "machinery" => 3.0,
            "scaffolding" => 1.0,
            _ => 0.5,
        }
    }

    pub fn check_construction_load(q_kn_m2: f64, limit: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-1-6", "§4", "4.1"), Quantity::force_kn(q_kn_m2), Quantity::force_kn(limit), "construction load", AnnexChoice::En)
    }
}
// #endregion 🔖️Part1_6

// #region 🔖️Part1_7
pub mod part_1_7 {
    use super::*;

    pub fn impact_force_kn(vehicle_mass_t: f64, speed_km_h: f64) -> f64 {
        0.5 * vehicle_mass_t * (speed_km_h / 3.6).powi(2) / 1000.0
    }

    pub fn explosion_pressure_kpa(mass_kg: f64, distance_m: f64) -> f64 {
        if distance_m < f64::EPSILON {
            return 0.0;
        }
        2.0 * mass_kg / (distance_m * distance_m)
    }

    pub fn check_accidental_pressure(p_kpa: f64, limit_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-1-7", "Annex B", "B.1"),
            Quantity::new(crate::document::QuantityKind::Pressure, p_kpa * 1000.0),
            Quantity::new(crate::document::QuantityKind::Pressure, limit_kpa * 1000.0),
            "accidental pressure",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖️Part1_7

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    pub fn lm1_udl_kn_m2(lane: u8) -> f64 {
        match lane {
            1 => 9.0,
            2 => 2.5,
            _ => 2.5,
        }
    }

    pub fn lm1_tandem_kn(lane: u8) -> f64 {
        match lane {
            1 => 300.0,
            2 => 200.0,
            _ => 200.0,
        }
    }

    /// 🌉️ α adjustment factor for LM1 tandem/UDL: DE-NA reduces lane 1 vs EN recommended 1.0 (DIN EN 1991-2/NA §4.3.2).
    pub fn alpha_q(annex: AnnexChoice, lane: u8) -> f64 {
        match (annex, lane) {
            (AnnexChoice::De, 1) => 0.9,
            (AnnexChoice::De, _) => 1.0,
            (AnnexChoice::En, _) => 1.0,
        }
    }

    /// 🌉️ Design tandem-system axle load [kN] including α_Q adjustment.
    pub fn lm1_design_tandem_kn(annex: AnnexChoice, lane: u8) -> f64 {
        alpha_q(annex, lane) * lm1_tandem_kn(lane)
    }

    /// 🌉️ Simply-supported mid-span bending moment [kNm] from LM1 tandem + UDL over a span.
    pub fn mid_span_moment_knm(span_m: f64, tandem_kn: f64, udl_kn_m2: f64, lane_width_m: f64) -> f64 {
        tandem_kn * span_m / 4.0 + udl_kn_m2 * lane_width_m * span_m * span_m / 8.0
    }

    pub fn check_imposed_bridge(lane_load_kn: f64, design_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-2", "§4", "4.3"), Quantity::force_kn(design_kn), Quantity::force_kn(lane_load_kn), "bridge imposed load", AnnexChoice::En)
    }

    /// ✅️ Check LM1-derived mid-span moment against section resistance.
    pub fn check_lm1_moment(annex: AnnexChoice, span_m: f64, lane: u8, lane_width_m: f64, resistance_knm: f64) -> CheckResult {
        let tandem = lm1_design_tandem_kn(annex, lane);
        let m_ed = mid_span_moment_knm(span_m, tandem, lm1_udl_kn_m2(lane), lane_width_m);
        CheckResult::from_utilization(ClauseId::new("EN 1991-2", "§4.3.2", "4.4"), Quantity::new(crate::document::QuantityKind::Moment, m_ed * 1000.0), Quantity::new(crate::document::QuantityKind::Moment, resistance_knm * 1000.0), "LM1 mid-span moment", annex)
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    pub fn crane_vertical_wheel_load(crane_class: &str) -> f64 {
        match crane_class {
            "HC1" => 50.0,
            "HC2" => 100.0,
            "HC3" => 160.0,
            "HC4" => 250.0,
            _ => 80.0,
        }
    }

    pub fn crane_horizontal_force_kn(vertical_load_kn: f64) -> f64 {
        0.1 * vertical_load_kn
    }

    /// 🏗️ Hoisting dynamic factor φ_2 per EN 1991-3 Table 2.4 (φ_2,min + β_2·v_h).
    pub fn phi_2(hoist_class: &str, hoisting_speed_m_s: f64) -> f64 {
        let (phi_2_min, beta_2) = match hoist_class {
            "HC1" => (1.05, 0.17),
            "HC2" => (1.10, 0.34),
            "HC3" => (1.15, 0.51),
            _ => (1.20, 0.68),
        };
        phi_2_min + beta_2 * hoisting_speed_m_s
    }

    /// 🏗️ Hoisting dynamic factor φ_1 per EN 1991-3 §2.4.2.1 (self-weight lift-off).
    pub const PHI_1: f64 = 1.1;

    /// 🏗️ Design vertical wheel load [kN] including hoisting dynamics.
    pub fn design_vertical_wheel_load(crane_class: &str, hoist_class: &str, hoisting_speed_m_s: f64) -> f64 {
        crane_vertical_wheel_load(crane_class) * PHI_1.max(phi_2(hoist_class, hoisting_speed_m_s))
    }

    pub fn check_crane_load(wheel_load_kn: f64, capacity_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1991-3", "§2", "2.3"), Quantity::force_kn(wheel_load_kn), Quantity::force_kn(capacity_kn), "crane wheel load", AnnexChoice::En)
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 🌾️ Janssen horizontal wall pressure p_h(z) [kPa] per EN 1991-4 Annex C Eq. C.4 (asymptotic silo pressure).
    pub fn janssen_horizontal_pressure_kpa(bulk_density_kn_m3: f64, hydraulic_radius_m: f64, mu: f64, k: f64, depth_m: f64) -> f64 {
        let asymptote = bulk_density_kn_m3 * hydraulic_radius_m / (mu * k);
        asymptote * (1.0 - (-depth_m * mu * k / hydraulic_radius_m).exp())
    }

    /// 🌾️ Legacy linear wall pressure surrogate, retained for simple hand checks.
    pub fn silo_wall_pressure_kpa(bulk_density_kn_m3: f64, height_m: f64, k: f64) -> f64 {
        k * bulk_density_kn_m3 * height_m
    }

    pub fn tank_hydrostatic_pressure_kpa(fluid_density_kn_m3: f64, fill_height_m: f64) -> f64 {
        fluid_density_kn_m3 * fill_height_m
    }

    pub fn check_silo_pressure(p_kpa: f64, limit_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1991-4", "§5", "5.1"),
            Quantity::new(crate::document::QuantityKind::Pressure, p_kpa * 1000.0),
            Quantity::new(crate::document::QuantityKind::Pressure, limit_kpa * 1000.0),
            "silo wall pressure",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖️Part4
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    #[test]
    fn snow_zone_2_ground_load() {
        assert!((part_1_3::ground_snow_load_zone(2) - 0.85).abs() < 1e-9);
        assert!((na_de::SnowZone::Zone2.s_k_kn_m2() - 0.85).abs() < 1e-9);
    }

    #[test]
    fn wind_peak_velocity_pressure_vb_25() {
        let q_b = part_1_4::basic_velocity_pressure(1.25, 25.0);
        assert!((q_b - 0.39).abs() < 0.01);
        let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
        let q_p = part_1_4::peak_velocity_pressure(1.25, 25.0, c_e);
        assert!(q_p > q_b);
    }

    #[test]
    fn imposed_categories_table_6_1() {
        assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::A), 2.0);
        assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::B), 2.5);
        assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::H), 20.0);
    }

    #[test]
    fn de_wind_zone_2_basic_velocity() {
        assert!((na_de::WindZone::Zone2.v_b_m_s() - 25.0).abs() < 1e-9);
    }

    #[test]
    fn snow_and_wind_de_vs_en_diverge_at_altitude() {
        let doc = crate::artifacts::en1991::En1991Snapshot { snow_altitude_m: 400.0, annex: AnnexChoice::De, ..crate::artifacts::en1991::En1991Snapshot::default() };
        let de_s_k = part_1_3::design_ground_snow_load(doc.annex, doc.snow_zone, doc.snow_altitude_m, doc.en_s_k_kn_m2);
        let en_s_k = part_1_3::design_ground_snow_load(AnnexChoice::En, doc.snow_zone, doc.snow_altitude_m, doc.en_s_k_kn_m2);
        assert!(de_s_k > en_s_k);
        assert!((en_s_k - doc.en_s_k_kn_m2).abs() < 1e-9);
    }

    #[test]
    fn bridge_lm1_alpha_q_diverges_de_vs_en() {
        let de = part_2::check_lm1_moment(AnnexChoice::De, 20.0, 1, 3.0, 3000.0);
        let en = part_2::check_lm1_moment(AnnexChoice::En, 20.0, 1, 3.0, 3000.0);
        assert!(de.computed.value < en.computed.value);
    }
}
//#endregion 🧪️ComplianceHelpersTests
