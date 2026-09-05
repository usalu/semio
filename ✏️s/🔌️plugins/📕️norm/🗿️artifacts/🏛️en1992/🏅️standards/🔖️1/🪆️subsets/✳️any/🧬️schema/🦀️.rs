//! 🧬️ En1992 artifact schema — every field of the artifact with its state class.

use crate::artifacts::en1992::part_1_2::FireRating;
use crate::artifacts::en1992::part_3::TightnessClass;
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full En1992 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub f_ck: f64,
    #[state(artifact)]
    pub b_mm: f64,
    #[state(artifact)]
    pub d_mm: f64,
    #[state(artifact)]
    pub a_s_mm2: f64,
    #[state(artifact)]
    pub f_yk: f64,
    #[state(artifact)]
    pub rho_l: f64,
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub p_kn: f64,
    #[state(artifact)]
    pub a_c_mm2: f64,
    #[state(artifact)]
    pub use_fem: bool,
    #[state(artifact)]
    pub span_m: f64,
    #[state(artifact)]
    pub udl_kn_m: f64,
    #[state(artifact)]
    pub fire_rating: FireRating,
    #[state(artifact)]
    pub provided_axis_distance_mm: f64,
    #[state(artifact)]
    pub bridge_sigma_c_mpa: f64,
    #[state(artifact)]
    pub bridge_delta_sigma_s_mpa: f64,
    #[state(artifact)]
    pub tightness_class: TightnessClass,
    #[state(artifact)]
    pub hd_over_h: f64,
    #[state(artifact)]
    pub liquid_sigma_s_mpa: f64,
    #[state(artifact)]
    pub liquid_rho_p_eff: f64,
    #[state(artifact)]
    pub liquid_f_ct_eff_mpa: f64,
    #[state(artifact)]
    pub liquid_e_s_mpa: f64,
    #[state(artifact)]
    pub liquid_s_r_max_mm: f64,
    #[state(artifact)]
    pub anchor_h_ef_mm: f64,
    #[state(artifact)]
    pub anchor_cracked: bool,
    #[state(artifact)]
    pub anchor_f_uk_mpa: f64,
    #[state(artifact)]
    pub anchor_f_yk_mpa: f64,
    #[state(artifact)]
    pub anchor_a_s_mm2: f64,
    #[state(artifact)]
    pub anchor_d_mm: f64,
    #[state(artifact)]
    pub anchor_c1_mm: f64,
    #[state(artifact)]
    pub anchor_n_ed_kn: f64,
    #[state(artifact)]
    pub anchor_v_ed_kn: f64,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1992Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1992::En1992Snapshot {
        crate::artifacts::en1992::En1992Snapshot {
            annex: self.annex,
            m_ed_knm: self.m_ed_knm,
            v_ed_kn: self.v_ed_kn,
            f_ck: self.f_ck,
            b_mm: self.b_mm,
            d_mm: self.d_mm,
            a_s_mm2: self.a_s_mm2,
            f_yk: self.f_yk,
            rho_l: self.rho_l,
            n_ed_kn: self.n_ed_kn,
            p_kn: self.p_kn,
            a_c_mm2: self.a_c_mm2,
            use_fem: self.use_fem,
            span_m: self.span_m,
            udl_kn_m: self.udl_kn_m,
            fire_rating: self.fire_rating,
            provided_axis_distance_mm: self.provided_axis_distance_mm,
            bridge_sigma_c_mpa: self.bridge_sigma_c_mpa,
            bridge_delta_sigma_s_mpa: self.bridge_delta_sigma_s_mpa,
            tightness_class: self.tightness_class,
            hd_over_h: self.hd_over_h,
            liquid_sigma_s_mpa: self.liquid_sigma_s_mpa,
            liquid_rho_p_eff: self.liquid_rho_p_eff,
            liquid_f_ct_eff_mpa: self.liquid_f_ct_eff_mpa,
            liquid_e_s_mpa: self.liquid_e_s_mpa,
            liquid_s_r_max_mm: self.liquid_s_r_max_mm,
            anchor_h_ef_mm: self.anchor_h_ef_mm,
            anchor_cracked: self.anchor_cracked,
            anchor_f_uk_mpa: self.anchor_f_uk_mpa,
            anchor_f_yk_mpa: self.anchor_f_yk_mpa,
            anchor_a_s_mm2: self.anchor_a_s_mm2,
            anchor_d_mm: self.anchor_d_mm,
            anchor_c1_mm: self.anchor_c1_mm,
            anchor_n_ed_kn: self.anchor_n_ed_kn,
            anchor_v_ed_kn: self.anchor_v_ed_kn,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1992::En1992Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            m_ed_knm: snapshot.m_ed_knm,
            v_ed_kn: snapshot.v_ed_kn,
            f_ck: snapshot.f_ck,
            b_mm: snapshot.b_mm,
            d_mm: snapshot.d_mm,
            a_s_mm2: snapshot.a_s_mm2,
            f_yk: snapshot.f_yk,
            rho_l: snapshot.rho_l,
            n_ed_kn: snapshot.n_ed_kn,
            p_kn: snapshot.p_kn,
            a_c_mm2: snapshot.a_c_mm2,
            use_fem: snapshot.use_fem,
            span_m: snapshot.span_m,
            udl_kn_m: snapshot.udl_kn_m,
            fire_rating: snapshot.fire_rating,
            provided_axis_distance_mm: snapshot.provided_axis_distance_mm,
            bridge_sigma_c_mpa: snapshot.bridge_sigma_c_mpa,
            bridge_delta_sigma_s_mpa: snapshot.bridge_delta_sigma_s_mpa,
            tightness_class: snapshot.tightness_class,
            hd_over_h: snapshot.hd_over_h,
            liquid_sigma_s_mpa: snapshot.liquid_sigma_s_mpa,
            liquid_rho_p_eff: snapshot.liquid_rho_p_eff,
            liquid_f_ct_eff_mpa: snapshot.liquid_f_ct_eff_mpa,
            liquid_e_s_mpa: snapshot.liquid_e_s_mpa,
            liquid_s_r_max_mm: snapshot.liquid_s_r_max_mm,
            anchor_h_ef_mm: snapshot.anchor_h_ef_mm,
            anchor_cracked: snapshot.anchor_cracked,
            anchor_f_uk_mpa: snapshot.anchor_f_uk_mpa,
            anchor_f_yk_mpa: snapshot.anchor_f_yk_mpa,
            anchor_a_s_mm2: snapshot.anchor_a_s_mm2,
            anchor_d_mm: snapshot.anchor_d_mm,
            anchor_c1_mm: snapshot.anchor_c1_mm,
            anchor_n_ed_kn: snapshot.anchor_n_ed_kn,
            anchor_v_ed_kn: snapshot.anchor_v_ed_kn,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1992::En1992Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1992` — twenty handcrafted schema leaves.
pub fn en1992_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1992",
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
    use crate::artifacts::en1992::{En1992Diff, En1992Mutation, En1992Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1992BuilderConstruction {
        snapshot: En1992Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1992BuilderConstruction {
        type Snapshot = En1992Snapshot;
        type Mutation = En1992Mutation;
        type Diff = En1992Diff;
        fn empty() -> Self {
            Self { snapshot: En1992Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1992Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1992Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::en1992::En1992Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1992Parts {
        pub snapshot: Option<En1992Snapshot>,
    }

    pub struct En1992AnalyzerAnalysis;

    impl ArtifactAnalysis for En1992AnalyzerAnalysis {
        type Parts = En1992Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1992", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1992Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1992Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1992Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1992BuilderFacets {
        construction: En1992BuilderConstruction,
        analysis: En1992AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1992ComposerComposition,
    }
    builder: En1992Builder,
    analyzer: En1992Analyzer,
    composer: En1992Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1992 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. Every `part_1_N`/`part_N` module (including the
/// `cross-fem`-gated `Fem` region) is a pure function library; the snapshot-level composition
/// (`evaluate`) lives in `💡️inferences`.
use crate::document::{table_lookup_linear, AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, Quantity, TableEntry1D};

// #region 🔖️NaDe
pub mod na_de {
    use super::AnnexChoice;

    /// 🇪️🇺️ Material factors that genuinely diverge between the EN-recommended values and DIN EN 1992-1-1/NA: α_cc, α_ct per §3.1.6(1)P/(2)P.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexParams {
        pub alpha_cc: f64,
        pub alpha_ct: f64,
        pub gamma_c: f64,
        pub gamma_s: f64,
    }

    impl AnnexParams {
        /// 🇪️🇺️ EN-recommended values (α_cc = α_ct = 1.0).
        pub fn en() -> Self {
            Self { alpha_cc: 1.0, alpha_ct: 1.0, gamma_c: 1.5, gamma_s: 1.15 }
        }

        /// 🇩️🇪️ DIN EN 1992-1-1/NA values (α_cc = α_ct = 0.85).
        pub fn de() -> Self {
            Self { alpha_cc: 0.85, alpha_ct: 0.85, gamma_c: 1.5, gamma_s: 1.15 }
        }

        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice {
                AnnexChoice::En => Self::en(),
                AnnexChoice::De => Self::de(),
            }
        }

        /// 📐️ Design compressive strength f_cd = α_cc·f_ck/γ_C [MPa].
        pub fn f_cd_mpa(&self, f_ck_mpa: f64) -> f64 {
            self.alpha_cc * f_ck_mpa / self.gamma_c
        }

        /// 📐️ Design tensile strength f_ctd = α_ct·f_ctk/γ_C [MPa].
        pub fn f_ctd_mpa(&self, f_ctk_mpa: f64) -> f64 {
            self.alpha_ct * f_ctk_mpa / self.gamma_c
        }
    }
}
// #endregion 🔖️NaDe

// #region 🔖️Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📐️ Flexural resistance M_Rd [kNm] per EN 1992-1-1 §6.1.
    pub fn flexural_resistance_knm(f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, annex: AnnexChoice) -> f64 {
        let f_cd = na_de::AnnexParams::for_choice(annex).f_cd_mpa(f_ck) / 1000.0;
        let f_yd = f_yk / 1.15 / 1000.0;
        let x = a_s_mm2 * f_yd / (0.8 * b_mm * f_cd);
        let z = d_mm - 0.4 * x;
        a_s_mm2 * f_yd * z / 1_000_000.0
    }

    /// 📐️ Shear resistance V_Rd,c [kN] per EN 1992-1-1 §6.2.2.
    pub fn shear_resistance_vrdc_kn(b_mm: f64, d_mm: f64, f_ck: f64, rho_l: f64, n_ed_kn: f64) -> f64 {
        let k = (200.0 / d_mm).min(2.0).sqrt();
        let sigma_cp = (n_ed_kn * 1000.0 / (b_mm * d_mm)).max(0.0);
        let v_min = 0.035 * k.powf(1.5) * f_ck.sqrt();
        let v_rd = (0.18 / 1.5) * k * (100.0 * rho_l * f_ck).sqrt() + 0.15 * sigma_cp;
        v_rd.max(v_min) * b_mm * d_mm / 1000.0
    }

    /// 🕳️ Punching shear strength v_Rd,max [MPa] per EN 1992-1-1 Eq. 6.50.
    pub fn punching_v_rd_max_mpa(f_ck: f64, annex: AnnexChoice) -> f64 {
        let f_cd = na_de::AnnexParams::for_choice(annex).f_cd_mpa(f_ck);
        let nu = 0.6 * (1.0 - f_ck / 250.0);
        0.5 * nu * f_cd
    }

    /// 🕳️ Punching shear resistance V_Rd,max [kN] around perimeter u_1.
    pub fn punching_resistance_kn(f_ck: f64, u_1_mm: f64, d_mm: f64, annex: AnnexChoice) -> f64 {
        punching_v_rd_max_mpa(f_ck, annex) * u_1_mm * d_mm / 1000.0
    }

    /// 🔁️ Torsional resistance T_Rd [kNm] per EN 1992-1-1 §6.3.2 (thin-walled hollow section).
    pub fn torsion_resistance_knm(f_ck: f64, a_k_mm2: f64, t_mm: f64, annex: AnnexChoice) -> f64 {
        let f_cd = na_de::AnnexParams::for_choice(annex).f_cd_mpa(f_ck) / 1000.0;
        let nu = 0.6 * (1.0 - f_ck / 250.0);
        let alpha_cw = 1.0;
        2.0 * nu * alpha_cw * f_cd * t_mm * a_k_mm2 / 1_000_000.0
    }

    /// 📏️ Slenderness λ = l_0 / i.
    pub fn slenderness_lambda(l_0_mm: f64, i_mm: f64) -> f64 {
        l_0_mm / i_mm
    }

    /// 📏️ Radius of gyration i [mm] from area and second moment.
    pub fn radius_of_gyration_mm(a_mm2: f64, i_mm4: f64) -> f64 {
        (i_mm4 / a_mm2).sqrt()
    }

    /// 🪟️ Crack width w_k [mm] per EN 1992-1-1 Eq. 7.8.
    pub fn crack_width_wk_mm(eps_sm: f64, eps_cm: f64, s_r_max_mm: f64) -> f64 {
        (eps_sm - eps_cm).max(0.0) * s_r_max_mm
    }

    /// 🪟️ Mean steel strain ε_sm per EN 1992-1-1 Eq. 7.9.
    pub fn steel_strain_eps_sm(sigma_s_mpa: f64, rho_p_eff: f64, f_ct_eff_mpa: f64, e_s_mpa: f64) -> f64 {
        let term = (f_ct_eff_mpa / rho_p_eff / e_s_mpa).max(0.6 * sigma_s_mpa / e_s_mpa);
        (sigma_s_mpa / e_s_mpa) * (1.0 - term).max(0.4)
    }

    /// 📉️ Immediate deflection δ [mm] of simply supported beam under UDL.
    pub fn deflection_ss_udl_mm(w_kn_m: f64, span_m: f64, e_mpa: f64, i_mm4: f64) -> f64 {
        let w = w_kn_m;
        let l = span_m * 1000.0;
        5.0 * w * l.powi(4) / (384.0 * e_mpa * i_mm4)
    }

    pub fn check_flexure(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.1", "6.1"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "flexural ULS",
            annex,
        )
    }

    pub fn check_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§6.2", "6.2"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "shear ULS", annex)
    }

    pub fn check_punching(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§6.4", "6.4"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "punching shear ULS", annex)
    }

    pub fn check_torsion(t_ed_knm: f64, t_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.3", "6.3"),
            Quantity::new(crate::document::QuantityKind::Moment, t_ed_knm * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, t_rd_knm * 1_000_000.0),
            "torsion ULS",
            annex,
        )
    }

    pub fn check_crack_width(w_k_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§7.3", "7.3"), Quantity::length_m(w_k_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "crack width SLS", annex)
    }

    pub fn check_deflection(delta_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§7.4", "7.4"), Quantity::length_m(delta_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "deflection SLS", annex)
    }

    /// 🎯️ Transfer stress σ_c = P / A_c [MPa] at prestressing.
    pub fn prestress_transfer_stress_mpa(p_kn: f64, a_c_mm2: f64) -> f64 {
        p_kn * 1000.0 / a_c_mm2
    }

    /// 🎯️ Maximum transfer stress limit 0.6·f_ck per EN 1992-1-1 §5.10.9.
    pub fn prestress_transfer_limit_mpa(f_ck_mpa: f64) -> f64 {
        0.6 * f_ck_mpa
    }

    pub fn check_prestress_transfer(sigma_c_mpa: f64, f_ck_mpa: f64, annex: AnnexChoice) -> CheckResult {
        let limit = prestress_transfer_limit_mpa(f_ck_mpa);
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§5.10", "5.10"), Quantity::stress_mpa(sigma_c_mpa), Quantity::stress_mpa(limit), "prestress transfer ULS", annex)
    }
}
// #endregion 🔖️Part1_1

// #region 🔖️Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🏗️ Structural element type for fire cover lookup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ElementType {
        Slab,
        Beam,
        Column,
    }

    /// 🔥️ Minimum axis distance a_min [mm] per EN 1992-1-2 Table 5.5 (simplified tabulated values).
    pub fn min_axis_distance_mm(element: ElementType, rating: FireRating) -> f64 {
        match (element, rating) {
            (ElementType::Slab, FireRating::R30) => 10.0,
            (ElementType::Slab, FireRating::R60) => 20.0,
            (ElementType::Slab, FireRating::R90) => 30.0,
            (ElementType::Slab, FireRating::R120) => 40.0,
            (ElementType::Beam, FireRating::R30) => 25.0,
            (ElementType::Beam, FireRating::R60) => 35.0,
            (ElementType::Beam, FireRating::R90) => 50.0,
            (ElementType::Beam, FireRating::R120) => 65.0,
            (ElementType::Column, FireRating::R30) => 25.0,
            (ElementType::Column, FireRating::R60) => 40.0,
            (ElementType::Column, FireRating::R90) => 55.0,
            (ElementType::Column, FireRating::R120) => 65.0,
        }
    }

    pub fn check_fire_cover(cover_mm: f64, element: ElementType, rating: FireRating) -> CheckResult {
        let required = min_axis_distance_mm(element, rating);
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-2", "§4.2", "4.2"), Quantity::length_m(required / 1000.0), Quantity::length_m(cover_mm / 1000.0), "fire axis distance", AnnexChoice::De)
    }

    /// 🔥️ Table 5.5 (b_min, a) [mm] combinations per EN 1992-1-2 §5.6.3 for simply-supported rectangular beams.
    fn table_5_5(rating: FireRating) -> &'static [TableEntry1D] {
        match rating {
            FireRating::R30 => &[TableEntry1D { x: 80.0, y: 25.0 }, TableEntry1D { x: 120.0, y: 15.0 }],
            FireRating::R60 => &[TableEntry1D { x: 120.0, y: 40.0 }, TableEntry1D { x: 160.0, y: 35.0 }, TableEntry1D { x: 200.0, y: 30.0 }, TableEntry1D { x: 300.0, y: 25.0 }],
            FireRating::R90 => &[TableEntry1D { x: 150.0, y: 55.0 }, TableEntry1D { x: 200.0, y: 45.0 }, TableEntry1D { x: 300.0, y: 40.0 }, TableEntry1D { x: 400.0, y: 35.0 }],
            FireRating::R120 => &[TableEntry1D { x: 200.0, y: 65.0 }, TableEntry1D { x: 240.0, y: 60.0 }, TableEntry1D { x: 300.0, y: 55.0 }, TableEntry1D { x: 500.0, y: 50.0 }],
        }
    }

    /// 🔥️ Required axis distance a [mm] for a simply-supported rectangular beam of given width, interpolated from Table 5.5.
    pub fn required_axis_distance_beam_mm(width_mm: f64, rating: FireRating) -> f64 {
        table_lookup_linear(table_5_5(rating), width_mm)
    }

    /// 🔥️ Simply-supported beam fire check: provided axis distance vs Table 5.5 requirement for the given width.
    pub fn check_fire_beam_axis_distance(width_mm: f64, provided_a_mm: f64, rating: FireRating) -> CheckResult {
        let required = required_axis_distance_beam_mm(width_mm, rating);
        CheckResult::from_minimum(ClauseId::new("EN 1992-1-2", "Table 5.5", "5.6.3"), Quantity::length_m(provided_a_mm / 1000.0), Quantity::length_m(required / 1000.0), "fire simply-supported beam axis distance", AnnexChoice::En)
    }
}
// #endregion 🔖️Part1_2

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🌉️ Load-cycle amplification factor γ_F,fat for the fatigue action per EN 1992-2 §6.8.
    pub const GAMMA_F_FAT: f64 = 1.0;

    /// 🌉️ Partial factor γ_S,fat for reinforcement fatigue resistance per EN 1992-1-1 §2.4.2.4.
    pub const GAMMA_S_FAT: f64 = 1.15;

    /// 🔁️ Reinforcement fatigue stress range Δσ_Rsk(N*) [MPa] at N* = 10⁶ cycles, straight bars, per EN 1992-1-1 Table 6.3N.
    pub const DELTA_SIGMA_RSK_MPA: f64 = 162.5;

    pub fn check_bridge_flexure(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::En)
    }

    /// 🌉️ Concrete compressive stress limit 0.6·f_ck [MPa] under the frequent combination per EN 1992-2 §7.2.
    pub fn concrete_stress_limit_frequent_mpa(f_ck: f64) -> f64 {
        0.6 * f_ck
    }

    pub fn check_bridge_concrete_stress(sigma_c_mpa: f64, f_ck: f64) -> CheckResult {
        let limit = concrete_stress_limit_frequent_mpa(f_ck);
        CheckResult::from_utilization(ClauseId::new("EN 1992-2", "§7.2", "7.2"), Quantity::stress_mpa(sigma_c_mpa), Quantity::stress_mpa(limit), "bridge concrete compressive stress, frequent combination", AnnexChoice::En)
    }

    /// 🔁️ Design fatigue resistance Δσ_Rsk(N*)/γ_S,fat [MPa] per EN 1992-1-1 §6.8.
    pub fn fatigue_resistance_design_mpa() -> f64 {
        DELTA_SIGMA_RSK_MPA / GAMMA_S_FAT
    }

    /// 🔁️ Reinforcement fatigue verification γ_F,fat·Δσ_s ≤ Δσ_Rsk(N*)/γ_S,fat per EN 1992-2 §6.8.
    pub fn check_bridge_fatigue(delta_sigma_s_mpa: f64) -> CheckResult {
        let demand = GAMMA_F_FAT * delta_sigma_s_mpa;
        let resistance = fatigue_resistance_design_mpa();
        CheckResult::from_utilization(ClauseId::new("EN 1992-2", "§6.8", "6.8.4"), Quantity::stress_mpa(demand), Quantity::stress_mpa(resistance), "reinforcement fatigue", AnnexChoice::En)
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    /// 💧️ Exposure class steel stress limit σ_s,lim [MPa] per EN 1992-3 Table 7.1N.
    pub fn steel_stress_limit_mpa(exposure: &str) -> f64 {
        match exposure {
            "XC1" | "XC2" => 250.0,
            "XC3" | "XC4" => 200.0,
            "XD1" | "XD2" | "XD3" => 160.0,
            "XS1" | "XS2" | "XS3" => 160.0,
            _ => 200.0,
        }
    }

    /// 🪟️ Liquid-retaining crack width w_k [mm] with steel stress limit per EN 1992-3 §7.
    pub fn crack_width_liquid_mm(sigma_s_mpa: f64, exposure: &str, s_r_max_mm: f64, e_s_mpa: f64) -> f64 {
        let limit = steel_stress_limit_mpa(exposure);
        let sigma_eff = sigma_s_mpa.min(limit);
        let eps_sm = sigma_eff / e_s_mpa;
        eps_sm * s_r_max_mm
    }

    pub fn check_liquid_crack_width(w_k: f64, limit: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-3", "§7", "7.1"), Quantity::length_m(w_k / 1000.0), Quantity::length_m(limit / 1000.0), "liquid retaining crack width SLS", AnnexChoice::En)
    }

    pub fn check_steel_stress(sigma_s_mpa: f64, exposure: &str) -> CheckResult {
        let limit = steel_stress_limit_mpa(exposure);
        CheckResult::from_utilization(ClauseId::new("EN 1992-3", "§7", "7.2"), Quantity::stress_mpa(sigma_s_mpa), Quantity::stress_mpa(limit), "liquid retaining steel stress SLS", AnnexChoice::En)
    }

    /// 💧️ Tightness-class crack-width limit w_k,lim [mm] per EN 1992-3 Table 7.1N; `None` means TC0 has no crack-width requirement. TC2 interpolates between w_k1 = 0.2mm (h_D/h = 5) and w_k1 = 0.05mm (h_D/h = 35).
    pub fn tightness_crack_width_limit_mm(class: TightnessClass, hd_over_h: f64) -> Option<f64> {
        match class {
            TightnessClass::Tc0 => None,
            TightnessClass::Tc1 => Some(0.3),
            TightnessClass::Tc2 => {
                let table = [TableEntry1D { x: 5.0, y: 0.2 }, TableEntry1D { x: 35.0, y: 0.05 }];
                Some(table_lookup_linear(&table, hd_over_h))
            }
        }
    }

    /// 🪟️ Liquid-retaining crack width [mm], reusing the general EN 1992-1-1 §7.3 mechanics (Eq. 7.8/7.9).
    pub fn crack_width_tightness_mm(sigma_s_mpa: f64, rho_p_eff: f64, f_ct_eff_mpa: f64, e_s_mpa: f64, s_r_max_mm: f64) -> f64 {
        let eps_sm = part_1_1::steel_strain_eps_sm(sigma_s_mpa, rho_p_eff, f_ct_eff_mpa, e_s_mpa);
        part_1_1::crack_width_wk_mm(eps_sm, 0.0, s_r_max_mm)
    }

    pub fn check_tightness_crack_width(w_k_mm: f64, class: TightnessClass, hd_over_h: f64) -> CheckResult {
        let clause = ClauseId::new("EN 1992-3", "Table 7.1N", "7.3.2");
        match tightness_crack_width_limit_mm(class, hd_over_h) {
            Some(limit) => CheckResult::from_utilization(clause, Quantity::length_m(w_k_mm / 1000.0), Quantity::length_m(limit / 1000.0), "liquid retaining tightness-class crack width", AnnexChoice::En),
            None => {
                CheckResult { clause, status: CheckStatus::NotApplicable, computed: Quantity::length_m(w_k_mm / 1000.0), limit: Quantity::length_m(0.0), utilization: 0.0, message: "TC0: no crack-width requirement".into(), annex: AnnexChoice::En }
            }
        }
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
/// ⚓️ EN 1992-4: design of fastenings (anchors) to concrete — steel, concrete cone and edge breakout failure modes.
pub mod part_4 {
    use super::*;

    /// ⚓️ Partial factor for concrete failure modes γ_Mc per EN 1992-4 §4.4.
    pub const GAMMA_MC: f64 = 1.5;

    /// ⚓️ Partial factor for steel failure γ_Ms = max(1.2·f_uk/f_yk, 1.4) per EN 1992-4 §4.4.2.
    pub fn gamma_ms(f_uk_mpa: f64, f_yk_mpa: f64) -> f64 {
        (1.2 * f_uk_mpa / f_yk_mpa).max(1.4)
    }

    /// ⚓️ Steel failure characteristic resistance N_Rk,s = A_s·f_uk [N] per EN 1992-4 §7.2.1.4.
    pub fn steel_resistance_n_rk_s_n(a_s_mm2: f64, f_uk_mpa: f64) -> f64 {
        a_s_mm2 * f_uk_mpa
    }

    pub fn steel_resistance_design_n(a_s_mm2: f64, f_uk_mpa: f64, f_yk_mpa: f64) -> f64 {
        steel_resistance_n_rk_s_n(a_s_mm2, f_uk_mpa) / gamma_ms(f_uk_mpa, f_yk_mpa)
    }

    /// ⚓️ Concrete cone factor k [N^0.5/mm^0.5] per EN 1992-4 §7.2.1.5: cracked vs uncracked concrete.
    pub fn concrete_cone_k(cracked: bool) -> f64 {
        if cracked {
            7.7
        } else {
            11.0
        }
    }

    /// ⚓️ Basic concrete cone characteristic resistance N⁰_Rk,c = k·√f_ck·h_ef^1.5 [N, mm, MPa] per EN 1992-4 Eq. 7.2.
    pub fn concrete_cone_resistance_n0_rk_c_n(f_ck_mpa: f64, h_ef_mm: f64, cracked: bool) -> f64 {
        concrete_cone_k(cracked) * f_ck_mpa.sqrt() * h_ef_mm.powf(1.5)
    }

    pub fn concrete_cone_resistance_design_n(f_ck_mpa: f64, h_ef_mm: f64, cracked: bool) -> f64 {
        concrete_cone_resistance_n0_rk_c_n(f_ck_mpa, h_ef_mm, cracked) / GAMMA_MC
    }

    /// ⚓️ Simplified single-anchor concrete edge breakout characteristic resistance V⁰_Rk,c = 1.6·√d·√h_ef·√f_ck·c₁^1.5 [N, mm, MPa], a simplified form of EN 1992-4 §7.2.2.5.
    pub fn concrete_edge_resistance_v0_rk_c_n(d_mm: f64, h_ef_mm: f64, f_ck_mpa: f64, c_1_mm: f64) -> f64 {
        1.6 * d_mm.sqrt() * h_ef_mm.sqrt() * f_ck_mpa.sqrt() * c_1_mm.powf(1.5)
    }

    pub fn concrete_edge_resistance_design_n(d_mm: f64, h_ef_mm: f64, f_ck_mpa: f64, c_1_mm: f64) -> f64 {
        concrete_edge_resistance_v0_rk_c_n(d_mm, h_ef_mm, f_ck_mpa, c_1_mm) / GAMMA_MC
    }

    pub fn check_anchor_steel(n_ed_n: f64, a_s_mm2: f64, f_uk_mpa: f64, f_yk_mpa: f64) -> CheckResult {
        let resistance = steel_resistance_design_n(a_s_mm2, f_uk_mpa, f_yk_mpa);
        CheckResult::from_utilization(ClauseId::new("EN 1992-4", "§7.2.1.4", "7.2.1.4"), Quantity::force_kn(n_ed_n / 1000.0), Quantity::force_kn(resistance / 1000.0), "anchor steel failure ULS", AnnexChoice::En)
    }

    pub fn check_anchor_concrete_cone(n_ed_n: f64, f_ck_mpa: f64, h_ef_mm: f64, cracked: bool) -> CheckResult {
        let resistance = concrete_cone_resistance_design_n(f_ck_mpa, h_ef_mm, cracked);
        CheckResult::from_utilization(ClauseId::new("EN 1992-4", "§7.2.1.5", "7.2.1.5"), Quantity::force_kn(n_ed_n / 1000.0), Quantity::force_kn(resistance / 1000.0), "anchor concrete cone failure ULS", AnnexChoice::En)
    }

    pub fn check_anchor_edge_shear(v_ed_n: f64, d_mm: f64, h_ef_mm: f64, f_ck_mpa: f64, c_1_mm: f64) -> CheckResult {
        let resistance = concrete_edge_resistance_design_n(d_mm, h_ef_mm, f_ck_mpa, c_1_mm);
        CheckResult::from_utilization(ClauseId::new("EN 1992-4", "§7.2.2.5", "7.2.2.5 (simplified)"), Quantity::force_kn(v_ed_n / 1000.0), Quantity::force_kn(resistance / 1000.0), "anchor concrete edge shear breakout (simplified)", AnnexChoice::En)
    }
}
// #endregion 🔖️Part4

/// 📋️ RC beam ULS check end-to-end.
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_rc_beam(m_ed_knm: f64, v_ed_kn: f64, f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, rho_l: f64, n_ed_kn: f64, annex: AnnexChoice) -> CheckReport {
    let m_rd = part_1_1::flexural_resistance_knm(f_ck, b_mm, d_mm, a_s_mm2, f_yk, annex);
    let v_rd = part_1_1::shear_resistance_vrdc_kn(b_mm, d_mm, f_ck, rho_l, n_ed_kn);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_flexure(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_shear(v_ed_kn, v_rd, annex));
    report
}

/// 📋️ Full EN 1992 RC beam check with optional prestress transfer.
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_full_rc_beam(m_ed_knm: f64, v_ed_kn: f64, f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, rho_l: f64, n_ed_kn: f64, p_kn: f64, a_c_mm2: f64, annex: AnnexChoice) -> CheckReport {
    let mut report = check_rc_beam(m_ed_knm, v_ed_kn, f_ck, b_mm, d_mm, a_s_mm2, f_yk, rho_l, n_ed_kn, annex);
    if p_kn > 0.0 {
        let sigma_c = part_1_1::prestress_transfer_stress_mpa(p_kn, a_c_mm2);
        report.push(part_1_1::check_prestress_transfer(sigma_c, f_ck, annex));
    }
    report
}

// #region 🔖️Fem
#[cfg(feature = "cross-fem")]
use fem::core::elements2d::BeamEb2;
#[cfg(feature = "cross-fem")]
use fem::core::{Dof, MemberUdl, Model, Node, Support};

#[cfg(feature = "cross-fem")]
fn max_beam_moment_knm(result: &fem::core::StaticResult, element_id: &str) -> f64 {
    let (_, fem::core::ElementResult::Beam { stations }) = result.elements.iter().find(|(id, _)| id == element_id).expect("beam element result") else {
        panic!("expected beam element result");
    };
    stations.iter().map(|s| s.m.abs()).fold(0.0_f64, f64::max) / 1000.0
}

#[cfg(feature = "cross-fem")]
fn max_beam_shear_kn(result: &fem::core::StaticResult, element_id: &str) -> f64 {
    let (_, fem::core::ElementResult::Beam { stations }) = result.elements.iter().find(|(id, _)| id == element_id).expect("beam element result") else {
        panic!("expected beam element result");
    };
    stations.iter().map(|s| s.v.abs()).fold(0.0_f64, f64::max) / 1000.0
}

/// 🏗️ Solve a simply supported RC beam with `fem_core` and run EN 1992 ULS checks.
#[cfg(feature = "cross-fem")]
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_rc_beam_from_fem(span_m: f64, udl_kn_m: f64, f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, rho_l: f64, annex: AnnexChoice) -> Result<CheckReport, fem::core::FemError> {
    let mut model = Model::default();
    model.nodes.push(Node { id: "n0".into(), pos: [0.0, 0.0, 0.0] });
    model.nodes.push(Node { id: "n1".into(), pos: [span_m, 0.0, 0.0] });
    model.supports.push(Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty] });
    model.supports.push(Support { node_id: "n1".into(), fixed: vec![Dof::Ty] });
    model.elements.push(Box::new(BeamEb2 { id: "b1".into(), start: "n0".into(), end: "n1".into(), e: 30e9, area: b_mm * d_mm / 1e6, iy: b_mm * d_mm.powi(3) / 12e12, density: 2500.0 }));
    model.member_loads.push(("b1".into(), MemberUdl { wx: 0.0, wy: -udl_kn_m * 1000.0, wz: 0.0 }));

    let result = fem::core::solve_linear_static(&model)?;
    let m_ed_knm = max_beam_moment_knm(&result, "b1");
    let v_ed_kn = max_beam_shear_kn(&result, "b1");

    Ok(check_rc_beam(m_ed_knm, v_ed_kn, f_ck, b_mm, d_mm, a_s_mm2, f_yk, rho_l, 0.0, annex))
}
// #endregion 🔖️Fem
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn rc_beam_e2e() {
        let report = check_rc_beam(120.0, 80.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, 200.0, AnnexChoice::De);
        assert!(!report.checks.is_empty());
        assert!(report.checks[0].utilization > 0.0);
    }

    #[semio_framework_async_macros::async_test]
    fn punching_v_rd_max_c30() {
        let v = part_1_1::punching_v_rd_max_mpa(30.0, AnnexChoice::De);
        assert!((v - 4.488).abs() < 0.1);
    }

    #[semio_framework_async_macros::async_test]
    fn slenderness_column() {
        let i = part_1_1::radius_of_gyration_mm(300_000.0, 2.25e9);
        let lambda = part_1_1::slenderness_lambda(3000.0, i);
        assert!((lambda - 34.6).abs() < 1.0);
    }

    #[semio_framework_async_macros::async_test]
    fn crack_width_wk() {
        let eps_sm = part_1_1::steel_strain_eps_sm(200.0, 0.01, 2.9, 200_000.0);
        let wk = part_1_1::crack_width_wk_mm(eps_sm, 0.0001, 300.0);
        assert!(wk > 0.0 && wk < 0.5);
    }

    #[semio_framework_async_macros::async_test]
    fn deflection_ss_udl() {
        let delta = part_1_1::deflection_ss_udl_mm(20.0, 6.0, 30_000.0, 1.875e9);
        assert!((delta - 6.0).abs() < 0.5);
    }

    #[semio_framework_async_macros::async_test]
    fn fire_cover_beam_r60() {
        let req = part_1_2::min_axis_distance_mm(part_1_2::ElementType::Beam, FireRating::R60);
        assert!((req - 35.0).abs() < 0.1);
    }

    #[semio_framework_async_macros::async_test]
    fn liquid_retaining_stress_limit() {
        assert!((part_3::steel_stress_limit_mpa("XD1") - 160.0).abs() < 0.1);
        let wk = part_3::crack_width_liquid_mm(220.0, "XD1", 250.0, 200_000.0);
        assert!(wk < 0.25);
    }

    #[semio_framework_async_macros::async_test]
    fn na_de_alpha_cc() {
        assert!((na_de::AnnexParams::de().alpha_cc - 0.85).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    #[cfg(feature = "cross-fem")]
    fn rc_beam_from_fem_e2e() {
        let report = check_rc_beam_from_fem(6.0, 20.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, AnnexChoice::De).expect("fem solve");
        assert!(!report.checks.is_empty());
        let m_ed = report.checks[0].computed.value / 1_000_000.0;
        assert!((m_ed - 90.0).abs() < 1.0);
    }

    #[semio_framework_async_macros::async_test]
    fn prestress_transfer_c30() {
        let sigma = part_1_1::prestress_transfer_stress_mpa(800.0, 135_000.0);
        assert!((sigma - 5.93).abs() < 0.1);
        let limit = part_1_1::prestress_transfer_limit_mpa(30.0);
        assert!((limit - 18.0).abs() < 1e-9);
        let report = check_full_rc_beam(120.0, 80.0, 30.0, 300.0, 450.0, 1200.0, 500.0, 0.01, 0.0, 800.0, 135_000.0, AnnexChoice::De);
        assert_eq!(report.checks.len(), 3);
        assert!(report.checks[2].utilization < 1.0);
    }

    #[semio_framework_async_macros::async_test]
    fn annex_params_alpha_cc_de_vs_en_divergence() {
        let f_ck = 30.0;
        let f_cd_de = na_de::AnnexParams::de().f_cd_mpa(f_ck);
        let f_cd_en = na_de::AnnexParams::en().f_cd_mpa(f_ck);
        assert!((f_cd_de - 17.0).abs() < 1e-9);
        assert!((f_cd_en - 20.0).abs() < 1e-9);
        assert!(f_cd_en > f_cd_de);
    }

    #[semio_framework_async_macros::async_test]
    fn flexural_resistance_annex_divergence() {
        let m_rd_de = part_1_1::flexural_resistance_knm(30.0, 300.0, 450.0, 1200.0, 500.0, AnnexChoice::De);
        let m_rd_en = part_1_1::flexural_resistance_knm(30.0, 300.0, 450.0, 1200.0, 500.0, AnnexChoice::En);
        assert!(m_rd_en > m_rd_de);
    }

    #[semio_framework_async_macros::async_test]
    fn fire_r60_required_axis_distance_at_160mm() {
        let a = part_1_2::required_axis_distance_beam_mm(160.0, FireRating::R60);
        assert!((a - 35.0).abs() < 1e-9);
        let pass = part_1_2::check_fire_beam_axis_distance(160.0, 35.0, FireRating::R60);
        assert!(pass.status != CheckStatus::Fail);
        let fail = part_1_2::check_fire_beam_axis_distance(160.0, 20.0, FireRating::R60);
        assert_eq!(fail.status, CheckStatus::Fail);
    }

    #[semio_framework_async_macros::async_test]
    fn bridge_concrete_stress_and_fatigue() {
        let limit = part_2::concrete_stress_limit_frequent_mpa(30.0);
        assert!((limit - 18.0).abs() < 1e-9);
        let ok = part_2::check_bridge_concrete_stress(12.0, 30.0);
        assert!(ok.status != CheckStatus::Fail);
        let resistance = part_2::fatigue_resistance_design_mpa();
        assert!((resistance - 141.304_347_826_086_96).abs() < 1e-6);
        let fatigue_ok = part_2::check_bridge_fatigue(100.0);
        assert!(fatigue_ok.status != CheckStatus::Fail);
        let fatigue_fail = part_2::check_bridge_fatigue(150.0);
        assert_eq!(fatigue_fail.status, CheckStatus::Fail);
    }

    #[semio_framework_async_macros::async_test]
    fn tightness_class_crack_width_limits() {
        assert!(part_3::tightness_crack_width_limit_mm(TightnessClass::Tc0, 10.0).is_none());
        assert!((part_3::tightness_crack_width_limit_mm(TightnessClass::Tc1, 10.0).unwrap() - 0.3).abs() < 1e-9);
        let tc2_mid = part_3::tightness_crack_width_limit_mm(TightnessClass::Tc2, 20.0).unwrap();
        assert!((tc2_mid - 0.125).abs() < 1e-9);
        let tc0_check = part_3::check_tightness_crack_width(0.2, TightnessClass::Tc0, 10.0);
        assert_eq!(tc0_check.status, CheckStatus::NotApplicable);
    }

    #[semio_framework_async_macros::async_test]
    fn anchor_m12_steel_and_concrete_cone_uncracked() {
        let f_ck = 30.0;
        let h_ef = 80.0;
        let n_rk_c = part_4::concrete_cone_resistance_n0_rk_c_n(f_ck, h_ef, false);
        assert!((n_rk_c - 43_111.0).abs() < 2.0);
        let n_rd_c = part_4::concrete_cone_resistance_design_n(f_ck, h_ef, false);
        assert!((n_rd_c - 28_740.7).abs() < 0.2);

        let f_uk = 800.0;
        let f_yk = 640.0;
        let a_s = 84.3;
        let gamma_ms = part_4::gamma_ms(f_uk, f_yk);
        assert!((gamma_ms - 1.5).abs() < 1e-9);
        let n_rk_s = part_4::steel_resistance_n_rk_s_n(a_s, f_uk);
        assert!((n_rk_s - 67_440.0).abs() < 1e-6);
        let n_rd_s = part_4::steel_resistance_design_n(a_s, f_uk, f_yk);
        assert!((n_rd_s - 44_960.0).abs() < 1e-3);

        let steel_check = part_4::check_anchor_steel(10_000.0, a_s, f_uk, f_yk);
        assert!(steel_check.status != CheckStatus::Fail);
        let cone_check = part_4::check_anchor_concrete_cone(10_000.0, f_ck, h_ef, false);
        assert!(cone_check.status != CheckStatus::Fail);
    }

    #[semio_framework_async_macros::async_test]
    fn anchor_edge_shear_breakout() {
        let v_rk_c = part_4::concrete_edge_resistance_v0_rk_c_n(12.0, 80.0, 30.0, 100.0);
        assert!(v_rk_c > 0.0);
        let check = part_4::check_anchor_edge_shear(5_000.0, 12.0, 80.0, 30.0, 100.0);
        assert!(check.status != CheckStatus::Fail);
    }
}
//#endregion 🧪️ComplianceHelpersTests
