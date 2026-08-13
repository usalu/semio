//! 🧬️ En1994 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1994 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Artifact {
    #[state(artifact)] pub annex: crate::document::AnnexChoice,
    #[state(artifact)] pub m_ed_knm: f64,
    #[state(artifact)] pub v_ed_kn: f64,
    #[state(artifact)] pub m_pla: f64,
    #[state(artifact)] pub m_pl_rd: f64,
    #[state(artifact)] pub eta: f64,
    #[state(artifact)] pub v_l_rd: f64,
    #[state(artifact)] pub insulation_thickness_mm: f64,
    #[state(artifact)] pub fire_rating: String,
    #[state(artifact)] pub deck_type: String,
    #[state(artifact)] pub delta_sigma_mpa: f64,
    #[state(artifact)] pub fatigue_detail: String,
    #[state(artifact)] pub d_mm: f64,
    #[state(artifact)] pub h_sc_mm: f64,
    #[state(artifact)] pub f_ck_mpa: f64,
    #[state(artifact)] pub f_u_mpa: f64,
    #[state(artifact)] pub e_cm_mpa: f64,
    #[state(artifact)] pub v_ed_per_stud_kn: f64,
    #[state(artifact)] pub span_m: f64,
    #[state(artifact)] pub f_y_mpa: f64,
    #[state(artifact)] pub n_cycles_stud: f64,
    #[state(artifact)] pub delta_tau_stud_mpa: f64,
    #[state(presence)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1994Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1994::En1994Snapshot {
        crate::artifacts::en1994::En1994Snapshot {
            annex: self.annex,
            m_ed_knm: self.m_ed_knm,
            v_ed_kn: self.v_ed_kn,
            m_pla: self.m_pla,
            m_pl_rd: self.m_pl_rd,
            eta: self.eta,
            v_l_rd: self.v_l_rd,
            insulation_thickness_mm: self.insulation_thickness_mm,
            fire_rating: self.fire_rating.clone(),
            deck_type: self.deck_type.clone(),
            delta_sigma_mpa: self.delta_sigma_mpa,
            fatigue_detail: self.fatigue_detail.clone(),
            d_mm: self.d_mm,
            h_sc_mm: self.h_sc_mm,
            f_ck_mpa: self.f_ck_mpa,
            f_u_mpa: self.f_u_mpa,
            e_cm_mpa: self.e_cm_mpa,
            v_ed_per_stud_kn: self.v_ed_per_stud_kn,
            span_m: self.span_m,
            f_y_mpa: self.f_y_mpa,
            n_cycles_stud: self.n_cycles_stud,
            delta_tau_stud_mpa: self.delta_tau_stud_mpa,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1994::En1994Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            m_ed_knm: snapshot.m_ed_knm,
            v_ed_kn: snapshot.v_ed_kn,
            m_pla: snapshot.m_pla,
            m_pl_rd: snapshot.m_pl_rd,
            eta: snapshot.eta,
            v_l_rd: snapshot.v_l_rd,
            insulation_thickness_mm: snapshot.insulation_thickness_mm,
            fire_rating: snapshot.fire_rating.clone(),
            deck_type: snapshot.deck_type.clone(),
            delta_sigma_mpa: snapshot.delta_sigma_mpa,
            fatigue_detail: snapshot.fatigue_detail.clone(),
            d_mm: snapshot.d_mm,
            h_sc_mm: snapshot.h_sc_mm,
            f_ck_mpa: snapshot.f_ck_mpa,
            f_u_mpa: snapshot.f_u_mpa,
            e_cm_mpa: snapshot.e_cm_mpa,
            v_ed_per_stud_kn: snapshot.v_ed_per_stud_kn,
            span_m: snapshot.span_m,
            f_y_mpa: snapshot.f_y_mpa,
            n_cycles_stud: snapshot.n_cycles_stud,
            delta_tau_stud_mpa: snapshot.delta_tau_stud_mpa,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1994::En1994Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1994` — twenty handcrafted schema leaves.
pub fn en1994_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1994",
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
    use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1994BuilderConstruction {
        snapshot: En1994Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1994BuilderConstruction {
        type Snapshot = En1994Snapshot;
        type Mutation = En1994Mutation;
        type Diff = En1994Diff;
        fn empty() -> Self { Self { snapshot: En1994Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::en1994::En1994Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1994Parts {
        pub snapshot: Option<En1994Snapshot>,
    }

    pub struct En1994AnalyzerAnalysis;

    impl ArtifactAnalysis for En1994AnalyzerAnalysis {
        type Parts = En1994Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1994", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1994Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1994Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1994Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1994BuilderFacets {
        construction: derived_construction::En1994BuilderConstruction,
        analysis: derived_analysis::En1994AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1994ComposerComposition,
    }
    builder: En1994Builder,
    analyzer: En1994Analyzer,
    composer: En1994Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1994 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. `part_1_1`/`part_1_2`/`part_2` and
/// `check_composite_beam` are pure function libraries; the snapshot-level composition
/// (`evaluate`, `check_full_composite`) lives in `💡️inferences`.
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity, QuantityKind};

// #region 🔖️AnnexParams
/// 🇪️🇺️ National-annex NDPs for EN 1994 (composite steel-concrete structures).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub choice: AnnexChoice,
    pub gamma_v: f64,
    pub gamma_c: f64,
    pub gamma_s: f64,
}

impl AnnexParams {
    /// 📖️ EN-recommended NDPs per EN 1994-1-1 §2.4.1.2.
    pub fn en() -> Self {
        Self { choice: AnnexChoice::En, gamma_v: 1.25, gamma_c: 1.5, gamma_s: 1.15 }
    }

    /// 🇩️🇪️ DIN EN 1994-1-1/NA does not amend γ_V, γ_C, or γ_S — intentionally equal to EN.
    pub fn de() -> Self {
        Self { choice: AnnexChoice::De, gamma_v: 1.25, gamma_c: 1.5, gamma_s: 1.15 }
    }

    pub fn for_annex(annex: AnnexChoice) -> Self {
        match annex {
            AnnexChoice::En => Self::en(),
            AnnexChoice::De => Self::de(),
        }
    }
}
// #endregion 🔖️AnnexParams

// #region 🔖️Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📐️ Full plastic moment M_pl,Rd [kNm] (steel + concrete).
    pub fn full_plastic_moment_knm(m_pla_knm: f64, m_plc_knm: f64) -> f64 {
        m_pla_knm + m_plc_knm
    }

    /// 📐️ Partial shear connection degree η = n_f / n_f,req.
    pub fn shear_connection_degree(n_f: u32, n_f_req: u32) -> f64 {
        n_f as f64 / n_f_req as f64
    }

    /// 📐️ Composite plastic moment with partial shear connection per EN 1994-1-1 §6.2.1(8).
    pub fn plastic_moment_partial_knm(m_pla_knm: f64, m_pl_rd_knm: f64, eta: f64) -> f64 {
        m_pla_knm + eta * (m_pl_rd_knm - m_pla_knm)
    }

    /// 📐️ Effective width b_eff [mm] per EN 1994-1-1 §5.4.1.2.
    pub fn effective_width_mm(span_mm: f64, b_0_mm: f64, beam_spacing_mm: f64) -> f64 {
        let be1 = span_mm / 8.0 + b_0_mm;
        let be2 = beam_spacing_mm / 2.0;
        (2.0 * be1).min(2.0 * be2)
    }

    /// 📐️ Longitudinal shear V_L [kN] per EN 1994-1-1 §6.6.2.
    pub fn longitudinal_shear_kn(delta_n_kn: f64, connector_spacing_mm: f64) -> f64 {
        delta_n_kn * 1000.0 / connector_spacing_mm
    }

    /// 📐️ Stud height-to-diameter reduction factor α per EN 1994-1-1 §6.6.3.1(1).
    pub fn stud_alpha(h_sc_mm: f64, d_mm: f64) -> f64 {
        let ratio = h_sc_mm / d_mm;
        if ratio > 4.0 {
            1.0
        } else {
            0.2 * (ratio + 1.0)
        }
    }

    /// 📐️ Shear connector resistance P_Rd [kN] per EN 1994-1-1 §6.6.3.1, Eq. 6.18/6.19 — governing branch is the lesser of stud shank shear-off and concrete/dowel crushing.
    pub fn connector_resistance_kn(d_mm: f64, h_sc_mm: f64, f_ck_mpa: f64, f_u_mpa: f64, e_cm_mpa: f64, annex: AnnexChoice) -> f64 {
        let params = AnnexParams::for_annex(annex);
        let alpha = stud_alpha(h_sc_mm, d_mm);
        let p_pl = 0.8 * f_u_mpa * std::f64::consts::PI * d_mm * d_mm / 4.0;
        let p_b = 0.29 * alpha * d_mm * d_mm * (f_ck_mpa * e_cm_mpa).sqrt();
        p_pl.min(p_b) / params.gamma_v / 1000.0
    }

    /// 📐️ Minimum degree of shear connection η_min per EN 1994-1-1 §6.6.1.2 (equal-flange rolled/welded sections, f_y ≤ 355 MPa).
    pub fn min_shear_connection_degree(span_m: f64, f_y_mpa: f64) -> f64 {
        let eta_min = if span_m <= 25.0 { 1.0 - (355.0 / f_y_mpa) * (0.75 - 0.03 * span_m) } else { 1.0 - (355.0 / f_y_mpa) * 0.30 };
        eta_min.max(0.4)
    }

    pub fn check_composite_bending(m_ed: f64, m_rd: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1994-1-1", "§6.2", "6.2"), Quantity::new(QuantityKind::Moment, m_ed * 1_000_000.0), Quantity::new(QuantityKind::Moment, m_rd * 1_000_000.0), "composite bending ULS", annex)
    }

    pub fn check_longitudinal_shear(v_ed_kn: f64, v_l_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1994-1-1", "§6.6", "6.6"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_l_rd_kn), "longitudinal shear", annex)
    }

    pub fn check_stud_resistance(v_ed_per_stud_kn: f64, d_mm: f64, h_sc_mm: f64, f_ck_mpa: f64, f_u_mpa: f64, e_cm_mpa: f64, annex: AnnexChoice) -> CheckResult {
        let p_rd = connector_resistance_kn(d_mm, h_sc_mm, f_ck_mpa, f_u_mpa, e_cm_mpa, annex);
        CheckResult::from_utilization(ClauseId::new("EN 1994-1-1", "§6.6.3.1", "6.6.3.1"), Quantity::force_kn(v_ed_per_stud_kn), Quantity::force_kn(p_rd), "shear stud resistance", annex)
    }

    pub fn check_shear_connection_degree(eta: f64, span_m: f64, f_y_mpa: f64, annex: AnnexChoice) -> CheckResult {
        let eta_min = min_shear_connection_degree(span_m, f_y_mpa);
        CheckResult::from_minimum(ClauseId::new("EN 1994-1-1", "§6.6.1.2", "6.6.1.2"), Quantity::new(QuantityKind::Dimensionless, eta), Quantity::new(QuantityKind::Dimensionless, eta_min), "minimum degree of shear connection", annex)
    }
}
// #endregion 🔖️Part1_1

// #region 🔖️Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥️ Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }

    /// 🔥️ Composite deck insulation thickness [mm] per EN 1994-1-2 Table 4.2.
    pub fn insulation_thickness_mm(rating: FireRating, deck_type: &str) -> f64 {
        let base = match rating {
            FireRating::R30 => 10.0,
            FireRating::R60 => 18.0,
            FireRating::R90 => 28.0,
            FireRating::R120 => 40.0,
        };
        match deck_type {
            "trapezoidal" => base,
            "re-entrant" => base * 1.1,
            _ => base,
        }
    }

    pub fn check_fire_composite(thickness_mm: f64, rating: FireRating, deck_type: &str) -> CheckResult {
        let required = insulation_thickness_mm(rating, deck_type);
        CheckResult::from_utilization(ClauseId::new("EN 1994-1-2", "§4.2", "4.2"), Quantity::length_m(required / 1000.0), Quantity::length_m(thickness_mm / 1000.0), "composite fire insulation", AnnexChoice::De)
    }
}
// #endregion 🔖️Part1_2

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🌉️ Bridge composite fatigue detail category per EN 1994-2 §8.
    pub fn bridge_fatigue_category(detail: &str) -> u8 {
        match detail {
            "stud_welded" => 80,
            "shear_connector" => 71,
            "reinforcement" => 90,
            _ => 71,
        }
    }

    /// 🇪️🇺️ Partial factor γ_Mf,s for shear stud fatigue per EN 1994-2 §6.8.3(1).
    pub const GAMMA_MF_S: f64 = 1.0;

    /// 📐️ Reference detail category Δτ_c [MPa] at N_ref cycles for a headed stud shear connector, per EN 1994-2 Table 6.1.
    pub const STUD_DELTA_TAU_C_MPA: f64 = 90.0;
    pub const STUD_N_REF: f64 = 2.0e6;
    pub const STUD_FATIGUE_SLOPE_M: f64 = 8.0;

    /// 📐️ Stud fatigue shear-stress resistance Δτ_c(N) [MPa] from the S-N curve per EN 1994-2 §6.8.3, Eq. 6.24.
    pub fn stud_fatigue_resistance_mpa(n_cycles: f64) -> f64 {
        STUD_DELTA_TAU_C_MPA * (STUD_N_REF / n_cycles).powf(1.0 / STUD_FATIGUE_SLOPE_M)
    }

    /// 🌉️ Stud fatigue check Δτ ≤ Δτ_c(N) / γ_Mf,s per EN 1994-2 §6.8.3.
    pub fn check_stud_fatigue(delta_tau_mpa: f64, n_cycles: f64) -> CheckResult {
        let limit = stud_fatigue_resistance_mpa(n_cycles) / GAMMA_MF_S;
        CheckResult::from_utilization(ClauseId::new("EN 1994-2", "§6.8.3", "6.8.3"), Quantity::stress_mpa(delta_tau_mpa), Quantity::stress_mpa(limit), "stud fatigue shear stress range", AnnexChoice::En)
    }

    /// 🌉️ Bridge composite bending + fatigue check.
    pub fn check_bridge_composite(m_ed_knm: f64, m_rd_knm: f64, delta_sigma_mpa: f64, detail: &str) -> CheckReport {
        let mut report = CheckReport::default();
        report.push(part_1_1::check_composite_bending(m_ed_knm, m_rd_knm, AnnexChoice::En));
        let category = bridge_fatigue_category(detail);
        let limit = category as f64;
        report.push(CheckResult::from_utilization(ClauseId::new("EN 1994-2", "§8", "8.1"), Quantity::stress_mpa(delta_sigma_mpa), Quantity::stress_mpa(limit), "bridge composite fatigue", AnnexChoice::En));
        report
    }
}
// #endregion 🔖️Part2

/// 📋️ Composite slab beam check.
pub fn check_composite_beam(m_ed_knm: f64, v_ed_kn: f64, m_pla: f64, m_pl_rd: f64, eta: f64, v_l_rd: f64, annex: AnnexChoice) -> CheckReport {
    let m_rd = part_1_1::plastic_moment_partial_knm(m_pla, m_pl_rd, eta);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_composite_bending(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_longitudinal_shear(v_ed_kn, v_l_rd, annex));
    report
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    #[test]
    fn composite_beam_e2e() {
        let report = check_composite_beam(200.0, 120.0, 80.0, 250.0, 0.75, 150.0, AnnexChoice::De);
        assert!(!report.checks.is_empty());
        let m_rd: f64 = 80.0 + 0.75 * (250.0 - 80.0);
        assert!((m_rd - 207.5).abs() < 0.1);
    }

    #[test]
    fn effective_width_8m_span() {
        let beff = part_1_1::effective_width_mm(8000.0, 80.0, 3000.0);
        assert!((beff - 2160.0).abs() < 1.0);
    }

    #[test]
    fn partial_shear_connection_eta() {
        let eta = part_1_1::shear_connection_degree(15, 20);
        assert!((eta - 0.75).abs() < 0.01);
        let m_rd = part_1_1::plastic_moment_partial_knm(100.0, 300.0, eta);
        assert!((m_rd - 250.0).abs() < 0.1);
    }

    #[test]
    fn longitudinal_shear_vl() {
        let v_l = part_1_1::longitudinal_shear_kn(500.0, 200.0);
        assert!((v_l - 2500.0).abs() < 1.0);
    }

    #[test]
    fn stud_connector_resistance_worked_example() {
        let d = 19.0_f64;
        let h_sc = 5.0 * d;
        let f_u: f64 = 450.0;
        let f_ck: f64 = 30.0;
        let e_cm: f64 = 33_000.0;
        let alpha = part_1_1::stud_alpha(h_sc, d);
        assert!((alpha - 1.0).abs() < 1e-9);
        let p_pl = 0.8 * f_u * std::f64::consts::PI * d * d / 4.0;
        let p_b = 0.29 * alpha * d * d * (f_ck * e_cm).sqrt();
        assert!(p_pl < p_b, "shank shear-off branch should govern for this worked example");
        let expected_p_rd_kn = p_pl / 1.25 / 1000.0;
        assert!((expected_p_rd_kn - 81.656).abs() < 0.01);
        let p_rd = part_1_1::connector_resistance_kn(d, h_sc, f_ck, f_u, e_cm, AnnexChoice::En);
        assert!((p_rd - expected_p_rd_kn).abs() < 1e-6);
    }

    #[test]
    fn min_shear_connection_degree_span_8m() {
        let eta_min = part_1_1::min_shear_connection_degree(8.0, 355.0);
        assert!((eta_min - 0.49).abs() < 1e-6);
    }

    #[test]
    fn fire_insulation_r60() {
        let t = part_1_2::insulation_thickness_mm(part_1_2::FireRating::R60, "trapezoidal");
        assert!((t - 18.0).abs() < 0.1);
    }

    #[test]
    fn bridge_composite_fatigue() {
        let report = part_2::check_bridge_composite(180.0, 250.0, 65.0, "stud_welded");
        assert_eq!(report.checks.len(), 2);
    }

    #[test]
    fn stud_fatigue_resistance_at_reference_cycles() {
        let delta_tau_c = part_2::stud_fatigue_resistance_mpa(part_2::STUD_N_REF);
        assert!((delta_tau_c - 90.0).abs() < 1e-9);
    }

    #[test]
    fn annex_params_document_equality() {
        let en = AnnexParams::en();
        let de = AnnexParams::de();
        assert!((en.gamma_v - de.gamma_v).abs() < 1e-9);
        assert!((en.gamma_c - de.gamma_c).abs() < 1e-9);
        assert!((en.gamma_s - de.gamma_s).abs() < 1e-9);
        let p_rd_en = part_1_1::connector_resistance_kn(19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::En);
        let p_rd_de = part_1_1::connector_resistance_kn(19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::De);
        assert!((p_rd_en - p_rd_de).abs() < 1e-9);
        let check_en = part_1_1::check_stud_resistance(40.0, 19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::En);
        let check_de = part_1_1::check_stud_resistance(40.0, 19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::De);
        assert!((check_en.utilization - check_de.utilization).abs() < 1e-9);
    }
}
//#endregion 🧪️ComplianceHelpersTests

