//! 🌍️ EN 1997 artifact schema — every field with its state class.

use crate::artifacts::en1997::En1997Snapshot;
use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1997 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Artifact {
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub h_ed_kn: f64,
    #[state(artifact)]
    pub footing_area_m2: f64,
    #[state(artifact)]
    pub phi_deg: f64,
    #[state(artifact)]
    pub c_kpa: f64,
    #[state(artifact)]
    pub gamma_kn_m3: f64,
    #[state(artifact)]
    pub b_m: f64,
    #[state(artifact)]
    pub d_f_m: f64,
    #[state(artifact)]
    pub e_s_mpa: f64,
    #[state(artifact)]
    pub nu: f64,
    #[state(artifact)]
    pub design_approach: String,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub settlement_limit_mm: f64,
    #[state(artifact)]
    pub n_pile_ed_kn: f64,
    #[state(artifact)]
    pub alpha_s: f64,
    #[state(artifact)]
    pub pile_d_m: f64,
    #[state(artifact)]
    pub q_s_kpa: f64,
    #[state(artifact)]
    pub pile_l_m: f64,
    #[state(artifact)]
    pub q_b_kpa: f64,
    #[state(artifact)]
    pub pile_base_area_m2: f64,
    #[state(artifact)]
    pub pile_n_profiles: u32,
    #[state(artifact)]
    pub z_investigated_m: f64,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1997Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1997Snapshot::default())
    }
}

