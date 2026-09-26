//! 🧬️ En1992 artifact schema — hierarchical RC structure + clause engines.

use crate::document::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::part_1_2::FireRating;
use crate::part_3::TightnessClass;
use crate::{Anchor, ExposureClass, LoadCaseActions, MemberKind, RcMember, Stirrups, SupportCondition};
use crate::En1992Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub title: String,
    #[state(artifact)]
    pub design_working_life_years: f64,
    #[state(artifact)]
    pub delta_c_dev: f64,
    #[state(artifact)]
    pub cement_type: String,
    #[state(artifact)]
    pub concrete_grades: Vec<crate::ConcreteGrade>,
    #[state(artifact)]
    pub reinforcement_grades: Vec<crate::ReinforcementGrade>,
    #[state(artifact)]
    pub prestress_steels: Vec<crate::PrestressSteel>,
    #[state(artifact)]
    pub members: Vec<RcMember>,
    #[state(artifact)]
    pub anchors: Vec<Anchor>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1992Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> En1992Snapshot {
        En1992Snapshot {
            annex: self.annex,
            title: self.title.clone(),
            design_working_life_years: self.design_working_life_years,
            delta_c_dev: self.delta_c_dev,
            cement_type: self.cement_type.clone(),
            concrete_grades: self.concrete_grades.clone(),
            reinforcement_grades: self.reinforcement_grades.clone(),
            prestress_steels: self.prestress_steels.clone(),
            members: self.members.clone(),
            anchors: self.anchors.clone(),
        }
    }

    /// 🧬️ Builds artifact from snapshot.
    pub fn from_snapshot(snapshot: &En1992Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            title: snapshot.title.clone(),
            design_working_life_years: snapshot.design_working_life_years,
            delta_c_dev: snapshot.delta_c_dev,
            cement_type: snapshot.cement_type.clone(),
            concrete_grades: snapshot.concrete_grades.clone(),
            reinforcement_grades: snapshot.reinforcement_grades.clone(),
            prestress_steels: snapshot.prestress_steels.clone(),
            members: snapshot.members.clone(),
            anchors: snapshot.anchors.clone(),
        }
    }

    /// 🔄 Overwrite persistent fields from a snapshot.
    pub fn set_snapshot(&mut self, snapshot: &En1992Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1992` — twenty handcrafted schema leaves.
pub fn en1992_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1992",
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



//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{En1992Diff, En1992Mutation, En1992Snapshot};
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
    use crate::En1992Snapshot;
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

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1992 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. Every `part_1_N`/`part_N` module (including the
/// `cross-fem`-gated `Fem` region) is a pure function library; the snapshot-level composition
/// (`evaluate`) lives in `💡️inferences`.
use crate::document::{table_lookup_linear, TableEntry1D};

// #region 🔖️NaDe


//#region 🔖️Helpers
fn moment_nm(nm: f64) -> Quantity {
    Quantity::new(QuantityKind::Moment, nm)
}
fn force_n(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}
fn stress_pa(pa: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, pa)
}
fn length_m(m: f64) -> Quantity {
    Quantity::length_m(m)
}
fn area_m2(a: f64) -> Quantity {
    Quantity::area_m2(a)
}
fn member_path(member_id: &str, field: &str) -> String {
    format!("members[id={member_id}].{field}")
}
fn layer_path(member_id: &str, layer_id: &str, field: &str) -> String {
    format!("members[id={member_id}].longitudinal[id={layer_id}].{field}")
}
fn anchor_path(anchor_id: &str, field: &str) -> String {
    format!("anchors[id={anchor_id}].{field}")
}
fn member_ref(doc: &En1992Snapshot, member: &RcMember, path: &str) -> SubjectRef {
    let base = member.subject_label();
    let label = L(
        &format!("{} — {}", doc.title, base.en),
        &format!("{} — {}", doc.title, base.de),
    );
    SubjectRef::new(member.id.clone(), path.to_string(), label)
}
fn L(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}
fn clause_11(section: &str) -> ClauseId {
    ClauseId::new("EN 1992-1-1", section, section.trim_start_matches('§'))
}
fn part_11() -> &'static str {
    "DIN EN 1992-1-1"
}
fn part_12() -> &'static str {
    "DIN EN 1992-1-2"
}
fn part_2() -> &'static str {
    "DIN EN 1992-2"
}
fn part_3() -> &'static str {
    "DIN EN 1992-3"
}
fn part_4() -> &'static str {
    "DIN EN 1992-4"
}
//#endregion 🔖️Helpers

/// 🇩🇪 National annex parameters for EN 1992.
pub mod na_de {
    use super::AnnexChoice;

    #[derive(Clone, Copy, Debug)]
    pub struct AnnexParams {
        pub alpha_cc: f64,
        pub alpha_ct: f64,
        pub gamma_c: f64,
        pub gamma_s: f64,
    }

    impl AnnexParams {
        pub fn en() -> Self {
            Self { alpha_cc: 1.0, alpha_ct: 1.0, gamma_c: 1.5, gamma_s: 1.15 }
        }
        pub fn de() -> Self {
            Self { alpha_cc: 0.85, alpha_ct: 0.85, gamma_c: 1.5, gamma_s: 1.15 }
        }
        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice {
                AnnexChoice::De => Self::de(),
                AnnexChoice::En => Self::en(),
            }
        }
        /// 🚨 Accidental situation — DE NA γ_c=1.3, γ_s=1.0.
        pub fn accidental(choice: AnnexChoice) -> Self {
            match choice {
                AnnexChoice::De => Self { alpha_cc: 0.85, alpha_ct: 0.85, gamma_c: 1.3, gamma_s: 1.0 },
                AnnexChoice::En => Self { alpha_cc: 1.0, alpha_ct: 1.0, gamma_c: 1.2, gamma_s: 1.0 },
            }
        }
        /// 🚨 Accidental / fire situations — reduced γ (DE: γ_c=1.3, γ_s=1.0).
        pub fn for_situation(choice: AnnexChoice, situation: &str) -> Self {
            match situation {
                "accidental" | "fire" => Self::accidental(choice),
                _ => Self::for_choice(choice),
            }
        }
        pub fn f_cd_pa(&self, f_ck_pa: f64) -> f64 {
            self.alpha_cc * f_ck_pa / self.gamma_c
        }
        pub fn f_yd_pa(&self, f_yk_pa: f64) -> f64 {
            f_yk_pa / self.gamma_s
        }
        /// 📐 DE-NA cot θ = clamp(1.2 + 0.2·σ_cp/f_cd, 1.0, 3.0); EN allows free choice in [1,3].
        pub fn cot_theta(&self, choice: AnnexChoice, sigma_cp_pa: f64, f_cd_pa: f64) -> f64 {
            match choice {
                AnnexChoice::De => {
                    let ratio = if f_cd_pa > 0.0 { (sigma_cp_pa / f_cd_pa).max(0.0) } else { 0.0 };
                    (1.2 + 0.2 * ratio).clamp(1.0, 3.0)
                }
                AnnexChoice::En => 1.0f64.max(2.5_f64.min(3.0)),
            }
        }
    }
}

/// 🧱 EN 1992-1-1 structural checks.
pub mod part_1_1 {
    use super::*;

    /// 📈 Mean tensile strength f_ctm [Pa] from f_ck.
    pub fn f_ctm_pa(f_ck_pa: f64) -> f64 {
        let f_ck_mpa = f_ck_pa / 1.0e6;
        if f_ck_mpa <= 50.0 {
            0.30 * f_ck_mpa.powf(2.0 / 3.0) * 1.0e6
        } else {
            2.12 * f64::ln(1.0 + (f_ck_mpa + 8.0) / 10.0) * 1.0e6
        }
    }

    /// 💪 Rectangular-section flexural resistance M_Rd [N·m] — §3.1.7 λ/η from n, x_lim from ε_cu2.
    pub fn flexural_resistance_nm(
        f_ck_pa: f64,
        b: f64,
        d: f64,
        a_s: f64,
        f_yk_pa: f64,
        n_ed: f64,
        annex: AnnexChoice,
        eps_cu2: f64,
        n_parabola: f64,
        e_s: f64,
    ) -> f64 {
        let p = na_de::AnnexParams::for_choice(annex);
        let eta = if n_parabola <= 2.0 { 1.0 } else { (1.0 - (f_ck_pa / 1.0e6 - 50.0) / 200.0).max(0.8) };
        let lambda = if n_parabola <= 2.0 { 0.8 } else { (0.8 - (f_ck_pa / 1.0e6 - 50.0) / 400.0).max(0.6) };
        let f_cd = eta * p.f_cd_pa(f_ck_pa);
        let f_yd = p.f_yd_pa(f_yk_pa);
        let f_s = a_s * f_yd;
        let f_c_req = (f_s - n_ed).max(0.0);
        let x = if f_cd * b * lambda > 0.0 { f_c_req / (f_cd * b * lambda) } else { 0.0 };
        let eps_yd = if e_s > 0.0 { f_yd / e_s } else { 0.0025 };
        let x_bal = if eps_cu2 + eps_yd > 0.0 { d * eps_cu2 / (eps_cu2 + eps_yd) } else { 0.45 * d };
        let x_lim = x_bal.min(0.45 * d);
        let x_use = x.min(x_lim).max(0.0);
        let z = d - 0.5 * lambda * x_use;
        let m_from_steel = f_s * z;
        if n_ed.abs() > 1.0 && b * d > 0.0 {
            let n_rd = f_cd * b * d + f_yd * a_s;
            let util_n = (-n_ed).max(0.0) / n_rd.max(1.0);
            m_from_steel * (1.0 - util_n).max(0.0)
        } else {
            m_from_steel
        }
    }

    /// ✂️ Shear resistance without stirrups V_Rd,c [N] — Eq. 6.2a/b.
    pub fn shear_v_rd_c_n(b: f64, d: f64, f_ck_pa: f64, rho_l: f64, n_ed: f64, annex: AnnexChoice) -> f64 {
        let f_ck_mpa = f_ck_pa / 1.0e6;
        let d_mm = d * 1000.0;
        let b_mm = b * 1000.0;
        let k = (1.0 + (200.0 / d_mm).sqrt()).min(2.0);
        let rho = rho_l.min(0.02);
        let a_c_mm2 = b_mm * d_mm;
        let sigma_cp_mpa = if a_c_mm2 > 0.0 { (n_ed / 1000.0) / a_c_mm2 * 1000.0 } else { 0.0 };
        let sigma_cp_mpa = sigma_cp_mpa.clamp(0.0, 0.2 * f_ck_mpa);
        let p = na_de::AnnexParams::for_choice(annex);
        // EN 1992-1-1: C_Rd,c = 0.18/γ_c; DE-NA: C_Rd,c = 0.15/γ_c
        let c_rd_c = match annex {
            AnnexChoice::De => 0.15 / p.gamma_c,
            AnnexChoice::En => 0.18 / p.gamma_c,
        };
        let v_min = 0.035 * k.powf(1.5) * f_ck_mpa.sqrt();
        let v_rd_c1 = (c_rd_c * k * (100.0 * rho * f_ck_mpa).powf(1.0 / 3.0) + 0.15 * sigma_cp_mpa) * b_mm * d_mm;
        let v_rd_c2 = (v_min + 0.15 * sigma_cp_mpa) * b_mm * d_mm;
        v_rd_c1.max(v_rd_c2)
    }

    /// ➰ Shear with stirrups V_Rd,s [N] — Eq. 6.8.
    pub fn shear_v_rd_s_n(asw_per_s: f64, z: f64, f_ywd_pa: f64, cot_theta: f64) -> f64 {
        asw_per_s * z * f_ywd_pa * cot_theta
    }

    /// 🧱 Crushing limit V_Rd,max [N] — Eq. 6.9.
    pub fn shear_v_rd_max_n(b: f64, z: f64, f_ck_pa: f64, cot_theta: f64, annex: AnnexChoice) -> f64 {
        let p = na_de::AnnexParams::for_choice(annex);
        let f_cd = p.f_cd_pa(f_ck_pa);
        let f_ck_mpa = f_ck_pa / 1.0e6;
        let nu1 = 0.6 * (1.0 - f_ck_mpa / 250.0);
        let alpha_cw = 1.0;
        let sin_cos = cot_theta / (1.0 + cot_theta * cot_theta); // = cot/(1+cot²) = cos·sin / sin² * sin... wait: sinθ cosθ = cotθ/(1+cot²θ)
        alpha_cw * b * z * nu1 * f_cd * sin_cos
    }

    /// 🕳️ Punching v_Rd,c [Pa] — Eq. 6.47; DE-NA v_Rd,max = 1.4·v_Rd,c.
    pub fn punching_v_rd_c_pa(f_ck_pa: f64, d: f64, rho_l: f64, annex: AnnexChoice) -> f64 {
        let f_ck_mpa = f_ck_pa / 1.0e6;
        let d_mm = d * 1000.0;
        let k = (1.0 + (200.0 / d_mm).sqrt()).min(2.0);
        let rho = rho_l.min(0.02);
        let p = na_de::AnnexParams::for_choice(annex);
        let c_rd_c = match annex {
            AnnexChoice::De => 0.15 / p.gamma_c,
            AnnexChoice::En => 0.18 / p.gamma_c,
        };
        let v_min = 0.035 * k.powf(1.5) * f_ck_mpa.sqrt();
        let v = (c_rd_c * k * (100.0 * rho * f_ck_mpa).powf(1.0 / 3.0)).max(v_min);
        v * 1.0e6
    }

    pub fn punching_v_rd_max_pa(v_rd_c_pa: f64, annex: AnnexChoice) -> f64 {
        match annex {
            AnnexChoice::De => 1.4 * v_rd_c_pa,
            AnnexChoice::En => 2.0 * v_rd_c_pa, // conservative stand-in for 0.5 ν f_cd path
        }
    }

    /// 🌀 Torsion T_Rd,max [N·m] for solid rectangle — Eq. 6.29 approx.
    pub fn torsion_t_rd_nm(f_ck_pa: f64, b: f64, h: f64, annex: AnnexChoice) -> f64 {
        let p = na_de::AnnexParams::for_choice(annex);
        let f_cd = p.f_cd_pa(f_ck_pa);
        let t_eff = (b.min(h)) / 6.0;
        let a_k = (b - t_eff) * (h - t_eff);
        let f_ck_mpa = f_ck_pa / 1.0e6;
        let nu = 0.6 * (1.0 - f_ck_mpa / 250.0);
        2.0 * a_k * t_eff * nu * f_cd * 0.5 // with cotθ≈1
    }

    /// 🪟 Crack width w_k [m] — Eq. 7.8 with ε_sm − ε_cm from 7.9.
    pub fn crack_width_m(sigma_s_pa: f64, rho_p_eff: f64, f_ct_eff_pa: f64, e_s_pa: f64, s_r_max: f64) -> f64 {
        if e_s_pa <= 0.0 || rho_p_eff <= 0.0 {
            return 0.0;
        }
        let alpha_e = e_s_pa / (22.0e9); // rough E_cm
        let eps = (sigma_s_pa - 0.4 * f_ct_eff_pa / rho_p_eff * (1.0 + alpha_e * rho_p_eff)).max(0.6 * sigma_s_pa) / e_s_pa;
        s_r_max * eps
    }

    /// 📉 Basic l/d limit Table 7.4N.
    pub fn basic_ld_limit(support: SupportCondition, rho: f64, rho_prime: f64) -> f64 {
        // EN 1992-1-1 §7.4.2 Table 7.4N — K factors; DE-NA adopts same K.
        let k = match support {
            SupportCondition::SimplySupported => 1.0,
            SupportCondition::Continuous | SupportCondition::Fixed => 1.5,
            SupportCondition::Cantilever => 0.4,
        };
        let f_yk_ref: f64 = 500.0;
        let rho0 = 1.0e-3 * f_yk_ref.sqrt();
        let rho = rho.max(1e-6);
        let rho_prime = rho_prime.max(0.0);
        if rho <= rho0 {
            k * (11.0 + 1.5 * f_yk_ref.sqrt() * rho0 / rho + 3.2 * (rho0 / rho - 1.0).max(0.0).powf(1.5))
        } else {
            k * (11.0 + 1.5 * f_yk_ref.sqrt() * rho0 / rho.max(rho0) + rho_prime / rho0 * f_yk_ref.sqrt() / 12.0)
        }
    }


    pub fn c_min_dur_m(exposure: ExposureClass) -> f64 {
        let mm = match exposure {
            ExposureClass::X0 => 10.0,
            ExposureClass::Xc1 => 15.0,
            ExposureClass::Xc2 | ExposureClass::Xc3 => 20.0,
            ExposureClass::Xc4 => 25.0,
            ExposureClass::Xd1 | ExposureClass::Xs1 => 40.0,
            ExposureClass::Xd2 | ExposureClass::Xs2 => 40.0,
            ExposureClass::Xd3 | ExposureClass::Xs3 => 45.0,
            ExposureClass::Xf1 => 20.0,
            ExposureClass::Xf2 | ExposureClass::Xf3 => 25.0,
            ExposureClass::Xf4 => 30.0,
            ExposureClass::Xa1 => 25.0,
            ExposureClass::Xa2 => 30.0,
            ExposureClass::Xa3 => 40.0,
        };
        mm / 1000.0
    }

    /// 🪟 w_max [m] from exposure (quasi-permanent) — Table 7.1N / DE-NA.
    pub fn w_max_m(exposure: ExposureClass) -> f64 {
        match exposure {
            ExposureClass::X0 | ExposureClass::Xc1 => 0.40e-3,
            _ => 0.30e-3,
        }
    }

    /// 📉 Min reinforcement A_s,min [m²] — §9.2.1.1 / 9.3.1.1.
    pub fn a_s_min_m2(f_ck_pa: f64, f_yk_pa: f64, b: f64, d: f64) -> f64 {
        let f_ctm = f_ctm_pa(f_ck_pa);
        (0.26 * f_ctm / f_yk_pa * b * d).max(0.0013 * b * d)
    }

    pub fn a_s_max_m2(b: f64, h: f64) -> f64 {
        0.04 * b * h
    }

    /// 📏 Slenderness λ = l0 / i.
    pub fn slenderness(l0: f64, b: f64, h: f64) -> f64 {
        let i = (b.min(h)) / 12.0f64.sqrt();
        if i <= 0.0 {
            0.0
        } else {
            l0 / i
        }
    }

    /// 🔍 Required A_s [m²] for M_Ed by bisection.
    pub fn required_a_s_m2(
        m_ed: f64,
        f_ck_pa: f64,
        b: f64,
        d: f64,
        f_yk_pa: f64,
        n_ed: f64,
        annex: AnnexChoice,
        eps_cu2: f64,
        n_parabola: f64,
        e_s: f64,
    ) -> f64 {
        let mut lo = 0.0;
        let mut hi = 0.04 * b * d;
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if flexural_resistance_nm(f_ck_pa, b, d, mid, f_yk_pa, n_ed, annex, eps_cu2, n_parabola, e_s) >= m_ed {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        hi
    }

    /// 🏗️ Structural class delta vs S4 (Table 4.3N / DE) from design life, cement, strength, kind.
    pub fn structural_class_delta(design_life_years: f64, cement_type: &str, f_ck_pa: f64, member_kind: crate::MemberKind) -> i32 {
        let mut sc = 4;
        if design_life_years >= 100.0 { sc += 2; }
        else if design_life_years <= 25.0 { sc -= 1; }
        let ct = cement_type.to_uppercase();
        if ct.contains('R') { sc -= 1; }
        if f_ck_pa / 1.0e6 >= 35.0 && matches!(member_kind, crate::MemberKind::Slab | crate::MemberKind::FlatSlab | crate::MemberKind::Beam) {
            sc -= 1;
        }
        (sc - 4).clamp(-1, 2)
    }

    /// 📏 c_min,dur with structural-class shift (±5 mm per class).
    pub fn c_min_dur_adjusted_m(exposure: ExposureClass, design_life_years: f64, cement_type: &str, f_ck_pa: f64, member_kind: crate::MemberKind) -> f64 {
        let base = c_min_dur_m(exposure);
        let delta_sc = structural_class_delta(design_life_years, cement_type, f_ck_pa, member_kind);
        (base + delta_sc as f64 * 0.005).max(0.010)
    }

    /// 🔗 c_min,b §4.4.1.2 — φ or φ_n=φ√n_b; +5 mm if aggregate > 32 mm.
    pub fn c_min_b_m(bar_diameter: f64, aggregate_size: f64, bundled_count: u32) -> f64 {
        let phi_eq = if bundled_count >= 2 { (bundled_count as f64).sqrt() * bar_diameter } else { bar_diameter };
        phi_eq + if aggregate_size > 0.032 { 0.005 } else { 0.0 }
    }

    /// 🧱 c_min = max(c_min,b, c_min,dur, 10 mm); c_nom = c_min + Δc_dev.
    pub fn c_nom_m(c_min_b: f64, c_min_dur: f64, delta_c_dev: f64) -> (f64, f64) {
        let c_min = c_min_b.max(c_min_dur).max(0.010);
        (c_min, c_min + delta_c_dev)
    }

    pub fn f_bd_pa(f_ctk_005: f64, eta1: f64, eta2: f64) -> f64 { 2.25 * eta1 * eta2 * f_ctk_005 }
    pub fn l_b_rqd_m(phi: f64, sigma_sd: f64, f_bd: f64) -> f64 {
        if f_bd <= 0.0 { return 50.0 * phi; }
        (phi / 4.0) * (sigma_sd / f_bd)
    }
    pub fn l_bd_m(l_b_rqd: f64, phi: f64, alpha: f64) -> f64 {
        let l = alpha * l_b_rqd;
        l.max((0.3 * l_b_rqd).max(10.0 * phi).max(0.100))
    }
    pub fn l_0_m(l_bd: f64, phi: f64, alpha6: f64) -> f64 {
        let l = alpha6 * l_bd;
        l.max((0.3 * alpha6 * l_bd).max(15.0 * phi).max(0.200))
    }
    pub fn de_ld_caps(k: f64, span: f64, deflection_sensitive: bool) -> f64 {
        let cap1 = k * 35.0;
        if deflection_sensitive && span > 0.0 { cap1.min((k * k) * 150.0 / span) } else { cap1 }
    }
    pub fn support_k(support: SupportCondition) -> f64 {
        match support {
            SupportCondition::SimplySupported => 1.0,
            SupportCondition::Continuous | SupportCondition::Fixed => 1.5,
            SupportCondition::Cantilever => 0.4,
        }
    }
    pub fn lambda_lim_de(n: f64) -> f64 {
        if n >= 0.41 { 25.0 } else { 16.0 / n.max(1e-6).sqrt() }
    }
    pub fn second_order_moment_nm(n_ed: f64, l_0: f64, d: f64, f_yd: f64, e_s: f64) -> f64 {
        let curv = (0.45 * f_yd / e_s.max(1.0)) / d.max(1e-6);
        let r = 1.0 / curv.max(1e-9);
        let e2 = l_0 * l_0 / (r * 10.0);
        n_ed.abs() * e2
    }
    pub fn punching_beta(position: &str) -> f64 {
        match position { "edge" => 1.40, "corner" => 1.50, _ => 1.10 }
    }
    pub fn punching_perimeters(c1: f64, c2: f64, d: f64) -> (f64, f64) {
        let u0 = 2.0 * (c1 + c2);
        (u0, u0 + 2.0 * std::f64::consts::PI * 2.0 * d)
    }

    /// 📈 Cracked-section steel stress under M [Pa] — elastic cracked rectangle.
    pub fn cracked_sigma_s_pa(m: f64, b: f64, d: f64, a_s: f64, e_s: f64, e_cm: f64) -> f64 {
        if a_s <= 0.0 || d <= 0.0 { return 0.0; }
        let alpha_e = e_s / e_cm.max(1.0);
        let rho = a_s / (b * d);
        let x = d * ((alpha_e * rho).powi(2) + 2.0 * alpha_e * rho).sqrt() - alpha_e * rho * d;
        let z = d - x / 3.0;
        if z <= 0.0 { return 0.0; }
        m.abs() / (a_s * z)
    }
    pub fn cracked_sigma_c_pa(m: f64, b: f64, d: f64, a_s: f64, e_s: f64, e_cm: f64) -> f64 {
        if a_s <= 0.0 || d <= 0.0 || b <= 0.0 { return 0.0; }
        let alpha_e = e_s / e_cm.max(1.0);
        let rho = a_s / (b * d);
        let x = d * ((alpha_e * rho).powi(2) + 2.0 * alpha_e * rho).sqrt() - alpha_e * rho * d;
        let i_cr = b * x.powi(3) / 3.0 + alpha_e * a_s * (d - x).powi(2);
        if i_cr <= 0.0 { return 0.0; }
        m.abs() * x / i_cr
    }

}

/// 🔥 EN 1992-1-2 tabulated fire resistance (Tables 5.2a–5.11) + DE NA restrictions.
pub mod part_1_2_fire {
    use super::*;

    fn r_idx(rating: FireRating) -> usize {
        match rating { FireRating::R30 => 0, FireRating::R60 => 1, FireRating::R90 => 2, FireRating::R120 => 3 }
    }

    /// 📋 Beam simply supported — Table 5.5: (b_min_mm, a_mm) for R30..R120 (web ≥b_min).
    pub fn beam_ss(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(80.0, 25.0), (120.0, 40.0), (150.0, 55.0), (200.0, 65.0)];
        let (b, a) = T[r_idx(rating)];
        (b / 1000.0, a / 1000.0)
    }
    /// 📋 Beam continuous — Table 5.6.
    pub fn beam_cont(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(80.0, 15.0), (120.0, 25.0), (150.0, 35.0), (200.0, 45.0)];
        let (b, a) = T[r_idx(rating)];
        (b / 1000.0, a / 1000.0)
    }
    /// 📋 Column method A — Table 5.2a: (b_min, a) [m].
    pub fn column_method_a(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(200.0, 25.0), (250.0, 35.0), (350.0, 45.0), (350.0, 45.0)];
        let (b, a) = T[r_idx(rating)];
        (b / 1000.0, a / 1000.0)
    }
    /// 📋 Column method B — Table 5.2b.
    pub fn column_method_b(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(150.0, 25.0), (200.0, 35.0), (300.0, 45.0), (400.0, 45.0)];
        let (b, a) = T[r_idx(rating)];
        (b / 1000.0, a / 1000.0)
    }
    /// 📋 Wall — Table 5.4: (h_min, a).
    pub fn wall(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(100.0, 10.0), (120.0, 10.0), (140.0, 20.0), (160.0, 30.0)];
        let (h, a) = T[r_idx(rating)];
        (h / 1000.0, a / 1000.0)
    }
    /// 📋 Tension member — Table 5.3.
    pub fn tension_member(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(80.0, 25.0), (120.0, 40.0), (150.0, 55.0), (200.0, 65.0)];
        let (b, a) = T[r_idx(rating)];
        (b / 1000.0, a / 1000.0)
    }
    /// 📋 One-way solid slab — Table 5.8: (h_min, a).
    pub fn slab_one_way(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(60.0, 10.0), (80.0, 20.0), (100.0, 30.0), (120.0, 40.0)];
        let (h, a) = T[r_idx(rating)];
        (h / 1000.0, a / 1000.0)
    }
    /// 📋 Two-way solid slab — Table 5.9.
    pub fn slab_two_way(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(60.0, 10.0), (80.0, 15.0), (100.0, 20.0), (120.0, 30.0)];
        let (h, a) = T[r_idx(rating)];
        (h / 1000.0, a / 1000.0)
    }
    /// 📋 Flat slab — Table 5.9 / 5.10.
    pub fn flat_slab(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(150.0, 10.0), (180.0, 15.0), (200.0, 25.0), (200.0, 35.0)];
        let (h, a) = T[r_idx(rating)];
        (h / 1000.0, a / 1000.0)
    }
    /// 📋 Ribbed slab — Table 5.11: (b_min rib, a).
    pub fn ribbed_slab(rating: FireRating) -> (f64, f64) {
        const T: &[(f64, f64)] = &[(80.0, 25.0), (100.0, 35.0), (120.0, 45.0), (160.0, 60.0)];
        let (b, a) = T[r_idx(rating)];
        (b / 1000.0, a / 1000.0)
    }

    /// 🎯 Required (min_dimension, a) for member kind + support.
    /// 🎯 Required (min_dimension, a) for member kind + fire options (Tables 5.2a–5.11).
    pub fn required_for(kind: crate::MemberKind, support: SupportCondition, rating: FireRating, column_method: &str, slab_system: &str) -> (f64, f64) {
        match kind {
            crate::MemberKind::Column => {
                if column_method.eq_ignore_ascii_case("B") { column_method_b(rating) } else { column_method_a(rating) }
            }
            crate::MemberKind::Wall => wall(rating),
            crate::MemberKind::TensionMember => tension_member(rating),
            crate::MemberKind::FlatSlab => {
                if slab_system.eq_ignore_ascii_case("two-way") { slab_two_way(rating) }
                else if slab_system.eq_ignore_ascii_case("ribbed") { ribbed_slab(rating) }
                else if slab_system.eq_ignore_ascii_case("one-way") { slab_one_way(rating) }
                else { flat_slab(rating) }
            }
            crate::MemberKind::RibbedSlab => ribbed_slab(rating),
            crate::MemberKind::Slab => {
                if slab_system.eq_ignore_ascii_case("two-way") { slab_two_way(rating) }
                else if slab_system.eq_ignore_ascii_case("ribbed") { ribbed_slab(rating) }
                else if slab_system.eq_ignore_ascii_case("flat") { flat_slab(rating) }
                else { slab_one_way(rating) }
            }
            crate::MemberKind::Beam => match support {
                SupportCondition::Continuous | SupportCondition::Fixed => beam_cont(rating),
                _ => beam_ss(rating),
            },
            crate::MemberKind::Bridge => beam_ss(rating),
            crate::MemberKind::LiquidRetaining => wall(rating),
        }
    }


    pub fn required_axis_distance_beam_m(width: f64, rating: FireRating) -> f64 {
        let b_mm = width * 1000.0;
        let table: &[(f64, f64)] = match rating {
            FireRating::R30 => &[(80.0, 25.0), (120.0, 15.0), (160.0, 15.0), (200.0, 10.0)],
            FireRating::R60 => &[(120.0, 40.0), (160.0, 35.0), (200.0, 30.0), (300.0, 25.0)],
            FireRating::R90 => &[(150.0, 55.0), (200.0, 45.0), (300.0, 40.0), (400.0, 35.0)],
            FireRating::R120 => &[(200.0, 65.0), (240.0, 60.0), (300.0, 55.0), (500.0, 45.0)],
        };
        let mut a = table[0].1;
        for &(bw, aa) in table {
            if b_mm >= bw {
                a = aa;
            }
        }
        a / 1000.0
    }
    pub fn b_min_beam_m(rating: FireRating) -> f64 { beam_ss(rating).0 }
    pub fn required_axis_distance_column_m(rating: FireRating) -> f64 { column_method_a(rating).1 }
}

/// 🌉 EN 1992-2/// 🌉 EN 1992-2 bridges.
pub mod part_2 {
    use super::*;

    pub fn concrete_stress_limit_pa(f_ck_pa: f64) -> f64 {
        0.6 * f_ck_pa
    }

    pub fn fatigue_limit_pa() -> f64 {
        // Δσ_Rsk = 162.5 MPa, γ_S,fat = 1.15
        162.5e6 / 1.15
    }
}

/// 💧 EN 1992-3 liquid retaining.
pub mod part_3_liquid {
    use super::*;

    pub fn tightness_crack_limit_m(class: TightnessClass, hd_over_h: f64) -> Option<f64> {
        match class {
            TightnessClass::Tc0 => None,
            TightnessClass::Tc1 => Some(0.3e-3),
            TightnessClass::Tc2 => {
                let t = ((hd_over_h - 5.0) / 30.0).clamp(0.0, 1.0);
                Some(0.2e-3 + (0.05e-3 - 0.2e-3) * t)
            }
        }
    }
}

/// ⚓️ EN 1992-4 fastenings.
pub mod part_4 {
    use super::*;

    const GAMMA_MC: f64 = 1.5;

    pub fn gamma_ms(f_uk_pa: f64, f_yk_pa: f64) -> f64 {
        if f_uk_pa / f_yk_pa.max(1.0) <= 1.25 { 1.4 } else { 1.5 }
    }

    pub fn steel_resistance_n(a_s: f64, f_uk_pa: f64, f_yk_pa: f64) -> f64 {
        let from_uk = a_s * f_uk_pa / gamma_ms(f_uk_pa, f_yk_pa);
        let from_yk = a_s * f_yk_pa / 1.2;
        from_uk.min(from_yk)
    }

    pub fn cone_resistance_n(f_ck_pa: f64, h_ef: f64, cracked: bool) -> f64 {
        let k = if cracked { 7.5 } else { 11.0 };
        let f_ck_mpa = f_ck_pa / 1.0e6;
        let h_ef_mm = h_ef * 1000.0;
        k * f_ck_mpa.sqrt() * h_ef_mm.powf(1.5) / GAMMA_MC
    }

    /// 🧲 Pull-out resistance (simplified EN 1992-4 §7.2.1.6) N_Rd,p ∝ A_s · f_uk / γ_Mp.
    pub fn pullout_resistance_n(a_s: f64, f_uk_pa: f64) -> f64 {
        a_s * f_uk_pa / 1.5
    }

    /// 🧱 Splitting resistance (simplified) — scales with h_ef and f_ck.
    pub fn splitting_resistance_n(f_ck_pa: f64, h_ef: f64, c1: f64) -> f64 {
        let f_ck_mpa = f_ck_pa / 1.0e6;
        let h_ef_mm = h_ef * 1000.0;
        let c1_mm = c1 * 1000.0;
        8.0 * f_ck_mpa.sqrt() * h_ef_mm.powf(1.5) * (c1_mm / (1.5 * h_ef_mm).max(1.0)).min(1.0) / GAMMA_MC
    }

    /// 🪝 Pry-out resistance ≈ k · V_Rd,c with k≈2 for single anchor.
    pub fn pryout_resistance_n(f_ck_pa: f64, h_ef: f64, cracked: bool) -> f64 {
        2.0 * cone_resistance_n(f_ck_pa, h_ef, cracked)
    }

    pub fn edge_resistance_n(d: f64, h_ef: f64, f_ck_pa: f64, c1: f64) -> f64 {
        let d_mm = d * 1000.0;
        let h_ef_mm = h_ef * 1000.0;
        let c1_mm = c1 * 1000.0;
        let f_ck_mpa = f_ck_pa / 1.0e6;
        let k1 = 1.7;
        k1 * d_mm.powf(0.5) * h_ef_mm.powf(0.2) * f_ck_mpa.sqrt() * c1_mm.powf(1.5) / GAMMA_MC
    }

    /// 🔗 Tension–shear interaction (EN 1992-4 §7.2.3) (N/N_Rd)^1.5 + (V/V_Rd)^1.5 ≤ 1.
    pub fn interaction_utilization(n_ed: f64, n_rd: f64, v_ed: f64, v_rd: f64) -> f64 {
        let tn = if n_rd > 0.0 { (n_ed / n_rd).max(0.0) } else { 0.0 };
        let tv = if v_rd > 0.0 { (v_ed / v_rd).max(0.0) } else { 0.0 };
        tn.powf(1.5) + tv.powf(1.5)
    }
}

//#region 🔖️MemberEvaluate

/// 📦 Design effects after EN 1990 combination (SI).
#[derive(Clone, Debug)]
pub struct DesignEffects {
    pub m_ed: f64,
    pub n_ed: f64,
    pub v_ed: f64,
    pub t_ed: f64,
    pub v_ed_punch: f64,
    pub combination_id: String,
    pub situation: String,
}

fn psi_factors(category: &str) -> (f64, f64, f64) {
    match category {
        "office" | "residential" | "congregation" => (0.7, 0.5, 0.3),
        "shopping" | "storage" => (0.7, 0.7, 0.6),
        "snow" | "snow_high" => (0.5, 0.2, 0.0),
        "wind" => (0.6, 0.2, 0.0),
        "temperature" => (0.6, 0.5, 0.0),
        "accidental" => (1.0, 1.0, 1.0),
        _ => (0.7, 0.5, 0.3),
    }
}

fn characteristic_effects(member: &RcMember, action: &LoadCaseActions) -> (f64, f64, f64, f64, f64) {
    if action.source == "external" || member.use_fem {
        return (action.m_k, action.n_k, action.v_k, action.t_k, action.v_k_punch);
    }
    let l = member.span.max(1e-6);
    let g_line = action.g_k_line + if action.kind == "permanent" { member.udl } else { 0.0 };
    let q_line = action.q_k_line;
    let q = match action.kind.as_str() {
        "permanent" | "prestress" => g_line,
        _ => q_line,
    };
    let (mut m, mut v) = match member.support {
        SupportCondition::SimplySupported => (q * l * l / 8.0, q * l / 2.0),
        SupportCondition::Cantilever => (q * l * l / 2.0, q * l),
        SupportCondition::Continuous | SupportCondition::Fixed => (q * l * l / 12.0, q * l / 2.0),
    };
    if action.point_force.abs() > 0.0 {
        let p = action.point_force.abs();
        m += p * l / 4.0;
        v += p / 2.0;
    }
    (m, 0.0, v, action.t_k, action.v_k_punch)
}

/// ⚖️ EN 1990 (+ DE NA) combinations — ULS 6.10a/b, accidental 6.11, SLS char/freq/qp.
pub fn combine_member_actions(member: &RcMember) -> Vec<DesignEffects> {
    let mut g = (0.0, 0.0, 0.0, 0.0, 0.0);
    let mut variables2: Vec<(&LoadCaseActions, (f64, f64, f64, f64, f64))> = Vec::new();
    let mut accidentals2 = Vec::new();
    let mut has_permanent = false;
    for a in &member.actions {
        let eff = characteristic_effects(member, a);
        match a.kind.as_str() {
            "permanent" | "prestress" => {
                has_permanent = true;
                g.0 += eff.0; g.1 += eff.1; g.2 += eff.2; g.3 += eff.3; g.4 += eff.4;
            }
            "accidental" => accidentals2.push((a, eff)),
            _ => variables2.push((a, eff)),
        }
    }
    if !has_permanent && member.udl > 0.0 && !member.use_fem {
        let l = member.span.max(1e-6);
        let q = member.udl;
        let (m, v) = match member.support {
            SupportCondition::SimplySupported => (q * l * l / 8.0, q * l / 2.0),
            SupportCondition::Cantilever => (q * l * l / 2.0, q * l),
            _ => (q * l * l / 12.0, q * l / 2.0),
        };
        g = (m, 0.0, v, 0.0, 0.0);
    }
    let gamma_g = 1.35;
    let gamma_q = 1.5;
    let mut out = Vec::new();
    if variables2.is_empty() {
        out.push(DesignEffects {
            m_ed: gamma_g * g.0, n_ed: gamma_g * g.1, v_ed: gamma_g * g.2, t_ed: gamma_g * g.3, v_ed_punch: gamma_g * g.4,
            combination_id: "ULS-6.10-G".into(), situation: "uls".into(),
        });
        out.push(DesignEffects {
            m_ed: g.0, n_ed: g.1, v_ed: g.2, t_ed: g.3, v_ed_punch: g.4,
            combination_id: "SLS-qp".into(), situation: "sls_qp".into(),
        });
        out.push(DesignEffects {
            m_ed: g.0, n_ed: g.1, v_ed: g.2, t_ed: g.3, v_ed_punch: g.4,
            combination_id: "SLS-char".into(), situation: "sls_char".into(),
        });
    } else {
        for (lead_i, (lead_a, lead_e)) in variables2.iter().enumerate() {
            let mut m = gamma_g * g.0 + gamma_q * lead_e.0;
            let mut n = gamma_g * g.1 + gamma_q * lead_e.1;
            let mut v = gamma_g * g.2 + gamma_q * lead_e.2;
            let mut t = gamma_g * g.3 + gamma_q * lead_e.3;
            let mut vp = gamma_g * g.4 + gamma_q * lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(&oa.category);
                m += gamma_q * psi0 * oe.0; n += gamma_q * psi0 * oe.1; v += gamma_q * psi0 * oe.2;
                t += gamma_q * psi0 * oe.3; vp += gamma_q * psi0 * oe.4;
            }
            let xi = 0.85;
            let mut m_b = xi * gamma_g * g.0 + gamma_q * lead_e.0;
            let mut n_b = xi * gamma_g * g.1 + gamma_q * lead_e.1;
            let mut v_b = xi * gamma_g * g.2 + gamma_q * lead_e.2;
            let mut t_b = xi * gamma_g * g.3 + gamma_q * lead_e.3;
            let mut vp_b = xi * gamma_g * g.4 + gamma_q * lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(&oa.category);
                m_b += gamma_q * psi0 * oe.0; n_b += gamma_q * psi0 * oe.1; v_b += gamma_q * psi0 * oe.2;
                t_b += gamma_q * psi0 * oe.3; vp_b += gamma_q * psi0 * oe.4;
            }
            let (mu, nu, vu, tu, vpu, tag) = if m.abs() >= m_b.abs() {
                (m, n, v, t, vp, "6.10a")
            } else {
                (m_b, n_b, v_b, t_b, vp_b, "6.10b")
            };
            out.push(DesignEffects { m_ed: mu, n_ed: nu, v_ed: vu, t_ed: tu, v_ed_punch: vpu,
                combination_id: format!("ULS-{}-{}", tag, lead_a.id), situation: "uls".into() });
            let mut ms = g.0 + lead_e.0;
            let mut ns = g.1 + lead_e.1;
            let mut vs = g.2 + lead_e.2;
            let mut ts = g.3 + lead_e.3;
            let mut vps = g.4 + lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(&oa.category);
                ms += psi0 * oe.0; ns += psi0 * oe.1; vs += psi0 * oe.2; ts += psi0 * oe.3; vps += psi0 * oe.4;
            }
            out.push(DesignEffects { m_ed: ms, n_ed: ns, v_ed: vs, t_ed: ts, v_ed_punch: vps,
                combination_id: format!("SLS-char-{}", lead_a.id), situation: "sls_char".into() });
            let (_, psi1_l, _) = psi_factors(&lead_a.category);
            let mut mf = g.0 + psi1_l * lead_e.0;
            let mut nf = g.1 + psi1_l * lead_e.1;
            let mut vf = g.2 + psi1_l * lead_e.2;
            let mut tf = g.3 + psi1_l * lead_e.3;
            let mut vpf = g.4 + psi1_l * lead_e.4;
            for (j, (oa, oe)) in variables2.iter().enumerate() {
                if j == lead_i { continue; }
                let (_, _, psi2) = psi_factors(&oa.category);
                mf += psi2 * oe.0; nf += psi2 * oe.1; vf += psi2 * oe.2; tf += psi2 * oe.3; vpf += psi2 * oe.4;
            }
            out.push(DesignEffects { m_ed: mf, n_ed: nf, v_ed: vf, t_ed: tf, v_ed_punch: vpf,
                combination_id: format!("SLS-freq-{}", lead_a.id), situation: "sls_freq".into() });
        }
        let mut mq = g.0; let mut nq = g.1; let mut vq = g.2; let mut tq = g.3; let mut vpq = g.4;
        for (oa, oe) in &variables2 {
            let (_, _, psi2) = psi_factors(&oa.category);
            mq += psi2 * oe.0; nq += psi2 * oe.1; vq += psi2 * oe.2; tq += psi2 * oe.3; vpq += psi2 * oe.4;
        }
        out.push(DesignEffects { m_ed: mq, n_ed: nq, v_ed: vq, t_ed: tq, v_ed_punch: vpq,
            combination_id: "SLS-qp".into(), situation: "sls_qp".into() });
    }
    for (aa, ae) in &accidentals2 {
        let mut m = g.0 + ae.0; let mut n = g.1 + ae.1; let mut v = g.2 + ae.2;
        let mut tt = g.3 + ae.3; let mut vp = g.4 + ae.4;
        for (oa, oe) in &variables2 {
            let (_, psi1, _) = psi_factors(&oa.category);
            m += psi1 * oe.0; n += psi1 * oe.1; v += psi1 * oe.2; tt += psi1 * oe.3; vp += psi1 * oe.4;
        }
        out.push(DesignEffects { m_ed: m, n_ed: n, v_ed: v, t_ed: tt, v_ed_punch: vp,
            combination_id: format!("ACC-6.11-{}", aa.id), situation: "accidental".into() });
    }
    out
}

fn governing<'a>(effects: &'a [DesignEffects], sit: &str) -> Option<&'a DesignEffects> {
    effects.iter().filter(|e| e.situation == sit).max_by(|a, b| {
        a.m_ed.abs().partial_cmp(&b.m_ed.abs()).unwrap_or(std::cmp::Ordering::Equal)
    })
}

/// 📋️ Evaluate one member against EN 1992-1-1/1-2 (+ optional bridge/liquid).
pub fn evaluate_member(doc: &En1992Snapshot, member: &RcMember) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let annex = doc.annex;
    let Some(concrete) = doc.concrete(&member.concrete_grade_id) else {
        let options: Vec<String> = doc.concrete_grades.iter().map(|g| g.id.clone()).collect();
        let mut missing = CheckResult::assess(format!("en1992.materials.concrete.{}", member.id), part_11(), clause_11("§3.1"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), L("Concrete grade", "Betonfestigkeitsklasse"))
            .annex(annex)
            .explanation(L(
                &format!("Concrete grade id '{}' is missing.", member.concrete_grade_id),
                &format!("Betonfestigkeitsklasse-Id '{}' fehlt.", member.concrete_grade_id),
            ))
            .status(CheckStatus::Fail);
        missing = missing.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "concreteGradeId")),
            if options.is_empty() { vec!["c30".into()] } else { options },
            L("Set concreteGradeId to a declared concrete grade.", "concreteGradeId auf eine deklarierte Betonfestigkeitsklasse setzen."),
        ));
        out.push(missing.build());
        return out;
    };
    let Some(reinf) = doc.reinforcement(&member.reinforcement_grade_id) else {
        let options: Vec<String> = doc.reinforcement_grades.iter().map(|g| g.id.clone()).collect();
        let mut missing = CheckResult::assess(format!("en1992.materials.steel.{}", member.id), part_11(), clause_11("§3.2"), member_ref(doc, member, &member_path(&member.id, "reinforcementGradeId")), L("Reinforcement grade", "Betonstahl"))
            .annex(annex)
            .explanation(L(
                &format!("Reinforcement grade id '{}' is missing.", member.reinforcement_grade_id),
                &format!("Betonstahl-Id '{}' fehlt.", member.reinforcement_grade_id),
            ))
            .status(CheckStatus::Fail);
        missing = missing.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "reinforcementGradeId")),
            if options.is_empty() { vec!["b500".into()] } else { options },
            L("Set reinforcementGradeId to a declared steel grade.", "reinforcementGradeId auf einen deklarierten Betonstahl setzen."),
        ));
        out.push(missing.build());
        return out;
    };
    let f_ck = concrete.f_ck;
    let f_yk = reinf.f_yk;
    let e_s = reinf.e_s;
    let eps_cu2 = concrete.eps_cu2();
    let n_parabola = concrete.n_parabola();
    let f_ck_cube = concrete.f_ck_cube();
    let b = member.width;
    let d = member.effective_depth;
    let h = member.height;
    let a_s = member.a_s_tension();
    let rho = member.rho_l();
    let params = na_de::AnnexParams::for_choice(annex);

    // --- Durability cover: c_min,b + c_min,dur (life/cement/class) + Δc_dev ---
    let phi0 = member.longitudinal.first().map(|l| l.diameter).unwrap_or(0.012);
    let agg0 = member.longitudinal.first().map(|l| l.aggregate_size).unwrap_or(0.016);
    let bundle0 = 1u32;
    let c_min_b = part_1_1::c_min_b_m(phi0, agg0, bundle0);
    let c_min_dur = part_1_1::c_min_dur_adjusted_m(member.exposure, doc.design_working_life_years, &doc.cement_type, f_ck, member.kind);
    let (c_min, c_nom) = part_1_1::c_nom_m(c_min_b, c_min_dur, doc.delta_c_dev);
    let cover_path = member_path(&member.id, "cover");
    let mut cover_check = CheckResult::assess(format!("en1992.4.4.cover.{}", member.id), part_11(), clause_11("§4.4"), member_ref(doc, member, &cover_path), L("Concrete cover for durability", "Betondeckung für Dauerhaftigkeit"))
        .minimum(length_m(member.cover), length_m(c_nom))
        .annex(annex)
        .explanation(L(
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), life={:.0} a, cement={}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), Nutzungsdauer={:.0} a, Zement={}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
        ));
    if member.cover < c_nom {
        cover_check = cover_check.remedy(Remedy::at_least(
            member_ref(doc, member, &cover_path),
            length_m(member.cover),
            length_m(c_nom),
            L(
                &format!("Increase cover of {} from {:.0} mm to at least {:.0} mm (c_nom).", member.id, member.cover * 1000.0, c_nom * 1000.0),
                &format!("Betondeckung von {} von {:.0} mm auf mindestens {:.0} mm (c_nom) erhöhen.", member.id, member.cover * 1000.0, c_nom * 1000.0),
            ),
        ));
    }
    out.push(cover_check.build());

    // §3.1 Table 3.1 — cylinder/cube + constitutive ε_cu2, n enter the utilization
    {
        let computed = f_ck * concrete.eps_c2() * n_parabola;
        let limit = f_ck_cube * eps_cu2 * 2.0;
        let mut mat = CheckResult::assess(format!("en1992.3.1.constitutive.{}", member.id), part_11(), clause_11("§3.1.7"), member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id)), L("Concrete Table 3.1 constitutive", "Beton Tabelle 3.1 Stoffgesetz"))
            .utilization(stress_pa(computed), stress_pa(limit.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("Table 3.1: f_ck={:.0} MPa / f_ck,cube={:.0} MPa; ε_c2={:.2}‰ / ε_cu2={:.2}‰; n={:.2}", f_ck / 1e6, f_ck_cube / 1e6, concrete.eps_c2() * 1e3, eps_cu2 * 1e3, n_parabola),
                &format!("Tabelle 3.1: f_ck={:.0} MPa / f_ck,cube={:.0} MPa; ε_c2={:.2}‰ / ε_cu2={:.2}‰; n={:.2}", f_ck / 1e6, f_ck_cube / 1e6, concrete.eps_c2() * 1e3, eps_cu2 * 1e3, n_parabola),
            ));
        if computed > limit {
            mat = mat.remedy(Remedy::at_most(
                member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id)),
                stress_pa(f_ck),
                stress_pa(f_ck * limit / computed.max(1.0)),
                L("Reduce f_ck toward a Table 3.1 class so constitutive ratios stay consistent.", "f_ck auf eine Klasse nach Tabelle 3.1 absenken, damit die Stoffgesetz-Verhältnisse konsistent bleiben."),
            ));
        }
        out.push(mat.build());
    }

    let effects = combine_member_actions(member);
    let uls = governing(&effects, "uls");
    let acc = governing(&effects, "accidental");
    let sls_qp = governing(&effects, "sls_qp");
    let sls_char = governing(&effects, "sls_char");

    if let Some(uls) = uls {
        let params = na_de::AnnexParams::for_situation(annex, &uls.situation);
        // Flexure + axial
        let m_rd = part_1_1::flexural_resistance_nm(f_ck, b, d, a_s, f_yk, uls.n_ed, annex, eps_cu2, n_parabola, e_s);
        let tension_layer = member.longitudinal.iter().find(|l| l.position == "bottom" || l.position == "tension").or_else(|| member.longitudinal.first());
        let mut flex = CheckResult::assess(format!("en1992.6.1.flexure.{}", member.id), part_11(), clause_11("§6.1"), member_ref(doc, member, &member_path(&member.id, "actions")), L("Bending with axial force", "Biegung mit Normalkraft"))
            .utilization(moment_nm(uls.m_ed.abs()), moment_nm(m_rd.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("|M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (α_cc={:.2}, governing {})", uls.m_ed.abs() / 1e3, m_rd / 1e3, params.alpha_cc, uls.combination_id),
                &format!("|M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (α_cc={:.2}, maßgebend {})", uls.m_ed.abs() / 1e3, m_rd / 1e3, params.alpha_cc, uls.combination_id),
            ));
        if uls.m_ed.abs() > m_rd {
            let a_req = part_1_1::required_a_s_m2(uls.m_ed.abs(), f_ck, b, d, f_yk, uls.n_ed, annex, eps_cu2, n_parabola, e_s);
            if let Some(layer) = tension_layer {
                let bar_a = std::f64::consts::PI * (layer.diameter * 0.5).powi(2);
                let count_req = if bar_a > 0.0 { (a_req / bar_a).ceil() } else { layer.count as f64 + 1.0 };
                flex = flex.remedy(Remedy::at_least(
                    member_ref(doc, member, &layer_path(&member.id, &layer.id, "count")),
                    Quantity::new(QuantityKind::Dimensionless, layer.count as f64),
                    Quantity::new(QuantityKind::Dimensionless, count_req),
                    L(
                        &format!("Increase tension bar count of {} layer {} from {} to at least {:.0} (A_s,req≈{:.0} mm²).", member.id, layer.id, layer.count, count_req, a_req * 1e6),
                        &format!("Zugstabanzahl von {} Lage {} von {} auf mindestens {:.0} erhöhen (A_s,erf≈{:.0} mm²).", member.id, layer.id, layer.count, count_req, a_req * 1e6),
                    ),
                ));
            }
            flex = flex.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                length_m(d),
                length_m(d * 1.15),
                L(
                    &format!("Increase effective depth of {} from {:.0} mm to at least {:.0} mm.", member.id, d * 1000.0, d * 1150.0),
                    &format!("Statische Nutzhöhe von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, d * 1000.0, d * 1150.0),
                ),
            ));
        }
        
        out.push(flex.build());
        if let Some(acc_e) = &acc {
            let params_acc = na_de::AnnexParams::for_situation(annex, "accidental");
            let f_cd_acc = params_acc.f_cd_pa(f_ck);
            let f_cd_uls = params.f_cd_pa(f_ck);
            let m_rd_acc = if f_cd_uls > 0.0 { m_rd * (f_cd_acc / f_cd_uls) } else { m_rd };
            let mut flex_acc = CheckResult::assess(format!("en1992.6.1.flexure.acc.{}", member.id), part_11(), clause_11("§6.1"), member_ref(doc, member, &member_path(&member.id, "actions")), L("Accidental bending ACC-6.11", "Außergewöhnliche Biegung ACC-6.11"))
                .utilization(moment_nm(acc_e.m_ed.abs()), moment_nm(m_rd_acc.max(1.0)))
                .annex(annex)
                .explanation(L(
                    &format!("ACC-6.11: |M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (γ_c={:.2}, γ_s={:.2}, {})", acc_e.m_ed.abs() / 1e3, m_rd_acc / 1e3, params_acc.gamma_c, params_acc.gamma_s, acc_e.combination_id),
                    &format!("Außergewöhnlich ACC-6.11: |M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (γ_c={:.2}, γ_s={:.2}, {})", acc_e.m_ed.abs() / 1e3, m_rd_acc / 1e3, params_acc.gamma_c, params_acc.gamma_s, acc_e.combination_id),
                ));
            if acc_e.m_ed.abs() > m_rd_acc {
                flex_acc = flex_acc.remedy(Remedy::at_least(
                    member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                    length_m(d),
                    length_m(d * 1.2),
                    L("Increase effective depth for accidental combination.", "Statische Nutzhöhe für außergewöhnliche Kombination erhöhen."),
                ));
            }
            out.push(flex_acc.build());
        }


        // Shear V_Rd,c / V_Rd,s / V_Rd,max
        let v_rd_c = part_1_1::shear_v_rd_c_n(b, d, f_ck, rho, uls.n_ed, annex);
        let z = 0.9 * d;
        let sigma_cp = if b * h > 0.0 { (-uls.n_ed).max(0.0) / (b * h) } else { 0.0 };
        let f_cd = params.f_cd_pa(f_ck);
        let cot = params.cot_theta(annex, sigma_cp, f_cd);
        let asw_s = member.asw_per_s();
        let f_ywd = params.f_yd_pa(f_yk);
        let v_rd_s = if asw_s > 0.0 { part_1_1::shear_v_rd_s_n(asw_s, z, f_ywd, cot) } else { 0.0 };
        let v_rd_max = part_1_1::shear_v_rd_max_n(b, z, f_ck, cot, annex);
        let v_rd = if uls.v_ed <= v_rd_c { v_rd_c } else { v_rd_s.min(v_rd_max).max(v_rd_c) };
        // If V_Ed > V_Rd,c need stirrups capacity
        let v_limit = if asw_s > 0.0 { v_rd_s.min(v_rd_max) } else { v_rd_c };
        let mut shear = CheckResult::assess(format!("en1992.6.2.shear.{}", member.id), part_11(), clause_11("§6.2"), member_ref(doc, member, &member_path(&member.id, "actions")), L("Shear resistance", "Querkrafttragfähigkeit"))
            .utilization(force_n(uls.v_ed), force_n(v_limit.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("V_Ed={:.1} kN; V_Rd,c={:.1}, V_Rd,s={:.1}, V_Rd,max={:.1} kN; cotθ={:.2}; governing {}", uls.v_ed / 1e3, v_rd_c / 1e3, v_rd_s / 1e3, v_rd_max / 1e3, cot, uls.combination_id),
                &format!("V_Ed={:.1} kN; V_Rd,c={:.1}, V_Rd,s={:.1}, V_Rd,max={:.1} kN; cotθ={:.2}; maßgebend {}", uls.v_ed / 1e3, v_rd_c / 1e3, v_rd_s / 1e3, v_rd_max / 1e3, cot, uls.combination_id),
            ));
        if uls.v_ed > v_limit {
            if let Some(st) = &member.stirrups {
                // required spacing: V_Rd,s = (Asw/s)*z*fywd*cot >= V_Ed → Asw/s >= V_Ed/(z*fywd*cot)
                let need = uls.v_ed / (z * f_ywd * cot).max(1.0);
                let a_bar = std::f64::consts::PI * (st.diameter * 0.5).powi(2) * st.legs as f64;
                let s_req = if need > 0.0 { a_bar / need } else { st.spacing };
                shear = shear.remedy(Remedy::at_most(
                    member_ref(doc, member, &member_path(&member.id, "stirrups.spacing")),
                    length_m(st.spacing),
                    length_m(s_req.min(st.spacing)),
                    L(
                        &format!("Reduce stirrup spacing of {} from {:.0} mm to at most {:.0} mm.", member.id, st.spacing * 1000.0, s_req * 1000.0),
                        &format!("Bügelabstand von {} von {:.0} mm auf höchstens {:.0} mm verringern.", member.id, st.spacing * 1000.0, s_req * 1000.0),
                    ),
                ));
            } else {
                let need = uls.v_ed / (z * f_ywd * cot).max(1.0);
                shear = shear.remedy(Remedy::at_least(
                    member_ref(doc, member, &member_path(&member.id, "stirrups")),
                    area_m2(0.0),
                    area_m2(need),
                    L(
                        &format!("Provide shear reinforcement Asw/s ≥ {:.1} mm²/m on {}.", need * 1e6, member.id),
                        &format!("Querkraftbewehrung Asw/s ≥ {:.1} mm²/m für {} anordnen.", need * 1e6, member.id),
                    ),
                ));
            }
        }
        out.push(shear.build());

        // Torsion
        if uls.t_ed > 0.0 {
            let t_rd = part_1_1::torsion_t_rd_nm(f_ck, b, h, annex);
            let mut tor = CheckResult::assess(format!("en1992.6.3.torsion.{}", member.id), part_11(), clause_11("§6.3"), member_ref(doc, member, &member_path(&member.id, "actions")), L("Torsion resistance", "Torsionstragfähigkeit"))
                .utilization(moment_nm(uls.t_ed), moment_nm(t_rd.max(1.0)))
                .annex(annex)
                .explanation(L(
                    &format!("Torsion check: T_Ed={:.2} kNm ≤ T_Rd={:.2} kNm (gov {})", uls.t_ed / 1e3, t_rd / 1e3, uls.combination_id),
                    &format!("Torsionsnachweis: T_Ed={:.2} kNm ≤ T_Rd={:.2} kNm (maßgebend {})", uls.t_ed / 1e3, t_rd / 1e3, uls.combination_id),
                ));
            if uls.t_ed > t_rd {
                let h_req = h * (uls.t_ed / t_rd).sqrt();
                tor = tor.remedy(Remedy::at_least(
                    member_ref(doc, member, &member_path(&member.id, "height")),
                    length_m(h),
                    length_m(h_req),
                    L(
                        &format!("Increase section height of {} from {:.0} mm to at least {:.0} mm for torsion.", member.id, h * 1000.0, h_req * 1000.0),
                        &format!("Querschnittshöhe von {} von {:.0} mm auf mindestens {:.0} mm für Torsion erhöhen.", member.id, h * 1000.0, h_req * 1000.0),
                    ),
                ));
            }
            out.push(tor.build());
        }

        // Punching — β, u0/u1 from column geometry; DE NA C_Rd,c=0.18/γ_c with u0/d reduction; asw
        if matches!(member.kind, MemberKind::FlatSlab) {
            if let Some(p) = &member.punching {
                let beta = part_1_1::punching_beta(&p.column_position);
                let (u0, u1) = part_1_1::punching_perimeters(p.column_width, p.column_depth, d);
                let u0_over_d = u0 / d.max(1e-6);
                let mut v_rd_c_pa = part_1_1::punching_v_rd_c_pa(f_ck, d, rho, annex);
                // DE NA: C_Rd,c = 0.18/γ_c and reduction when u0/d < 4
                if matches!(annex, AnnexChoice::De) {
                    let ppar = na_de::AnnexParams::for_choice(annex);
                    let f_ck_mpa = f_ck / 1.0e6;
                    let d_mm = d * 1000.0;
                    let k = (1.0 + (200.0 / d_mm).sqrt()).min(2.0);
                    let rho_u = rho.min(0.02);
                    let mut c_rd = 0.18 / ppar.gamma_c;
                    if u0_over_d < 4.0 {
                        c_rd *= (0.5 + 0.125 * u0_over_d).min(1.0);
                    }
                    let v_min = 0.035 * k.powf(1.5) * f_ck_mpa.sqrt();
                    v_rd_c_pa = (c_rd * k * (100.0 * rho_u * f_ck_mpa).powf(1.0 / 3.0)).max(v_min) * 1.0e6;
                }
                let v_rd_max_pa = part_1_1::punching_v_rd_max_pa(v_rd_c_pa, annex);
                let v_ed_pa = if u1 * d > 0.0 { beta * uls.v_ed_punch / (u1 * d) } else { 0.0 };
                let v_rd_cs_pa = if p.asw > 0.0 {
                    v_rd_c_pa + 0.75 * p.asw * params.f_yd_pa(f_yk) / (u1 * d).max(1e-9)
                } else {
                    v_rd_c_pa
                };
                let mut punch = CheckResult::assess(format!("en1992.6.4.punching.{}", member.id), part_11(), clause_11("§6.4"), member_ref(doc, member, &member_path(&member.id, "punching")), L("Punching shear", "Durchstanztragfähigkeit"))
                    .utilization(stress_pa(v_ed_pa), stress_pa(v_rd_cs_pa.max(1.0)))
                    .annex(annex)
                    .explanation(L(
                        &format!("Punching: v_Ed={:.2} MPa (β={:.2}, u₁={:.2} m, u₀/d={:.2}) ≤ v_Rd,c={:.2} MPa; asw={:.0} mm²/m ({})", v_ed_pa / 1e6, beta, u1, u0_over_d, v_rd_cs_pa / 1e6, p.asw * 1e6, uls.combination_id),
                        &format!("Durchstanzen: v_Ed={:.2} MPa (β={:.2}, u₁={:.2} m, u₀/d={:.2}) ≤ v_Rd,c={:.2} MPa; asw={:.0} mm²/m ({})", v_ed_pa / 1e6, beta, u1, u0_over_d, v_rd_cs_pa / 1e6, p.asw * 1e6, uls.combination_id),
                    ));
                if v_ed_pa > v_rd_cs_pa {
                    let d_req = if u1 > 0.0 && v_rd_c_pa > 0.0 { beta * uls.v_ed_punch / (u1 * v_rd_c_pa) } else { d * 1.2 };
                    punch = punch.remedy(Remedy::at_least(
                        member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                        length_m(d),
                        length_m(d_req),
                        L(
                            &format!("Increase effective depth of {} from {:.0} mm to at least {:.0} mm.", member.id, d * 1000.0, d_req * 1000.0),
                            &format!("Statische Nutzhöhe von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, d * 1000.0, d_req * 1000.0),
                        ),
                    ));
                    punch = punch.remedy(Remedy::at_least(
                        member_ref(doc, member, &member_path(&member.id, "punching.asw")),
                        area_m2(p.asw),
                        area_m2((p.asw + 5.0e-4).max(5.0e-4)),
                        L(
                            &format!("Provide/increase punching reinforcement asw on {}.", member.id),
                            &format!("Durchstanzbewehrung asw für {} anordnen/erhöhen.", member.id),
                        ),
                    ));
                }
                out.push(punch.build());
                let mut pmax = CheckResult::assess(format!("en1992.6.4.punching-max.{}", member.id), part_11(), clause_11("§6.4.5"), member_ref(doc, member, &member_path(&member.id, "punching")), L("Punching crushing limit", "Durchstanzen Druckstrebe"))
                    .utilization(stress_pa(v_ed_pa), stress_pa(v_rd_max_pa.max(1.0)))
                    .annex(annex)
                    .explanation(L("Punching strut: v_Ed ≤ v_Rd,max (DE-NA 1.4·v_Rd,c)", "Durchstanz-Druckstrebe: v_Ed ≤ v_Rd,max (DE-NA 1,4·v_Rd,c)"));
                if v_ed_pa > v_rd_max_pa {
                    pmax = pmax.remedy(Remedy::at_least(
                        member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                        length_m(d),
                        length_m(d * v_ed_pa / v_rd_max_pa),
                        L("Increase slab depth to satisfy punching crushing.", "Deckendicke erhöhen, um Durchstanz-Druckstrebe zu erfüllen."),
                    ));
                }
                out.push(pmax.build());
            }
        }
    } else {
        out.push(
            CheckResult::assess(format!("en1992.uls.na.{}", member.id), part_11(), clause_11("§6"), member_ref(doc, member, &member_path(&member.id, "actions")), L("ULS actions", "Einwirkungen GZT"))
                .not_applicable(L("No ULS load case on member", "Kein GZT-Lastfall am Bauteil"))
                .annex(annex)
                .build(),
        );
    }

    // Crack width SLS
    if let Some(sls) = sls_qp {
        let w_max = part_1_1::w_max_m(member.exposure);
        // approx sigma_s from M / (z As)
        let z = 0.9 * d;
        let sigma_s = if a_s * z > 0.0 { sls.m_ed.abs() / (a_s * z) } else { 0.0 };
        let f_ct_eff = part_1_1::f_ctm_pa(f_ck);
        let rho_p = if b * (2.5 * (member.cover + 0.5 * 0.016)).min(0.5 * h) > 0.0 {
            a_s / (b * (2.5 * (member.cover + 0.5 * 0.016)).min(0.5 * h))
        } else {
            rho
        };
        let s_r = 3.4 * member.cover + 0.425 * 0.8 * 0.5 * 0.016 / rho_p.max(1e-6);
        let w_k = part_1_1::crack_width_m(sigma_s, rho_p.max(1e-4), f_ct_eff, e_s, s_r);
        let mut crack = CheckResult::assess(format!("en1992.7.3.crack.{}", member.id), part_11(), clause_11("§7.3.4"), member_ref(doc, member, &member_path(&member.id, "actions")), L("Crack width", "Rissbreite"))
            .utilization(length_m(w_k), length_m(w_max))
            .annex(annex)
            .explanation(L(
                &format!("Crack width w_k={:.3} mm ≤ w_max={:.2} mm (SLS {})", w_k * 1000.0, w_max * 1000.0, sls.combination_id),
                &format!("Rissbreite w_k={:.3} mm ≤ w_max={:.2} mm (GZG {})", w_k * 1000.0, w_max * 1000.0, sls.combination_id),
            ));
        if w_k > w_max {
            let a_req = a_s * (w_k / w_max);
            if let Some(layer) = member.longitudinal.first() {
                let d_req = (layer.diameter * (a_req / a_s.max(1e-12)).sqrt()).max(layer.diameter * 1.05);
                crack = crack.remedy(Remedy::at_least(
                    member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                    length_m(layer.diameter),
                    length_m(d_req),
                    L(
                        &format!("Increase bar diameter of {} so A_s ≥ {:.0} mm² to limit crack width.", member.id, a_req * 1e6),
                        &format!("Stabdurchmesser von {} erhöhen, damit A_s ≥ {:.0} mm² (Rissbreite).", member.id, a_req * 1e6),
                    ),
                ));
            }
        }
        out.push(crack.build());
    }

    // Min reinforcement
    let a_min = part_1_1::a_s_min_m2(f_ck, f_yk, b, d);
    let a_max = part_1_1::a_s_max_m2(b, h);
    let mut amin = CheckResult::assess(format!("en1992.9.minas.{}", member.id), part_11(), clause_11("§9.2.1"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), L("Minimum reinforcement", "Mindestbewehrung"))
        .minimum(area_m2(a_s), area_m2(a_min))
        .annex(annex)
        .explanation(L(
            &format!("Minimum steel: A_s={:.0} mm² ≥ A_s,min={:.0} mm²", a_s * 1e6, a_min * 1e6),
            &format!("Mindestbewehrung: A_s={:.0} mm² ≥ A_s,min={:.0} mm²", a_s * 1e6, a_min * 1e6),
        ));
    if a_s < a_min {
        if let Some(layer) = member.longitudinal.first() {
            let d_req = (layer.diameter * (a_min / a_s.max(1e-12)).sqrt()).max(layer.diameter * 1.05);
            amin = amin.remedy(Remedy::at_least(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                length_m(layer.diameter),
                length_m(d_req),
                L(
                    &format!("Increase bar diameter of {} so A_s ≥ {:.0} mm².", member.id, a_min * 1e6),
                    &format!("Stabdurchmesser von {} erhöhen, damit A_s ≥ {:.0} mm².", member.id, a_min * 1e6),
                ),
            ));
        }
    }
    out.push(amin.build());
    let mut amax = CheckResult::assess(format!("en1992.9.maxas.{}", member.id), part_11(), clause_11("§9.2.1"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), L("Maximum reinforcement", "Höchstbewehrung"))
        .utilization(area_m2(a_s), area_m2(a_max.max(1e-9)))
        .annex(annex)
        .explanation(L(
            &format!("Maximum steel: A_s={:.0} mm² ≤ A_s,max={:.0} mm²", a_s * 1e6, a_max * 1e6),
            &format!("Höchstbewehrung: A_s={:.0} mm² ≤ A_s,max={:.0} mm²", a_s * 1e6, a_max * 1e6),
        ));
    if a_s > a_max {
        if let Some(layer) = member.longitudinal.first() {
            let d_req = (layer.diameter * (a_max / a_s.max(1e-12)).sqrt()).min(layer.diameter * 0.95).max(0.006);
            amax = amax.remedy(Remedy::at_most(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                length_m(layer.diameter),
                length_m(d_req),
                L("Reduce bar diameter so reinforcement stays below 4% of Ac.", "Stabdurchmesser reduzieren, damit Bewehrung unter 4 % von Ac bleibt."),
            ));
        }
    }
    out.push(amax.build());

    // Deflection l/d — Table 7.4N + DE-NA K·35 and K²·150/l when deflection-sensitive
    if member.span > 0.0 && matches!(member.kind, MemberKind::Beam | MemberKind::Slab | MemberKind::FlatSlab) {
        let lim_table = part_1_1::basic_ld_limit(member.support, rho.max(1e-4), 0.0);
        let k = part_1_1::support_k(member.support);
        let lim_de = part_1_1::de_ld_caps(k, member.span, member.deflection_sensitive);
        let lim = lim_table.min(lim_de);
        let ld = member.span / d.max(1e-6);
        let mut defl = CheckResult::assess(format!("en1992.7.4.ld.{}", member.id), part_11(), clause_11("§7.4.2"), member_ref(doc, member, &member_path(&member.id, "effectiveDepth")), L("Span-to-depth limit", "Biegeschlankheit l/d"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, ld), Quantity::new(QuantityKind::Dimensionless, lim.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("l/d={:.1} ≤ {:.1} (Table 7.4N={:.1}; DE K·35 / K²·150/l={:.1}; K={:.2}; sensitive={})", ld, lim, lim_table, lim_de, k, member.deflection_sensitive),
                &format!("l/d={:.1} ≤ {:.1} (Tab. 7.4N={:.1}; DE K·35 / K²·150/l={:.1}; K={:.2}; empfindlich={})", ld, lim, lim_table, lim_de, k, member.deflection_sensitive),
            ));
        if ld > lim {
            let d_req = member.span / lim;
            defl = defl.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                length_m(d),
                length_m(d_req),
                L(
                    &format!("Increase effective depth of {} from {:.0} mm to at least {:.0} mm.", member.id, d * 1000.0, d_req * 1000.0),
                    &format!("Statische Nutzhöhe von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, d * 1000.0, d_req * 1000.0),
                ),
            ));
        }
        out.push(defl.build());
    }

    // Column slenderness — DE NA λ_lim + nominal curvature §5.8.8 when exceeded
    if matches!(member.kind, MemberKind::Column | MemberKind::Wall) && member.buckling_length > 0.0 {
        let lam = part_1_1::slenderness(member.buckling_length, b, h);
        let n_ed_col = uls.map(|u| u.n_ed).unwrap_or(0.0);
        let n_ratio = (n_ed_col.abs() / ((b * h) * params.f_cd_pa(f_ck)).max(1.0)).min(1.0);
        let lim = part_1_1::lambda_lim_de(n_ratio);
        let m2 = if lam > lim {
            part_1_1::second_order_moment_nm(n_ed_col, member.buckling_length, d, params.f_yd_pa(f_yk), e_s)
        } else {
            0.0
        };
        let m_ed1 = uls.map(|u| u.m_ed.abs()).unwrap_or(0.0);
        let m_ed_tot = m_ed1 + m2;
        let m_rd = part_1_1::flexural_resistance_nm(f_ck, b, d, a_s, f_yk, n_ed_col, annex, eps_cu2, n_parabola, e_s);
        let ok_cap = m_ed_tot <= m_rd + 1.0;
        let mut sl = CheckResult::assess(format!("en1992.5.8.slender.{}", member.id), part_11(), clause_11("§5.8.3"), member_ref(doc, member, &member_path(&member.id, "bucklingLength")), L("Column slenderness", "Stützenschlankheit"))
            .utilization(moment_nm(m_ed_tot.abs()), moment_nm(m_rd.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("Slenderness λ={:.1} (limit {:.1}, n={:.2}); second-order M_Ed+M₂={:.1} kNm ≤ M_Rd={:.1} kNm", lam, lim, n_ratio, m_ed_tot / 1e3, m_rd / 1e3),
                &format!("Schlankheit λ={:.1} (Grenze {:.1}, n={:.2}); Theorie II. Ordnung M_Ed+M₂={:.1} kNm ≤ M_Rd={:.1} kNm", lam, lim, n_ratio, m_ed_tot / 1e3, m_rd / 1e3),
            ));
        if lam > lim {
            let b_req = member.buckling_length / lim * 12.0f64.sqrt();
            sl = sl.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "width")),
                length_m(b),
                length_m(b_req),
                L(
                    &format!("Increase column width of {} to at least {:.0} mm to reduce slenderness.", member.id, b_req * 1000.0),
                    &format!("Stützenbreite von {} auf mindestens {:.0} mm erhöhen, um die Schlankheit zu verringern.", member.id, b_req * 1000.0),
                ),
            ));
            sl = sl.remedy(Remedy::at_most(
                member_ref(doc, member, &member_path(&member.id, "bucklingLength")),
                length_m(member.buckling_length),
                length_m(member.buckling_length * lim / lam),
                L(
                    &format!("Reduce buckling length of {} to ≤ {:.2} m.", member.id, member.buckling_length * lim / lam),
                    &format!("Knicklänge von {} auf ≤ {:.2} m verringern.", member.id, member.buckling_length * lim / lam),
                ),
            ));
        }
        out.push(sl.build());
    }

    // Prestress §5.10.2.1 / §5.10.3 stress limits using f_pk and f_p0,1k
    if let Some(ps) = &member.prestress {
        let steel = doc.prestress_steels.iter().find(|s| s.id == member.prestress_steel_id);
        if member.prestress_steel_id.is_empty() || steel.is_none() {
            let options: Vec<String> = doc.prestress_steels.iter().map(|s| s.id.clone()).collect();
            let mut dangling = CheckResult::assess(
                format!("en1992.integrity.prestressSteel.{}", member.id),
                part_11(),
                clause_11("§5.10"),
                member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
                L("Prestressing steel reference", "Spannstahl-Referenz"),
            )
            .annex(annex)
            .explanation(L(
                &format!("Member {} references prestressSteelId='{}' which is missing; choose an existing prestressing steel id.", member.id, member.prestress_steel_id),
                &format!("Bauteil {} verweist auf prestressSteelId='{}', das fehlt; eine vorhandene Spannstahl-Id wählen.", member.id, member.prestress_steel_id),
            ))
            .status(CheckStatus::Fail);
            dangling = dangling.remedy(Remedy::one_of(
                member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
                if options.is_empty() { vec!["yp1860".into()] } else { options },
                L(
                    "Set prestressSteelId to one of the declared prestressing steel ids.",
                    "prestressSteelId auf eine der deklarierten Spannstahl-Ids setzen.",
                ),
            ));
            out.push(dangling.build());
        } else {
        let steel = steel.unwrap();
        let p_m0 = ps.force;
        let p_minf = p_m0 * (1.0 - ps.loss_ratio.clamp(0.0, 0.5));
        let a_p = ps.area.max(1e-12);
        let sigma_pm0 = p_m0 / a_p;
        let sigma_pminf = p_minf / a_p;
        let f_pk = steel.f_pk;
        let f_p01k = steel.f_p0_1k;
        let sigma_p_max = (0.8 * f_pk).min(0.9 * f_p01k);
        let sigma_pm0_lim = (0.75 * f_pk).min(0.85 * f_p01k);
        let ac = (b * h).max(1e-12);
        let i = b * h.powi(3) / 12.0;
        let sigma_c = p_m0 / ac + p_m0 * ps.eccentricity.abs() * (h * 0.5) / i.max(1e-18);
        let lim_c = 0.6 * f_ck;
        let util_fpk = sigma_pm0 / (0.75 * f_pk).max(1.0);
        let util_fp01 = sigma_pm0 / (0.85 * f_p01k).max(1.0);
        let util_pmax = sigma_pm0 / sigma_p_max.max(1.0);
        let util_c = sigma_c / lim_c.max(1.0);
        let util_loss = ps.loss_ratio / 0.30;
        let util_inf = sigma_pminf / sigma_pm0_lim.max(1.0);
        let util = util_fpk.max(util_fp01).max(util_pmax).max(util_c).max(util_loss).max(util_inf);
        let mut pre = CheckResult::assess(format!("en1992.5.10.prestress.{}", member.id), part_11(), clause_11("§5.10.2"), member_ref(doc, member, &member_path(&member.id, "prestress")), L("Prestress stress limits", "Vorspannungs-Spannungsgrenzen"))
            .utilization(stress_pa(util * lim_c), stress_pa(lim_c.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("Prestress: σ_pm0={:.0} MPa ≤ min(0.75 f_pk, 0.85 f_p0,1k)={:.0} MPa; σ_pm,∞={:.0} MPa; σ_p,max={:.0} MPa; σ_c={:.1} MPa ≤ 0.6 f_ck; P_m,∞={:.0} kN (losses {:.0}%, steel {})",
                    sigma_pm0 / 1e6, sigma_pm0_lim / 1e6, sigma_pminf / 1e6, sigma_p_max / 1e6, sigma_c / 1e6, p_minf / 1e3, ps.loss_ratio * 100.0, steel.id),
                &format!("Vorspannung: σ_pm0={:.0} MPa ≤ min(0,75 f_pk, 0,85 f_p0,1k)={:.0} MPa; σ_pm,∞={:.0} MPa; σ_p,max={:.0} MPa; σ_c={:.1} MPa ≤ 0,6 f_ck; P_m,∞={:.0} kN (Verluste {:.0} %, Stahl {})",
                    sigma_pm0 / 1e6, sigma_pm0_lim / 1e6, sigma_pminf / 1e6, sigma_p_max / 1e6, sigma_c / 1e6, p_minf / 1e3, ps.loss_ratio * 100.0, steel.id),
            ));
        if util > 1.0 {
            pre = pre.remedy(Remedy::at_most(
                member_ref(doc, member, &member_path(&member.id, "prestress.force")),
                force_n(p_m0),
                force_n(p_m0 / util),
                L("Reduce prestressing force to satisfy §5.10 stress limits.", "Vorspannkraft reduzieren, damit Spannungsgrenzen nach §5.10 eingehalten werden."),
            ));
        }
        let mut decomp = CheckResult::assess(format!("en1992.5.10.decompression.{}", member.id), part_11(), clause_11("§5.10.9"), member_ref(doc, member, &member_path(&member.id, "prestress")), L("Decompression SLS", "Dekompression GZG"))
            .utilization(stress_pa((-sigma_c).max(0.0)), stress_pa(0.45 * f_ck))
            .annex(annex)
            .explanation(L(
                &format!("Decompression SLS: concrete stress under prestress σ_c={:.2} MPa (compression positive); steel id={}", sigma_c / 1e6, member.prestress_steel_id),
                &format!("Dekompression GZG: Betonspannung unter Vorspannung σ_c={:.2} MPa (Druck positiv); Spannstahl-Id={}", sigma_c / 1e6, member.prestress_steel_id),
            ));
        if sigma_c < 0.0 {
            decomp = decomp.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "prestress.force")),
                force_n(p_m0),
                force_n(p_m0 * 1.1),
                L("Increase prestress to maintain decompression.", "Vorspannung erhöhen, um Dekompression sicherzustellen."),
            ));
        }
        out.push(pre.build());
        out.push(decomp.build());
        }
    } else if !member.prestress_steel_id.is_empty() {
        let options: Vec<String> = doc.prestress_steels.iter().map(|s| s.id.clone()).collect();
        let mut dangling = CheckResult::assess(
            format!("en1992.integrity.prestressSteel.orphan.{}", member.id),
            part_11(),
            clause_11("§5.10"),
            member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
            L("Orphan prestressing steel id", "Verwaiste Spannstahl-Id"),
        )
        .annex(annex)
        .explanation(L(
            &format!("Member {} sets prestressSteelId='{}' without a prestress specification.", member.id, member.prestress_steel_id),
            &format!("Bauteil {} setzt prestressSteelId='{}' ohne Vorspannungsangabe.", member.id, member.prestress_steel_id),
        ))
        .status(CheckStatus::Fail);
        dangling = dangling.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
            if options.is_empty() { vec![String::new()] } else { options },
            L("Clear prestressSteelId or add a prestress block linked to a declared steel.", "prestressSteelId leeren oder einen Vorspannungsblock mit deklariertem Stahl ergänzen."),
        ));
        out.push(dangling.build());
    }

    // Reinforcement ductility Annex C Table C.1 — k and ε_uk vs declared class
    {
        let (k_min, eps_min) = reinf.ductility.table_c1_minima();
        let ok_k = reinf.k >= k_min;
        let ok_e = reinf.eps_uk >= eps_min;
        let util_d = (k_min / reinf.k.max(1e-6)).max(eps_min / reinf.eps_uk.max(1e-9));
        let eps_ud = 0.9 * reinf.eps_uk;
        let mut duct = CheckResult::assess(format!("en1992.annexC.ductility.{}", member.id), part_11(), clause_11("§3.2.7"), member_ref(doc, member, &format!("reinforcementGrades[id={}].ductility", member.reinforcement_grade_id)), L("Reinforcement ductility class", "Duktilitätsklasse Betonstahl"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, util_d), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(L(
                &format!("Ductility {:?}: k={:.2} ≥ {:.2}, ε_uk={:.1}% ≥ {:.1}%, ε_ud=0.9ε_uk={:.2}% (inclined top branch)", reinf.ductility, reinf.k, k_min, reinf.eps_uk * 100.0, eps_min * 100.0, eps_ud * 100.0),
                &format!("Duktilität {:?}: k={:.2} ≥ {:.2}, ε_uk={:.1} % ≥ {:.1} %, ε_ud=0,9ε_uk={:.2} % (geneigter oberer Ast)", reinf.ductility, reinf.k, k_min, reinf.eps_uk * 100.0, eps_min * 100.0, eps_ud * 100.0),
            ));
        if !(ok_k && ok_e) {
            duct = duct.remedy(Remedy::at_least(
                member_ref(doc, member, &format!("reinforcementGrades[id={}].k", member.reinforcement_grade_id)),
                Quantity::new(QuantityKind::Dimensionless, reinf.k),
                Quantity::new(QuantityKind::Dimensionless, k_min),
                L("Raise k / ε_uk to Annex C Table C.1 minima for the declared ductility class.", "k / ε_uk auf die Mindestwerte nach Anhang C Tabelle C.1 der angegebenen Duktilitätsklasse anheben."),
            ));
        }
        out.push(duct.build());
    }

// Fire — Tables 5.2a–5.11 by element type
    if let Some(fire) = &member.fire {
        let (b_min, a_req) = part_1_2_fire::required_for(member.kind, member.support, fire.rating, &fire.column_method, &fire.slab_system);
        let mut fa = CheckResult::assess(format!("en1992.1-2.a.{}", member.id), part_12(), ClauseId::new("EN 1992-1-2", "§5.6", "5.6"), member_ref(doc, member, &member_path(&member.id, "fire.axisDistance")), L("Fire axis distance", "Achsabstand Brandschutz"))
            .minimum(length_m(fire.axis_distance), length_m(a_req))
            .annex(annex)
            .explanation(L(
                &format!("Fire axis distance a={:.0} mm ≥ a_req={:.0} mm ({:?}; method {}; slab {})", fire.axis_distance * 1000.0, a_req * 1000.0, fire.rating, fire.column_method, fire.slab_system),
                &format!("Brandschutz-Achsabstand a={:.0} mm ≥ a_erf={:.0} mm ({:?}; Verfahren {}; Platte {})", fire.axis_distance * 1000.0, a_req * 1000.0, fire.rating, fire.column_method, fire.slab_system),
            ));
        if fire.axis_distance < a_req {
            fa = fa.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "fire.axisDistance")),
                length_m(fire.axis_distance),
                length_m(a_req),
                L(
                    &format!("Increase fire axis distance of {} from {:.0} mm to at least {:.0} mm.", member.id, fire.axis_distance * 1000.0, a_req * 1000.0),
                    &format!("Achsabstand Brandschutz von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, fire.axis_distance * 1000.0, a_req * 1000.0),
                ),
            ));
        }
        out.push(fa.build());
        let mut fb = CheckResult::assess(format!("en1992.1-2.bmin.{}", member.id), part_12(), ClauseId::new("EN 1992-1-2", "§5.6", "5.6"), member_ref(doc, member, &member_path(&member.id, "width")), L("Fire minimum width", "Mindestbreite Brandschutz"))
            .minimum(length_m(b), length_m(b_min))
            .annex(annex)
            .explanation(L(
                &format!("Fire min. dimension b/h={:.0} mm ≥ {:.0} mm", b * 1000.0, b_min * 1000.0),
                &format!("Brandschutz Mindestdickemaß b/h={:.0} mm ≥ {:.0} mm", b * 1000.0, b_min * 1000.0),
            ));
        if b < b_min {
            fb = fb.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "width")),
                length_m(b),
                length_m(b_min),
                L(
                    &format!("Increase width of {} from {:.0} mm to at least {:.0} mm for fire.", member.id, b * 1000.0, b_min * 1000.0),
                    &format!("Breite von {} von {:.0} mm auf mindestens {:.0} mm für Brandschutz erhöhen.", member.id, b * 1000.0, b_min * 1000.0),
                ),
            ));
        }
        out.push(fb.build());
    }


    // SLS stress limits §7.2 DE NA (k1–k4 style: 0.6 f_ck char in XD/XF/XS; 0.45 f_ck qp; 0.8 f_yk)
    if let Some(sc) = sls_char {
        let sig_s = part_1_1::cracked_sigma_s_pa(sc.m_ed, b, d, a_s, e_s, concrete.e_cm());
        let sig_c = part_1_1::cracked_sigma_c_pa(sc.m_ed, b, d, a_s, e_s, concrete.e_cm());
        let lim_s = 0.8 * f_yk;
        let harsh = matches!(member.exposure, ExposureClass::Xd1 | ExposureClass::Xd2 | ExposureClass::Xd3 | ExposureClass::Xs1 | ExposureClass::Xs2 | ExposureClass::Xs3 | ExposureClass::Xf1 | ExposureClass::Xf2 | ExposureClass::Xf3 | ExposureClass::Xf4);
        let lim_c = if harsh { 0.60 * f_ck } else { 0.60 * f_ck };
        let mut ss = CheckResult::assess(format!("en1992.7.2.sigma-s.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), L("SLS steel stress", "GZG Stahlspannung"))
            .utilization(stress_pa(sig_s), stress_pa(lim_s.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("σ_s={:.0} MPa ≤ 0.8 f_yk={:.0} MPa (characteristic {})", sig_s / 1e6, lim_s / 1e6, sc.combination_id),
                &format!("σ_s={:.0} MPa ≤ 0,8 f_yk={:.0} MPa (charakteristisch {})", sig_s / 1e6, lim_s / 1e6, sc.combination_id),
            ));
        if sig_s > lim_s {
            if let Some(layer) = member.longitudinal.first() {
                let a_need = a_s * sig_s / lim_s.max(1e-12);
                let d_req = (layer.diameter * (a_need / a_s.max(1e-12)).sqrt()).max(layer.diameter * 1.05);
                ss = ss.remedy(Remedy::at_least(
                    member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                    length_m(layer.diameter),
                    length_m(d_req),
                    L("Increase bar diameter to reduce SLS steel stress.", "Stabdurchmesser erhöhen, um die GZG-Stahlspannung zu senken."),
                ));
            }
        }
        out.push(ss.build());
        let mut scc = CheckResult::assess(format!("en1992.7.2.sigma-c.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), L("SLS concrete stress", "GZG Betondruckspannung"))
            .utilization(stress_pa(sig_c), stress_pa(lim_c.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("σ_c={:.1} MPa ≤ {:.0}% f_ck={:.1} MPa (char {})", sig_c / 1e6, if harsh {60} else {60}, lim_c / 1e6, sc.combination_id),
                &format!("σ_c={:.1} MPa ≤ {:.0}% f_ck={:.1} MPa (char {})", sig_c / 1e6, if harsh {60} else {60}, lim_c / 1e6, sc.combination_id),
            ));
        scc = scc.explanation(L(
            &format!("Concrete stress σ_c={:.1} MPa ≤ 0.60 f_ck under characteristic SLS ({})", sig_c / 1e6, sc.combination_id),
            &format!("Betondruckspannung σ_c={:.1} MPa ≤ 0,60 f_ck unter charakteristischer GZG ({})", sig_c / 1e6, sc.combination_id),
        ));
        if sig_c > lim_c {
            scc = scc.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "height")),
                length_m(h),
                length_m(h * 1.1),
                L("Increase section height to reduce concrete stress.", "Querschnittshöhe erhöhen, um die Betonspannung zu senken."),
            ));
        }
        out.push(scc.build());
    }
    if let Some(qp) = sls_qp {
        let sig_c = part_1_1::cracked_sigma_c_pa(qp.m_ed, b, d, a_s, e_s, concrete.e_cm());
        let lim = 0.45 * f_ck;
        let mut qp_c = CheckResult::assess(format!("en1992.7.2.creep.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), L("Quasi-permanent concrete stress", "Quasi-ständige Betondruckspannung"))
            .utilization(stress_pa(sig_c), stress_pa(lim.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("σ_c={:.1} MPa ≤ 0.45 f_ck={:.1} MPa for linear creep ({})", sig_c / 1e6, lim / 1e6, qp.combination_id),
                &format!("σ_c={:.1} MPa ≤ 0,45 f_ck={:.1} MPa für lineares Kriechen ({})", sig_c / 1e6, lim / 1e6, qp.combination_id),
            ));
        if sig_c > lim {
            qp_c = qp_c.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                length_m(d),
                length_m(d * 1.1),
                L("Increase depth to satisfy creep stress limit.", "Nutzhöhe erhöhen für Kriechspannungsgrenze."),
            ));
        }
        out.push(qp_c.build());
    }

    // Anchorage §8.4 and laps §8.7
    for layer in &member.longitudinal {
        let eta1 = if layer.bond_condition == "poor" { 0.7 } else { 1.0 };
        let eta2 = if layer.diameter <= 0.032 { 1.0 } else { ((132.0 - layer.diameter * 1000.0) / 100.0).clamp(0.7, 1.0) };
        let eta_agg = if layer.aggregate_size > 0.016 { 0.95 } else { 1.0 };
        let f_bd = part_1_1::f_bd_pa(concrete.f_ctk_005(), eta1, eta2) * eta_agg;
        let sigma_sd = params.f_yd_pa(f_yk);
        let lb_rqd = part_1_1::l_b_rqd_m(layer.diameter, sigma_sd, f_bd);
        let lbd = part_1_1::l_bd_m(lb_rqd, layer.diameter, 1.0);
        let mut anc = CheckResult::assess(format!("en1992.8.4.anchorage.{}.{}", member.id, layer.id), part_11(), clause_11("§8.4"), member_ref(doc, member, &layer_path(&member.id, &layer.id, "anchorageLength")), L("Anchorage length", "Verankerungslänge"))
            .minimum(length_m(layer.anchorage_length), length_m(lbd))
            .annex(annex)
            .explanation(L(
                &format!("l_bd,prov={:.0} mm ≥ l_bd={:.0} mm (f_bd={:.2} MPa, η1={:.2})", layer.anchorage_length * 1000.0, lbd * 1000.0, f_bd / 1e6, eta1),
                &format!("l_bd,vorh={:.0} mm ≥ l_bd={:.0} mm (f_bd={:.2} MPa, η1={:.2})", layer.anchorage_length * 1000.0, lbd * 1000.0, f_bd / 1e6, eta1),
            ));
        if layer.anchorage_length < lbd {
            anc = anc.remedy(Remedy::at_least(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "anchorageLength")),
                length_m(layer.anchorage_length),
                length_m(lbd),
                L(
                    &format!("Increase anchorage of {}/{} to ≥ {:.0} mm.", member.id, layer.id, lbd * 1000.0),
                    &format!("Verankerung von {}/{} auf ≥ {:.0} mm erhöhen.", member.id, layer.id, lbd * 1000.0),
                ),
            ));
            anc = anc.remedy(Remedy::at_most(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                length_m(layer.diameter),
                length_m((layer.diameter - 0.002).max(0.008)),
                L("Reduce bar diameter to shorten required anchorage.", "Stabdurchmesser verringern, um die erforderliche Verankerung zu verkürzen."),
            ));
        }
        out.push(anc.build());
        let alpha6 = 1.5;
        let l0 = part_1_1::l_0_m(lbd, layer.diameter, alpha6);
        let mut lap = CheckResult::assess(format!("en1992.8.7.lap.{}.{}", member.id, layer.id), part_11(), clause_11("§8.7"), member_ref(doc, member, &layer_path(&member.id, &layer.id, "lapLength")), L("Lap length", "Übergreifungslänge"))
            .minimum(length_m(layer.lap_length), length_m(l0))
            .annex(annex)
            .explanation(L(
                &format!("l_0,prov={:.0} mm ≥ l_0={:.0} mm (α6={:.1})", layer.lap_length * 1000.0, l0 * 1000.0, alpha6),
                &format!("l_0,vorh={:.0} mm ≥ l_0={:.0} mm (α6={:.1})", layer.lap_length * 1000.0, l0 * 1000.0, alpha6),
            ));
        if layer.lap_length < l0 {
            lap = lap.remedy(Remedy::at_least(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "lapLength")),
                length_m(layer.lap_length),
                length_m(l0),
                L(
                    &format!("Increase lap of {}/{} to ≥ {:.0} mm.", member.id, layer.id, l0 * 1000.0),
                    &format!("Übergreifung von {}/{} auf ≥ {:.0} mm erhöhen.", member.id, layer.id, l0 * 1000.0),
                ),
            ));
        }
        out.push(lap.build());
    }

    // Bridge
    if matches!(member.kind, MemberKind::Bridge) || member.bridge_sigma_c > 0.0 || member.bridge_delta_sigma_s > 0.0 {
        if member.bridge_sigma_c > 0.0 {
            let lim = part_2::concrete_stress_limit_pa(f_ck);
            let mut br = CheckResult::assess(format!("en1992-2.7.2.{}", member.id), part_2(), ClauseId::new("EN 1992-2", "§7.2", "7.2"), member_ref(doc, member, &member_path(&member.id, "bridgeSigmaC")), L("Bridge concrete stress (frequent)", "Brücken Betonspannung (häufig)"))
                .utilization(stress_pa(member.bridge_sigma_c), stress_pa(lim))
                .annex(annex)
                .explanation(L("Bridge frequent concrete stress σ_c ≤ 0.6 f_ck", "Brücke häufige Betondruckspannung σ_c ≤ 0,6 f_ck"));
            if member.bridge_sigma_c > lim {
                br = br.remedy(Remedy::at_most(
                    member_ref(doc, member, &member_path(&member.id, "bridgeSigmaC")),
                    stress_pa(member.bridge_sigma_c),
                    stress_pa(lim),
                    L("Reduce frequent concrete stress to ≤ 0.6 f_ck.", "Häufige Betonspannung auf ≤ 0,6 f_ck reduzieren."),
                ));
            }
            out.push(br.build());
        }
        if member.bridge_delta_sigma_s > 0.0 {
            let lim = part_2::fatigue_limit_pa();
            let mut bf = CheckResult::assess(format!("en1992-2.6.8.{}", member.id), part_2(), ClauseId::new("EN 1992-2", "§6.8.4", "6.8.4"), member_ref(doc, member, &member_path(&member.id, "bridgeDeltaSigmaS")), L("Bridge reinforcement fatigue", "Brücken Ermüdung Betonstahl"))
                .utilization(stress_pa(member.bridge_delta_sigma_s), stress_pa(lim))
                .annex(annex)
                .explanation(L("Bridge reinforcement fatigue Δσ_s ≤ Δσ_Rsk/γ_S,fat", "Brücke Betonstahl-Ermüdung Δσ_s ≤ Δσ_Rsk/γ_S,fat"));
            if member.bridge_delta_sigma_s > lim {
                bf = bf.remedy(Remedy::at_most(
                    member_ref(doc, member, &member_path(&member.id, "bridgeDeltaSigmaS")),
                    stress_pa(member.bridge_delta_sigma_s),
                    stress_pa(lim),
                    L("Reduce fatigue stress range.", "Ermüdungsspannungsschwingbreite reduzieren."),
                ));
            }
            out.push(bf.build());
        }
    }

    // Liquid retaining
    if matches!(member.kind, MemberKind::LiquidRetaining) || member.tightness.is_some() {
        if let Some(tc) = member.tightness {
            match part_3_liquid::tightness_crack_limit_m(tc, member.hd_over_h) {
                None => out.push(
                    CheckResult::assess(format!("en1992-3.crack.{}", member.id), part_3(), ClauseId::new("EN 1992-3", "§7.3", "7.3"), member_ref(doc, member, &member_path(&member.id, "tightness")), L("Liquid-retaining crack width", "Rissbreite Flüssigkeitsbehälter"))
                        .not_applicable(L("TC0: no crack-width requirement", "TC0: keine Rissbreitenanforderung"))
                        .annex(annex)
                        .build(),
                ),
                Some(lim) => {
                    let w_k = part_1_1::crack_width_m(member.liquid_sigma_s, member.liquid_rho_p_eff.max(1e-4), member.liquid_f_ct_eff, e_s, member.liquid_s_r_max);
                    let mut liq = CheckResult::assess(format!("en1992-3.crack.{}", member.id), part_3(), ClauseId::new("EN 1992-3", "§7.3", "7.3"), member_ref(doc, member, &member_path(&member.id, "liquidSigmaS")), L("Liquid-retaining crack width", "Rissbreite Flüssigkeitsbehälter"))
                        .utilization(length_m(w_k), length_m(lim))
                        .annex(annex)
                        .explanation(L(&format!("w_k={:.3} mm, limit={:.3} mm", w_k * 1000.0, lim * 1000.0), &format!("w_k={:.3} mm, Grenzwert={:.3} mm", w_k * 1000.0, lim * 1000.0)));
                    if w_k > lim {
                        let sig_req = member.liquid_sigma_s * lim / w_k;
                        liq = liq.remedy(Remedy::at_most(
                            member_ref(doc, member, &member_path(&member.id, "liquidSigmaS")),
                            stress_pa(member.liquid_sigma_s),
                            stress_pa(sig_req),
                            L("Reduce steel stress under quasi-permanent combination.", "Stahlspannung unter quasi-ständiger Einwirkungskombination reduzieren."),
                        ));
                    }
                    out.push(liq.build());
                }
            }
        }
    }

    out
}


/// ⚖️ Combine characteristic anchor actions (N_k, V_k and allied extras) per EN 1990 into design N_Ed, V_Ed.
pub fn combine_anchor_actions(anchor: &Anchor) -> Vec<DesignEffects> {
    let mut g = (0.0_f64, 0.0_f64);
    let mut variables: Vec<(&LoadCaseActions, (f64, f64))> = Vec::new();
    let mut accidentals = Vec::new();
    for a in &anchor.actions {
        let mut n = a.n_k;
        let mut v = a.v_k;
        match a.source.as_str() {
            "udl" => {
                n += a.g_k_line.abs() + a.q_k_line.abs();
            }
            "point" => {
                n += a.point_force.abs();
            }
            "external" => {}
            _ => {
                n += a.point_force.abs() + a.g_k_line.abs() + a.q_k_line.abs();
            }
        }
        if anchor.h_ef > 0.0 {
            n += a.m_k.abs() / anchor.h_ef;
        }
        if anchor.c1 > 0.0 {
            v += a.t_k.abs() / anchor.c1;
        }
        v += a.v_k_punch.abs();
        let eff = (n, v);
        match a.kind.as_str() {
            "permanent" | "prestress" => { g.0 += eff.0; g.1 += eff.1; }
            "accidental" => accidentals.push((a, eff)),
            _ => variables.push((a, eff)),
        }
    }
    let gamma_g = 1.35;
    let gamma_q = 1.5;
    let mut out = Vec::new();
    if variables.is_empty() {
        out.push(DesignEffects {
            m_ed: 0.0, n_ed: gamma_g * g.0, v_ed: gamma_g * g.1, t_ed: 0.0, v_ed_punch: 0.0,
            combination_id: "ULS-6.10-G".into(), situation: "uls".into(),
        });
    } else {
        for (lead_i, (lead_a, lead_e)) in variables.iter().enumerate() {
            let mut n = gamma_g * g.0 + gamma_q * lead_e.0;
            let mut v = gamma_g * g.1 + gamma_q * lead_e.1;
            for (j, (oa, oe)) in variables.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(&oa.category);
                n += gamma_q * psi0 * oe.0;
                v += gamma_q * psi0 * oe.1;
            }
            out.push(DesignEffects {
                m_ed: 0.0, n_ed: n, v_ed: v, t_ed: 0.0, v_ed_punch: 0.0,
                combination_id: format!("ULS-6.10b-{}", lead_a.id), situation: "uls".into(),
            });
            let (_, _, psi2) = psi_factors(&lead_a.category);
            let mut n_qp = g.0 + psi2 * lead_e.0;
            let mut v_qp = g.1 + psi2 * lead_e.1;
            for (j, (oa, oe)) in variables.iter().enumerate() {
                if j == lead_i { continue; }
                let (_, _, p2) = psi_factors(&oa.category);
                n_qp += p2 * oe.0;
                v_qp += p2 * oe.1;
            }
            out.push(DesignEffects {
                m_ed: 0.0, n_ed: n_qp, v_ed: v_qp, t_ed: 0.0, v_ed_punch: 0.0,
                combination_id: format!("SLS-qp-{}", lead_a.id), situation: "sls_qp".into(),
            });
        }
    }
    for (acc_a, acc_e) in &accidentals {
        let mut n = 1.0 * g.0 + acc_e.0;
        let mut v = 1.0 * g.1 + acc_e.1;
        for (oa, oe) in &variables {
            let (_, psi1, _) = psi_factors(&oa.category);
            n += psi1 * oe.0;
            v += psi1 * oe.1;
        }
        out.push(DesignEffects {
            m_ed: 0.0, n_ed: n, v_ed: v, t_ed: 0.0, v_ed_punch: 0.0,
            combination_id: format!("ACC-6.11-{}", acc_a.id), situation: "accidental".into(),
        });
    }
    out
}

/// ⚓️ Evaluate anchors with EN 1990-combined characteristic actions + EN 1992-4 resistances.
pub fn evaluate_anchor(doc: &En1992Snapshot, anchor: &Anchor) -> Vec<CheckResult> {
    let annex = doc.annex;
    let label = L(
        &format!("{} — Anchor {}", doc.title, anchor.id),
        &format!("{} — Dübel {}", doc.title, anchor.id),
    );
    let mut out = Vec::new();
    let effects = combine_anchor_actions(anchor);
    let uls = governing(&effects, "uls").cloned().or_else(|| effects.first().cloned());
    let Some(uls) = uls else {
        out.push(CheckResult::assess(format!("en1992-4.na.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7", "7"), SubjectRef::new(&anchor.id, format!("anchors[id={}]", anchor.id), label), L("Anchor actions", "Dübeleinwirkungen"))
            .not_applicable(L("No characteristic actions declared for anchor.", "Keine charakteristischen Einwirkungen für den Dübel angegeben."))
            .build());
        return out;
    };
    let n_ed = uls.n_ed.max(0.0);
    let v_ed = uls.v_ed.max(0.0);

    let n_rd_s = part_4::steel_resistance_n(anchor.a_s, anchor.f_uk, anchor.f_yk);
    let mut s = CheckResult::assess(format!("en1992-4.steel.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.4", "7.2.1.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()), L("Anchor steel tension", "Dübel Stahlzugtragfähigkeit"))
        .utilization(force_n(n_ed), force_n(n_rd_s.max(1.0)))
        .annex(annex)
        .explanation(L(
            &format!("Anchor steel: N_Ed={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_ed / 1e3, n_rd_s / 1e3, uls.combination_id),
            &format!("Dübel Stahlzug: N_Ed={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_ed / 1e3, n_rd_s / 1e3, uls.combination_id),
        ));
    if n_ed > n_rd_s {
        let a_req = anchor.a_s * n_ed / n_rd_s;
        s = s.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()),
            area_m2(anchor.a_s), area_m2(a_req),
            L(&format!("Increase anchor steel area to ≥ {:.1} mm².", a_req * 1e6), &format!("Dübelstahlquerschnitt auf ≥ {:.1} mm² erhöhen.", a_req * 1e6)),
        ));
    }
    out.push(s.build());

    if let Some(sls) = governing(&effects, "sls_qp") {
        let n_sls = sls.n_ed.max(0.0);
        let n_rd_sls = anchor.a_s * anchor.f_yk / 1.15;
        let mut ss = CheckResult::assess(format!("en1992-4.steel.sls.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.4", "7.2.1.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].actions", anchor.id), label.clone()), L("Anchor steel SLS (ψ₂)", "Dübel Stahlzug GZG (ψ₂)"))
            .utilization(force_n(n_sls), force_n(n_rd_sls.max(1.0)))
            .annex(annex)
            .explanation(L(
                &format!("Anchor SLS: N_Ed,qp={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_sls / 1e3, n_rd_sls / 1e3, sls.combination_id),
                &format!("Dübel GZG: N_Ed,qp={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_sls / 1e3, n_rd_sls / 1e3, sls.combination_id),
            ));
        if n_sls > n_rd_sls {
            ss = ss.remedy(Remedy::at_least(
                SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()),
                area_m2(anchor.a_s), area_m2(anchor.a_s * n_sls / n_rd_sls),
                L("Increase anchor steel area for SLS.", "Dübelstahlquerschnitt für GZG erhöhen."),
            ));
        }
        out.push(ss.build());
    }

    let n_rd_c = part_4::cone_resistance_n(anchor.f_ck, anchor.h_ef, anchor.cracked);
    let mut c = CheckResult::assess(format!("en1992-4.cone.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.5", "7.2.1.5"), SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()), L("Concrete cone", "Betonausbruchkegel"))
        .utilization(force_n(n_ed), force_n(n_rd_c.max(1.0)))
        .annex(annex)
        .explanation(L(
            &format!("Concrete cone: N_Ed={:.1} kN ≤ N_Rd,c={:.1} kN", n_ed / 1e3, n_rd_c / 1e3),
            &format!("Betonausbruchkegel: N_Ed={:.1} kN ≤ N_Rd,c={:.1} kN", n_ed / 1e3, n_rd_c / 1e3),
        ));
    if n_ed > n_rd_c {
        let h_req = anchor.h_ef * (n_ed / n_rd_c).powf(2.0 / 3.0);
        c = c.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()),
            length_m(anchor.h_ef), length_m(h_req),
            L(&format!("Increase effective embedment to ≥ {:.0} mm.", h_req * 1000.0), &format!("Effektive Verankerungstiefe auf ≥ {:.0} mm erhöhen.", h_req * 1000.0)),
        ));
    }
    out.push(c.build());

    let n_rd_p = part_4::pullout_resistance_n(anchor.a_s, anchor.f_uk);
    let mut p = CheckResult::assess(format!("en1992-4.pullout.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.6", "7.2.1.6"), SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()), L("Pull-out", "Herausziehen"))
        .utilization(force_n(n_ed), force_n(n_rd_p.max(1.0)))
        .annex(annex)
        .explanation(L(
            &format!("Pull-out: N_Ed={:.1} kN ≤ N_Rd,p={:.1} kN", n_ed / 1e3, n_rd_p / 1e3),
            &format!("Herausziehen: N_Ed={:.1} kN ≤ N_Rd,p={:.1} kN", n_ed / 1e3, n_rd_p / 1e3),
        ));
    if n_ed > n_rd_p {
        p = p.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()),
            area_m2(anchor.a_s), area_m2(anchor.a_s * n_ed / n_rd_p),
            L("Increase steel area against pull-out.", "Stahlquerschnitt gegen Herausziehen vergrößern."),
        ));
    }
    out.push(p.build());

    let n_rd_sp = part_4::splitting_resistance_n(anchor.f_ck, anchor.h_ef, anchor.c1);
    let mut sp = CheckResult::assess(format!("en1992-4.splitting.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.7", "7.2.1.7"), SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()), L("Splitting", "Spalten"))
        .utilization(force_n(n_ed), force_n(n_rd_sp.max(1.0)))
        .annex(annex)
        .explanation(L(
            &format!("Splitting: N_Ed={:.1} kN ≤ N_Rd,sp={:.1} kN", n_ed / 1e3, n_rd_sp / 1e3),
            &format!("Spalten: N_Ed={:.1} kN ≤ N_Rd,sp={:.1} kN", n_ed / 1e3, n_rd_sp / 1e3),
        ));
    if n_ed > n_rd_sp {
        sp = sp.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()),
            length_m(anchor.c1), length_m(anchor.c1 * 1.25),
            L("Increase edge distance against splitting.", "Randabstand gegen Spalten vergrößern."),
        ));
    }
    out.push(sp.build());

    let n_rd_pr = part_4::pryout_resistance_n(anchor.f_ck, anchor.h_ef, anchor.cracked);
    let mut pr = CheckResult::assess(format!("en1992-4.pryout.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.2.4", "7.2.2.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()), L("Pry-out", "Hebelausbruch"))
        .utilization(force_n(v_ed), force_n(n_rd_pr.max(1.0)))
        .annex(annex)
        .explanation(L(
            &format!("Pry-out: V_Ed={:.1} kN ≤ V_Rd,cp={:.1} kN", v_ed / 1e3, n_rd_pr / 1e3),
            &format!("Hebelausbruch: V_Ed={:.1} kN ≤ V_Rd,cp={:.1} kN", v_ed / 1e3, n_rd_pr / 1e3),
        ));
    if v_ed > n_rd_pr {
        pr = pr.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()),
            length_m(anchor.h_ef), length_m(anchor.h_ef * 1.2),
            L("Increase embedment against pry-out.", "Verankerungstiefe gegen Hebelausbruch erhöhen."),
        ));
    }
    out.push(pr.build());

    let v_rd = part_4::edge_resistance_n(anchor.d, anchor.h_ef, anchor.f_ck, anchor.c1);
    let mut e = CheckResult::assess(format!("en1992-4.edge.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.2.5", "7.2.2.5"), SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()), L("Edge breakout shear", "Randausbruch Querkraft"))
        .utilization(force_n(v_ed), force_n(v_rd.max(1.0)))
        .annex(annex)
        .explanation(L(
            &format!("Edge shear: V_Ed={:.1} kN ≤ V_Rd,c={:.1} kN", v_ed / 1e3, v_rd / 1e3),
            &format!("Randausbruch Querkraft: V_Ed={:.1} kN ≤ V_Rd,c={:.1} kN", v_ed / 1e3, v_rd / 1e3),
        ));
    if v_ed > v_rd {
        let c_req = anchor.c1 * (v_ed / v_rd).powf(2.0 / 3.0);
        e = e.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()),
            length_m(anchor.c1), length_m(c_req),
            L(&format!("Increase edge distance c1 to ≥ {:.0} mm.", c_req * 1000.0), &format!("Randabstand c1 auf ≥ {:.0} mm erhöhen.", c_req * 1000.0)),
        ));
    }
    out.push(e.build());

    let n_rd_t = n_rd_s.min(n_rd_c).min(n_rd_p).min(n_rd_sp);
    let v_rd_t = v_rd.min(n_rd_pr);
    let inter = part_4::interaction_utilization(n_ed, n_rd_t, v_ed, v_rd_t);
    let mut ix = CheckResult::assess(format!("en1992-4.interaction.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.3", "7.2.3"), SubjectRef::new(&anchor.id, format!("anchors[id={}]", anchor.id), label.clone()), L("Tension–shear interaction", "Zug–Querkraft-Interaktion"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, inter), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(L(
            &format!("Interaction: (N/N_Rd)^1.5+(V/V_Rd)^1.5={:.3} ≤ 1 (N_Ed={:.1} kN, V_Ed={:.1} kN)", inter, n_ed / 1e3, v_ed / 1e3),
            &format!("Interaktion: (N/N_Rd)^1,5+(V/V_Rd)^1,5={:.3} ≤ 1 (N_Ed={:.1} kN, V_Ed={:.1} kN)", inter, n_ed / 1e3, v_ed / 1e3),
        ));
    if inter > 1.0 {
        ix = ix.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label),
            area_m2(anchor.a_s), area_m2(anchor.a_s * inter),
            L("Increase anchor capacity for tension–shear interaction.", "Dübeltragfähigkeit für Zug–Querkraft-Interaktion erhöhen."),
        ));
    }
    out.push(ix.build());
    out
}

//#endregion 🔖️MemberEvaluate

/// 🧭 Field metadata hook for structured inputs editor (B2).
/// 🏷️ Prefer [`crate::field_meta::en1992_field_meta`] (B2 NormFieldMetaFn).
pub fn field_metadata() -> &'static [(&'static str, &'static str, &'static str, &'static str)] {
    &[]
}

#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
