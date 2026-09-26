//! 🧬️ Din16798 artifact schema + DIN EN 16798 compliance helpers.

use crate::document::{
    AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, OccupancyType, Quantity, QuantityKind, Remedy, RemedyBound, SubjectRef,
};
use crate::{VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub theta_rm_c: f64,
    #[state(artifact)]
    pub outdoor_co2_ppm: f64,
    #[state(artifact)]
    pub zones: Vec<ZoneDocument>,
    #[state(artifact)]
    pub vent_systems: Vec<VentSystemDocument>,
    #[state(artifact)]
    pub envelope_n50_h_inv: f64,
    #[state(artifact)]
    pub envelope_volume_m3: f64,
    #[state(artifact)]
    pub cellar_area_m2: f64,
    #[state(artifact)]
    pub cellar_ventilation_m3_h: f64,
    #[state(artifact)]
    pub night_setback_k: f64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din16798Artifact {
    pub fn to_snapshot(&self) -> crate::Din16798Snapshot {
        crate::Din16798Snapshot {
            annex: self.annex,
            theta_rm_c: self.theta_rm_c,
            outdoor_co2_ppm: self.outdoor_co2_ppm,
            zones: self.zones.clone(),
            vent_systems: self.vent_systems.clone(),
            envelope_n50_h_inv: self.envelope_n50_h_inv,
            envelope_volume_m3: self.envelope_volume_m3,
            cellar_area_m2: self.cellar_area_m2,
            cellar_ventilation_m3_h: self.cellar_ventilation_m3_h,
            night_setback_k: self.night_setback_k,
        }
    }
    pub fn from_snapshot(snapshot: crate::Din16798Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            theta_rm_c: snapshot.theta_rm_c,
            outdoor_co2_ppm: snapshot.outdoor_co2_ppm,
            zones: snapshot.zones,
            vent_systems: snapshot.vent_systems,
            envelope_n50_h_inv: snapshot.envelope_n50_h_inv,
            envelope_volume_m3: snapshot.envelope_volume_m3,
            cellar_area_m2: snapshot.cellar_area_m2,
            cellar_ventilation_m3_h: snapshot.cellar_ventilation_m3_h,
            night_setback_k: snapshot.night_setback_k,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: crate::Din16798Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din16798` — twenty handcrafted schema leaves.
pub fn din16798_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.din16798",
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
    use crate::{Din16798Diff, Din16798Mutation, Din16798Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Din16798BuilderConstruction {
        snapshot: Din16798Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Din16798BuilderConstruction {
        type Snapshot = Din16798Snapshot;
        type Mutation = Din16798Mutation;
        type Diff = Din16798Diff;
        fn empty() -> Self {
            Self { snapshot: Din16798Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Din16798Diff as protocol::MutationDiff<Din16798Snapshot>>::apply(&diff, &self.snapshot)?;
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
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::Din16798Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Din16798Parts {
        pub snapshot: Option<Din16798Snapshot>,
    }

    pub struct Din16798AnalyzerAnalysis;

    impl ArtifactAnalysis for Din16798AnalyzerAnalysis {
        type Parts = Din16798Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.din16798", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Din16798Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Din16798BuilderFacets {
        construction: Din16798BuilderConstruction,
        analysis: Din16798AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Din16798ComposerComposition,
    }
    builder: Din16798Builder,
    analyzer: Din16798Analyzer,
    composer: Din16798Composer,
);
//#endregion 🧬️DerivedArtifactFacets


//#region 🔖️ComplianceHelpers

/// 🎛️ Comfort evaluation model wire values (EN 16798-1 §7 / Annex A) — single source of truth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComfortModel {
    FixedHvac,
    Adaptive,
}

impl ComfortModel {
    pub fn parse(s: &str) -> Self {
        match s.trim() {
            "adaptive" => Self::Adaptive,
            _ => Self::FixedHvac,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixedHvac => "fixed_hvac",
            Self::Adaptive => "adaptive",
        }
    }
}

/// 🌬️ Ventilation system type wire values (EN 16798-3) — single source of truth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VentSystemType {
    CentralMech,
    DecentralMech,
    Natural,
}

impl VentSystemType {
    pub fn parse(s: &str) -> Self {
        match s.trim() {
            "decentral_mech" => Self::DecentralMech,
            "natural" => Self::Natural,
            _ => Self::CentralMech,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CentralMech => "central_mech",
            Self::DecentralMech => "decentral_mech",
            Self::Natural => "natural",
        }
    }
}


/// 🌬️ EN 16798-1 §6.3 ventilation design method wire values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VentMethod {
    Method1PerceivedAirQuality,
    Method2LimitConcentration,
    Method3PredefinedRates,
}

impl VentMethod {
    pub fn parse(s: &str) -> Self {
        match s.trim() {
            "method_2_limit_concentration" => Self::Method2LimitConcentration,
            "method_3_predefined_rates" => Self::Method3PredefinedRates,
            _ => Self::Method1PerceivedAirQuality,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Method1PerceivedAirQuality => "method_1_perceived_air_quality",
            Self::Method2LimitConcentration => "method_2_limit_concentration",
            Self::Method3PredefinedRates => "method_3_predefined_rates",
        }
    }
}


fn loc(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn zone_subject(zone: &ZoneDocument, path: String) -> SubjectRef {
    SubjectRef::new(
        &zone.id,
        path,
        loc(
            format!("Zone: {}", zone.name),
            format!("Raumzone: {}", zone.name),
        ),
    )
}

fn vent_subject(vent: &VentSystemDocument, path: String) -> SubjectRef {
    SubjectRef::new(
        &vent.id,
        path,
        loc(
            format!("Ventilation system «{}»", vent.name),
            format!("Lüftungsanlage «{}»", vent.name),
        ),
    )
}

pub fn parse_usage(s: &str) -> OccupancyType {
    match s.to_ascii_lowercase().as_str() {
        "residential" => OccupancyType::Residential,
        "classroom" => OccupancyType::Classroom,
        "retail" => OccupancyType::Retail,
        "meeting" => OccupancyType::Meeting,
        "kitchen" => OccupancyType::Kitchen,
        "corridor" => OccupancyType::Corridor,
        _ => OccupancyType::Office,
    }
}

/// 🏷️ EN 16798-1 IEQ category I–IV.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComfortCategory { I, II, III, IV }

pub fn parse_category(s: &str) -> ComfortCategory {
    match s.trim().to_ascii_uppercase().as_str() {
        "I" | "1" => ComfortCategory::I,
        "III" | "3" => ComfortCategory::III,
        "IV" | "4" => ComfortCategory::IV,
        _ => ComfortCategory::II,
    }
}

/// 🏭️ Building pollution class for ventilation rate method (EN 16798-1 Table B.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PollutionClass { VeryLow, Low, NonLow }

pub fn parse_pollution(s: &str) -> PollutionClass {
    match s.to_ascii_lowercase().replace('-', "_").as_str() {
        "very_low" | "verylow" => PollutionClass::VeryLow,
        "non_low" | "nonlow" | "high" => PollutionClass::NonLow,
        _ => PollutionClass::Low,
    }
}

pub mod annex_params {
    use super::*;
    pub struct AnnexParams {
        pub choice: AnnexChoice,
        pub acoustic_limit_residential_db: f64,
        pub co2_absolute_residential_ppm: f64,
        pub co2_absolute_classroom_ppm: f64,
        pub co2_absolute_other_ppm: f64,
    }
    impl AnnexParams {
        pub fn en() -> Self {
            Self { choice: AnnexChoice::En, acoustic_limit_residential_db: 30.0,
                co2_absolute_residential_ppm: 1500.0, co2_absolute_classroom_ppm: 1000.0, co2_absolute_other_ppm: 1000.0 }
        }
        pub fn de() -> Self {
            Self { choice: AnnexChoice::De, acoustic_limit_residential_db: 25.0,
                co2_absolute_residential_ppm: 1200.0, co2_absolute_classroom_ppm: 800.0, co2_absolute_other_ppm: 900.0 }
        }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice { AnnexChoice::En => Self::en(), AnnexChoice::De => Self::de() }
        }
    }
}

pub mod part_1 {
    pub use super::{ComfortCategory, PollutionClass};

    use super::*;
    use super::annex_params::AnnexParams;

    pub fn pmv_limit(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 0.2, ComfortCategory::II => 0.5, ComfortCategory::III => 0.7, ComfortCategory::IV => 1.0 }
    }
    pub fn ppd_limit(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 6.0, ComfortCategory::II => 10.0, ComfortCategory::III => 15.0, ComfortCategory::IV => 25.0 }
    }
    pub fn operative_temp_band(cat: ComfortCategory, season: &str) -> (f64, f64) {
        // EN 16798-1 Table B.2 design operative temperatures for offices (sedentary)
        match (cat, season) {
            (ComfortCategory::I, "winter") => (21.0, 23.0),
            (ComfortCategory::II, "winter") => (20.0, 24.0),
            (ComfortCategory::III, "winter") => (19.0, 25.0),
            (ComfortCategory::IV, "winter") => (18.0, 25.0),
            (ComfortCategory::I, _) => (23.5, 25.5),
            (ComfortCategory::II, _) => (23.0, 26.0),
            (ComfortCategory::III, _) => (22.0, 27.0),
            (ComfortCategory::IV, _) => (21.0, 28.0),
        }
    }
    pub fn humidity_band(cat: ComfortCategory) -> (f64, f64) {
        match cat {
            ComfortCategory::I => (30.0, 50.0),
            ComfortCategory::II => (25.0, 60.0),
            ComfortCategory::III => (20.0, 70.0),
            ComfortCategory::IV => (15.0, 75.0),
        }
    }
    pub fn co2_above_outdoor_ppm(cat: ComfortCategory) -> f64 {
        // EN 16798-1 Table B.6
        match cat { ComfortCategory::I => 550.0, ComfortCategory::II => 800.0, ComfortCategory::III => 1350.0, ComfortCategory::IV => 1350.0 }
    }
    pub fn illuminance_min_lx(usage: OccupancyType, cat: ComfortCategory) -> f64 {
        let base = match usage {
            OccupancyType::Office | OccupancyType::Meeting => 500.0,
            OccupancyType::Classroom => 300.0,
            OccupancyType::Retail => 300.0,
            OccupancyType::Kitchen => 500.0,
            OccupancyType::Corridor => 100.0,
            OccupancyType::Residential => 200.0,
        };
        match cat {
            ComfortCategory::I => base,
            ComfortCategory::II => base,
            ComfortCategory::III => base * 0.8,
            ComfortCategory::IV => base * 0.6,
        }
    }
    pub fn acoustic_limit_db(cat: ComfortCategory, usage: OccupancyType, annex: &AnnexParams) -> f64 {
        if matches!(usage, OccupancyType::Residential) {
            return annex.acoustic_limit_residential_db.min(match cat {
                ComfortCategory::I => 30.0, ComfortCategory::II => 35.0, ComfortCategory::III => 40.0, ComfortCategory::IV => 45.0,
            });
        }
        match cat { ComfortCategory::I => 30.0, ComfortCategory::II => 35.0, ComfortCategory::III => 40.0, ComfortCategory::IV => 45.0 }
    }
    pub fn adaptive_band_k(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 2.0, ComfortCategory::II => 3.0, ComfortCategory::III => 4.0, ComfortCategory::IV => 5.0 }
    }
    pub fn adaptive_comfort_temperature_c(theta_rm_c: f64) -> f64 { 0.33 * theta_rm_c + 18.8 }

    fn saturation_pressure_pa(t_c: f64) -> f64 { 611.2 * (17.67 * t_c / (t_c + 243.5)).exp() }

    fn solve_clothing_temp_c(t_a: f64, t_r: f64, m: f64, w: f64, i_cl: f64, f_cl: f64, v: f64) -> f64 {
        // ISO 7730 clothing temperature — use max(natural, forced) h_c and under-relaxation so winter clo≈1.0 converges.
        let mut t_cl = t_a + (35.5 - t_a) / (3.5 * i_cl + 1.0);
        for _ in 0..100 {
            let h_c_nat = 2.38 * (t_cl - t_a).abs().max(1e-6).powf(0.25);
            let h_c_for = 12.1 * v.max(0.0).sqrt();
            let h_c = h_c_nat.max(h_c_for).max(1e-3);
            let t_cl_k = t_cl + 273.15;
            let t_r_k = t_r + 273.15;
            let rad = 3.96e-8 * f_cl * (t_cl_k.powi(4) - t_r_k.powi(4));
            let conv = f_cl * h_c * (t_cl - t_a);
            let t_new = 35.7 - 0.028 * (m - w) - i_cl * (rad + conv);
            if !t_new.is_finite() {
                break;
            }
            if (t_new - t_cl).abs() < 0.001 {
                return t_new;
            }
            t_cl = 0.5 * (t_cl + t_new);
        }
        if t_cl.is_finite() { t_cl } else { t_a }
    }

    /// 😌️ ISO 7730 PMV full iteration with metabolic rate [met] and clothing [clo].
    pub fn pmv_iso7730(t_op_c: f64, rh_percent: f64, air_speed_m_s: f64, metabolic_rate_met: f64, clothing_clo: f64) -> f64 {
        let m = metabolic_rate_met * 58.15;
        let w = 0.0;
        let i_cl = 0.155 * clothing_clo;
        let f_cl = if i_cl <= 0.078 { 1.0 + 1.29 * i_cl } else { 1.05 + 0.645 * i_cl };
        let t_a = t_op_c;
        let t_r = t_op_c;
        let v = air_speed_m_s.max(0.0);
        let p_a = (rh_percent / 100.0).clamp(0.0, 1.0) * saturation_pressure_pa(t_a);
        let t_cl = solve_clothing_temp_c(t_a, t_r, m, w, i_cl, f_cl, v);
        let h_c = (2.38 * (t_cl - t_a).abs().max(1e-6).powf(0.25)).max(12.1 * v.sqrt()).max(1e-3);
        let t_cl_k = t_cl + 273.15;
        let t_r_k = t_r + 273.15;
        let e_r = 3.96e-8 * f_cl * (t_cl_k.powi(4) - t_r_k.powi(4));
        let e_c = f_cl * h_c * (t_cl - t_a);
        let e_sw = 3.05e-3 * (5733.0 - 6.99 * m - p_a).max(0.0);
        let e_diff = if m > 58.15 { 0.42 * (m - 58.15) } else { 0.0 };
        let e = e_sw + e_diff;
        let c_res = 1.7e-5 * m * (34.0 - t_a);
        let l = m - w - e - e_r - e_c - c_res;
        let pmv = (0.303 * (-0.035 * m).exp() + 0.028) * l; if pmv.is_finite() { pmv.clamp(-3.0, 3.0) } else { 0.0 }
    }

    pub fn ppd_from_pmv(pmv: f64) -> f64 {
        let pmv_c = pmv.clamp(-3.0, 3.0);
        100.0 - 95.0 * (-0.03353 * pmv_c.powi(4) - 0.2179 * pmv_c.powi(2)).exp()
    }

    /// 🔍 Bisect operative temperature so |PMV| ≤ limit (summer side).
    pub fn invert_t_op_for_pmv(rh: f64, v: f64, met: f64, clo: f64, limit: f64, prefer_cool: bool) -> f64 {
        let (mut lo, mut hi) = if prefer_cool { (16.0, 30.0) } else { (16.0, 30.0) };
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let pmv = pmv_iso7730(mid, rh, v, met, clo);
            if prefer_cool {
                if pmv > limit { hi = mid; } else { lo = mid; }
            } else {
                if pmv < -limit { lo = mid; } else { hi = mid; }
            }
        }
        0.5 * (lo + hi)
    }

    // --- ventilation rate method Table B.6/B.7 ---
    pub fn outdoor_air_per_person_l_s(cat: ComfortCategory) -> f64 {
        match cat { ComfortCategory::I => 10.0, ComfortCategory::II => 7.0, ComfortCategory::III => 4.0, ComfortCategory::IV => 2.5 }
    }
    pub fn outdoor_air_per_area_l_s_m2(cat: ComfortCategory, pollution: PollutionClass) -> f64 {
        let (a, b, c, d) = match pollution {
            PollutionClass::VeryLow => (0.5, 0.35, 0.2, 0.15),
            PollutionClass::Low => (1.0, 0.7, 0.4, 0.3),
            PollutionClass::NonLow => (2.0, 1.4, 0.8, 0.6),
        };
        match cat { ComfortCategory::I => a, ComfortCategory::II => b, ComfortCategory::III => c, ComfortCategory::IV => d }
    }
    /// 📐️ Required outdoor airflow [m³/h] = (n·q_p + A·q_a) · 3.6
    pub fn required_outdoor_air_m3_h(occupants: u32, area_m2: f64, cat: ComfortCategory, pollution: PollutionClass) -> f64 {
        let q_l_s = occupants as f64 * outdoor_air_per_person_l_s(cat) + area_m2 * outdoor_air_per_area_l_s_m2(cat, pollution);
        q_l_s * 3.6
    }

    /// 🏠 EN 16798-1 Annex B predefined total outdoor-air intensity [L/(s·m²)] for method 3.
    pub fn predefined_outdoor_air_l_s_m2(cat: ComfortCategory) -> f64 {
        match cat {
            ComfortCategory::I => 1.4,
            ComfortCategory::II => 1.0,
            ComfortCategory::III => 0.6,
            ComfortCategory::IV => 0.4,
        }
    }

    /// 🏭️ Pollution multiplier applied to method-3 area intensity (EN 16798-1 Annex B building-emission classes).
    pub fn pollution_scale(pollution: PollutionClass) -> f64 {
        match pollution {
            PollutionClass::VeryLow => 0.8,
            PollutionClass::Low => 1.0,
            PollutionClass::NonLow => 1.25,
        }
    }

    /// 🧪 Method 2 metabolic CO₂ generation [L/s] ≈ 0.005 L/s per person at 1.2 met.
    pub fn metabolic_co2_generation_l_s(occupants: u32, metabolic_rate_met: f64) -> f64 {
        occupants as f64 * 0.005 * (metabolic_rate_met / 1.2).max(0.5)
    }

    /// 🌬️ Required outdoor air [m³/h] for the selected EN 16798-1 §6.3 design method.
    pub fn required_outdoor_air_for_method(
        method: VentMethod,
        occupants: u32,
        area_m2: f64,
        cat: ComfortCategory,
        pollution: PollutionClass,
        metabolic_rate_met: f64,
        annex_co2_limit_above_ppm: f64,
    ) -> f64 {
        match method {
            VentMethod::Method1PerceivedAirQuality => required_outdoor_air_m3_h(occupants, area_m2, cat, pollution),
            VentMethod::Method3PredefinedRates => {
                // Annex B predefined rates combine area intensity (× pollution class) with a per-person term.
                let q_area = area_m2 * predefined_outdoor_air_l_s_m2(cat) * pollution_scale(pollution) * 3.6;
                let q_person = occupants as f64 * outdoor_air_per_person_l_s(cat) * 3.6;
                q_area + q_person
            },
            VentMethod::Method2LimitConcentration => {
                let g = metabolic_co2_generation_l_s(occupants, metabolic_rate_met);
                let delta = annex_co2_limit_above_ppm.max(100.0);
                (1.0e6 * g / delta) * 3.6
            }
        }
    }

    /// 💨 ISO 7730 draught rate DR [%].
    pub fn draught_rate_percent(t_a_c: f64, air_speed_m_s: f64, turbulence_intensity_percent: f64) -> f64 {
        let v = air_speed_m_s.max(0.05);
        let tu = turbulence_intensity_percent.max(0.0);
        let dr = (34.0 - t_a_c) * (v - 0.05).max(0.0).powf(0.62) * (0.37 * v * tu + 3.14);
        dr.clamp(0.0, 100.0)
    }

    pub fn draught_limit_percent(cat: ComfortCategory) -> f64 {
        match cat {
            ComfortCategory::I => 10.0,
            ComfortCategory::II => 20.0,
            ComfortCategory::III => 30.0,
            ComfortCategory::IV => 40.0,
        }
    }
}

pub mod part_3 {
    use super::*;

    pub const SFP_CLASS_BOUNDS_W_M3_S: [f64; 8] = [300.0, 500.0, 750.0, 1250.0, 2000.0, 3000.0, 4500.0, 6500.0]; // SFP 0..7

    pub fn sfp_bound(class: u8) -> f64 {
        let idx = (class as usize).min(7);
        SFP_CLASS_BOUNDS_W_M3_S[idx]
    }

    /// 🎯 Minimum heat-recovery temperature efficiency for balanced mechanical systems (EN 16798-3 / Ecodesign).
    pub fn heat_recovery_eta_min(system_type: &str) -> Option<f64> {
        match VentSystemType::parse(system_type) {
            VentSystemType::CentralMech | VentSystemType::DecentralMech => Some(0.73),
            VentSystemType::Natural => None,
        }
    }

    pub fn inspection_interval_years(system_type: &str) -> u32 {
        match VentSystemType::parse(system_type) {
            VentSystemType::DecentralMech => 5,
            _ => 3,
        }
    }

    /// 🧽 Required SUP filter class rank for ODA class (EN 16798-3 §7.2 / ISO 16890-1 ODA). Higher = finer.
    pub fn filter_rank(class: &str) -> u8 {
        match class {
            "ePM1_80_G" | "ePM1≥80+G" => 5,
            "ePM1_80" | "ePM1≥80" => 4,
            "ePM1_55" | "ePM1≥55" | "ePM1_50" => 3,
            "ePM2_5_65" | "ePM2.5_65" => 2,
            "ePM10_50" | "ePM10≥50" | "G4" => 1,
            _ => 0,
        }
    }
    pub fn required_filter_for_oda(oda: &str) -> (&'static str, u8) {
        match oda.to_ascii_uppercase().as_str() {
            "ODA1" => ("ePM10_50", 1),
            "ODA3" => ("ePM1_80", 4),
            "ODA4" => ("ePM1_80_G", 5),
            _ => ("ePM1_55", 3),
        }
    }

    pub fn duct_leakage_limit(class: &str, pressure_pa: f64) -> f64 {
        let c = match class.to_ascii_uppercase().as_str() {
            "A" => 0.027, "B" => 0.009, "C" => 0.003, "D" => 0.001, _ => 0.003,
        };
        c * pressure_pa.powf(0.65)
    }
}

pub mod part_5_1 {
    use super::*;
    pub fn fan_energy_kwh(sfp: f64, q_v: f64, t_run: f64) -> f64 { sfp * q_v * t_run / 1000.0 }
    pub fn night_setback_min_k(usage: OccupancyType) -> f64 {
        match usage {
            OccupancyType::Residential => 3.0,
            OccupancyType::Office | OccupancyType::Meeting | OccupancyType::Classroom => 4.0,
            _ => 2.0,
        }
    }
}

pub mod part_7 {
    use super::*;
    /// 🌬️ Infiltration volume flow [m³/h] ≈ n50 · V / 20 (shielding-corrected rule of thumb used with n50 limit).
    pub fn infiltration_m3_h(n50: f64, volume_m3: f64) -> f64 { n50 * volume_m3 / 20.0 }
    pub fn n50_max_mechanical() -> f64 { 3.0 }
    pub fn cellar_ventilation_required_m3_h(area_m2: f64) -> f64 { 0.3 * area_m2 }
}

pub mod part_17 {
    use super::part_3;
    pub use part_3::duct_leakage_limit;
}

/// ✅️ Evaluate the complete DIN EN 16798 building subject.
pub fn check_full_environment(doc: &crate::Din16798Snapshot) -> CheckReport {
    let annex = annex_params::AnnexParams::for_choice(doc.annex);
    let mut report = CheckReport::default();
    if doc.zones.is_empty() {
        report.push(CheckResult::assess(
            "din16798.zones.empty", "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7", "scope"),
            SubjectRef::whole(loc("Building", "Gebäude")),
            loc("Zone inventory", "Zonenverzeichnis"),
        ).not_applicable(loc("No zones defined.", "Keine Zonen definiert.")).annex(doc.annex).build());
    }
    for (zi, zone) in doc.zones.iter().enumerate() {
        report.extend(evaluate_zone(doc, zi, zone, &annex));
    }
    for (vi, vent) in doc.vent_systems.iter().enumerate() {
        report.extend(evaluate_vent(doc, vi, vent, &annex));
    }
    report.extend(evaluate_envelope(doc, &annex));
    report
}

fn evaluate_zone(doc: &crate::Din16798Snapshot, _zi: usize, zone: &ZoneDocument, annex: &annex_params::AnnexParams) -> Vec<CheckResult> {
    use part_1::*;
    let mut out = Vec::new();
    let cat = parse_category(&zone.comfort_category);
    let usage = parse_usage(&zone.usage_type);
    let pollution = parse_pollution(&zone.pollution_class);
    let method = VentMethod::parse(&zone.vent_method);
    let path = |field: &str| format!("zones[id={}].{field}", zone.id);
    let linked = doc.vent_systems.iter().find(|v| v.id == zone.vent_system_id);
    let linked_name = linked.map(|v| v.name.as_str()).unwrap_or("—");
    let linked_type = linked.map(|v| VentSystemType::parse(&v.system_type));

    // Referential integrity — zone must link an existing ventilation system
    {
        let mut bi = CheckResult::assess(
            format!("din16798.zone.ventSystem.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§6.3", "system"),
            zone_subject(zone, path("ventSystemId")), loc("Zone ventilation-system link", "Zonen-Lüftungsanlagen-Zuordnung"),
        ).annex(doc.annex);
        if linked.is_some() {
            bi = bi.status(crate::document::CheckStatus::Pass).explanation(loc(
                format!("Zone linked to ventilation system «{linked_name}» ({})", zone.vent_system_id),
                format!("Zone verknüpft mit Lüftungsanlage «{linked_name}» ({})", zone.vent_system_id),
            ));
        } else {
            let options: Vec<String> = doc.vent_systems.iter().map(|v| v.id.clone()).collect();
            bi = bi.status(crate::document::CheckStatus::Fail)
                .explanation(loc(
                    format!("Unresolved ventSystemId «{}» — no matching ventSystems[].id", zone.vent_system_id),
                    format!("Unaufgelöste ventSystemId «{}» — keine passende ventSystems[].id", zone.vent_system_id),
                ))
                .remedy({
                    let labels_en: Vec<String> = doc.vent_systems.iter().map(|v| format!("{} ({})", v.name, v.id)).collect();
                    let labels_de: Vec<String> = doc.vent_systems.iter().map(|v| format!("{} ({})", v.name, v.id)).collect();
                    Remedy {
                        target: zone_subject(zone, path("ventSystemId")),
                        current: Quantity::new(QuantityKind::Dimensionless, 0.0),
                        required: Quantity::new(QuantityKind::Dimensionless, 1.0),
                        bound: RemedyBound::OneOf,
                        options: if options.is_empty() { vec!["vent-central".into()] } else { options },
                        action: loc(
                            format!(
                                "Set ventSystemId to an existing system (unresolved «{}»). Choices: {}.",
                                zone.vent_system_id,
                                if labels_en.is_empty() { "—".into() } else { labels_en.join(", ") }
                            ),
                            format!(
                                "ventSystemId auf eine bestehende Anlage setzen (unaufgelöst «{}»). Auswahl: {}.",
                                zone.vent_system_id,
                                if labels_de.is_empty() { "—".into() } else { labels_de.join(", ") }
                            ),
                        ),
                        applicable: true,
                    }
                });
        }
        out.push(bi.build());
    }

    // Operative temperature — summer design
    let (t_min, t_max) = operative_temp_band(cat, "summer");
    let t = zone.t_op_summer_c;
    let mut b = CheckResult::assess(
        format!("din16798-1.top.summer.{}", zone.id), "DIN EN 16798-1",
        ClauseId::new("EN 16798-1", "Table B.2", "θ_op"),
        zone_subject(zone, path("tOpSummerC")),
        loc("Summer operative temperature", "Operative Sommertemperatur"),
    ).annex(doc.annex);
    if t < t_min {
        b = b.minimum(Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_min))
            .explanation(loc(format!("Summer θ_op={t:.1} °C is below the {t_min:.1} °C category floor."), format!("Operative Sommertemperatur θ_op={t:.1} °C unterschreitet die Kategorieuntergrenze {t_min:.1} °C.")))
            .remedy(Remedy::at_least(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_min),
                loc(format!("Raise summer operative temperature to at least {t_min:.1} °C."), format!("Operative Sommertemperatur auf mindestens {t_min:.1} °C erhöhen."))));
    } else if t > t_max {
        b = b.utilization(Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_max))
            .explanation(loc(format!("Summer θ_op={t:.1} °C exceeds the {t_max:.1} °C category ceiling."), format!("Operative Sommertemperatur θ_op={t:.1} °C überschreitet die Kategorieobergrenze {t_max:.1} °C.")))
            .remedy(Remedy::at_most(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_max),
                loc(format!("Lower summer operative temperature to at most {t_max:.1} °C."), format!("Operative Sommertemperatur auf höchstens {t_max:.1} °C senken."))));
    } else {
        b = b.utilization(Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_max))
            .explanation(loc(format!("Summer θ_op={t:.1} °C lies within {t_min:.1}–{t_max:.1} °C."), format!("Operative Sommertemperatur θ_op={t:.1} °C liegt im Band {t_min:.1}–{t_max:.1} °C.")));
    }
    out.push(b.build());

    // Winter operative temperature
    let (tw_min, tw_max) = operative_temp_band(cat, "winter");
    let tw = zone.t_op_winter_c;
    let mut bw = CheckResult::assess(
        format!("din16798-1.top.winter.{}", zone.id), "DIN EN 16798-1",
        ClauseId::new("EN 16798-1", "Table B.2", "θ_op,w"),
        zone_subject(zone, path("tOpWinterC")),
        loc("Winter operative temperature", "Operative Wintertemperatur"),
    ).annex(doc.annex);
    if tw < tw_min {
        bw = bw.minimum(Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_min))
            .explanation(loc(format!("Winter θ_op={tw:.1} °C is below the {tw_min:.1} °C category floor."), format!("Operative Wintertemperatur θ_op={tw:.1} °C unterschreitet die Kategorieuntergrenze {tw_min:.1} °C.")))
            .remedy(Remedy::at_least(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_min),
                loc(format!("Raise winter operative temperature to at least {tw_min:.1} °C."), format!("Operative Wintertemperatur auf mindestens {tw_min:.1} °C erhöhen."))));
    } else if tw > tw_max {
        bw = bw.utilization(Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_max))
            .explanation(loc(format!("Winter θ_op={tw:.1} °C exceeds the {tw_max:.1} °C category ceiling."), format!("Operative Wintertemperatur θ_op={tw:.1} °C überschreitet die Kategorieobergrenze {tw_max:.1} °C.")))
            .remedy(Remedy::at_most(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_max),
                loc(format!("Lower winter operative temperature to at most {tw_max:.1} °C."), format!("Operative Wintertemperatur auf höchstens {tw_max:.1} °C senken."))));
    } else {
        bw = bw.utilization(Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_max))
            .explanation(loc(format!("Winter θ_op={tw:.1} °C lies within {tw_min:.1}–{tw_max:.1} °C."), format!("Operative Wintertemperatur θ_op={tw:.1} °C liegt im Band {tw_min:.1}–{tw_max:.1} °C.")));
    }
    out.push(bw.build());

    // PMV / PPD (fixed HVAC, both seasons) vs adaptive Annex B.2 — ComfortModel wire is the sole gate.
    let clo_summer = zone.clothing_clo;
    let clo_winter = 1.0; // EN 16798-1 typical winter clothing when only one clo field is stored
    if ComfortModel::parse(&zone.comfort_model) == ComfortModel::Adaptive {
        for (suffix, title_en, title_de) in [
            ("pmv.summer", "Summer PMV (ISO 7730)", "Sommer-PMV (ISO 7730)"),
            ("ppd.summer", "Summer PPD (ISO 7730)", "Sommer-PPD (ISO 7730)"),
            ("pmv.winter", "Winter PMV (ISO 7730)", "Winter-PMV (ISO 7730)"),
            ("ppd.winter", "Winter PPD (ISO 7730)", "Winter-PPD (ISO 7730)"),
        ] {
            out.push(CheckResult::assess(
                format!("din16798-1.{suffix}.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2", "PMV"),
                zone_subject(zone, path("comfortModel")), loc(title_en, title_de),
            ).annex(doc.annex)
            .not_applicable(loc(
                "Adaptive model selected; PMV/PPD (ISO 7730) are not evaluated under Annex B.2.",
                "Adaptives Modell gewählt; PMV/PPD (ISO 7730) werden unter Anhang B.2 nicht bewertet.",
            )).build());
        }
        let centre = adaptive_comfort_temperature_c(doc.theta_rm_c);
        let band = adaptive_band_k(cat);
        let deviation = (zone.t_op_summer_c - centre).abs();
        let mut ba = CheckResult::assess(
            format!("din16798-1.adaptive.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B.2", "θ_c"),
            zone_subject(zone, path("tOpSummerC")), loc("Adaptive comfort (running-mean)", "Adaptiver Komfort (Außenluftmittel)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Temperature, deviation), Quantity::new(QuantityKind::Temperature, band))
        .explanation(loc(
            format!("|θ_op−θ_c|={deviation:.2} K; Cat band ±{band:.1} K around θ_c={centre:.1} °C (θ_rm={:.1} °C)", doc.theta_rm_c),
            format!("|θ_op−θ_c|={deviation:.2} K; Kat.-Band ±{band:.1} K um θ_c={centre:.1} °C (θ_rm={:.1} °C)", doc.theta_rm_c),
        ));
        if deviation > band {
            let target = centre + band.copysign(zone.t_op_summer_c - centre);
            ba = ba.remedy(Remedy::at_most(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, target),
                loc(format!("Adjust operative temperature toward {centre:.1} °C (within ±{band:.1} K)."), format!("Operative Temperatur Richtung {centre:.1} °C anpassen (innerhalb ±{band:.1} K)."))));
        }
        out.push(ba.build());
    } else {
        out.push(CheckResult::assess(
            format!("din16798-1.adaptive.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B.2", "θ_c"),
            zone_subject(zone, path("comfortModel")),
            loc("Adaptive comfort (running-mean)", "Adaptiver Komfort (Außenluftmittel)"),
        ).annex(doc.annex)
        .not_applicable(loc(
            "Fixed HVAC comfort model selected; Annex B.2 adaptive comfort does not apply.",
            "Festes HVAC-Komfortmodell gewählt; adaptiver Komfort nach Anhang B.2 gilt nicht.",
        )).build());

        let lim = pmv_limit(cat);
        let plim = ppd_limit(cat);
        // Summer PMV/PPD — subject clothing (design summer clo)
        let pmv_s = pmv_iso7730(zone.t_op_summer_c, zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer);
        let mut bp_s = CheckResult::assess(
            format!("din16798-1.pmv.summer.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PMV"),
            zone_subject(zone, path("tOpSummerC")), loc("Summer PMV (ISO 7730)", "Sommer-PMV (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, pmv_s.abs()), Quantity::new(QuantityKind::Dimensionless, lim))
        .explanation(loc(format!("Summer PMV={pmv_s:.3} at {clo_summer:.2} clo; |PMV|≤{lim}"), format!("Sommer-PMV={pmv_s:.3} bei {clo_summer:.2} clo; |PMV|≤{lim}")));
        if pmv_s.abs() > lim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer, lim, pmv_s > 0.0);
            bp_s = bp_s.remedy(Remedy::exactly(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Set summer operative temperature to ≈{t_req:.1} °C so |PMV|≤{lim}."), format!("Operative Sommertemperatur auf ≈{t_req:.1} °C setzen, damit |PMV|≤{lim}."))));
        }
        out.push(bp_s.build());
        let ppd_s = ppd_from_pmv(pmv_s);
        let mut bppd_s = CheckResult::assess(
            format!("din16798-1.ppd.summer.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PPD"),
            zone_subject(zone, path("tOpSummerC")), loc("Summer PPD (ISO 7730)", "Sommer-PPD (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, ppd_s), Quantity::new(QuantityKind::Dimensionless, plim))
        .explanation(loc(format!("Summer PPD={ppd_s:.1} %; limit {plim:.0} %"), format!("Sommer-PPD={ppd_s:.1} %; Grenzwert {plim:.0} %")));
        if ppd_s > plim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer, lim, pmv_s > 0.0);
            bppd_s = bppd_s.remedy(Remedy::exactly(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Adjust summer θ_op to ≈{t_req:.1} °C to keep PPD ≤{plim:.0} %."), format!("Operative Sommertemperatur auf ≈{t_req:.1} °C anpassen, damit PPD ≤{plim:.0} %."))));
        }
        out.push(bppd_s.build());

        // Winter PMV/PPD — EN 16798-1 typical winter clothing 1.0 clo
        let pmv_w = pmv_iso7730(zone.t_op_winter_c, zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter);
        let mut bp_w = CheckResult::assess(
            format!("din16798-1.pmv.winter.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PMV"),
            zone_subject(zone, path("tOpWinterC")), loc("Winter PMV (ISO 7730)", "Winter-PMV (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, pmv_w.abs()), Quantity::new(QuantityKind::Dimensionless, lim))
        .explanation(loc(format!("Winter PMV={pmv_w:.3} at {clo_winter:.2} clo; |PMV|≤{lim}"), format!("Winter-PMV={pmv_w:.3} bei {clo_winter:.2} clo; |PMV|≤{lim}")));
        if pmv_w.abs() > lim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter, lim, pmv_w > 0.0);
            bp_w = bp_w.remedy(Remedy::exactly(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, zone.t_op_winter_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Set winter operative temperature to ≈{t_req:.1} °C so |PMV|≤{lim}."), format!("Operative Wintertemperatur auf ≈{t_req:.1} °C setzen, damit |PMV|≤{lim}."))));
        }
        out.push(bp_w.build());
        let ppd_w = ppd_from_pmv(pmv_w);
        let mut bppd_w = CheckResult::assess(
            format!("din16798-1.ppd.winter.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PPD"),
            zone_subject(zone, path("tOpWinterC")), loc("Winter PPD (ISO 7730)", "Winter-PPD (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, ppd_w), Quantity::new(QuantityKind::Dimensionless, plim))
        .explanation(loc(format!("Winter PPD={ppd_w:.1} %; limit {plim:.0} %"), format!("Winter-PPD={ppd_w:.1} %; Grenzwert {plim:.0} %")));
        if ppd_w > plim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter, lim, pmv_w > 0.0);
            bppd_w = bppd_w.remedy(Remedy::exactly(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, zone.t_op_winter_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Adjust winter θ_op to ≈{t_req:.1} °C to keep PPD ≤{plim:.0} %."), format!("Operative Wintertemperatur auf ≈{t_req:.1} °C anpassen, damit PPD ≤{plim:.0} %."))));
        }
        out.push(bppd_w.build());
    }

    // Ventilation rate — EN 16798-1 §6.3 method selector
    let dlim_for_method = co2_above_outdoor_ppm(cat);
    let abs_lim_early = match usage {
        OccupancyType::Residential => annex.co2_absolute_residential_ppm,
        OccupancyType::Classroom => annex.co2_absolute_classroom_ppm,
        _ => annex.co2_absolute_other_ppm,
    };
    let method_delta = if doc.annex == AnnexChoice::De { (abs_lim_early - doc.outdoor_co2_ppm).min(dlim_for_method) } else { dlim_for_method };
    let q_req = required_outdoor_air_for_method(method, zone.occupants, zone.floor_area_m2, cat, pollution, zone.metabolic_rate_met, method_delta);
    let q = zone.outdoor_air_supplied_m3_h;
    let method_clause = match method {
        VentMethod::Method1PerceivedAirQuality => ("Table B.6/B.7", "Method 1 perceived air quality", "Methode 1 empfundene Luftqualität"),
        VentMethod::Method2LimitConcentration => ("§6.3 method 2", "Method 2 limit concentration", "Methode 2 Grenzkonzentration"),
        VentMethod::Method3PredefinedRates => ("Annex B predefined", "Method 3 predefined rates", "Methode 3 vorgegebene Raten"),
    };
    let system_note = match linked_type {
        Some(VentSystemType::Natural) => ("natural ventilation system", "natürliche Lüftung"),
        Some(VentSystemType::DecentralMech) => ("decentral mechanical system", "dezentrale mechanische Lüftung"),
        Some(VentSystemType::CentralMech) => ("central mechanical system", "zentrale mechanische Lüftung"),
        None => ("unresolved system", "unaufgelöste Anlage"),
    };
    let mut bv = CheckResult::assess(
        format!("din16798-1.vent.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", method_clause.0, "q"),
        zone_subject(zone, path("outdoorAirSuppliedM3H")), loc(method_clause.1, method_clause.2),
    ).annex(doc.annex).minimum(Quantity::new(QuantityKind::VentilationRate, q), Quantity::new(QuantityKind::VentilationRate, q_req))
    .explanation(loc(
        format!("q={q:.1} m³/h; required {q_req:.1} m³/h via {}; served by «{linked_name}» ({})", method.as_str(), system_note.0),
        format!("q={q:.1} m³/h; erforderlich {q_req:.1} m³/h nach {}; versorgt durch «{linked_name}» ({})", method.as_str(), system_note.1),
    ));
    if q < q_req {
        bv = bv.remedy(Remedy::at_least(zone_subject(zone, path("outdoorAirSuppliedM3H")), Quantity::new(QuantityKind::VentilationRate, q), Quantity::new(QuantityKind::VentilationRate, q_req),
            loc(
                format!("Raise outdoor air on this zone (system «{linked_name}») to at least {q_req:.1} m³/h."),
                format!("Außenluft dieser Zone (Anlage «{linked_name}») auf mindestens {q_req:.1} m³/h erhöhen."),
            )));
    }
    out.push(bv.build());

    // CO2 above outdoor
    let dco2 = (zone.co2_ppm - doc.outdoor_co2_ppm).max(0.0);
    let dlim = co2_above_outdoor_ppm(cat);
    // Also respect DE absolute cap when annex=De
    let abs_lim = match usage {
        OccupancyType::Residential => annex.co2_absolute_residential_ppm,
        OccupancyType::Classroom => annex.co2_absolute_classroom_ppm,
        _ => annex.co2_absolute_other_ppm,
    };
    let effective_limit_above = if doc.annex == AnnexChoice::De {
        (abs_lim - doc.outdoor_co2_ppm).min(dlim)
    } else { dlim };
    let mut bc = CheckResult::assess(
        format!("din16798-1.co2.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", annex.choice.label(), "CO2"),
        zone_subject(zone, path("co2Ppm")), loc("CO₂ above outdoor", "CO₂ über Außenluft"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, dco2), Quantity::new(QuantityKind::Dimensionless, effective_limit_above))
    .explanation(loc(format!("ΔCO₂={dco2:.0} ppm; limit {effective_limit_above:.0} ppm (Cat {:?}, {})", cat, annex.choice.label()), format!("ΔCO₂={dco2:.0} ppm; Grenzwert {effective_limit_above:.0} ppm (Kat. {:?}, {})", cat, annex.choice.label())));
    if dco2 > effective_limit_above {
        let target_ppm = doc.outdoor_co2_ppm + effective_limit_above;
        // Remedy via more outdoor air (proportional) or lower CO2 design
        let q_boost = if dco2 > 0.0 { q * (dco2 / effective_limit_above) } else { q_req };
        let q_needed = q_boost.max(q_req);
        bc = bc
            .remedy(Remedy::at_most(
                zone_subject(zone, path("co2Ppm")),
                Quantity::new(QuantityKind::Dimensionless, zone.co2_ppm),
                Quantity::new(QuantityKind::Dimensionless, target_ppm),
                loc(
                    format!("Reduce design CO₂ to at most {target_ppm:.0} ppm, or raise outdoor air on system «{linked_name}» to ≥{q_boost:.0} m³/h."),
                    format!("Auslegungs-CO₂ auf höchstens {target_ppm:.0} ppm senken oder Außenluft an Anlage «{linked_name}» auf ≥{q_boost:.0} m³/h erhöhen."),
                ),
            ))
            .remedy(Remedy::at_least(
                zone_subject(zone, path("outdoorAirSuppliedM3H")),
                Quantity::new(QuantityKind::VentilationRate, q),
                Quantity::new(QuantityKind::VentilationRate, q_needed),
                loc(
                    format!("Raise outdoor-air flow to at least {q_needed:.1} m³/h to dilute CO₂."),
                    format!("Außenluftvolumenstrom auf mindestens {q_needed:.1} m³/h erhöhen zur CO₂-Verdünnung."),
                ),
            ));
    }
    out.push(bc.build());

    // Humidity
    let (rh_min, rh_max) = humidity_band(cat);
    let rh = zone.rh_percent;
    let mut brh = CheckResult::assess(
        format!("din16798-1.rh.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Table B.16", "RH"),
        zone_subject(zone, path("rhPercent")), loc("Relative humidity", "Relative Feuchte"),
    ).annex(doc.annex);
    if rh < rh_min {
        brh = brh.minimum(Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_min))
            .remedy(Remedy::at_least(zone_subject(zone, path("rhPercent")), Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_min),
                loc(format!("Raise relative humidity to at least {rh_min:.0} %."), format!("Relative Feuchte auf mindestens {rh_min:.0} % erhöhen."))));
    } else if rh > rh_max {
        brh = brh.utilization(Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_max))
            .remedy(Remedy::at_most(zone_subject(zone, path("rhPercent")), Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_max),
                loc(format!("Lower relative humidity to at most {rh_max:.0} %."), format!("Relative Feuchte auf höchstens {rh_max:.0} % senken."))));
    } else {
        brh = brh.utilization(Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_max));
    }
    out.push(brh.explanation(loc(format!("RH={rh:.0} %; band {rh_min:.0}–{rh_max:.0} %"), format!("RF={rh:.0} %; Band {rh_min:.0}–{rh_max:.0} %"))).build());

    // Illuminance
    let e_min = illuminance_min_lx(usage, cat);
    let mut bi = CheckResult::assess(
        format!("din16798-1.illuminance.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B", "E"),
        zone_subject(zone, path("illuminanceLx")), loc("Illuminance", "Beleuchtungsstärke"),
    ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Dimensionless, zone.illuminance_lx), Quantity::new(QuantityKind::Dimensionless, e_min))
    .explanation(loc(format!("E={:.0} lx; min {:.0} lx", zone.illuminance_lx, e_min), format!("E={:.0} lx; min. {:.0} lx", zone.illuminance_lx, e_min)));
    if zone.illuminance_lx < e_min {
        bi = bi.remedy(Remedy::at_least(zone_subject(zone, path("illuminanceLx")), Quantity::new(QuantityKind::Dimensionless, zone.illuminance_lx), Quantity::new(QuantityKind::Dimensionless, e_min),
            loc(format!("Raise design illuminance to at least {e_min:.0} lx."), format!("Beleuchtungsstärke auf mindestens {e_min:.0} lx erhöhen."))));
    }
    out.push(bi.build());

    // Noise
    let nlim = acoustic_limit_db(cat, usage, annex);
    let mut bn = CheckResult::assess(
        format!("din16798-1.noise.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B", "L_Aeq"),
        zone_subject(zone, path("noiseDb")), loc("Indoor noise level", "Innengeräuschpegel"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, zone.noise_db), Quantity::new(QuantityKind::Dimensionless, nlim))
    .explanation(loc(format!("L_Aeq={:.1} dB; limit {:.1} dB", zone.noise_db, nlim), format!("L_Aeq={:.1} dB; Grenzwert {:.1} dB", zone.noise_db, nlim)));
    if zone.noise_db > nlim {
        bn = bn.remedy(Remedy::at_most(zone_subject(zone, path("noiseDb")), Quantity::new(QuantityKind::Dimensionless, zone.noise_db), Quantity::new(QuantityKind::Dimensionless, nlim),
            loc(format!("Reduce ventilation noise to at most {nlim:.1} dB(A)."), format!("Lüftungsgeräusch auf höchstens {nlim:.1} dB(A) senken."))));
    }
    out.push(bn.build());

    // Draught rate DR — ISO 7730 / EN 16798-1 category limits
    let dr = draught_rate_percent(zone.t_op_summer_c, zone.air_speed_m_s, zone.turbulence_intensity_percent);
    let dr_lim = draught_limit_percent(cat);
    let mut bd = CheckResult::assess(
        format!("din16798-1.draught.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2 / ISO 7730", "DR"),
        zone_subject(zone, path("airSpeedMS")), loc("Draught rate (ISO 7730)", "Zugluftrate (ISO 7730)"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, dr), Quantity::new(QuantityKind::Dimensionless, dr_lim))
    .explanation(loc(
        format!("DR={dr:.1} % at v={:.2} m/s, Tu={:.0} %, θ={:.1} °C; Cat limit {dr_lim:.0} %", zone.air_speed_m_s, zone.turbulence_intensity_percent, zone.t_op_summer_c),
        format!("DR={dr:.1} % bei v={:.2} m/s, Tu={:.0} %, θ={:.1} °C; Kat.-Grenzwert {dr_lim:.0} %", zone.air_speed_m_s, zone.turbulence_intensity_percent, zone.t_op_summer_c),
    ));
    if dr > dr_lim {
        // Invert roughly: lower air speed until DR ≤ limit (bounded search)
        let mut v_req = zone.air_speed_m_s;
        for step in 0..40 {
            let trial = (zone.air_speed_m_s * (1.0 - 0.025 * (step as f64 + 1.0))).max(0.05);
            if draught_rate_percent(zone.t_op_summer_c, trial, zone.turbulence_intensity_percent) <= dr_lim {
                v_req = trial;
                break;
            }
            v_req = trial;
        }
        bd = bd.remedy(Remedy::at_most(zone_subject(zone, path("airSpeedMS")), Quantity::new(QuantityKind::Dimensionless, zone.air_speed_m_s), Quantity::new(QuantityKind::Dimensionless, v_req),
            loc(
                format!("Reduce local air speed to at most {v_req:.2} m/s so DR ≤ {dr_lim:.0} %."),
                format!("Lokale Luftgeschwindigkeit auf höchstens {v_req:.2} m/s senken, damit DR ≤ {dr_lim:.0} %."),
            )));
    }
    out.push(bd.build());


    // Night setback deferred to envelope
    out
}

fn evaluate_vent(doc: &crate::Din16798Snapshot, _vi: usize, vent: &VentSystemDocument, _annex: &annex_params::AnnexParams) -> Vec<CheckResult> {
    use part_3::*;
    let mut out = Vec::new();
    let path = |field: &str| format!("ventSystems[id={}].{field}", vent.id);
    let served: Vec<&ZoneDocument> = doc.zones.iter().filter(|z| z.vent_system_id == vent.id).collect();
    let q_served: f64 = served.iter().map(|z| z.outdoor_air_supplied_m3_h).sum();
    let q_design = vent.design_airflow_m3_h;
    let q_design_m3_s = (q_design / 3600.0).max(1e-9);

    // Capacity: sum of linked zone supplies ≤ system design outdoor-air capacity
    {
        let mut bc = CheckResult::assess(
            format!("din16798-3.capacity.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§6", "q_design"),
            vent_subject(vent, path("designAirflowM3H")), loc("Design outdoor-air capacity", "Auslegungs-Außenluftkapazität"),
        ).annex(doc.annex);
        if served.is_empty() {
            bc = bc.not_applicable(loc(
                "No zones link this ventilation system.",
                "Keine Zone verweist auf diese Lüftungsanlage.",
            ));
        } else {
            bc = bc.utilization(Quantity::new(QuantityKind::VentilationRate, q_served), Quantity::new(QuantityKind::VentilationRate, q_design.max(1e-9)))
                .explanation(loc(
                    format!("q_served={q_served:.1} m³/h from {} zone(s); design capacity {q_design:.1} m³/h", served.len()),
                    format!("q_served={q_served:.1} m³/h aus {} Zone(n); Auslegungskapazität {q_design:.1} m³/h", served.len()),
                ));
            if q_served > q_design {
                bc = bc.remedy(Remedy::at_least(vent_subject(vent, path("designAirflowM3H")), Quantity::new(QuantityKind::VentilationRate, q_design), Quantity::new(QuantityKind::VentilationRate, q_served),
                    loc(
                        format!("Increase design outdoor-air capacity to at least {q_served:.1} m³/h."),
                        format!("Auslegungs-Außenluftkapazität auf mindestens {q_served:.1} m³/h erhöhen."),
                    )));
            }
        }
        out.push(bc.build());
    }

    // Operating fan flow vs design capacity
    {
        let q_fan_m3_h = vent.fan_q_v_m3_s * 3600.0;
        let mut bf = CheckResult::assess(
            format!("din16798-5-1.fan-flow.{}", vent.id), "DIN EN 16798-5-1", ClauseId::new("EN 16798-5-1", "§6.1", "q_fan"),
            vent_subject(vent, path("fanQVM3S")), loc("Fan operating flow vs design", "Ventilator-Betriebsvolumenstrom vs. Auslegung"),
        ).annex(doc.annex);
        // Relative ε avoids float Fail when fan_q_v_m3_s was authored as design/3600.
        if q_fan_m3_h <= q_design * (1.0 + 1e-6) {
            bf = bf.status(crate::document::CheckStatus::Pass)
                .utilization(Quantity::new(QuantityKind::VentilationRate, q_fan_m3_h.min(q_design)), Quantity::new(QuantityKind::VentilationRate, q_design.max(1e-9)))
                .explanation(loc(
                    format!("Fan flow {q_fan_m3_h:.1} m³/h vs design {q_design:.1} m³/h"),
                    format!("Ventilatorvolumenstrom {q_fan_m3_h:.1} m³/h gegenüber Auslegung {q_design:.1} m³/h"),
                ));
        } else {
            bf = bf.utilization(Quantity::new(QuantityKind::VentilationRate, q_fan_m3_h), Quantity::new(QuantityKind::VentilationRate, q_design.max(1e-9)))
                .explanation(loc(
                    format!("Fan flow {q_fan_m3_h:.1} m³/h exceeds design {q_design:.1} m³/h"),
                    format!("Ventilatorvolumenstrom {q_fan_m3_h:.1} m³/h überschreitet Auslegung {q_design:.1} m³/h"),
                ))
                .remedy(Remedy::at_most(vent_subject(vent, path("fanQVM3S")), Quantity::new(QuantityKind::VentilationRate, vent.fan_q_v_m3_s), Quantity::new(QuantityKind::VentilationRate, q_design_m3_s),
                    loc(
                        format!("Reduce fan operating flow to at most {q_design:.1} m³/h ({q_design_m3_s:.4} m³/s)."),
                        format!("Ventilator-Betriebsvolumenstrom auf höchstens {q_design:.1} m³/h ({q_design_m3_s:.4} m³/s) senken."),
                    )));
        }
        out.push(bf.build());
    }

    // SFP
    let bound = sfp_bound(vent.sfp_required_class);
    let mut bs = CheckResult::assess(
        format!("din16798-3.sfp.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "Table 10", "SFP"),
        vent_subject(vent, path("sfpWM3S")), loc("Specific fan power", "Spezifische Ventilatorleistung"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Power, vent.sfp_w_m3_s), Quantity::new(QuantityKind::Power, bound))
    .explanation(loc(format!("SFP={:.0} W/(m³/s); class {} bound {:.0}", vent.sfp_w_m3_s, vent.sfp_required_class, bound), format!("SFP={:.0} W/(m³/s); Klasse {} Grenze {:.0}", vent.sfp_w_m3_s, vent.sfp_required_class, bound)));
    if vent.sfp_w_m3_s > bound {
        bs = bs.remedy(Remedy::at_most(vent_subject(vent, path("sfpWM3S")), Quantity::new(QuantityKind::Power, vent.sfp_w_m3_s), Quantity::new(QuantityKind::Power, bound),
            loc(format!("Reduce SFP to at most {bound:.0} W/(m³/s) (SFP class {}).", vent.sfp_required_class), format!("SFP auf höchstens {bound:.0} W/(m³/s) senken (SFP-Klasse {}).", vent.sfp_required_class))));
    }
    out.push(bs.build());

    // Heat recovery
    match heat_recovery_eta_min(&vent.system_type) {
        None => out.push(CheckResult::assess(
            format!("din16798-3.hr.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.3", "η"),
            vent_subject(vent, path("heatRecoveryEta")), loc("Heat recovery efficiency", "Wärmerückgewinnungsgrad"),
        ).not_applicable(loc("Natural/hybrid systems have no HR efficiency requirement.", "Natürliche/hybride Systeme ohne WRG-Anforderung.")).annex(doc.annex).build()),
        Some(eta_min) => {
            let mut bh = CheckResult::assess(
                format!("din16798-3.hr.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.3", "η"),
                vent_subject(vent, path("heatRecoveryEta")), loc("Heat recovery efficiency", "Wärmerückgewinnungsgrad"),
            ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Dimensionless, vent.heat_recovery_eta), Quantity::new(QuantityKind::Dimensionless, eta_min))
            .explanation(loc(format!("Heat-recovery η={:.2}; minimum {:.2}", vent.heat_recovery_eta, eta_min), format!("Wärmerückgewinnungsgrad η={:.2}; Mindestwert {:.2}", vent.heat_recovery_eta, eta_min)));
            if vent.heat_recovery_eta < eta_min {
                bh = bh.remedy(Remedy::at_least(vent_subject(vent, path("heatRecoveryEta")), Quantity::new(QuantityKind::Dimensionless, vent.heat_recovery_eta), Quantity::new(QuantityKind::Dimensionless, eta_min),
                    loc(format!("Increase heat-recovery efficiency to at least {eta_min:.2}."), format!("Wärmerückgewinnungsgrad auf mindestens {eta_min:.2} erhöhen."))));
            }
            out.push(bh.build());
        }
    }

    // Filter / ODA
    let (req_filter, req_rank) = required_filter_for_oda(&vent.oda_class);
    let have = filter_rank(&vent.filter_sup_class);
    let mut bf = CheckResult::assess(
        format!("din16798-3.filter.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7", "filter"),
        vent_subject(vent, path("filterSupClass")), loc("SUP filter vs ODA class", "Zuluftfilter vs. ODA-Klasse"),
    ).annex(doc.annex);
    if have >= req_rank {
        // utilization(have, req) would be >1 when overspecified — force Pass after recording ranks.
        bf = bf.utilization(Quantity::new(QuantityKind::Dimensionless, have as f64), Quantity::new(QuantityKind::Dimensionless, req_rank.max(1) as f64))
            .status(crate::document::CheckStatus::Pass)
            .explanation(loc(format!("{} with {} OK (need ≥{})", vent.oda_class, vent.filter_sup_class, req_filter), format!("{} mit {} OK (mind. {})", vent.oda_class, vent.filter_sup_class, req_filter)));
    } else {
        let filter_remedy = Remedy {
            target: vent_subject(vent, path("filterSupClass")),
            current: Quantity::new(QuantityKind::Dimensionless, have as f64),
            required: Quantity::new(QuantityKind::Dimensionless, req_rank as f64),
            bound: RemedyBound::OneOf,
            options: vec![req_filter.to_string()],
            action: loc(
                format!("Set SUP filter class to {req_filter}."),
                format!("Zuluftfilterklasse auf {req_filter} setzen."),
            ),
            applicable: true,
        };
        bf = bf.status(crate::document::CheckStatus::Fail).utilization(Quantity::new(QuantityKind::Dimensionless, (req_rank - have) as f64 + 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .explanation(loc(format!("{} requires ≥{}; have {}", vent.oda_class, req_filter, vent.filter_sup_class), format!("{} erfordert ≥{}; vorhanden {}", vent.oda_class, req_filter, vent.filter_sup_class)))
            .remedy(filter_remedy);
    }
    out.push(bf.build());

    // Inspection
    let interval = inspection_interval_years(&vent.system_type);
    let mut bi = CheckResult::assess(
        format!("din16798-3.inspection.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§8.1", "inspection"),
        vent_subject(vent, path("yearsSinceInspection")), loc("Ventilation inspection interval", "Lüftungsinspektionsintervall"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, vent.years_since_inspection as f64), Quantity::new(QuantityKind::Dimensionless, interval as f64))
    .explanation(loc(format!("{} years since inspection; allowed interval {} years", vent.years_since_inspection, interval), format!("{} Jahre seit der Inspektion; zulässiges Intervall {} Jahre", vent.years_since_inspection, interval)));
    if vent.years_since_inspection > interval {
        bi = bi.remedy(Remedy::at_most(vent_subject(vent, path("yearsSinceInspection")), Quantity::new(QuantityKind::Dimensionless, vent.years_since_inspection as f64), Quantity::new(QuantityKind::Dimensionless, 0.0),
            loc("Perform inspection and reset years since inspection to 0.", "Inspektion durchführen und Jahre seit Inspektion auf 0 setzen.")));
    }
    out.push(bi.build());

    // Humidification — NA for natural systems or when required=0
    let vent_kind = VentSystemType::parse(&vent.system_type);
    if matches!(vent_kind, VentSystemType::Natural) {
        out.push(CheckResult::assess(
            format!("din16798-3.humid.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.4", "humidification"),
            vent_subject(vent, path("humidificationProvidedKgH")), loc("Humidification capacity", "Befeuchtungsleistung"),
        ).not_applicable(loc(
            "Natural ventilation systems have no humidification requirement.",
            "Natürliche Lüftung ohne Befeuchtungsanforderung.",
        )).annex(doc.annex).build());
    } else if vent.humidification_required_kg_h <= 0.0 {
        out.push(CheckResult::assess(
            format!("din16798-3.humid.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.4", "humidification"),
            vent_subject(vent, path("humidificationProvidedKgH")), loc("Humidification capacity", "Befeuchtungsleistung"),
        ).annex(doc.annex)
         .status(crate::document::CheckStatus::Pass)
         .utilization(
            Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h),
            Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h.abs().max(1e-9)),
         )
         .explanation(loc(
            format!("No humidification required; provided {:.2} kg/h recorded", vent.humidification_provided_kg_h),
            format!("Keine Befeuchtung erforderlich; bereitgestellt {:.2} kg/h erfasst", vent.humidification_provided_kg_h),
         ))
         .build());
    } else {
        let mut bh = CheckResult::assess(
            format!("din16798-3.humid.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.4", "humidification"),
            vent_subject(vent, path("humidificationProvidedKgH")), loc("Humidification capacity", "Befeuchtungsleistung"),
        ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h), Quantity::new(QuantityKind::Mass, vent.humidification_required_kg_h));
        if vent.humidification_provided_kg_h < vent.humidification_required_kg_h {
            bh = bh.remedy(Remedy::at_least(vent_subject(vent, path("humidificationProvidedKgH")), Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h), Quantity::new(QuantityKind::Mass, vent.humidification_required_kg_h),
                loc(format!("Increase humidification capacity to at least {:.2} kg/h.", vent.humidification_required_kg_h), format!("Befeuchtungsleistung auf mindestens {:.2} kg/h erhöhen.", vent.humidification_required_kg_h))));
        }
        out.push(bh.build());
    }

    // Duct leakage Part 17
    let lim = duct_leakage_limit(&vent.duct_class, vent.duct_test_pressure_pa);
    let mut bd = CheckResult::assess(
        format!("din16798-17.duct.{}", vent.id), "DIN EN 16798-17", ClauseId::new("EN 16798-17", "§8.2", "leakage"),
        vent_subject(vent, path("ductLeakageM3SM2")), loc("Ductwork leakage", "Kanalnetzleckage"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::VentilationRate, vent.duct_leakage_m3_s_m2), Quantity::new(QuantityKind::VentilationRate, lim))
    .explanation(loc(format!("f={:.4}; limit {:.4} at {:.0} Pa class {}", vent.duct_leakage_m3_s_m2, lim, vent.duct_test_pressure_pa, vent.duct_class), format!("f={:.4}; Grenzwert {:.4} bei {:.0} Pa Klasse {}", vent.duct_leakage_m3_s_m2, lim, vent.duct_test_pressure_pa, vent.duct_class)));
    if vent.duct_leakage_m3_s_m2 > lim {
        bd = bd.remedy(Remedy::at_most(vent_subject(vent, path("ductLeakageM3SM2")), Quantity::new(QuantityKind::VentilationRate, vent.duct_leakage_m3_s_m2), Quantity::new(QuantityKind::VentilationRate, lim),
            loc(format!("Reduce duct leakage to at most {lim:.4} m³/(s·m²)."), format!("Kanalnetzleckage auf höchstens {lim:.4} m³/(s·m²) senken."))));
    }
    out.push(bd.build());

    // Annual fan energy vs independent EN 16798-5-1 §6.1 benchmark using design airflow (not declared SFP class).
    const FAN_ENERGY_BENCHMARK_SFP_W_M3_S: f64 = 1000.0;
    let e = part_5_1::fan_energy_kwh(vent.sfp_w_m3_s, q_design_m3_s, vent.fan_t_run_h);
    let e_ref = part_5_1::fan_energy_kwh(FAN_ENERGY_BENCHMARK_SFP_W_M3_S, q_design_m3_s, vent.fan_t_run_h).max(1e-9);
    let mut be = CheckResult::assess(
        format!("din16798-5-1.fan.{}", vent.id), "DIN EN 16798-5-1", ClauseId::new("EN 16798-5-1", "§6.1", "E_fan"),
        vent_subject(vent, path("sfpWM3S")), loc("Annual fan electrical energy", "Jährliche Ventilator-Elektroenergie"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Energy, e), Quantity::new(QuantityKind::Energy, e_ref))
    .explanation(loc(
        format!("E_fan={e:.1} kWh/a from SFP·q·t; benchmark {e_ref:.1} kWh/a at SFP_ref={FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s)"),
        format!("E_fan={e:.1} kWh/a aus SFP·q·t; Richtwert {e_ref:.1} kWh/a bei SFP_ref={FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s)"),
    ));
    if e > e_ref {
        be = be.remedy(Remedy::at_most(vent_subject(vent, path("sfpWM3S")), Quantity::new(QuantityKind::Power, vent.sfp_w_m3_s), Quantity::new(QuantityKind::Power, FAN_ENERGY_BENCHMARK_SFP_W_M3_S),
            loc(format!("Reduce SFP to ≤{FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s) so annual fan energy ≤{e_ref:.1} kWh."), format!("SFP auf ≤{FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s) senken, damit die jährliche Ventilatorenergie ≤{e_ref:.1} kWh bleibt."))));
    }
    out.push(be.build());

    out
}

fn evaluate_envelope(doc: &crate::Din16798Snapshot, _annex: &annex_params::AnnexParams) -> Vec<CheckResult> {
    use part_7::*;
    let mut out = Vec::new();
    let n50_max = n50_max_mechanical();
    let mut bn = CheckResult::assess(
        "din16798-7.n50", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.1", "n50"),
        SubjectRef::new("", "envelopeN50HInv", loc("Building envelope", "Gebäudehülle")),
        loc("Airtightness n50", "Luftdichtheit n50"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, doc.envelope_n50_h_inv), Quantity::new(QuantityKind::Dimensionless, n50_max))
    .explanation(loc(format!("n50={:.2} 1/h; max {:.1} 1/h for mechanical ventilation", doc.envelope_n50_h_inv, n50_max), format!("n50={:.2} 1/h; max. {:.1} 1/h bei mechanischer Lüftung", doc.envelope_n50_h_inv, n50_max)));
    if doc.envelope_n50_h_inv > n50_max {
        bn = bn.remedy(Remedy::at_most(SubjectRef::new("", "envelopeN50HInv", loc("Building envelope", "Gebäudehülle")), Quantity::new(QuantityKind::Dimensionless, doc.envelope_n50_h_inv), Quantity::new(QuantityKind::Dimensionless, n50_max),
            loc(format!("Improve airtightness to n50 ≤ {n50_max:.1} 1/h."), format!("Luftdichtheit auf n50 ≤ {n50_max:.1} 1/h verbessern."))));
    }
    out.push(bn.build());

    if doc.envelope_volume_m3 > 0.0 {
        let q_inf = infiltration_m3_h(doc.envelope_n50_h_inv, doc.envelope_volume_m3);
        let q_cap = infiltration_m3_h(n50_max, doc.envelope_volume_m3);
        let mut bi = CheckResult::assess(
            "din16798-7.infiltration", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.1", "q_inf"),
            SubjectRef::new("", "envelopeVolumeM3", loc("Building envelope volume", "Gebäudehüllvolumen")),
            loc("Infiltration volume flow", "Infiltrationsvolumenstrom"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::VentilationRate, q_inf), Quantity::new(QuantityKind::VentilationRate, q_cap.max(1e-9)))
        .explanation(loc(
            format!("q_inf≈{q_inf:.1} m³/h from n50·V/20; cap {q_cap:.1} m³/h at n50≤{n50_max:.1}"),
            format!("q_inf≈{q_inf:.1} m³/h aus n50·V/20; Grenze {q_cap:.1} m³/h bei n50≤{n50_max:.1}"),
        ));
        if q_inf > q_cap {
            bi = bi.remedy(Remedy::at_most(SubjectRef::new("", "envelopeN50HInv", loc("Building envelope", "Gebäudehülle")), Quantity::new(QuantityKind::Dimensionless, doc.envelope_n50_h_inv), Quantity::new(QuantityKind::Dimensionless, n50_max),
                loc(format!("Improve airtightness to n50 ≤ {n50_max:.1} 1/h."), format!("Luftdichtheit auf n50 ≤ {n50_max:.1} 1/h verbessern."))));
        }
        out.push(bi.build());
    }

    if doc.cellar_area_m2 <= 0.0 {
        out.push(CheckResult::assess(
            "din16798-7.cellar", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.2", "cellar"),
            SubjectRef::new("", "cellarAreaM2", loc("Cellar floor area", "Kellerfläche")), loc("Cellar ventilation", "Kellerlüftung"),
        ).annex(doc.annex)
         .not_applicable(loc(
            "No cellar floor area; EN 16798-7 §6.2 cellar ventilation does not apply.",
            "Keine Kellerfläche; Kellerlüftung nach EN 16798-7 §6.2 gilt nicht.",
         ))
         .build());
    } else {
        let req = cellar_ventilation_required_m3_h(doc.cellar_area_m2);
        let mut bc = CheckResult::assess(
            "din16798-7.cellar", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.2", "cellar"),
            SubjectRef::new("", "cellarVentilationM3H", loc("Cellar", "Keller")),
            loc("Cellar ventilation", "Kellerlüftung"),
        ).annex(doc.annex).minimum(Quantity::new(QuantityKind::VentilationRate, doc.cellar_ventilation_m3_h), Quantity::new(QuantityKind::VentilationRate, req));
        if doc.cellar_ventilation_m3_h < req {
            bc = bc.remedy(Remedy::at_least(SubjectRef::new("", "cellarVentilationM3H", loc("Cellar", "Keller")), Quantity::new(QuantityKind::VentilationRate, doc.cellar_ventilation_m3_h), Quantity::new(QuantityKind::VentilationRate, req),
                loc(format!("Raise cellar ventilation to at least {req:.1} m³/h."), format!("Kellerlüftung auf mindestens {req:.1} m³/h erhöhen."))));
        }
        out.push(bc.build());
    }

    // Night setback using first zone usage if any
    let usage = doc.zones.first().map(|z| parse_usage(&z.usage_type)).unwrap_or(OccupancyType::Office);
    let sb_min = part_5_1::night_setback_min_k(usage);
    let mut bs = CheckResult::assess(
        "din16798-5-1.setback", "DIN EN 16798-5-1", ClauseId::new("EN 16798-5-1", "§6.2", "setback"),
        SubjectRef::new("", "nightSetbackK", loc("Building", "Gebäude")),
        loc("Night setback", "Nachtabsenkung"),
    ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Temperature, doc.night_setback_k), Quantity::new(QuantityKind::Temperature, sb_min))
    .explanation(loc(format!("Δθ_night={:.1} K; min {:.1} K", doc.night_setback_k, sb_min), format!("Δθ_Nacht={:.1} K; min. {:.1} K", doc.night_setback_k, sb_min)));
    if doc.night_setback_k < sb_min {
        bs = bs.remedy(Remedy::at_least(SubjectRef::new("", "nightSetbackK", loc("Building", "Gebäude")), Quantity::new(QuantityKind::Temperature, doc.night_setback_k), Quantity::new(QuantityKind::Temperature, sb_min),
            loc(format!("Increase night setback to at least {sb_min:.1} K."), format!("Nachtabsenkung auf mindestens {sb_min:.1} K erhöhen."))));
    }
    out.push(bs.build());
    out
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