impl From<En1997Snapshot> for En1997Artifact {
    fn from(snapshot: En1997Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1997Artifact {
    pub fn to_snapshot(&self) -> En1997Snapshot {
        En1997Snapshot {
            v_ed_kn: self.v_ed_kn.clone(),
            h_ed_kn: self.h_ed_kn.clone(),
            footing_area_m2: self.footing_area_m2.clone(),
            phi_deg: self.phi_deg.clone(),
            c_kpa: self.c_kpa.clone(),
            gamma_kn_m3: self.gamma_kn_m3.clone(),
            b_m: self.b_m.clone(),
            d_f_m: self.d_f_m.clone(),
            e_s_mpa: self.e_s_mpa.clone(),
            nu: self.nu.clone(),
            design_approach: self.design_approach.clone(),
            annex: self.annex.clone(),
            settlement_limit_mm: self.settlement_limit_mm.clone(),
            n_pile_ed_kn: self.n_pile_ed_kn.clone(),
            alpha_s: self.alpha_s.clone(),
            pile_d_m: self.pile_d_m.clone(),
            q_s_kpa: self.q_s_kpa.clone(),
            pile_l_m: self.pile_l_m.clone(),
            q_b_kpa: self.q_b_kpa.clone(),
            pile_base_area_m2: self.pile_base_area_m2.clone(),
            pile_n_profiles: self.pile_n_profiles.clone(),
            z_investigated_m: self.z_investigated_m.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1997Snapshot) -> Self {
        Self {
            v_ed_kn: snapshot.v_ed_kn,
            h_ed_kn: snapshot.h_ed_kn,
            footing_area_m2: snapshot.footing_area_m2,
            phi_deg: snapshot.phi_deg,
            c_kpa: snapshot.c_kpa,
            gamma_kn_m3: snapshot.gamma_kn_m3,
            b_m: snapshot.b_m,
            d_f_m: snapshot.d_f_m,
            e_s_mpa: snapshot.e_s_mpa,
            nu: snapshot.nu,
            design_approach: snapshot.design_approach,
            annex: snapshot.annex,
            settlement_limit_mm: snapshot.settlement_limit_mm,
            n_pile_ed_kn: snapshot.n_pile_ed_kn,
            alpha_s: snapshot.alpha_s,
            pile_d_m: snapshot.pile_d_m,
            q_s_kpa: snapshot.q_s_kpa,
            pile_l_m: snapshot.pile_l_m,
            q_b_kpa: snapshot.q_b_kpa,
            pile_base_area_m2: snapshot.pile_base_area_m2,
            pile_n_profiles: snapshot.pile_n_profiles,
            z_investigated_m: snapshot.z_investigated_m,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: En1997Snapshot) {
        self.v_ed_kn = snapshot.v_ed_kn;
        self.h_ed_kn = snapshot.h_ed_kn;
        self.footing_area_m2 = snapshot.footing_area_m2;
        self.phi_deg = snapshot.phi_deg;
        self.c_kpa = snapshot.c_kpa;
        self.gamma_kn_m3 = snapshot.gamma_kn_m3;
        self.b_m = snapshot.b_m;
        self.d_f_m = snapshot.d_f_m;
        self.e_s_mpa = snapshot.e_s_mpa;
        self.nu = snapshot.nu;
        self.design_approach = snapshot.design_approach;
        self.annex = snapshot.annex;
        self.settlement_limit_mm = snapshot.settlement_limit_mm;
        self.n_pile_ed_kn = snapshot.n_pile_ed_kn;
        self.alpha_s = snapshot.alpha_s;
        self.pile_d_m = snapshot.pile_d_m;
        self.q_s_kpa = snapshot.q_s_kpa;
        self.pile_l_m = snapshot.pile_l_m;
        self.q_b_kpa = snapshot.q_b_kpa;
        self.pile_base_area_m2 = snapshot.pile_base_area_m2;
        self.pile_n_profiles = snapshot.pile_n_profiles;
        self.z_investigated_m = snapshot.z_investigated_m;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1997_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1997",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
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
    use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1997BuilderConstruction {
        snapshot: En1997Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1997BuilderConstruction {
        type Snapshot = En1997Snapshot;
        type Mutation = En1997Mutation;
        type Diff = En1997Diff;
        fn empty() -> Self {
            Self { snapshot: En1997Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1997Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1997Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1997Diff as protocol::MutationDiff<En1997Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::en1997::En1997Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1997Parts {
        pub snapshot: Option<En1997Snapshot>,
    }

    pub struct En1997AnalyzerAnalysis;

    impl ArtifactAnalysis for En1997AnalyzerAnalysis {
        type Parts = En1997Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1997", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1997Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1997Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1997Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1997BuilderFacets {
        construction: En1997BuilderConstruction,
        analysis: En1997AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1997ComposerComposition,
    }
    builder: En1997Builder,
    analyzer: En1997Analyzer,
    composer: En1997Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1997 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. `na_de`, `DesignApproach`, `AnnexParams`,
/// `part_1`/`part_2` and `check_shallow_foundation` are pure function libraries; the snapshot-level
/// composition (`evaluate`, `check_full_geotechnical`) lives in `💡️inferences`. `na_de` re-exports
/// `crate::artifacts::en1990`'s relocated `NaDe`.
/// 📚️ Models the classic (pre-2024) Eurocode 7 generation only: EN 1997-1 (general design rules,
/// including piles) + EN 1997-2 (ground investigation and testing); the second-generation EN 1997-3
/// does not apply here.
use crate::document::{CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::na_de::NaDe;

    /// 🇩️🇪️ Partial factor on cohesion per DIN EN 1997-1/NA.
    pub fn gamma_c() -> f64 {
        1.4
    }

    /// 🇩️🇪️ Partial factor on angle of shearing resistance per DIN EN 1997-1/NA.
    pub fn gamma_phi() -> f64 {
        1.25
    }

    /// 🇩️🇪️ Partial factor on soil weight per DIN EN 1997-1/NA.
    pub fn gamma_gamma() -> f64 {
        1.0
    }
}

/// ⚖️ Design approach per EN 1997-1 §2.4.7.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignApproach {
    Da1Str,
    Da1Geo,
    Da2,
    Da3,
}

/// 🇩️🇪️🇺️ Annex-resolved partial factor set for a design approach; DE selects DA2* (combined STR/GEO verification) where EN keeps standard DA2.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub gamma_g: f64,
    pub gamma_q: f64,
    pub gamma_r_v: f64,
    pub gamma_r_h: f64,
    pub gamma_c: f64,
    pub gamma_phi: f64,
    pub gamma_gamma: f64,
    pub gamma_b: f64,
    pub gamma_s: f64,
}

impl DesignApproach {
    /// 🏷️ Approach label as resolved under the national annex (DE's `Da2` is DA2*, EN's is DA2).
    pub fn label(self, annex: AnnexChoice) -> &'static str {
        match (self, annex) {
            (Self::Da2, AnnexChoice::De) => "DA2*",
            (Self::Da2, AnnexChoice::En) => "DA2",
            (Self::Da1Str, _) => "DA1-C1",
            (Self::Da1Geo, _) => "DA1-C2",
            (Self::Da3, _) => "DA3",
        }
    }

    /// ⚖️ Resolve the full partial factor set for this approach under the given national annex per EN 1997-1 Annex A / DIN EN 1997-1/NA.
    pub fn annex_params(self, annex: AnnexChoice) -> AnnexParams {
        match (self, annex) {
            (Self::Da2, AnnexChoice::De) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.4, gamma_r_h: 1.1, gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1 },
            (Self::Da2, AnnexChoice::En) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0, gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1 },
            (Self::Da1Str, _) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0, gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(), gamma_b: 1.0, gamma_s: 1.0 },
            (Self::Da1Geo, _) => AnnexParams { gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.4, gamma_r_h: 1.1, gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1 },
            (Self::Da3, _) => AnnexParams { gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0, gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(), gamma_b: 1.15, gamma_s: 1.15 },
        }
    }
}

// #region 🔖️Part1
pub mod part_1 {
    use super::*;

    /// 📐️ Bearing capacity factor N_q (Meyerhof).
    pub fn bearing_factor_n_q(phi_deg: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        (std::f64::consts::PI * phi_rad.tan()).exp() * ((45.0 + 0.5 * phi_deg).to_radians().tan()).powi(2)
    }

    /// 📐️ Bearing capacity factor N_c (Meyerhof) with depth correction.
    pub fn bearing_factor_n_c(phi_deg: f64, d_f_m: f64, b_m: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        if phi_deg <= 0.0 {
            return 5.14;
        }
        let n_q = bearing_factor_n_q(phi_deg);
        let s_c = if b_m > 0.0 { 1.0 + 0.2 * (d_f_m / b_m) * (20.0 - phi_deg).max(0.0) / 20.0 } else { 1.0 };
        (n_q - 1.0) / phi_rad.tan() * s_c
    }

    /// 📐️ Bearing capacity factor N_γ (Meyerhof).
    pub fn bearing_factor_n_gamma(phi_deg: f64, d_f_m: f64, b_m: f64) -> f64 {
        let phi_rad = phi_deg.to_radians();
        let n_q = bearing_factor_n_q(phi_deg);
        let s_gamma = if b_m > 0.0 { 1.0 + 0.1 * (d_f_m / b_m) * (20.0 - phi_deg).max(0.0) / 20.0 } else { 1.0 };
        2.0 * (n_q + 1.0) * phi_rad.tan() * s_gamma
    }

    /// 📐️ Ultimate bearing capacity q_ult [kPa] (Meyerhof).
    pub fn ultimate_bearing_capacity_kpa(phi_deg: f64, c_kpa: f64, gamma_kn_m3: f64, b_m: f64, d_f_m: f64) -> f64 {
        let n_c = bearing_factor_n_c(phi_deg, d_f_m, b_m);
        let n_q = bearing_factor_n_q(phi_deg);
        let n_gamma = bearing_factor_n_gamma(phi_deg, d_f_m, b_m);
        c_kpa * n_c + gamma_kn_m3 * d_f_m * n_q + 0.5 * gamma_kn_m3 * b_m * n_gamma
    }

    pub fn design_bearing_capacity_kpa(phi_deg: f64, c_kpa: f64, gamma_kn_m3: f64, b_m: f64, d_f_m: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        let p = approach.annex_params(annex);
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan().to_degrees();
        let c_d = c_kpa / p.gamma_c;
        let gamma_d = gamma_kn_m3 / p.gamma_gamma;
        ultimate_bearing_capacity_kpa(phi_d, c_d, gamma_d, b_m, d_f_m) / p.gamma_r_v
    }

    pub fn bearing_resistance_kn(a_m2: f64, q_d_kpa: f64) -> f64 {
        a_m2 * q_d_kpa
    }

    /// 📐️ Sliding resistance [kPa]: (c_d + σ·tan(φ_d)) / γ_R;h.
    pub fn sliding_resistance_kpa(phi_deg: f64, c_kpa: f64, sigma_kpa: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        let p = approach.annex_params(annex);
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan();
        let c_d = c_kpa / p.gamma_c;
        (c_d + sigma_kpa * phi_d.tan()) / p.gamma_r_h
    }

    pub fn sliding_resistance_kn(phi_deg: f64, c_kpa: f64, sigma_kpa: f64, area_m2: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        sliding_resistance_kpa(phi_deg, c_kpa, sigma_kpa, approach, annex) * area_m2
    }

    /// 📐️ Elastic settlement [mm] (Boussinesq simplified).
    pub fn elastic_settlement_mm(e_s_mpa: f64, nu: f64, b_m: f64, q_kpa: f64) -> f64 {
        let i_f = 0.88;
        let q_mpa = q_kpa / 1000.0;
        i_f * q_mpa * b_m * (1.0 - nu * nu) / e_s_mpa * 1000.0
    }

    // #region 🔖️Piles
    /// 📐️ Shaft resistance R_s = α_s · π · D · q_s · L [kN] per EN 1997-1 §7.6.2 (ground-test-derived α_s and q_s from EN 1997-2 investigation).
    pub fn shaft_resistance_kn(alpha_s: f64, d_m: f64, q_s_kpa: f64, l_m: f64) -> f64 {
        alpha_s * std::f64::consts::PI * d_m * q_s_kpa * l_m
    }

    /// 📐️ Base resistance R_b = q_b · A_b [kN] per EN 1997-1 §7.6.2.
    pub fn base_resistance_kn(q_b_kpa: f64, a_base_m2: f64) -> f64 {
        q_b_kpa * a_base_m2
    }

    /// 📊️ Correlation factors ξ₃ (on mean) / ξ₄ (on min) per EN 1997-1 Annex A Table A.10, by number of test profiles n.
    pub fn pile_correlation_factors(n_profiles: u32) -> (f64, f64) {
        match n_profiles {
            0 | 1 => (1.40, 1.40),
            2 => (1.35, 1.27),
            3 | 4 => {
                let t = (n_profiles - 2) as f64 / 3.0;
                (1.35 + t * (1.30 - 1.35), 1.27 + t * (1.15 - 1.27))
            }
            _ => (1.30, 1.15),
        }
    }

    /// 📐️ Characteristic pile resistance R_k = min(mean(R_cal)/ξ₃, min(R_cal)/ξ₄) per EN 1997-1 §7.6.2.2.
    pub fn pile_characteristic_resistance_kn(mean_r_cal_kn: f64, min_r_cal_kn: f64, n_profiles: u32) -> f64 {
        let (xi_3, xi_4) = pile_correlation_factors(n_profiles);
        (mean_r_cal_kn / xi_3).min(min_r_cal_kn / xi_4)
    }

    /// 📐️ Design pile compressive resistance R_c,d = R_b,k/γ_b + R_s,k/γ_s per EN 1997-1 §7.6.2.
    pub fn pile_design_resistance_kn(r_b_k_kn: f64, r_s_k_kn: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        let p = approach.annex_params(annex);
        r_b_k_kn / p.gamma_b + r_s_k_kn / p.gamma_s
    }
    // #endregion 🔖️Piles

    pub fn check_bearing(v_ed_kn: f64, r_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§6.5", "6.5"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(r_d_kn), "bearing resistance ULS", annex)
    }

    pub fn check_sliding(h_ed_kn: f64, r_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§6.5", "6.5.3"), Quantity::force_kn(h_ed_kn), Quantity::force_kn(r_d_kn), "sliding resistance ULS", annex)
    }

    pub fn check_settlement(s_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§6.6", "6.6"), Quantity::length_m(s_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "settlement SLS", annex)
    }

    /// ✅️ Pile axial compressive resistance check per EN 1997-1 §7.6.2.
    pub fn check_pile_axial(n_ed_kn: f64, r_c_d_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1997-1", "§7", "7.6.2"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(r_c_d_kn), "pile axial compressive resistance ULS", annex)
    }
}
// #endregion 🔖️Part1

// #region 🔖️Part2
pub mod part_2 {
    use super::{AnnexChoice, CheckResult, ClauseId, Quantity};

    /// 📐️ Effective friction angle φ′ [deg] from CPT cone resistance q_c and effective overburden σ′_v0, per EN 1997-2 Annex D (Robertson & Campanella 1983).
    pub fn phi_from_cpt_deg(q_c_kpa: f64, sigma_v0_kpa: f64) -> f64 {
        let ratio = (q_c_kpa / sigma_v0_kpa.max(1.0)).max(1.0);
        (0.38 * ratio.log10() + 0.1).atan().to_degrees()
    }

    /// 📐️ Effective friction angle φ′ [deg] from SPT blow count N, per EN 1997-2 Annex D (Peck–Hanson–Thornburn correlation).
    pub fn phi_from_spt_deg(n_spt: f64) -> f64 {
        27.1 + 0.3 * n_spt - 0.00054 * n_spt * n_spt
    }

    /// 📐️ Minimum ground investigation depth [m] per EN 1997-2 §2.4.2: z ≥ 3·b.
    pub fn min_investigation_depth_m(b_m: f64) -> f64 {
        3.0 * b_m
    }

    /// ✅️ Ground investigation depth adequacy check per EN 1997-2 §2.4.2.
    pub fn check_investigation_depth(z_investigated_m: f64, b_m: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_minimum(ClauseId::new("EN 1997-2", "§2.4", "2.4.2"), Quantity::length_m(z_investigated_m), Quantity::length_m(min_investigation_depth_m(b_m)), "minimum ground investigation depth", annex)
    }
}
// #endregion 🔖️Part2

/// 📋️ Shallow foundation check.
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_shallow_foundation(
    v_ed_kn: f64,
    h_ed_kn: f64,
    footing_area_m2: f64,
    phi_deg: f64,
    c_kpa: f64,
    gamma_kn_m3: f64,
    b_m: f64,
    d_f_m: f64,
    e_s_mpa: f64,
    nu: f64,
    approach: DesignApproach,
    annex: AnnexChoice,
    settlement_limit_mm: f64,
) -> CheckReport {
    let q_d = part_1::design_bearing_capacity_kpa(phi_deg, c_kpa, gamma_kn_m3, b_m, d_f_m, approach, annex);
    let r_d = part_1::bearing_resistance_kn(footing_area_m2, q_d);
    let sigma = v_ed_kn / footing_area_m2;
    let r_sliding = part_1::sliding_resistance_kn(phi_deg, c_kpa, sigma, footing_area_m2, approach, annex);
    let settlement = part_1::elastic_settlement_mm(e_s_mpa, nu, b_m, sigma);
    let mut report = CheckReport::default();
    report.push(part_1::check_bearing(v_ed_kn, r_d, annex));
    report.push(part_1::check_sliding(h_ed_kn, r_sliding, annex));
    report.push(part_1::check_settlement(settlement, settlement_limit_mm, annex));
    report
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;
    use crate::document::CheckStatus;

    #[semio_framework_async_macros::async_test]
    fn bearing_factor_n_c_phi30() {
        let n_c = part_1::bearing_factor_n_c(30.0, 1.5, 2.0);
        assert!((n_c - 30.1).abs() < 0.5);
    }

    #[semio_framework_async_macros::async_test]
    fn shallow_foundation_e2e() {
        let report = check_shallow_foundation(500.0, 80.0, 2.0, 30.0, 0.0, 18.0, 2.0, 1.5, 30_000.0, 0.3, DesignApproach::Da1Str, AnnexChoice::De, 25.0);
        assert!(!report.checks.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    fn pile_design_resistance_worked() {
        let r_s = part_1::shaft_resistance_kn(0.7, 0.6, 80.0, 12.0);
        assert!((r_s - 1266.69).abs() < 1.0);
        let r_b = part_1::base_resistance_kn(2500.0, 0.28);
        assert!((r_b - 700.0).abs() < 0.1);
        let r_c_d = part_1::pile_design_resistance_kn(r_b, r_s, DesignApproach::Da2, AnnexChoice::De);
        let expected = r_b / 1.1 + r_s / 1.1;
        assert!((r_c_d - expected).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn pile_correlation_factors_n2_matches_annex_a_table() {
        let (xi_3, xi_4) = part_1::pile_correlation_factors(2);
        assert!((xi_3 - 1.35).abs() < 1e-9);
        assert!((xi_4 - 1.27).abs() < 1e-9);
        let mean_r_cal_kn = 1000.0;
        let min_r_cal_kn = 900.0;
        let r_k = part_1::pile_characteristic_resistance_kn(mean_r_cal_kn, min_r_cal_kn, 2);
        let expected = (mean_r_cal_kn / 1.35_f64).min(min_r_cal_kn / 1.27_f64);
        assert!((r_k - expected).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn pile_correlation_factors_boundary_cases() {
        assert_eq!(part_1::pile_correlation_factors(1), (1.40, 1.40));
        assert_eq!(part_1::pile_correlation_factors(5), (1.30, 1.15));
        assert_eq!(part_1::pile_correlation_factors(9), (1.30, 1.15));
    }

    #[semio_framework_async_macros::async_test]
    fn investigation_depth_check_pass_and_fail() {
        let b_m = 2.0;
        let min_depth = part_2::min_investigation_depth_m(b_m);
        assert!((min_depth - 6.0).abs() < 1e-9);
        let pass = part_2::check_investigation_depth(8.0, b_m, AnnexChoice::De);
        assert_eq!(pass.status, CheckStatus::Pass);
        let fail = part_2::check_investigation_depth(4.0, b_m, AnnexChoice::De);
        assert_eq!(fail.status, CheckStatus::Fail);
    }

    #[semio_framework_async_macros::async_test]
    fn phi_from_cpt_worked_example() {
        let q_c_kpa = 15_000.0;
        let sigma_v0_kpa = 100.0;
        let phi = part_2::phi_from_cpt_deg(q_c_kpa, sigma_v0_kpa);
        let expected = (0.38 * (q_c_kpa / sigma_v0_kpa).log10() + 0.1).atan().to_degrees();
        assert!((phi - expected).abs() < 1e-9);
        assert!(phi > 25.0 && phi < 45.0);
    }

    #[semio_framework_async_macros::async_test]
    fn phi_from_spt_worked_example() {
        let phi = part_2::phi_from_spt_deg(20.0);
        let expected = 27.1 + 0.3 * 20.0 - 0.00054 * 20.0 * 20.0;
        assert!((phi - expected).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn da2_star_de_diverges_from_da2_en_on_same_footing() {
        let q_d_de = part_1::design_bearing_capacity_kpa(30.0, 0.0, 18.0, 2.0, 1.5, DesignApproach::Da2, AnnexChoice::De);
        let q_d_en = part_1::design_bearing_capacity_kpa(30.0, 0.0, 18.0, 2.0, 1.5, DesignApproach::Da2, AnnexChoice::En);
        assert!(q_d_de < q_d_en);
        assert!((q_d_en / q_d_de - 1.4).abs() < 1e-9);
        assert_eq!(DesignApproach::Da2.label(AnnexChoice::De), "DA2*");
        assert_eq!(DesignApproach::Da2.label(AnnexChoice::En), "DA2");
    }
}
//#endregion 🧪️ComplianceHelpersTests
