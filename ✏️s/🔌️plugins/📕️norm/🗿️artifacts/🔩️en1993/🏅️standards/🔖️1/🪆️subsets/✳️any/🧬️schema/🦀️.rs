//! 🧬️ En1993 artifact schema — hierarchical steel-structure subject + compliance helpers.

use crate::{
    BridgeFatigue, ColdFormedMember, CraneRunway, FatigueDetail, FireExposure, LoadCase, MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial,
    SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg,
};
use crate::document::AnnexChoice;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full En1993 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub materials: Vec<SteelMaterial>,
    #[state(artifact)]
    pub sections: Vec<SteelSection>,
    #[state(artifact)]
    pub members: Vec<SteelMember>,
    #[state(artifact)]
    pub load_cases: Vec<LoadCase>,
    #[state(artifact)]
    pub member_actions: Vec<MemberAction>,
    #[state(artifact)]
    pub joints: Vec<SteelJoint>,
    #[state(artifact)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[state(artifact)]
    pub fire_exposures: Vec<FireExposure>,
    #[state(artifact)]
    pub cold_formed_members: Vec<ColdFormedMember>,
    #[state(artifact)]
    pub plated_panels: Vec<PlatedPanel>,
    #[state(artifact)]
    pub silo_shells: Vec<SiloShell>,
    #[state(artifact)]
    pub tension_components: Vec<TensionComponent>,
    #[state(artifact)]
    pub bridge_fatigue: Vec<BridgeFatigue>,
    #[state(artifact)]
    pub tower_legs: Vec<TowerLeg>,
    #[state(artifact)]
    pub piles: Vec<SteelPile>,
    #[state(artifact)]
    pub crane_runways: Vec<CraneRunway>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1993Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::En1993Snapshot {
        crate::En1993Snapshot {
            annex: self.annex,
            materials: self.materials.clone(),
            sections: self.sections.clone(),
            members: self.members.clone(),
            load_cases: self.load_cases.clone(),
            member_actions: self.member_actions.clone(),
            joints: self.joints.clone(),
            fatigue_details: self.fatigue_details.clone(),
            fire_exposures: self.fire_exposures.clone(),
            cold_formed_members: self.cold_formed_members.clone(),
            plated_panels: self.plated_panels.clone(),
            silo_shells: self.silo_shells.clone(),
            tension_components: self.tension_components.clone(),
            bridge_fatigue: self.bridge_fatigue.clone(),
            tower_legs: self.tower_legs.clone(),
            piles: self.piles.clone(),
            crane_runways: self.crane_runways.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::En1993Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            materials: snapshot.materials,
            sections: snapshot.sections,
            members: snapshot.members,
            load_cases: snapshot.load_cases,
            member_actions: snapshot.member_actions,
            joints: snapshot.joints,
            fatigue_details: snapshot.fatigue_details,
            fire_exposures: snapshot.fire_exposures,
            cold_formed_members: snapshot.cold_formed_members,
            plated_panels: snapshot.plated_panels,
            silo_shells: snapshot.silo_shells,
            tension_components: snapshot.tension_components,
            bridge_fatigue: snapshot.bridge_fatigue,
            tower_legs: snapshot.tower_legs,
            piles: snapshot.piles,
            crane_runways: snapshot.crane_runways,
        }
    }

    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::En1993Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1993` — twenty handcrafted schema leaves.
pub fn en1993_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1993",
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
    use crate::{En1993Diff, En1993Mutation, En1993Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1993BuilderConstruction {
        snapshot: En1993Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1993BuilderConstruction {
        type Snapshot = En1993Snapshot;
        type Mutation = En1993Mutation;
        type Diff = En1993Diff;
        fn empty() -> Self {
            Self { snapshot: En1993Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1993Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1993Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1993Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1993Parts {
        pub snapshot: Option<En1993Snapshot>,
    }

    pub struct En1993AnalyzerAnalysis;

    impl ArtifactAnalysis for En1993AnalyzerAnalysis {
        type Parts = En1993Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1993", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1993Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1993Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1993Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1993BuilderFacets {
        construction: En1993BuilderConstruction,
        analysis: En1993AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1993ComposerComposition,
    }
    builder: En1993Builder,
    analyzer: En1993Analyzer,
    composer: En1993Composer,
);
//#endregion 🧬️DerivedArtifactFacets


//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1993 compliance helpers — hierarchical subject, computed χ/χ_LT, remedies.
use crate::document::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::snapshot::rolled_heb_catalogue;
use crate::En1993Snapshot;

/// 🇩🇪 EN vs DIN EN NA γ factors for EN 1993.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub choice: AnnexChoice,
    pub gamma_m0: f64,
    pub gamma_m1: f64,
    pub gamma_m2: f64,
    pub gamma_m3: f64,
    pub gamma_mf: f64,
}

/// 📐️ Shared EN 1993-1-1 partial factors (also published by catalogue `reference_tables`).
pub const GAMMA_M0_EN: f64 = 1.0;
pub const GAMMA_M1_EN: f64 = 1.0;
pub const GAMMA_M2_EN: f64 = 1.25;
pub const GAMMA_M3_EN: f64 = 1.25;
pub const GAMMA_MF_EN: f64 = 1.15;
pub const GAMMA_M0_DE: f64 = 1.0;
pub const GAMMA_M1_DE: f64 = 1.1;
pub const GAMMA_M2_DE: f64 = 1.25;
pub const GAMMA_M3_DE: f64 = 1.25;
pub const GAMMA_MF_DE: f64 = 1.15;
/// 🧮 EN 1990 STR combination factors (identical EN recommended / DE NA).
pub const GAMMA_G_STR: f64 = 1.35;
pub const GAMMA_Q_STR: f64 = 1.50;
pub const XI_STR: f64 = 0.85;

impl AnnexParams {
    /// 🇪🇺 EN recommended values.
    pub fn en() -> Self {
        Self { choice: AnnexChoice::En, gamma_m0: GAMMA_M0_EN, gamma_m1: GAMMA_M1_EN, gamma_m2: GAMMA_M2_EN, gamma_m3: GAMMA_M3_EN, gamma_mf: GAMMA_MF_EN }
    }

    /// 🇩🇪 DIN EN 1993-1-1/NA: γ_M1 = 1.1.
    pub fn de() -> Self {
        Self { choice: AnnexChoice::De, gamma_m0: GAMMA_M0_DE, gamma_m1: GAMMA_M1_DE, gamma_m2: GAMMA_M2_DE, gamma_m3: GAMMA_M3_DE, gamma_mf: GAMMA_MF_DE }
    }

    pub fn for_choice(choice: AnnexChoice) -> Self {
        match choice {
            AnnexChoice::En => Self::en(),
            AnnexChoice::De => Self::de(),
        }
    }
}

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn subject_member(member: &SteelMember, path: &str) -> SubjectRef {
    let de_label = format!("Bauteil {} ({})", member.id, member.label);
    SubjectRef::new(&member.id, path, loc(&member.label, &de_label))
}

fn subject_joint(joint: &SteelJoint, path: &str) -> SubjectRef {
    SubjectRef::new(&joint.id, path, loc(&format!("Joint {}", joint.id), &format!("Anschluss {}", joint.id)))
}

fn force_n(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}

fn moment_nm(m: f64) -> Quantity {
    Quantity::new(QuantityKind::Moment, m)
}

fn stress_pa(s: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, s)
}

fn length_m(l: f64) -> Quantity {
    Quantity::length_m(l)
}

fn area_m2(a: f64) -> Quantity {
    Quantity::area_m2(a)
}

fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

fn temperature_c(t: f64) -> Quantity {
    Quantity::new(QuantityKind::Temperature, t)
}

pub mod part_1_1 {
    use super::*;

    /// 📏️ ε = √(235/fy_MPa).
    pub fn epsilon(fy_pa: f64) -> f64 {
        (235.0 / (fy_pa / 1.0e6)).sqrt()
    }

    /// 🏷️ Flange class Table 5.2 (outstand in compression); c,t in m.
    pub fn flange_class(c: f64, t: f64, fy_pa: f64) -> u8 {
        let eps = epsilon(fy_pa);
        let ratio = c / t;
        if ratio <= 9.0 * eps {
            1
        } else if ratio <= 10.0 * eps {
            2
        } else if ratio <= 14.0 * eps {
            3
        } else {
            4
        }
    }

    /// 🏷️ Web class Table 5.2 (web in bending).
    pub fn web_class(c: f64, t: f64, fy_pa: f64) -> u8 {
        let eps = epsilon(fy_pa);
        let ratio = c / t;
        if ratio <= 72.0 * eps {
            1
        } else if ratio <= 83.0 * eps {
            2
        } else if ratio <= 124.0 * eps {
            3
        } else {
            4
        }
    }

    /// 🏷️ Governing rolled I section class from geometry.
    pub fn section_class_rolled_i(section: &SteelSection, fy_pa: f64) -> u8 {
        let c_flange = (section.b - section.tw - 2.0 * section.r) / 2.0;
        let c_web = section.h - 2.0 * section.tf - 2.0 * section.r;
        flange_class(c_flange, section.tf, fy_pa).max(web_class(c_web, section.tw, fy_pa))
    }


    /// 📐️ EN 1993-1-5 §4.4 plate buckling reduction ρ for class 4 outstand/internal.
    pub fn plate_rho(lambda_p: f64, psi: f64) -> f64 {
        if lambda_p <= 0.673 {
            1.0
        } else {
            let num = lambda_p - 0.055 * (3.0 + psi);
            (num / (lambda_p * lambda_p)).min(1.0).max(0.0)
        }
    }

    /// 📐️ Effective area for class 4 rolled I (flange reduction dominant).
    pub fn effective_area_class4(section: &SteelSection, fy_pa: f64) -> f64 {
        let epsilon = (235e6 / fy_pa).sqrt();
        let c = (section.b - section.tw - 2.0 * section.r).max(1e-6) / 2.0;
        let lambda_p = (c / section.tf) / (28.0 * epsilon);
        let rho = plate_rho(lambda_p, 1.0);
        let a_flange = 2.0 * section.b * section.tf;
        let a_web = section.h.max(section.tw) * section.tw; // approx
        section.area - a_flange * (1.0 - rho).max(0.0) * 0.5
    }

    /// 📐️ Effective W_el,y for class 4 (reduced flanges).
    pub fn effective_w_el_y_class4(section: &SteelSection, fy_pa: f64) -> f64 {
        let a_eff = effective_area_class4(section, fy_pa);
        section.w_el_y * (a_eff / section.area.max(1e-12)).min(1.0)
    }


    /// 📐️ N_Rd = A·fy/γ_M0 [N].
    pub fn axial_resistance_n(area: f64, fy_pa: f64, params: AnnexParams) -> f64 {
        area * fy_pa / params.gamma_m0
    }

    /// 📐️ M_c,Rd = W_pl·fy/γ_M0 for class ≤ 2 [N·m].
    pub fn bending_resistance_nm(w_pl: f64, fy_pa: f64, class: u8, params: AnnexParams) -> f64 {
        let w = if class <= 2 { w_pl } else { w_pl }; // class 3 uses Wel — caller selects
        w * fy_pa / params.gamma_m0
    }

    /// 📐️ V_pl,Rd = A_v·(fy/√3)/γ_M0 [N].
    pub fn shear_resistance_n(a_v: f64, fy_pa: f64, params: AnnexParams) -> f64 {
        a_v * (fy_pa / 3.0_f64.sqrt()) / params.gamma_m0
    }

    /// 📐️ Net tension N_t,Rd [N].
    pub fn net_tension_resistance_n(area: f64, area_net: f64, fy_pa: f64, fu_pa: f64, params: AnnexParams) -> f64 {
        let gross = area * fy_pa / params.gamma_m0;
        let net = 0.9 * area_net * fu_pa / params.gamma_m2;
        gross.min(net)
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BucklingCurve {
        A0,
        A,
        B,
        C,
        D,
    }

    impl BucklingCurve {
        /// 📉 Imperfection factor α Table 6.1.
        pub fn alpha(self) -> f64 {
            match self {
                Self::A0 => 0.13,
                Self::A => 0.21,
                Self::B => 0.34,
                Self::C => 0.49,
                Self::D => 0.76,
            }
        }
    }

    /// 📉 Buckling curve for rolled I per Table 6.2 (h/b, axis, fy).
    pub fn buckling_curve_rolled_i(section: &SteelSection, axis: char, fy_pa: f64) -> BucklingCurve {
        let hb = section.h / section.b;
        if fy_pa > 460.0e6 {
            return if axis == 'y' { BucklingCurve::A0 } else { BucklingCurve::A0 };
        }
        if hb > 1.2 {
            if axis == 'y' {
                if section.tf <= 0.040 {
                    BucklingCurve::A
                } else {
                    BucklingCurve::B
                }
            } else if section.tf <= 0.040 {
                BucklingCurve::B
            } else {
                BucklingCurve::C
            }
        } else if axis == 'y' {
            if section.tf <= 0.100 {
                BucklingCurve::B
            } else {
                BucklingCurve::D
            }
        } else if section.tf <= 0.100 {
            BucklingCurve::C
        } else {
            BucklingCurve::D
        }
    }

    /// 📉️ χ from λ̄ and curve (Eq. 6.49).
    pub fn chi(lambda_bar: f64, curve: BucklingCurve) -> f64 {
        if lambda_bar <= 0.2 {
            return 1.0;
        }
        let alpha = curve.alpha();
        let phi = 0.5 * (1.0 + alpha * (lambda_bar - 0.2) + lambda_bar * lambda_bar);
        (1.0 / (phi + (phi * phi - lambda_bar * lambda_bar).max(0.0).sqrt())).min(1.0)
    }

    /// 📉️ λ̄ = L_cr/(i·λ_1) with λ_1 = π·√(E/fy).
    pub fn lambda_bar(l_cr: f64, i: f64, e_pa: f64, fy_pa: f64) -> f64 {
        let lambda_1 = std::f64::consts::PI * (e_pa / fy_pa).sqrt();
        (l_cr / i) / lambda_1
    }

    /// 📉️ Radius of gyration.
    pub fn radius_of_gyration(i_moment: f64, area: f64) -> f64 {
        (i_moment / area).sqrt()
    }

    /// 📉️ N_b,Rd = χ·A·fy/γ_M1 [N].
    pub fn buckling_resistance_n(area: f64, fy_pa: f64, chi: f64, params: AnnexParams) -> f64 {
        chi * area * fy_pa / params.gamma_m1
    }

    /// 🔄 M_cr for rolled I (general method / 6.3.2.3 simplified).
    pub fn m_cr_rolled_nm(section: &SteelSection, material: &SteelMaterial, l_lt: f64, psi: f64, load_app: &str, moment_diagram: &str) -> f64 {
        let e = material.e_modulus;
        let g = material.g_modulus;
        let c1_psi = if psi >= 1.0 {
            1.0
        } else if psi >= 0.0 {
            1.75 - 1.05 * psi + 0.3 * psi * psi
        } else {
            (1.75 - 1.05 * psi + 0.3 * psi * psi).min(2.5)
        };
        let c1 = match moment_diagram {
            "uniform" => 1.13,
            "parabolic" => 1.35,
            "linear" => c1_psi.min(2.5),
            _ => c1_psi.min(2.5),
        };
        let zg = match load_app {
            "topFlange" => section.h / 2.0,
            "bottomFlange" => -section.h / 2.0,
            _ => 0.0,
        };
        let term = (std::f64::consts::PI * std::f64::consts::PI * e * section.iz / (l_lt * l_lt)).max(1.0);
        let m_cr0 = c1 * term.sqrt() * ((g * section.it) + (term * section.iw / section.iz)).sqrt();
        let kg = if zg.abs() < 1e-12 { 1.0 } else { 1.0 / (1.0 + (zg / (section.h)).abs() * 0.5).max(0.5) };
        m_cr0 * kg
    }


    /// 📉 LTB curve Table 6.4 (rolled/welded I) — DE-NA uses same selection; §6.3.2.3 special case optional.
    pub fn ltb_curve_table_6_4(section: &SteelSection) -> BucklingCurve {
        let hb = section.h / section.b.max(1e-9);
        let welded = section.kind.to_ascii_lowercase().contains("weld");
        if welded {
            if hb <= 2.0 { BucklingCurve::C } else { BucklingCurve::D }
        } else if hb <= 2.0 {
            BucklingCurve::A
        } else {
            BucklingCurve::B
        }
    }

    /// 📉 χ_LT with Table 6.4 curve (replaces hardcoded curve b).
    /// 📉 χ_LT from λ̄_LT (Eq. 6.56).
    pub fn chi_lt(lambda_lt: f64, curve: BucklingCurve) -> f64 {
        chi(lambda_lt, curve)
    }

    /// 📉 λ̄_LT = √(W_y·fy/M_cr).
    pub fn lambda_lt(w_y: f64, fy_pa: f64, m_cr: f64) -> f64 {
        if m_cr <= 0.0 {
            return 10.0;
        }
        (w_y * fy_pa / m_cr).sqrt()
    }

    /// 📐️ M_b,Rd = χ_LT·W_y·fy/γ_M1 [N·m].
    pub fn ltb_resistance_nm(w_y: f64, fy_pa: f64, chi_lt: f64, params: AnnexParams) -> f64 {
        chi_lt * w_y * fy_pa / params.gamma_m1
    }

    /// 🔗 Interaction factors k_yy, k_yz, k_zy, k_zz Annex B (method 2, class ≤ 3).
    pub fn interaction_kij(n_ed: f64, n_rk: f64, lambda_y: f64, lambda_z: f64, psi: f64, class: u8) -> (f64, f64, f64, f64) {
        let n_ratio = if n_rk.abs() < 1e-9 { 0.0 } else { (n_ed / n_rk).abs() };
        let cm = 0.6 + 0.4 * psi;
        let cm = cm.max(0.4);
        let kyy = if class >= 4 {
            cm * (1.0 + 0.6 * lambda_y * n_ratio)
        } else {
            cm * (1.0 + (lambda_y - 0.2) * n_ratio).min(cm * (1.0 + 0.8 * n_ratio))
        };
        let kzz = if class >= 4 {
            cm * (1.0 + 0.6 * lambda_z * n_ratio)
        } else {
            cm * (1.0 + (lambda_z - 0.2) * n_ratio).min(cm * (1.0 + 0.8 * n_ratio))
        };
        let kyz = 0.6 * kzz;
        let kzy = 0.6 * kyy;
        (kyy.max(0.0), kyz.max(0.0), kzy.max(0.0), kzz.max(0.0))
    }

    /// 🔗 η from Eqs. 6.61 / 6.62.
    pub fn interaction_eta(n_ed: f64, n_b_rd_y: f64, n_b_rd_z: f64, my_ed: f64, mz_ed: f64, m_rd_y: f64, m_rd_z: f64, kyy: f64, kyz: f64, kzy: f64, kzz: f64) -> (f64, f64) {
        let eta61 = (n_ed / n_b_rd_y).abs() + kyy * (my_ed / m_rd_y).abs() + kyz * (mz_ed / m_rd_z).abs();
        let eta62 = (n_ed / n_b_rd_z).abs() + kzy * (my_ed / m_rd_y).abs() + kzz * (mz_ed / m_rd_z).abs();
        (eta61, eta62)
    }

    /// 📉 Reduced moment for high shear (§6.2.8): if V_Ed > 0.5 V_pl,Rd reduce M.
    pub fn shear_reduced_m_rd(m_rd: f64, v_ed: f64, v_rd: f64) -> f64 {
        if v_rd <= 0.0 {
            return m_rd;
        }
        let rho = ((2.0 * v_ed / v_rd) - 1.0).max(0.0).powi(2);
        if v_ed > 0.5 * v_rd {
            m_rd * (1.0 - rho)
        } else {
            m_rd
        }
    }


    /// 🔗 §6.2.9 class ≤2 doubly-symmetric I: n, a, M_N,y,Rd, M_N,z,Rd and bi-axial η (6.31–6.41).
    pub fn mn_interaction_eta(
        n_ed: f64,
        my_ed: f64,
        mz_ed: f64,
        n_pl_rd: f64,
        m_pl_y_rd: f64,
        m_pl_z_rd: f64,
        area: f64,
        b: f64,
        tf: f64,
        class: u8,
    ) -> f64 {
        if n_pl_rd <= 0.0 || m_pl_y_rd <= 0.0 {
            return 0.0;
        }
        if class >= 3 {
            return (n_ed / n_pl_rd).abs() + (my_ed / m_pl_y_rd).abs() + (mz_ed / m_pl_z_rd.max(1e-9)).abs();
        }
        let n = (n_ed / n_pl_rd).abs();
        let a = ((area - 2.0 * b * tf) / area).clamp(0.0, 0.5);
        let m_n_y = if n <= 1e-9 {
            m_pl_y_rd
        } else {
            (m_pl_y_rd * (1.0 - n) / (1.0 - 0.5 * a)).min(m_pl_y_rd)
        };
        let m_n_z = if n <= a {
            m_pl_z_rd
        } else if (1.0 - a).abs() < 1e-9 {
            0.0
        } else {
            m_pl_z_rd * (1.0 - ((n - a) / (1.0 - a)).powi(2))
        };
        let alpha = 2.0;
        let beta = (5.0 * n).max(1.0);
        if m_n_y <= 1e-12 || m_n_z <= 1e-12 {
            return n + (my_ed / m_pl_y_rd).abs() + (mz_ed / m_pl_z_rd.max(1e-9)).abs();
        }
        (my_ed.abs() / m_n_y).powf(alpha) + (mz_ed.abs() / m_n_z).powf(beta)
    }

    /// 📏 Elastic midspan deflection δ = 5·q·L⁴/(384·E·I) for UDL equivalent from M = qL²/8.
    pub fn deflection_from_moment(m_ed: f64, length: f64, e: f64, iy: f64) -> f64 {
        if iy <= 0.0 || e <= 0.0 || length <= 0.0 {
            return 0.0;
        }
        let q = 8.0 * m_ed / (length * length);
        5.0 * q * length.powi(4) / (384.0 * e * iy)
    }
}

pub mod part_1_2 {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }

    pub fn parse_rating(value: &str) -> FireRating {
        match value.to_ascii_lowercase().as_str() {
            "r30" => FireRating::R30,
            "r90" => FireRating::R90,
            "r120" => FireRating::R120,
            _ => FireRating::R60,
        }
    }

    pub fn rating_minutes(rating: FireRating) -> f64 {
        match rating {
            FireRating::R30 => 30.0,
            FireRating::R60 => 60.0,
            FireRating::R90 => 90.0,
            FireRating::R120 => 120.0,
        }
    }

    /// 🔥 θ_a,cr from μ0 (Eq. 4.22) [°C].
    pub fn critical_temperature_c(mu0: f64) -> f64 {
        let mu = mu0.clamp(0.02, 1.0);
        39.19 * (1.0 / 0.9674 / mu.powf(3.833) - 1.0).ln() + 482.0
    }

    /// 🔥 k_y,θ Table 3.1 (carbon steel).
    pub fn k_y_theta(theta: f64) -> f64 {
        let t = theta;
        if t < 100.0 { 1.0 }
        else if t < 200.0 { 1.0 }
        else if t < 300.0 { 1.0 }
        else if t < 400.0 { 1.0 }
        else if t < 500.0 { 0.78 }
        else if t < 600.0 { 0.47 }
        else if t < 700.0 { 0.23 }
        else if t < 800.0 { 0.11 }
        else if t < 900.0 { 0.06 }
        else if t < 1000.0 { 0.04 }
        else if t < 1100.0 { 0.02 }
        else { 0.0 }
    }

    /// 🔥 k_E,θ Table 3.1.
    pub fn k_e_theta(theta: f64) -> f64 {
        let t = theta;
        if t < 100.0 { 1.0 }
        else if t < 200.0 { 0.9 }
        else if t < 300.0 { 0.8 }
        else if t < 400.0 { 0.7 }
        else if t < 500.0 { 0.6 }
        else if t < 600.0 { 0.31 }
        else if t < 700.0 { 0.13 }
        else if t < 800.0 { 0.09 }
        else if t < 900.0 { 0.0675 }
        else if t < 1000.0 { 0.045 }
        else if t < 1100.0 { 0.0225 }
        else { 0.0 }
    }

    /// 🔥 χ_fi §4.2.3.2 with λ̄_θ and α = 0.65√(235/f_y).
    pub fn chi_fi(lambda_bar_theta: f64, fy_pa: f64) -> f64 {
        let alpha = 0.65 * (235.0 / (fy_pa / 1e6).max(1.0)).sqrt();
        if lambda_bar_theta <= 0.2 {
            return 1.0;
        }
        let phi = 0.5 * (1.0 + alpha * lambda_bar_theta + lambda_bar_theta * lambda_bar_theta);
        (1.0 / (phi + (phi * phi - lambda_bar_theta * lambda_bar_theta).max(0.0).sqrt())).min(1.0)
    }

    fn iso_gas_temperature(t_min: f64) -> f64 {
        20.0 + 345.0 * (8.0 * t_min.max(0.0) + 1.0).log10()
    }

    fn net_heat_flux(theta_g: f64, theta_a: f64) -> f64 {
        let alpha_c = 25.0;
        let phi = 1.0;
        let sigma = 5.67e-8;
        let epsilon = 0.7;
        alpha_c * (theta_g - theta_a) + phi * epsilon * sigma * (((theta_g + 273.0).powi(4)) - ((theta_a + 273.0).powi(4)))
    }

    /// 🔥 §4.2.5 incremental steel temperature [°C] — unprotected (d_p≤0) or protected.
    pub fn steel_temperature_c(
        rating: FireRating,
        section_factor: f64,
        d_p: f64,
        lambda_p: f64,
        rho_p: f64,
        c_p: f64,
    ) -> f64 {
        let t_end = rating_minutes(rating);
        let c_a = 600.0;
        let rho_a = 7850.0;
        let dt = 0.5_f64;
        let mut theta_a = 20.0;
        let mut t = 0.0;
        let am_v = section_factor.max(1.0);
        let k_sh = 1.0;
        while t < t_end - 1e-9 {
            let theta_g = iso_gas_temperature(t);
            let h_net = net_heat_flux(theta_g, theta_a);
            let d_theta = if d_p <= 1e-6 {
                k_sh * am_v / (c_a * rho_a) * h_net * dt
            } else {
                let phi = (c_p * rho_p / (c_a * rho_a)) * d_p * am_v;
                let theta_g_next = iso_gas_temperature(t + dt);
                let d_theta_g = theta_g_next - theta_g;
                let factor = (lambda_p / d_p) / (c_a * rho_a) * am_v * (1.0 / (1.0 + phi / 3.0));
                factor * (theta_g - theta_a) * dt - (phi.exp().powf(0.1) - 1.0) * d_theta_g
            };
            theta_a = (theta_a + d_theta.max(-50.0)).clamp(20.0, 1200.0);
            t += dt;
        }
        theta_a
    }

    /// 🔥 Required protection thickness [m] so θ_a(t) ≤ θ_cr (bounded search on d_p).
    pub fn required_protection_thickness(
        rating: FireRating,
        section_factor: f64,
        mu0: f64,
        lambda_p: f64,
        rho_p: f64,
        c_p: f64,
    ) -> f64 {
        let theta_cr = critical_temperature_c(mu0);
        if steel_temperature_c(rating, section_factor, 0.0, lambda_p, rho_p, c_p) <= theta_cr {
            return 0.0;
        }
        let mut lo = 0.0;
        let mut hi = 0.080;
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if steel_temperature_c(rating, section_factor, mid, lambda_p, rho_p, c_p) <= theta_cr {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        hi
    }
}

pub mod part_1_3 {
    /// ❄️ λ_p for cold-formed flat (b̄,t in m, fy in Pa).
    pub fn lambda_p(b_bar: f64, t: f64, fy_pa: f64, k_sigma: f64) -> f64 {
        let eps = (235.0 / (fy_pa / 1e6)).sqrt();
        (b_bar / t) / (28.4 * eps * k_sigma.sqrt())
    }

    pub fn reduction_factor(lambda_p: f64, psi: f64) -> f64 {
        if lambda_p <= 0.673 {
            1.0
        } else {
            let rho = (lambda_p - 0.055 * (3.0 + psi)) / (lambda_p * lambda_p);
            rho.clamp(0.0, 1.0)
        }
    }
}

pub mod part_1_4 {
    use super::AnnexParams;
    /// ✨ Stainless γ_M0 = 1.1 (EN 1993-1-4).
    pub fn bending_resistance_nm(w_pl: f64, fy_pa: f64) -> f64 {
        let params = AnnexParams { choice: super::AnnexChoice::En, gamma_m0: 1.1, gamma_m1: 1.1, gamma_m2: 1.25, gamma_m3: 1.25, gamma_mf: 1.15 };
        w_pl * fy_pa / params.gamma_m0
    }
}

pub mod part_1_5 {
    use super::AnnexParams;
    /// 🧱 λ_p from plate geometry (a,b,t in m).
    pub fn lambda_p(b: f64, t: f64, fy_pa: f64, k_sigma: f64) -> f64 {
        let eps = (235.0 / (fy_pa / 1e6)).sqrt();
        (b / t) / (28.4 * eps * k_sigma.sqrt())
    }

    pub fn plate_reduction_factor(lambda_p: f64) -> f64 {
        if lambda_p <= 0.673 {
            1.0
        } else {
            ((lambda_p - 0.055) / (lambda_p * lambda_p)).min(1.0).max(0.0)
        }
    }

    pub fn local_buckling_stress_rd(fy_pa: f64, lambda_p: f64, params: AnnexParams) -> f64 {
        plate_reduction_factor(lambda_p) * fy_pa / params.gamma_m0
    }
}

pub mod part_1_6 {
    use super::AnnexParams;
    pub fn sigma_x_rcr(t: f64, r: f64, e_pa: f64) -> f64 {
        0.605 * e_pa * t / r
    }

    pub fn lambda_bar(fy_pa: f64, sigma_rcr: f64) -> f64 {
        (fy_pa / sigma_rcr).sqrt()
    }

    pub fn alpha_imperfection(r: f64, t: f64) -> f64 {
        0.62 / (1.0 + 1.91 * (r / t / 400.0).powf(1.44)).powf(0.6)
    }

    pub fn chi(lambda_bar: f64, alpha: f64) -> f64 {
        if lambda_bar <= 0.2 {
            1.0
        } else {
            let phi = 0.5 * (1.0 + alpha * (lambda_bar - 0.2) + lambda_bar * lambda_bar);
            (1.0 / (phi + (phi * phi - lambda_bar * lambda_bar).max(0.0).sqrt())).min(1.0)
        }
    }

    pub fn design_resistance(fy_pa: f64, chi: f64, params: AnnexParams) -> f64 {
        chi * fy_pa / params.gamma_m1
    }
}

pub mod part_1_8 {
    /// 🔩 Bolt tensile stress area from diameter [m] → m² (approx ISO coarse).
    pub fn bolt_as(d: f64) -> f64 {
        let d_mm = d * 1000.0;
        let as_mm2 = match d_mm.round() as i32 {
            12 => 84.3,
            16 => 157.0,
            20 => 245.0,
            22 => 303.0,
            24 => 353.0,
            27 => 459.0,
            30 => 561.0,
            _ => 0.785 * (d_mm - 0.9382 * 1.0_f64.max(d_mm * 0.1)).powi(2),
        };
        as_mm2 * 1e-6
    }

    pub fn bolt_fub(class: &str) -> f64 {
        match class {
            "10.9" => 1000.0e6,
            "12.9" => 1200.0e6,
            _ => 800.0e6,
        }
    }

    pub fn bolt_shear_resistance_n(n_bolts: u32, a_s: f64, f_ub: f64, n_planes: u32, gamma_m2: f64) -> f64 {
        let alpha_v = 0.6;
        n_bolts as f64 * n_planes as f64 * alpha_v * a_s * f_ub / gamma_m2
    }

    pub fn bearing_alpha_b(e1: f64, p1: f64, d0: f64, f_ub: f64, f_u: f64) -> f64 {
        let from_e = e1 / (3.0 * d0);
        let from_p = p1 / (3.0 * d0) - 0.25;
        from_e.min(from_p).min(f_ub / f_u).min(1.0).max(0.0)
    }

    pub fn bearing_k1(e2: f64, p2: f64, d0: f64) -> f64 {
        let from_e = 2.8 * e2 / d0 - 1.7;
        let from_p = 1.4 * p2 / d0 - 1.7;
        from_e.min(from_p).min(2.5).max(0.0)
    }

    pub fn bolt_bearing_resistance_n(k1: f64, alpha_b: f64, f_u: f64, d: f64, t: f64, n_bolts: u32, gamma_m2: f64) -> f64 {
        n_bolts as f64 * k1 * alpha_b * f_u * d * t / gamma_m2
    }

    pub fn bolt_tension_resistance_n(n_bolts: u32, a_s: f64, f_ub: f64, gamma_m2: f64) -> f64 {
        n_bolts as f64 * 0.9 * a_s * f_ub / gamma_m2
    }

    pub fn beta_w(grade: &str) -> f64 {
        match grade {
            "S235" => 0.8,
            "S275" => 0.85,
            "S355" | "S420" | "S460" => 0.9,
            _ => 0.9,
        }
    }


    /// 🔩 Preload F_p,C = 0.7 f_ub A_s (EN 1993-1-8 §3.9) when not declared.
    pub fn preload_force_n(a_s: f64, f_ub: f64, declared: f64) -> f64 {
        if declared > 0.0 { declared } else { 0.7 * f_ub * a_s }
    }

    /// 🔩 Slip resistance F_s,Rd = k_s n μ F_p,C / γ_M3 (§3.9).
    pub fn slip_resistance_n(k_s: f64, n_surfaces: u32, mu: f64, f_p_c: f64, n_bolts: u32, gamma_m3: f64) -> f64 {
        k_s * n_surfaces as f64 * mu * f_p_c * n_bolts as f64 / gamma_m3.max(1e-9)
    }

    /// 🔧 Simplified fillet weld F_w,Rd.
    pub fn fillet_weld_simplified_n(a: f64, length: f64, f_u: f64, beta_w: f64, gamma_m2: f64) -> f64 {
        a * length * f_u / (beta_w * 3.0_f64.sqrt() * gamma_m2)
    }

    /// 🔧 Directional method — resultant of σ⊥, τ⊥, τ∥ with equal share assumption.
    pub fn fillet_weld_directional_n(a: f64, length: f64, f_u: f64, beta_w: f64, gamma_m2: f64, f_ed: f64) -> (f64, f64) {
        let area = a * length;
        if area <= 0.0 {
            return (f_ed, 0.0);
        }
        let sigma_perp = f_ed / area / 2.0_f64.sqrt();
        let tau_perp = f_ed / area / 2.0_f64.sqrt();
        let tau_par: f64 = 0.0;
        let equiv = (sigma_perp.powi(2) + 3.0 * (tau_perp.powi(2) + tau_par.powi(2))).sqrt();
        let limit = f_u / (beta_w * gamma_m2);
        (equiv * area, limit * area)
    }
}

pub mod part_1_9 {
    pub fn detail_category_pa(category: u8) -> f64 {
        match category {
            36 | 40 | 45 | 50 | 56 | 63 | 71 | 80 | 90 | 100 | 112 | 125 | 140 | 160 => category as f64 * 1e6,
            _ => 71.0e6,
        }
    }

    pub enum AssessmentMethod {
        DamageTolerant,
        SafeLife,
        LowConsequence,
    }


    /// 🔄 Palmgren-Miner damage with tri-linear Δσ_C/Δσ_D/Δσ_L (m=3 then m=5).
    pub fn miner_damage(spectrum: &[(f64, f64)], delta_c: f64, gamma_mf: f64) -> f64 {
        let delta_c = delta_c / gamma_mf.max(1e-9);
        let delta_d = delta_c * (2.0e6_f64 / 5.0e6).powf(1.0 / 3.0);
        let delta_l = delta_d * (5.0e6_f64 / 1.0e8).powf(1.0 / 5.0);
        let mut d = 0.0;
        for &(ds, n) in spectrum {
            if ds <= delta_l || n <= 0.0 {
                continue;
            }
            let n_r = if ds > delta_d {
                2.0e6 * (delta_c / ds).powi(3)
            } else {
                5.0e6 * (delta_d / ds).powi(5)
            };
            d += n / n_r.max(1e-12);
        }
        d
    }

    pub fn parse_method(value: &str) -> AssessmentMethod {
        match value.to_ascii_lowercase().as_str() {
            "safe_life" => AssessmentMethod::SafeLife,
            "low_consequence" => AssessmentMethod::LowConsequence,
            _ => AssessmentMethod::DamageTolerant,
        }
    }

    pub fn gamma_mf(method: AssessmentMethod) -> f64 {
        match method {
            AssessmentMethod::DamageTolerant => 1.0,
            AssessmentMethod::SafeLife => 1.15,
            AssessmentMethod::LowConsequence => 1.35,
        }
    }
}

pub mod part_1_10 {
    pub fn max_permissible_thickness(subgrade: &str, t_ed_c: f64) -> f64 {
        let idx = match subgrade.to_ascii_uppercase().as_str() {
            "JR" => 0,
            "J0" => 1,
            "J2" => 2,
            "K2" | "M" | "N" => 3,
            _ => 2,
        };
        // Table 2.1 bilinear simplified at σ_Ed = 0.75 fy
        let base_mm = [20.0, 30.0, 50.0, 70.0][idx];
        let adj = if t_ed_c < -20.0 { -10.0 } else if t_ed_c < 0.0 { -5.0 } else { 0.0 };
        (base_mm + adj) / 1000.0
    }
}

pub mod part_1_11 {
    pub fn tension_component_resistance_n(f_uk: f64, f_k: f64) -> f64 {
        (f_uk / 1.5).min(f_k)
    }
}

pub mod part_1_12 {
    use super::AnnexParams;
    pub fn elastic_bending_resistance_nm(w_el: f64, fy_pa: f64, class: u8, params: AnnexParams) -> f64 {
        if class > 3 {
            0.0
        } else {
            w_el * fy_pa / params.gamma_m0
        }
    }
}

pub mod part_2 {
    pub fn bridge_interaction_eta(n_ed: f64, n_rd: f64, m_ed: f64, m_rd: f64) -> f64 {
        (n_ed / n_rd).abs() + (m_ed / m_rd).abs()
    }

    pub fn damage_equivalent_stress(lambda: f64, phi2: f64, delta_sigma_p: f64) -> f64 {
        lambda * phi2 * delta_sigma_p
    }
}

pub mod part_3 {
    use super::AnnexParams;
    /// 🗼 EN 1993-3-1 Annex B — design buckling resistance reduced by force coefficient c_f and dynamic factor c_d.
    pub fn tower_buckling_n(area: f64, fy_pa: f64, chi: f64, force_coefficient: f64, dynamic_factor: f64, params: AnnexParams) -> f64 {
        let wind_amp = (force_coefficient.max(0.1) * dynamic_factor.max(0.1)).max(1.0);
        chi * area * fy_pa / params.gamma_m1 / wind_amp
    }
}

pub mod part_4 {
    /// 🛢️ Janssen horizontal pressure [Pa].
    pub fn janssen_pressure(k: f64, gamma: f64, depth: f64) -> f64 {
        // gamma is N/m³ (formerly kN/m³ * 1000)
        k * gamma * depth
    }

    pub fn membrane_hoop_stress(p_h: f64, r: f64, t: f64) -> f64 {
        p_h * r / t
    }
}

pub mod part_5 {
    use super::AnnexParams;
    pub fn pile_compression_n(area: f64, fy_pa: f64, k_red: f64, params: AnnexParams) -> f64 {
        k_red * area * fy_pa / params.gamma_m0
    }

    pub fn pile_driving_stress_limit(fy_pa: f64) -> f64 {
        0.9 * fy_pa
    }

    /// 🪵 EN 1993-5 §5.3 — driving-damage reduction from driving stress level; shaft geometry scales geotechnical factor.
    pub fn pile_soil_reduction(driving_stress: f64, fy_pa: f64, embedded_length: f64, shaft_perimeter: f64) -> f64 {
        let ratio = driving_stress / fy_pa.max(1.0);
        let drive = if ratio <= 0.6 {
            1.0
        } else if ratio <= 0.9 {
            1.0 - 0.25 * (ratio - 0.6) / 0.3
        } else {
            0.75
        };
        let geo = if embedded_length > 0.0 && shaft_perimeter > 0.0 {
            (0.85 + 0.02 * (embedded_length * shaft_perimeter).sqrt()).min(1.0)
        } else {
            1.0
        };
        drive * geo
    }
}

pub mod part_6 {
    pub fn effective_length(contact: f64, dispersion: f64) -> f64 {
        contact + 2.0 * dispersion
    }

    pub fn wheel_load_web_stress(f_z: f64, l_eff: f64, t_w: f64) -> f64 {
        f_z / (l_eff * t_w)
    }
}

fn next_section_options(current: &SteelSection, need_area: f64, need_wpl: f64) -> Vec<String> {
    let mut opts: Vec<String> = rolled_heb_catalogue()
        .into_iter()
        .filter(|s| s.area + 1e-12 >= need_area && s.w_pl_y + 1e-12 >= need_wpl && s.id != current.id)
        .map(|s| s.id)
        .collect();
    if opts.is_empty() {
        opts = rolled_heb_catalogue()
            .into_iter()
            .filter(|s| (s.area > current.area + 1e-12 || s.w_pl_y > current.w_pl_y + 1e-12) && s.id != current.id)
            .map(|s| s.id)
            .collect();
    }
    if opts.is_empty() {
        if let Some(best) = rolled_heb_catalogue().into_iter().max_by(|a, b| a.area.partial_cmp(&b.area).unwrap_or(std::cmp::Ordering::Equal)) {
            if best.id != current.id {
                opts.push(best.id);
            }
        }
    }
    opts
}

fn find_material<'a>(doc: &'a En1993Snapshot, id: &str) -> Option<&'a crate::SteelMaterial> {
    doc.materials.iter().find(|m| m.id == id)
}

fn find_section<'a>(doc: &'a En1993Snapshot, id: &str) -> Option<&'a SteelSection> {
    doc.sections.iter().find(|s| s.id == id)
}


/// 📋️ Full hierarchical EN 1993 evaluation with remedies.


/// ⚖️ EN 1990 ψ₀/ψ₁/ψ₂ by action category (Table A1.1) — DE NA diverges for snow.
fn psi_factors(category: &str, annex: AnnexChoice) -> (f64, f64, f64) {
    match (category, annex) {
        ("office" | "residential" | "congregation", _) => (0.7, 0.5, 0.3),
        ("shopping" | "storage", _) => (0.7, 0.7, 0.6),
        ("snow" | "snow_high", AnnexChoice::De) => (0.7, 0.2, 0.0),
        ("snow" | "snow_high", AnnexChoice::En) => (0.5, 0.2, 0.0),
        ("wind", _) => (0.6, 0.2, 0.0),
        ("temperature", _) => (0.6, 0.5, 0.0),
        ("self" | "permanent", _) => (1.0, 1.0, 1.0),
        ("accidental", _) => (1.0, 1.0, 1.0),
        _ => (0.7, 0.5, 0.3),
    }
}

/// 🧮 EN 1990 γ_G / γ_Q / ξ (Eq. 6.10a/b) — STR values identical for EN recommended and DE NA.
fn gamma_factors() -> (f64, f64, f64) {
    (GAMMA_G_STR, GAMMA_Q_STR, XI_STR)
}

/// 🧮 Combined scalar design effect from characteristic forces linked to load cases.
#[derive(Clone, Debug)]
struct CombinedScalar {
    combination_id: String,
    situation: String,
    value: f64,
}


/// 🧩 Normalize memberType (beam / column / beamColumn / brace / tie) for check gating.
fn member_kind(raw: &str) -> String {
    raw.to_ascii_lowercase().replace(['_', '-'], "")
}

fn kind_allows_flexural_buckling(kind: &str) -> bool {
    !matches!(kind, "beam" | "tie")
}

fn kind_allows_ltb(kind: &str) -> bool {
    !matches!(kind, "column" | "brace" | "tie")
}

fn kind_allows_bending(kind: &str) -> bool {
    !matches!(kind, "brace" | "tie")
}

fn kind_allows_compression(kind: &str) -> bool {
    kind != "tie"
}

fn kind_allows_member_interaction(kind: &str) -> bool {
    matches!(kind, "beamcolumn" | "column") || kind.is_empty()
}

fn combine_scalar_forces(document: &En1993Snapshot, forces: &[(String, f64)]) -> Vec<CombinedScalar> {
    let mut g = 0.0;
    let mut variables: Vec<(&crate::LoadCase, f64)> = Vec::new();
    for (lc_id, force) in forces {
        let Some(lc) = document.load_cases.iter().find(|l| l.id == *lc_id) else { continue };
        match lc.kind.as_str() {
            "permanent" | "prestress" => g += *force,
            _ => variables.push((lc, *force)),
        }
    }
    let (gamma_g, gamma_q, xi) = gamma_factors();
    let mut out = Vec::new();
    if variables.is_empty() {
        if g.abs() > 0.0 {
            out.push(CombinedScalar { combination_id: "uls-g".into(), situation: "uls".into(), value: gamma_g * g });
        }
        out.push(CombinedScalar { combination_id: "sls-char".into(), situation: "sls".into(), value: g });
        return out;
    }
    for (i, (lead, lead_v)) in variables.iter().enumerate() {
        let mut a = gamma_g * g + gamma_q * lead_v;
        let mut b = xi * gamma_g * g + gamma_q * lead_v;
        for (j, (other, ov)) in variables.iter().enumerate() {
            if i == j { continue; }
            let (psi0, _, _) = psi_factors(&other.category, document.annex);
            a += gamma_q * psi0 * ov;
            b += gamma_q * psi0 * ov;
        }
        out.push(CombinedScalar { combination_id: format!("uls-6.10a-{}", lead.id), situation: "uls".into(), value: a });
        out.push(CombinedScalar { combination_id: format!("uls-6.10b-{}", lead.id), situation: "uls".into(), value: b });
    }
    let (lead, lead_v) = &variables[0];
    let mut s = g + lead_v;
    for (j, (other, ov)) in variables.iter().enumerate() {
        if j == 0 { continue; }
        let (psi0, _, _) = psi_factors(&other.category, document.annex);
        s += psi0 * ov;
    }
    out.push(CombinedScalar { combination_id: format!("sls-char-{}", lead.id), situation: "sls".into(), value: s });
    out
}

fn governing_scalar<'a>(effects: &'a [CombinedScalar], sit: &str) -> Option<&'a CombinedScalar> {
    effects.iter().filter(|e| e.situation == sit).max_by(|a, b| a.value.abs().partial_cmp(&b.value.abs()).unwrap_or(std::cmp::Ordering::Equal))
}

/// 🧮 Combined design effects for one member (EN 1990 §6.4.3 + DE NA ψ).
#[derive(Clone, Debug)]
struct CombinedEffects {
    combination_id: String,
    situation: String,
    action: crate::DesignAction,
}

fn combine_member_actions(document: &En1993Snapshot, member_id: &str) -> Vec<CombinedEffects> {
    let mut g = crate::DesignAction { n: 0.0, vy: 0.0, vz: 0.0, my: 0.0, mz: 0.0, t: 0.0 };
    let mut variables: Vec<(&crate::LoadCase, crate::DesignAction)> = Vec::new();
    for row in document.member_actions.iter().filter(|a| a.member_id == member_id) {
        let Some(lc) = document.load_cases.iter().find(|l| l.id == row.load_case_id) else { continue };
        match lc.kind.as_str() {
            "permanent" | "prestress" => {
                g.n += row.action.n; g.vy += row.action.vy; g.vz += row.action.vz;
                g.my += row.action.my; g.mz += row.action.mz; g.t += row.action.t;
            }
            _ => variables.push((lc, row.action.clone())),
        }
    }
    let (gamma_g, gamma_q, xi) = gamma_factors();
    let mut out = Vec::new();
    if variables.is_empty() {
        if g.n.abs() + g.my.abs() + g.vz.abs() > 0.0 {
            out.push(CombinedEffects {
                combination_id: "uls-g".into(),
                situation: "uls".into(),
                action: crate::DesignAction {
                    n: gamma_g * g.n, vy: gamma_g * g.vy, vz: gamma_g * g.vz,
                    my: gamma_g * g.my, mz: gamma_g * g.mz, t: gamma_g * g.t,
                },
            });
        }
    } else {
        for (i, (lead, lead_a)) in variables.iter().enumerate() {
            let mut a = crate::DesignAction {
                n: gamma_g * g.n + gamma_q * lead_a.n,
                vy: gamma_g * g.vy + gamma_q * lead_a.vy,
                vz: gamma_g * g.vz + gamma_q * lead_a.vz,
                my: gamma_g * g.my + gamma_q * lead_a.my,
                mz: gamma_g * g.mz + gamma_q * lead_a.mz,
                t: gamma_g * g.t + gamma_q * lead_a.t,
            };
            let mut b = crate::DesignAction {
                n: xi * gamma_g * g.n + gamma_q * lead_a.n,
                vy: xi * gamma_g * g.vy + gamma_q * lead_a.vy,
                vz: xi * gamma_g * g.vz + gamma_q * lead_a.vz,
                my: xi * gamma_g * g.my + gamma_q * lead_a.my,
                mz: xi * gamma_g * g.mz + gamma_q * lead_a.mz,
                t: xi * gamma_g * g.t + gamma_q * lead_a.t,
            };
            for (j, (other, oa)) in variables.iter().enumerate() {
                if i == j { continue; }
                let (psi0, _, _) = psi_factors(&other.category, document.annex);
                a.n += gamma_q * psi0 * oa.n; a.vy += gamma_q * psi0 * oa.vy; a.vz += gamma_q * psi0 * oa.vz;
                a.my += gamma_q * psi0 * oa.my; a.mz += gamma_q * psi0 * oa.mz; a.t += gamma_q * psi0 * oa.t;
                b.n += gamma_q * psi0 * oa.n; b.vy += gamma_q * psi0 * oa.vy; b.vz += gamma_q * psi0 * oa.vz;
                b.my += gamma_q * psi0 * oa.my; b.mz += gamma_q * psi0 * oa.mz; b.t += gamma_q * psi0 * oa.t;
            }
            out.push(CombinedEffects { combination_id: format!("uls-6.10a-{}", lead.id), situation: "uls".into(), action: a });
            out.push(CombinedEffects { combination_id: format!("uls-6.10b-{}", lead.id), situation: "uls".into(), action: b });
        }
    }
    if variables.is_empty() {
        out.push(CombinedEffects { combination_id: "sls-char".into(), situation: "sls".into(), action: g });
    } else {
        let (lead, lead_a) = &variables[0];
        let mut s = crate::DesignAction {
            n: g.n + lead_a.n, vy: g.vy + lead_a.vy, vz: g.vz + lead_a.vz,
            my: g.my + lead_a.my, mz: g.mz + lead_a.mz, t: g.t + lead_a.t,
        };
        for (j, (other, oa)) in variables.iter().enumerate() {
            if j == 0 { continue; }
            let (psi0, _, _) = psi_factors(&other.category, document.annex);
            s.n += psi0 * oa.n; s.vy += psi0 * oa.vy; s.vz += psi0 * oa.vz;
            s.my += psi0 * oa.my; s.mz += psi0 * oa.mz; s.t += psi0 * oa.t;
        }
        out.push(CombinedEffects { combination_id: format!("sls-char-{}", lead.id), situation: "sls".into(), action: s });
    }
    out
}

fn governing_effects<'a>(effects: &'a [CombinedEffects], sit: &str) -> Option<&'a CombinedEffects> {
    effects.iter().filter(|e| e.situation == sit).max_by(|a, b| {
        let ka = a.action.n.abs() + a.action.my.abs() + a.action.vz.abs();
        let kb = b.action.n.abs() + b.action.my.abs() + b.action.vz.abs();
        ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
    })
}


fn push_duplicate_ids(report: &mut CheckReport, annex: AnnexChoice, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, loc(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let mut builder = CheckResult::assess(
            format!("en1993.integrity.duplicate.{table}.{id}"),
            "EN 1993 integrity",
            ClauseId::new("EN 1993", "§1", "id"),
            subject.clone(),
            loc(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
        )
        .annex(annex)
        .explanation(loc(
            &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(CheckStatus::Fail);
        builder = builder.remedy(Remedy::one_of(
            subject,
            if options.is_empty() { vec![format!("{id}-unique")] } else { options },
            loc(
                &format!("Rename the duplicated '{id}' entry to a free id."),
                &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

fn push_dangling_ref(report: &mut CheckReport, annex: AnnexChoice, check_id: String, path: String, subject_id: &str, label_en: &str, label_de: &str, current: &str, options: Vec<String>) {
    let subject = SubjectRef::new(subject_id, &path, loc(label_en, label_de));
    let opts = if options.is_empty() { vec![format!("{current}-missing")] } else { options };
    report.push(
        CheckResult::assess(
            check_id,
            "EN 1993 integrity",
            ClauseId::new("EN 1993", "§1", "reference"),
            subject.clone(),
            loc(&format!("Referential integrity — {label_en}"), &format!("Referenzintegrität — {label_de}")),
        )
        .annex(annex)
        .explanation(loc(
            &format!("'{current}' does not reference an existing target; dependent checks for this row must not Pass."),
            &format!("'{current}' verweist auf kein vorhandenes Ziel; abhängige Nachweise für diese Zeile dürfen nicht bestehen."),
        ))
        .status(CheckStatus::Fail)
        .remedy(Remedy::one_of(
            subject,
            opts,
            loc(
                &format!("Set {label_en} to one of the existing target ids."),
                &format!("{label_de} auf eine der vorhandenen Ziel-Ids setzen."),
            ),
        ))
        .build(),
    );
}

fn push_referential_integrity(report: &mut CheckReport, document: &En1993Snapshot) {
    let annex = document.annex;
    let material_ids: Vec<String> = document.materials.iter().map(|m| m.id.clone()).collect();
    let section_ids: Vec<String> = document.sections.iter().map(|s| s.id.clone()).collect();
    let member_ids: Vec<String> = document.members.iter().map(|m| m.id.clone()).collect();
    let load_case_ids: Vec<String> = document.load_cases.iter().map(|l| l.id.clone()).collect();
    let action_ids: Vec<String> = document.member_actions.iter().map(|a| a.id.clone()).collect();

    push_duplicate_ids(report, annex, "materials", &material_ids, &|id| format!("materials[id={id}].id"));
    push_duplicate_ids(report, annex, "sections", &section_ids, &|id| format!("sections[id={id}].id"));
    push_duplicate_ids(report, annex, "members", &member_ids, &|id| format!("members[id={id}].id"));
    push_duplicate_ids(report, annex, "loadCases", &load_case_ids, &|id| format!("loadCases[id={id}].id"));
    push_duplicate_ids(report, annex, "memberActions", &action_ids, &|id| format!("memberActions[id={id}].id"));
    push_duplicate_ids(report, annex, "joints", &document.joints.iter().map(|j| j.id.clone()).collect::<Vec<_>>(), &|id| format!("joints[id={id}].id"));
    push_duplicate_ids(report, annex, "fatigueDetails", &document.fatigue_details.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("fatigueDetails[id={id}].id"));
    push_duplicate_ids(report, annex, "fireExposures", &document.fire_exposures.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("fireExposures[id={id}].id"));
    push_duplicate_ids(report, annex, "coldFormedMembers", &document.cold_formed_members.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), &|id| format!("coldFormedMembers[id={id}].id"));
    push_duplicate_ids(report, annex, "platedPanels", &document.plated_panels.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("platedPanels[id={id}].id"));
    push_duplicate_ids(report, annex, "siloShells", &document.silo_shells.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("siloShells[id={id}].id"));
    push_duplicate_ids(report, annex, "tensionComponents", &document.tension_components.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("tensionComponents[id={id}].id"));
    push_duplicate_ids(report, annex, "bridgeFatigue", &document.bridge_fatigue.iter().map(|b| b.id.clone()).collect::<Vec<_>>(), &|id| format!("bridgeFatigue[id={id}].id"));
    push_duplicate_ids(report, annex, "towerLegs", &document.tower_legs.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("towerLegs[id={id}].id"));
    push_duplicate_ids(report, annex, "piles", &document.piles.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("piles[id={id}].id"));
    push_duplicate_ids(report, annex, "craneRunways", &document.crane_runways.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), &|id| format!("craneRunways[id={id}].id"));

    let designations: Vec<String> = rolled_heb_catalogue().into_iter().map(|s| s.designation).collect();
    for section in &document.sections {
        if !designations.iter().any(|d| d == &section.designation) {
            push_dangling_ref(
                report,
                annex,
                format!("en1993.integrity.sections.{}.designation", section.id),
                format!("sections[id={}].designation", section.id),
                &section.id,
                "Section designation",
                "Querschnittsbezeichnung",
                &section.designation,
                designations.clone(),
            );
        }
    }

    for member in &document.members {
        if !section_ids.iter().any(|id| id == &member.section_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.members.{}.sectionId", member.id), format!("members[id={}].sectionId", member.id), &member.id, "Member sectionId", "Bauteil sectionId", &member.section_id, section_ids.clone());
        }
        if !material_ids.iter().any(|id| id == &member.material_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.members.{}.materialId", member.id), format!("members[id={}].materialId", member.id), &member.id, "Member materialId", "Bauteil materialId", &member.material_id, material_ids.clone());
        }
    }
    for action in &document.member_actions {
        if !member_ids.iter().any(|id| id == &action.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.memberActions.{}.memberId", action.id), format!("memberActions[id={}].memberId", action.id), &action.id, "Action memberId", "Einwirkung memberId", &action.member_id, member_ids.clone());
        }
        if !load_case_ids.iter().any(|id| id == &action.load_case_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.memberActions.{}.loadCaseId", action.id), format!("memberActions[id={}].loadCaseId", action.id), &action.id, "Action loadCaseId", "Einwirkung loadCaseId", &action.load_case_id, load_case_ids.clone());
        }
    }
    for joint in &document.joints {
        if !member_ids.iter().any(|id| id == &joint.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.joints.{}.memberId", joint.id), format!("joints[id={}].memberId", joint.id), &joint.id, "Joint memberId", "Anschluss memberId", &joint.member_id, member_ids.clone());
        }
        for ja in &joint.actions {
            if !load_case_ids.iter().any(|id| id == &ja.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.joints.{}.actions.{}.loadCaseId", joint.id, ja.id), format!("joints[id={}].actions[id={}].loadCaseId", joint.id, ja.id), &joint.id, "Joint action loadCaseId", "Anschluss-Einwirkung loadCaseId", &ja.load_case_id, load_case_ids.clone());
            }
        }
    }
    for fat in &document.fatigue_details {
        if !member_ids.iter().any(|id| id == &fat.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.fatigueDetails.{}.memberId", fat.id), format!("fatigueDetails[id={}].memberId", fat.id), &fat.id, "Fatigue memberId", "Ermüdung memberId", &fat.member_id, member_ids.clone());
        }
    }
    for fire in &document.fire_exposures {
        if !member_ids.iter().any(|id| id == &fire.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.fireExposures.{}.memberId", fire.id), format!("fireExposures[id={}].memberId", fire.id), &fire.id, "Fire memberId", "Brand memberId", &fire.member_id, member_ids.clone());
        }
    }
    for br in &document.bridge_fatigue {
        if !member_ids.iter().any(|id| id == &br.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.bridgeFatigue.{}.memberId", br.id), format!("bridgeFatigue[id={}].memberId", br.id), &br.id, "Bridge memberId", "Brücke memberId", &br.member_id, member_ids.clone());
        }
    }
    for tower in &document.tower_legs {
        if !member_ids.iter().any(|id| id == &tower.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.towerLegs.{}.memberId", tower.id), format!("towerLegs[id={}].memberId", tower.id), &tower.id, "Tower memberId", "Turm memberId", &tower.member_id, member_ids.clone());
        }
        for fa in &tower.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.towerLegs.{}.actions.{}.loadCaseId", tower.id, fa.id), format!("towerLegs[id={}].actions[id={}].loadCaseId", tower.id, fa.id), &tower.id, "Tower action loadCaseId", "Turm-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for pile in &document.piles {
        if !section_ids.iter().any(|id| id == &pile.section_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.piles.{}.sectionId", pile.id), format!("piles[id={}].sectionId", pile.id), &pile.id, "Pile sectionId", "Pfahl sectionId", &pile.section_id, section_ids.clone());
        }
        if !material_ids.iter().any(|id| id == &pile.material_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.piles.{}.materialId", pile.id), format!("piles[id={}].materialId", pile.id), &pile.id, "Pile materialId", "Pfahl materialId", &pile.material_id, material_ids.clone());
        }
        for fa in &pile.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.piles.{}.actions.{}.loadCaseId", pile.id, fa.id), format!("piles[id={}].actions[id={}].loadCaseId", pile.id, fa.id), &pile.id, "Pile action loadCaseId", "Pfahl-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for crane in &document.crane_runways {
        if !member_ids.iter().any(|id| id == &crane.member_id) {
            push_dangling_ref(report, annex, format!("en1993.integrity.craneRunways.{}.memberId", crane.id), format!("craneRunways[id={}].memberId", crane.id), &crane.id, "Crane memberId", "Kranbahn memberId", &crane.member_id, member_ids.clone());
        }
        for fa in &crane.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.craneRunways.{}.actions.{}.loadCaseId", crane.id, fa.id), format!("craneRunways[id={}].actions[id={}].loadCaseId", crane.id, fa.id), &crane.id, "Crane action loadCaseId", "Kranbahn-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for cf in &document.cold_formed_members {
        for fa in &cf.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.coldFormedMembers.{}.actions.{}.loadCaseId", cf.id, fa.id), format!("coldFormedMembers[id={}].actions[id={}].loadCaseId", cf.id, fa.id), &cf.id, "Cold-formed action loadCaseId", "Kaltprofil-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for pp in &document.plated_panels {
        for fa in &pp.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.platedPanels.{}.actions.{}.loadCaseId", pp.id, fa.id), format!("platedPanels[id={}].actions[id={}].loadCaseId", pp.id, fa.id), &pp.id, "Plated action loadCaseId", "Beulpanel-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
    for ten in &document.tension_components {
        for fa in &ten.actions {
            if !load_case_ids.iter().any(|id| id == &fa.load_case_id) {
                push_dangling_ref(report, annex, format!("en1993.integrity.tensionComponents.{}.actions.{}.loadCaseId", ten.id, fa.id), format!("tensionComponents[id={}].actions[id={}].loadCaseId", ten.id, fa.id), &ten.id, "Tension action loadCaseId", "Zugglied-Einwirkung loadCaseId", &fa.load_case_id, load_case_ids.clone());
            }
        }
    }
}

pub fn check_full_steel_structure(document: &En1993Snapshot) -> CheckReport {
    let annex = document.annex;
    let mut params = AnnexParams::for_choice(annex);
    let mut report = CheckReport::default();
    push_referential_integrity(&mut report, document);

    if document.members.is_empty()
        && document.joints.is_empty()
        && document.fatigue_details.is_empty()
        && document.fire_exposures.is_empty()
        && document.cold_formed_members.is_empty()
        && document.plated_panels.is_empty()
        && document.silo_shells.is_empty()
        && document.tension_components.is_empty()
        && document.bridge_fatigue.is_empty()
        && document.tower_legs.is_empty()
        && document.piles.is_empty()
        && document.crane_runways.is_empty()
    {
        report.push(
            CheckResult::assess(
                "en1993.empty",
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993", "1-1", "1"),
                SubjectRef::whole(loc("Steel structure", "Stahltragwerk")),
                loc("Subject presence", "Gegenstand vorhanden"),
            )
            .annex(annex)
            .not_applicable(loc("No members or special elements to check.", "Keine Bauteile oder Sonderelemente zu prüfen."))
            .build(),
        );
        return report;
    }

    for member in &document.members {
        let Some(section) = find_section(document, &member.section_id) else { continue };
        let Some(material) = find_material(document, &member.material_id) else { continue };
        let kind = member_kind(&member.member_type);
        let class = part_1_1::section_class_rolled_i(section, material.fy);

        {
            let mut b = CheckResult::assess(
                format!("en1993.5.2.class.{}", member.id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "5", "5.2"),
                subject_member(member, &format!("members[id={}].sectionId", member.id)),
                loc("Cross-section classification", "Querschnittsklassifizierung"),
            )
            .annex(annex)
            .explanation(loc(
                &format!("Section {} is class {} per Table 5.2{}.", section.designation, class, if class == 4 { " — effective A_eff/W_eff per EN 1993-1-5; class 4 is not admitted without effective section redesign" } else { "" }),
                &format!("Querschnitt {} ist Klasse {} nach Tabelle 5.2{}.", section.designation, class, if class == 4 { " — wirksame A_eff/W_eff nach EN 1993-1-5; Klasse 4 ist ohne wirksamen Querschnitt unzulässig" } else { "" }),
            ));
            if class > 3 {
                let a_eff = part_1_1::effective_area_class4(section, material.fy);
                b = b.utilization(dimensionless(section.area), dimensionless(a_eff.max(1e-12)));
                let options = next_section_options(section, section.area, section.w_pl_y);
                b = b.remedy(Remedy::one_of(
                    subject_member(member, &format!("members[id={}].sectionId", member.id)),
                    options.clone(),
                    loc(
                        &format!("Select a class ≤ 3 section; options: {}.", options.join(", ")),
                        &format!("Querschnitt Klasse ≤ 3 wählen; Optionen: {}.", options.join(", ")),
                    ),
                ));
            }
            report.push(b.build());
        }

        if member.analysis.eq_ignore_ascii_case("plastic") && class > 1 {
            let mut b = CheckResult::assess(
                format!("en1993.5.2.plastic.{}", member.id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "5", "5.2.2"),
                subject_member(member, &format!("members[id={}].sectionId", member.id)),
                loc("Plastic global analysis class", "Klasse für plastische Schnittgrößenermittlung"),
            )
            .annex(annex)
            .utilization(dimensionless(class as f64), dimensionless(1.0))
            .explanation(loc(
                &format!("Plastic analysis requires class 1; section is class {}.", class),
                &format!("Plastische Schnittgrößenermittlung erfordert Klasse 1; Querschnitt ist Klasse {}.", class),
            ));
            let options = next_section_options(section, section.area, section.w_pl_y);
            b = b.remedy(Remedy::one_of(
                subject_member(member, &format!("members[id={}].sectionId", member.id)),
                options.clone(),
                loc(
                    &format!("Select a class-1 section; options: {}.", options.join(", ")),
                    &format!("Klasse-1-Querschnitt wählen; Optionen: {}.", options.join(", ")),
                ),
            ));
            report.push(b.build());
        }

        let a_res = if class == 4 { part_1_1::effective_area_class4(section, material.fy) } else { section.area };
        let w_y = if class == 4 {
            part_1_1::effective_w_el_y_class4(section, material.fy)
        } else if class <= 2 {
            section.w_pl_y
        } else {
            section.w_el_y
        };
        let w_z = if class == 4 {
            section.w_el_z * (a_res / section.area.max(1e-12)).min(1.0)
        } else if class <= 2 {
            section.w_pl_z
        } else {
            section.w_el_z
        };

        let iy = part_1_1::radius_of_gyration(section.iy, section.area);
        let iz = part_1_1::radius_of_gyration(section.iz, section.area);
        let curve_y = part_1_1::buckling_curve_rolled_i(section, 'y', material.fy);
        let curve_z = part_1_1::buckling_curve_rolled_i(section, 'z', material.fy);
        let lambda_y = part_1_1::lambda_bar(member.buckling_length_y, iy, material.e_modulus, material.fy);
        let lambda_z = part_1_1::lambda_bar(member.buckling_length_z, iz, material.e_modulus, material.fy);
        let chi_y = part_1_1::chi(lambda_y, curve_y);
        let chi_z = part_1_1::chi(lambda_z, curve_z);
        let n_b_rd_y = part_1_1::buckling_resistance_n(a_res, material.fy, chi_y, params);
        let n_b_rd_z = part_1_1::buckling_resistance_n(a_res, material.fy, chi_z, params);
        let n_b_rd = n_b_rd_y.min(n_b_rd_z);

        let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length.max(member.ltb_restraint_spacing), member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
        let lambda_lt = part_1_1::lambda_lt(w_y, material.fy, m_cr);
        let curve_lt = part_1_1::ltb_curve_table_6_4(section);
        let chi_lt = part_1_1::chi_lt(lambda_lt, curve_lt);
        let m_b_rd = part_1_1::ltb_resistance_nm(w_y, material.fy, chi_lt, params);

        let combinations = combine_member_actions(document, &member.id);
        if governing_effects(&combinations, "uls").is_none() {
            report.push(
                CheckResult::assess(
                    format!("en1993.6.2.uls.{}", member.id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.2"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("ULS action effects", "Einwirkungen im GZT"),
                )
                .annex(annex)
                .not_applicable(loc(
                    "No characteristic actions to combine for this member.",
                    "Keine charakteristischen Einwirkungen zum Kombinieren für dieses Bauteil.",
                ))
                .build(),
            );
        }

        let uls_actions: Vec<&CombinedEffects> = governing_effects(&combinations, "uls").into_iter().collect();
        for action_row in uls_actions {
            let a = &action_row.action;
            let n_rd = part_1_1::axial_resistance_n(a_res, material.fy, params);
            let m_y_rd = part_1_1::bending_resistance_nm(w_y, material.fy, class, params);
            let m_z_rd = part_1_1::bending_resistance_nm(w_z, material.fy, class, params);
            let v_z_rd = part_1_1::shear_resistance_n(section.shear_area_z, material.fy, params);
            let v_y_rd = part_1_1::shear_resistance_n(section.shear_area_y, material.fy, params);
            let m_y_rd_red = part_1_1::shear_reduced_m_rd(m_y_rd, a.vz, v_z_rd);

            // 6.2.4 compression / 6.2.3 tension
            if a.n >= 0.0 {
            if kind_allows_compression(&kind) {
                let mut builder = CheckResult::assess(
                    format!("en1993.6.2.4.n.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.2.4"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("Cross-section compression", "Querschnitt Druck"),
                )
                .annex(annex)
                .utilization(force_n(a.n), force_n(n_rd))
                .explanation(loc(
                    &format!("N_Ed={:.1} kN, N_Rd={:.1} kN (A·f_y/γ_M0).", a.n / 1000.0, n_rd / 1000.0),
                    &format!("Druckkraft N_Ed={:.1} kN gegenüber Querschnittstragfähigkeit N_Rd={:.1} kN (A·f_y/γ_M0).", a.n / 1000.0, n_rd / 1000.0),
                ));
                if a.n > n_rd {
                    let a_req = a.n * params.gamma_m0 / material.fy;
                    let options = next_section_options(section, a_req, w_y);
                    if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                }
                }
                report.push(builder.build());
            }
            } else {
                let n_t_rd = part_1_1::net_tension_resistance_n(section.area, section.area_net, material.fy, material.fu, params);
                let n_t = -a.n;
                let mut builder = CheckResult::assess(
                    format!("en1993.6.2.3.nt.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.2.3"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("Net section tension", "Zugtragfähigkeit Nettoquerschnitt"),
                )
                .annex(annex)
                .utilization(force_n(n_t), force_n(n_t_rd))
                .explanation(loc(
                    &format!("N_t,Ed={:.1} kN, N_t,Rd={:.1} kN.", n_t / 1000.0, n_t_rd / 1000.0),
                    &format!("Zug N_t,Ed={:.1} kN, N_t,Rd={:.1} kN.", n_t / 1000.0, n_t_rd / 1000.0),
                ));
                if n_t > n_t_rd {
                    let a_net_req = n_t * params.gamma_m2 / (0.9 * material.fu);
                    builder = builder.remedy(Remedy::at_least(
                        subject_member(member, &format!("sections[id={}].areaNet", section.id)),
                        area_m2(section.area_net),
                        area_m2(a_net_req),
                        loc(
                            &format!("Increase net area from {:.1} cm² to at least {:.1} cm².", section.area_net * 1e4, a_net_req * 1e4),
                            &format!("Nettoquerschnitt von {:.1} cm² auf mindestens {:.1} cm² erhöhen.", section.area_net * 1e4, a_net_req * 1e4),
                        ),
                    ));
                }
                report.push(builder.build());
            }

            // 6.2.5 bending My
            if kind_allows_bending(&kind) {
                let mut builder = CheckResult::assess(
                    format!("en1993.6.2.5.my.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.2.5"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("Bending resistance My", "Biegetragfähigkeit My"),
                )
                .annex(annex)
                .utilization(moment_nm(a.my.abs()), moment_nm(m_y_rd_red))
                .explanation(loc(
                    &format!("M_y,Ed={:.1} kNm, M_c,Rd={:.1} kNm (shear-reduced).", a.my.abs() / 1000.0, m_y_rd_red / 1000.0),
                    &format!("M_y,Ed={:.1} kNm, M_c,Rd={:.1} kNm (abgemindert bei Querkraft).", a.my.abs() / 1000.0, m_y_rd_red / 1000.0),
                ));
                if a.my.abs() > m_y_rd_red {
                    let w_req = a.my.abs() * params.gamma_m0 / material.fy;
                    let options = next_section_options(section, section.area, w_req);
                    if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                }
                }
                report.push(builder.build());
            }

            // 6.2.6 shear
            {
                let v_ed = a.vz.abs().max(a.vy.abs());
                let v_rd = if a.vz.abs() >= a.vy.abs() { v_z_rd } else { v_y_rd };
                let mut builder = CheckResult::assess(
                    format!("en1993.6.2.6.v.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.2.6"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("Shear resistance", "Querkrafttragfähigkeit"),
                )
                .annex(annex)
                .utilization(force_n(v_ed), force_n(v_rd))
                .explanation(loc(
                    &format!("V_Ed={:.1} kN, V_pl,Rd={:.1} kN.", v_ed / 1000.0, v_rd / 1000.0),
                    &format!("Querkraft V_Ed={:.1} kN gegenüber V_pl,Rd={:.1} kN.", v_ed / 1000.0, v_rd / 1000.0),
                ));
                if v_ed > v_rd {
                    let av_req = v_ed * 3.0_f64.sqrt() * params.gamma_m0 / material.fy;
                    builder = builder.remedy(Remedy::at_least(
                        subject_member(member, &format!("sections[id={}].shearAreaZ", section.id)),
                        area_m2(section.shear_area_z),
                        area_m2(av_req),
                        loc(
                            &format!("Increase shear area to at least {:.1} cm².", av_req * 1e4),
                            &format!("Schubfläche auf mindestens {:.1} cm² erhöhen.", av_req * 1e4),
                        ),
                    ));
                }
                report.push(builder.build());
            }

            // 6.2.9 / 6.2.10 M+N (+V already in reduced M)
            {
                let eta = part_1_1::mn_interaction_eta(
                    a.n,
                    a.my,
                    a.mz,
                    n_rd,
                    m_y_rd_red,
                    m_z_rd,
                    section.area,
                    section.b,
                    section.tf,
                    class,
                );
                let mut builder = CheckResult::assess(
                    format!("en1993.6.2.9.mn.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.2.9"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("M+N interaction", "M+N Interaktion"),
                )
                .annex(annex)
                .utilization(dimensionless(eta), dimensionless(1.0))
                .explanation(loc(
                    &format!("η={:.3} from §6.2.9/6.2.10 (n, a, M_N,y/M_N,z, α/β).", eta),
                    &format!("η={:.3} nach §6.2.9/6.2.10 (n, a, M_N,y/M_N,z, α/β).", eta),
                ));
                if eta > 1.0 {
                    let options = next_section_options(section, section.area * eta, w_y * eta);
                    if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                }
                }
                report.push(builder.build());
            }

            // 6.3.1 flexural buckling — χ computed, never user-supplied
            if a.n > 0.0 && kind_allows_flexural_buckling(&kind) {
                let mut builder = CheckResult::assess(
                    format!("en1993.6.3.1.nb.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.3.1"),
                    subject_member(member, &format!("members[id={}].bucklingLengthY", member.id)),
                    loc("Flexural buckling", "Biegeknicken"),
                )
                .annex(annex)
                .utilization(force_n(a.n), force_n(n_b_rd))
                .explanation(loc(
                    &format!(
                        "N_Ed={:.1} kN, N_b,Rd={:.1} kN (χ_y={:.3} λ̄_y={:.3}, χ_z={:.3} λ̄_z={:.3}, γ_M1={}).",
                        a.n / 1000.0,
                        n_b_rd / 1000.0,
                        chi_y,
                        lambda_y,
                        chi_z,
                        lambda_z,
                        params.gamma_m1
                    ),
                    &format!(
                        "Biegeknicken: N_Ed={:.1} kN, N_b,Rd={:.1} kN (χ_y={:.3} λ̄_y={:.3}, χ_z={:.3} λ̄_z={:.3}, γ_M1={}).",
                        a.n / 1000.0,
                        n_b_rd / 1000.0,
                        chi_y,
                        lambda_y,
                        chi_z,
                        lambda_z,
                        params.gamma_m1
                    ),
                ));
                if a.n > n_b_rd {
                    // Required L_cr such that χ gives N_b,Rd >= N_Ed — binary search on L
                    let mut lo = 0.1;
                    let mut hi = member.buckling_length_y;
                    for _ in 0..40 {
                        let mid = 0.5 * (lo + hi);
                        let lam = part_1_1::lambda_bar(mid, iy.min(iz), material.e_modulus, material.fy);
                        let curve = if curve_y.alpha() >= curve_z.alpha() { curve_y } else { curve_z };
                        let ch = part_1_1::chi(lam, curve);
                        let nb = part_1_1::buckling_resistance_n(section.area, material.fy, ch, params);
                        if nb >= a.n {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    let l_req = lo;
                    builder = builder.remedy(Remedy::at_most(
                        subject_member(member, &format!("members[id={}].bucklingLengthY", member.id)),
                        length_m(member.buckling_length_y),
                        length_m(l_req),
                        loc(
                            &format!("Reduce buckling length from {:.2} m to at most {:.2} m (add restraints).", member.buckling_length_y, l_req),
                            &format!("Knicklänge von {:.2} m auf höchstens {:.2} m reduzieren (Halterungen).", member.buckling_length_y, l_req),
                        ),
                    ));
                    let options = next_section_options(section, section.area * (a.n / n_b_rd), w_y);
                    if !options.is_empty() {
                        builder = builder.remedy(Remedy::one_of(
                            subject_member(member, &format!("members[id={}].sectionId", member.id)),
                            options.clone(),
                            loc(
                                &format!("Or select larger section: {}.", options.join(", ")),
                                &format!("Oder größeren Querschnitt wählen: {}.", options.join(", ")),
                            ),
                        ));
                    }
                }
                report.push(builder.build());
            }

            // 6.3.2 LTB
            if a.my.abs() > 0.0 && kind_allows_ltb(&kind) {
                let mut builder = CheckResult::assess(
                    format!("en1993.6.3.2.mb.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.3.2"),
                    subject_member(member, &format!("members[id={}].ltbRestraintSpacing", member.id)),
                    loc("Lateral-torsional buckling", "Biegedrillknicken"),
                )
                .annex(annex)
                .utilization(moment_nm(a.my.abs()), moment_nm(m_b_rd))
                .explanation(loc(
                    &format!("M_y,Ed={:.1} kNm, M_b,Rd={:.1} kNm (M_cr={:.1} kNm, χ_LT={:.3}, λ̄_LT={:.3}).", a.my.abs() / 1000.0, m_b_rd / 1000.0, m_cr / 1000.0, chi_lt, lambda_lt),
                    &format!("Biegedrillknicken M_y,Ed={:.1} kNm, M_b,Rd={:.1} kNm (M_cr={:.1} kNm, χ_LT={:.3}, λ̄_LT={:.3}).", a.my.abs() / 1000.0, m_b_rd / 1000.0, m_cr / 1000.0, chi_lt, lambda_lt),
                ));
                if a.my.abs() > m_b_rd {
                    // search restraint spacing
                    let mut lo = 0.2;
                    let mut hi = member.ltb_restraint_spacing.max(member.ltb_length);
                    for _ in 0..40 {
                        let mid = 0.5 * (lo + hi);
                        let mcr = part_1_1::m_cr_rolled_nm(section, material, mid, member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
                        let lam = part_1_1::lambda_lt(w_y, material.fy, mcr);
                        let ch = part_1_1::chi_lt(lam, curve_lt);
                        let mb = part_1_1::ltb_resistance_nm(w_y, material.fy, ch, params);
                        if mb >= a.my.abs() {
                            lo = mid; // can allow longer? actually shorter spacing increases Mcr
                            // wait: shorter L => higher Mcr => higher Mb. So if mb >= need, try larger spacing
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    // Fix search: we want maximum spacing that still passes, starting from small
                    let mut ok = 0.2;
                    let mut step = member.ltb_length;
                    while step > 0.05 {
                        let trial = ok + step;
                        let mcr = part_1_1::m_cr_rolled_nm(section, material, trial, member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
                        let lam = part_1_1::lambda_lt(w_y, material.fy, mcr);
                        let ch = part_1_1::chi_lt(lam, curve_lt);
                        let mb = part_1_1::ltb_resistance_nm(w_y, material.fy, ch, params);
                        if mb >= a.my.abs() {
                            ok = trial;
                        }
                        step *= 0.5;
                    }
                    // Actually for fail we need REQUIRED max spacing that works - binary search for max L where Mb>=Med
                    let mut lo2 = 0.1;
                    let mut hi2 = member.ltb_length.max(1.0);
                    // Find any passing small L first
                    for _ in 0..40 {
                        let mid = 0.5 * (lo2 + hi2);
                        let mcr = part_1_1::m_cr_rolled_nm(section, material, mid, member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
                        let lam = part_1_1::lambda_lt(w_y, material.fy, mcr);
                        let ch = part_1_1::chi_lt(lam, curve_lt);
                        let mb = part_1_1::ltb_resistance_nm(w_y, material.fy, ch, params);
                        if mb >= a.my.abs() {
                            lo2 = mid; // can go longer
                        } else {
                            hi2 = mid;
                        }
                    }
                    let spacing_req = lo2;
                    builder = builder.remedy(Remedy::at_most(
                        subject_member(member, &format!("members[id={}].ltbRestraintSpacing", member.id)),
                        length_m(member.ltb_restraint_spacing),
                        length_m(spacing_req),
                        loc(
                            &format!("Provide LTB restraints at most every {:.2} m (currently {:.2} m).", spacing_req, member.ltb_restraint_spacing),
                            &format!("BDK-Halterungen höchstens alle {:.2} m anordnen (aktuell {:.2} m).", spacing_req, member.ltb_restraint_spacing),
                        ),
                    ));
                }
                report.push(builder.build());
            }

            // 6.3.3 interaction 6.61/6.62
            if kind_allows_member_interaction(&kind) && (a.n > 0.0 || a.my.abs() > 0.0 || a.mz.abs() > 0.0) {
                let n_rk = section.area * material.fy;
                let (kyy, kyz, kzy, kzz) = part_1_1::interaction_kij(a.n, n_rk, lambda_y, lambda_z, member.end_moment_ratio_psi, class);
                let (eta61, eta62) = part_1_1::interaction_eta(a.n, n_b_rd_y, n_b_rd_z, a.my, a.mz, m_b_rd.max(m_y_rd_red), m_z_rd, kyy, kyz, kzy, kzz);
                let eta = eta61.max(eta62);
                let mut builder = CheckResult::assess(
                    format!("en1993.6.3.3.int.{}", action_row.combination_id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.3.3"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("Member buckling interaction 6.61/6.62", "Bauteilinteraktion 6.61/6.62"),
                )
                .annex(annex)
                .utilization(dimensionless(eta), dimensionless(1.0))
                .explanation(loc(
                    &format!("η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_zz={:.3}).", eta61, eta62, kyy, kzz),
                    &format!("Nachweis: η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_zz={:.3}).", eta61, eta62, kyy, kzz),
                ));
                if eta > 1.0 {
                    let options = next_section_options(section, section.area * eta, w_y * eta);
                    if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                }
                }
                report.push(builder.build());
            }
        }

        // SLS deflection — EN 1990 characteristic combination
        if let Some(action_row) = governing_effects(&combinations, "sls") {
            let delta = part_1_1::deflection_from_moment(action_row.action.my.abs(), member.length, material.e_modulus, section.iy);
            let limit = member.length / member.deflection_limit_ratio.max(1.0);
            let mut builder = CheckResult::assess(
                format!("en1993.7.sls.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "7", "7.2"),
                subject_member(member, &format!("members[id={}].deflectionLimitRatio", member.id)),
                loc("SLS deflection", "Verformung GZG"),
            )
            .annex(annex)
            .utilization(length_m(delta), length_m(limit))
            .explanation(loc(
                &format!(
                    "δ={:.1} mm ≤ L/{}={:.1} mm (characteristic {}).",
                    delta * 1000.0, member.deflection_limit_ratio, limit * 1000.0, action_row.combination_id
                ),
                &format!(
                    "δ={:.1} mm ≤ L/{}={:.1} mm (charakteristische Kombination {}).",
                    delta * 1000.0, member.deflection_limit_ratio, limit * 1000.0, action_row.combination_id
                ),
            ));
            if delta > limit {
                let options = next_section_options(section, section.area, w_y);
                if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                }
            }
            report.push(builder.build());
        }
    }


    // Joints 1-8
    for joint in &document.joints {
        let shear_forces: Vec<(String, f64)> = joint.actions.iter().map(|a| (a.load_case_id.clone(), a.shear)).collect();
        let tension_forces: Vec<(String, f64)> = joint.actions.iter().map(|a| (a.load_case_id.clone(), a.tension)).collect();
        let shear_combos = combine_scalar_forces(document, &shear_forces);
        let tension_combos = combine_scalar_forces(document, &tension_forces);
        let shear_ed = governing_scalar(&shear_combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let tension_ed = governing_scalar(&tension_combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let shear_gov = governing_scalar(&shear_combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let tension_gov = governing_scalar(&tension_combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");

        if joint.kind == "bolted" {
            let n_bolts = joint.bolt_rows * joint.bolts_per_row;
            let a_s = part_1_8::bolt_as(joint.bolt_diameter);
            let f_ub = part_1_8::bolt_fub(&joint.bolt_class);
            let d0 = joint.bolt_diameter + 0.002;
            let p1 = joint.pitch;
            let p2 = if joint.gauge > 0.0 { joint.gauge } else { joint.pitch };
            let fv = part_1_8::bolt_shear_resistance_n(n_bolts, a_s, f_ub, joint.shear_planes.max(1), params.gamma_m2);
            let alpha_b = part_1_8::bearing_alpha_b(joint.end_distance, p1, d0, f_ub, joint.plate_fu);
            let k1 = part_1_8::bearing_k1(joint.edge_distance, p2, d0);
            let fb = part_1_8::bolt_bearing_resistance_n(k1, alpha_b, joint.plate_fu, joint.bolt_diameter, joint.plate_thickness, n_bolts, params.gamma_m2);
            let ft = part_1_8::bolt_tension_resistance_n(n_bolts, a_s, f_ub, params.gamma_m2);

            let mut b = CheckResult::assess(
                format!("en1993.1-8.3.6.shear.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "3", "3.6.1"),
                subject_joint(joint, &format!("joints[id={}].actions", joint.id)),
                loc("Bolt shear", "Schraubenschub"),
            )
            .annex(annex)
            .utilization(force_n(shear_ed), force_n(fv))
            .explanation(loc(
                &format!("F_v,Ed={:.1} kN (gov. {}), F_v,Rd={:.1} kN ({}×M{:.0} {}).", shear_ed / 1000.0, shear_gov, fv / 1000.0, n_bolts, joint.bolt_diameter * 1000.0, joint.bolt_class),
                &format!("Schraubenschub F_v,Ed={:.1} kN (maßgebend {}), F_v,Rd={:.1} kN ({}×M{:.0} {}).", shear_ed / 1000.0, shear_gov, fv / 1000.0, n_bolts, joint.bolt_diameter * 1000.0, joint.bolt_class),
            ));
            if shear_ed > fv {
                let n_req = ((shear_ed * params.gamma_m2 / (joint.shear_planes.max(1) as f64 * 0.6 * a_s * f_ub)).ceil() as u32).max(1);
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].boltRows", joint.id)),
                    dimensionless(joint.bolt_rows as f64),
                    dimensionless((n_req / joint.bolts_per_row.max(1)).max(1) as f64),
                    loc(
                        &format!("Increase to at least {} bolts (e.g. {} rows × {}).", n_req, (n_req + joint.bolts_per_row - 1) / joint.bolts_per_row.max(1), joint.bolts_per_row),
                        &format!("Mindestens {} Schrauben vorsehen (z. B. {} Reihen × {}).", n_req, (n_req + joint.bolts_per_row - 1) / joint.bolts_per_row.max(1), joint.bolts_per_row),
                    ),
                ));
                let d_opts = ["0.022", "0.024", "0.027", "0.030"].iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
                b = b.remedy(Remedy::one_of(
                    subject_joint(joint, &format!("joints[id={}].boltDiameter", joint.id)),
                    d_opts.clone(),
                    loc(
                        &format!("Or increase bolt diameter (m): {}.", d_opts.join(", ")),
                        &format!("Oder Schraubendurchmesser erhöhen (m): {}.", d_opts.join(", ")),
                    ),
                ));
            }
            report.push(b.build());

            let mut b = CheckResult::assess(
                format!("en1993.1-8.3.6.bearing.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "3", "3.6.1"),
                subject_joint(joint, &format!("joints[id={}].pitch", joint.id)),
                loc("Bolt bearing", "Lochleibung"),
            )
            .annex(annex)
            .utilization(force_n(shear_ed), force_n(fb))
            .explanation(loc(
                &format!("F_b,Ed={:.1} kN (gov. {}), F_b,Rd={:.1} kN (α_b={:.2} from e1/p1, k1={:.2} from e2/p2).", shear_ed / 1000.0, shear_gov, fb / 1000.0, alpha_b, k1),
                &format!("Lochleibung F_b,Ed={:.1} kN (maßgebend {}), F_b,Rd={:.1} kN (α_b={:.2} aus e1/p1, k1={:.2} aus e2/p2).", shear_ed / 1000.0, shear_gov, fb / 1000.0, alpha_b, k1),
            ));
            if shear_ed > fb {
                let need_alpha = shear_ed * params.gamma_m2 / (k1.max(0.1) * joint.plate_fu * joint.bolt_diameter * joint.plate_thickness * n_bolts as f64);
                let e1_req = (need_alpha.min(1.0) * 3.0 * d0).max(joint.end_distance);
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].endDistance", joint.id)),
                    length_m(joint.end_distance),
                    length_m(e1_req),
                    loc(
                        &format!("Increase end distance e1 from {:.0} mm to at least {:.0} mm.", joint.end_distance * 1000.0, e1_req * 1000.0),
                        &format!("Randabstand e1 von {:.0} mm auf mindestens {:.0} mm erhöhen.", joint.end_distance * 1000.0, e1_req * 1000.0),
                    ),
                ));
                let p1_req = ((need_alpha.min(1.0) + 0.25) * 3.0 * d0).max(joint.pitch);
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].pitch", joint.id)),
                    length_m(joint.pitch),
                    length_m(p1_req),
                    loc(
                        &format!("Or increase pitch p1 to at least {:.0} mm.", p1_req * 1000.0),
                        &format!("Oder Lochabstand p1 auf mindestens {:.0} mm erhöhen.", p1_req * 1000.0),
                    ),
                ));
            }
            report.push(b.build());

            if tension_ed > 0.0 {
                let mut b = CheckResult::assess(
                    format!("en1993.1-8.3.6.tension.{}", joint.id),
                    "DIN EN 1993-1-8",
                    ClauseId::new("EN 1993-1-8", "3", "3.6.1"),
                    subject_joint(joint, &format!("joints[id={}].actions", joint.id)),
                    loc("Bolt tension", "Schrauben Zug"),
                )
                .annex(annex)
                .utilization(force_n(tension_ed), force_n(ft))
                .explanation(loc(
                    &format!("F_t,Ed={:.1} kN (gov. {}), F_t,Rd={:.1} kN.", tension_ed / 1000.0, tension_gov, ft / 1000.0),
                    &format!("Schrauben Zug F_t,Ed={:.1} kN (maßgebend {}), F_t,Rd={:.1} kN.", tension_ed / 1000.0, tension_gov, ft / 1000.0),
                ));
                if tension_ed > ft {
                    let n_req = ((tension_ed / (0.9 * a_s * f_ub / params.gamma_m2)).ceil() as u32).max(1);
                    b = b.remedy(Remedy::at_least(
                        subject_joint(joint, &format!("joints[id={}].boltRows", joint.id)),
                        dimensionless(joint.bolt_rows as f64),
                        dimensionless((n_req / joint.bolts_per_row.max(1)).max(1) as f64),
                        loc(
                            &format!("Increase bolt count to at least {} for tension.", n_req),
                            &format!("Schraubenanzahl für Zug auf mindestens {} erhöhen.", n_req),
                        ),
                    ));
                }
                report.push(b.build());
            }

            let cat = joint.category.to_ascii_uppercase();
            if cat == "B" || cat == "C" {
                let f_p_c = part_1_8::preload_force_n(a_s, f_ub, joint.preload_force);
                let fs = part_1_8::slip_resistance_n(
                    joint.slip_factor_ks.max(0.1),
                    joint.friction_surfaces.max(1),
                    joint.friction_mu.max(0.1),
                    f_p_c,
                    n_bolts.max(1),
                    params.gamma_m3,
                );
                let mut b = CheckResult::assess(
                    format!("en1993.1-8.3.9.slip.{}", joint.id),
                    "DIN EN 1993-1-8",
                    ClauseId::new("EN 1993-1-8", "3", "3.9"),
                    subject_joint(joint, &format!("joints[id={}].frictionMu", joint.id)),
                    loc("Slip resistance", "Gleitfestigkeit"),
                )
                .annex(annex)
                .utilization(force_n(shear_ed), force_n(fs))
                .explanation(loc(
                    &format!("F_s,Ed={:.1} kN (gov. {}), F_s,Rd={:.1} kN (cat. {}, k_s={:.2}, n={}, μ={:.2}, F_p,C={:.1} kN, γ_M3={}).", shear_ed / 1000.0, shear_gov, fs / 1000.0, cat, joint.slip_factor_ks, joint.friction_surfaces, joint.friction_mu, f_p_c / 1000.0, params.gamma_m3),
                    &format!("Gleitfestigkeit F_s,Ed={:.1} kN (maßgebend {}), F_s,Rd={:.1} kN (Kat. {}, k_s={:.2}, n={}, μ={:.2}, F_p,C={:.1} kN, γ_M3={}).", shear_ed / 1000.0, shear_gov, fs / 1000.0, cat, joint.slip_factor_ks, joint.friction_surfaces, joint.friction_mu, f_p_c / 1000.0, params.gamma_m3),
                ));
                if shear_ed > fs {
                    let mu_req = shear_ed * params.gamma_m3 / (joint.slip_factor_ks.max(0.1) * joint.friction_surfaces.max(1) as f64 * f_p_c * n_bolts.max(1) as f64);
                    b = b.remedy(Remedy::at_least(
                        subject_joint(joint, &format!("joints[id={}].frictionMu", joint.id)),
                        dimensionless(joint.friction_mu),
                        dimensionless(mu_req.min(0.5)),
                        loc(
                            &format!("Increase friction coefficient μ to at least {:.2}.", mu_req.min(0.5)),
                            &format!("Reibungszahl μ auf mindestens {:.2} erhöhen.", mu_req.min(0.5)),
                        ),
                    ));
                    let fp_req = shear_ed * params.gamma_m3 / (joint.slip_factor_ks.max(0.1) * joint.friction_surfaces.max(1) as f64 * joint.friction_mu.max(0.1) * n_bolts.max(1) as f64);
                    b = b.remedy(Remedy::at_least(
                        subject_joint(joint, &format!("joints[id={}].preloadForce", joint.id)),
                        force_n(joint.preload_force),
                        force_n(fp_req),
                        loc(
                            &format!("Increase preload F_p,C to at least {:.1} kN.", fp_req / 1000.0),
                            &format!("Vorspannkraft F_p,C auf mindestens {:.1} kN erhöhen.", fp_req / 1000.0),
                        ),
                    ));
                }
                report.push(b.build());
            }
        } else if joint.kind == "welded" {
            let beta = part_1_8::beta_w(&joint.weld_grade);
            let fw_s = part_1_8::fillet_weld_simplified_n(joint.weld_throat, joint.weld_length, joint.weld_fu, beta, params.gamma_m2);
            let (_f_ed_dir, fw_d) = part_1_8::fillet_weld_directional_n(joint.weld_throat, joint.weld_length, joint.weld_fu, beta, params.gamma_m2, shear_ed);
            let mut b = CheckResult::assess(
                format!("en1993.1-8.4.5.simplified.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "4", "4.5.3.3"),
                subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                loc("Fillet weld simplified method", "Kehlnaht vereinfachtes Verfahren"),
            )
            .annex(annex)
            .utilization(force_n(shear_ed), force_n(fw_s))
            .explanation(loc(
                &format!("F_w,Ed={:.1} kN (gov. {}), F_w,Rd={:.1} kN (a={:.1} mm, ℓ={:.0} mm).", shear_ed / 1000.0, shear_gov, fw_s / 1000.0, joint.weld_throat * 1000.0, joint.weld_length * 1000.0),
                &format!("Kehlnaht (vereinfacht) F_w,Ed={:.1} kN (maßgebend {}), F_w,Rd={:.1} kN (a={:.1} mm, ℓ={:.0} mm).", shear_ed / 1000.0, shear_gov, fw_s / 1000.0, joint.weld_throat * 1000.0, joint.weld_length * 1000.0),
            ));
            if shear_ed > fw_s {
                let a_req = shear_ed * beta * 3.0_f64.sqrt() * params.gamma_m2 / (joint.weld_fu * joint.weld_length.max(1e-6));
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                    length_m(joint.weld_throat),
                    length_m(a_req),
                    loc(
                        &format!("Increase weld throat a from {:.1} mm to at least {:.1} mm.", joint.weld_throat * 1000.0, a_req * 1000.0),
                        &format!("Nahtdicke a von {:.1} mm auf mindestens {:.1} mm erhöhen.", joint.weld_throat * 1000.0, a_req * 1000.0),
                    ),
                ));
            }
            report.push(b.build());

            let mut b = CheckResult::assess(
                format!("en1993.1-8.4.5.directional.{}", joint.id),
                "DIN EN 1993-1-8",
                ClauseId::new("EN 1993-1-8", "4", "4.5.3.2"),
                subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                loc("Fillet weld directional method", "Kehlnaht gerichtetes Verfahren"),
            )
            .annex(annex)
            .utilization(force_n(shear_ed), force_n(fw_d))
            .explanation(loc(
                &format!("Directional F_w,Ed={:.1} kN (gov. {}), F_w,Rd={:.1} kN.", shear_ed / 1000.0, shear_gov, fw_d / 1000.0),
                &format!("Gerichtetes Verfahren F_w,Ed={:.1} kN (maßgebend {}), F_w,Rd={:.1} kN.", shear_ed / 1000.0, shear_gov, fw_d / 1000.0),
            ));
            if shear_ed > fw_d {
                let a_req = joint.weld_throat * (shear_ed / fw_d.max(1e-9));
                b = b.remedy(Remedy::at_least(
                    subject_joint(joint, &format!("joints[id={}].weldThroat", joint.id)),
                    length_m(joint.weld_throat),
                    length_m(a_req),
                    loc(
                        &format!("Increase weld throat a to at least {:.1} mm.", a_req * 1000.0),
                        &format!("Nahtdicke a auf mindestens {:.1} mm erhöhen.", a_req * 1000.0),
                    ),
                ));
            }
            report.push(b.build());
        }
    }

    // Fatigue 1-9 — Palmgren-Miner
    for fat in &document.fatigue_details {
        let method = part_1_9::parse_method(&fat.method);
        let gmf = part_1_9::gamma_mf(method);
        params.gamma_mf = gmf;
        let delta_c = part_1_9::detail_category_pa(fat.category);
        let spectrum: Vec<(f64, f64)> = fat.spectrum.iter().map(|b| (b.delta_sigma, b.cycles)).collect();
        let damage = part_1_9::miner_damage(&spectrum, delta_c, gmf);
        let mut b = CheckResult::assess(
            format!("en1993.1-9.fat.{}", fat.id),
            "DIN EN 1993-1-9",
            ClauseId::new("EN 1993-1-9", "8", "8"),
            SubjectRef::new(&fat.id, &format!("fatigueDetails[id={}].spectrum", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id))),
            loc("Fatigue Palmgren-Miner damage", "Ermüdungsschädigung Palmgren-Miner"),
        )
        .annex(annex)
        .utilization(dimensionless(damage), dimensionless(1.0))
        .explanation(loc(
            &format!("D={:.3} (Σ n_i/N_i, cat. {}, γ_Mf={}, {} bands).", damage, fat.category, gmf, fat.spectrum.len()),
            &format!("Schädigung D={:.3} (Σ n_i/N_i, Kerbfall {}, γ_Mf={}, {} Stufen).", damage, fat.category, gmf, fat.spectrum.len()),
        ));
        if damage > 1.0 {
            let cats = [80u8, 90, 100, 112, 125, 140, 160];
            let options: Vec<String> = cats.iter().filter(|c| {
                part_1_9::miner_damage(&spectrum, part_1_9::detail_category_pa(**c), gmf) <= 1.0
            }).map(|c| c.to_string()).collect();
            if !options.is_empty() {
                b = b.remedy(Remedy::one_of(
                    SubjectRef::new(&fat.id, &format!("fatigueDetails[id={}].category", fat.id), loc("Detail category", "Kerbfall")),
                    options.clone(),
                    loc(
                        &format!("Improve detail category to one of: {}.", options.join(", ")),
                        &format!("Kerbfall verbessern auf eine von: {}.", options.join(", ")),
                    ),
                ));
            }
            if let Some(peak) = fat.spectrum.iter().max_by(|a, b| a.delta_sigma.partial_cmp(&b.delta_sigma).unwrap()) {
                let ds_req = peak.delta_sigma / damage.max(1.0).powf(1.0 / 3.0);
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new(&fat.id, &format!("fatigueDetails[id={}].spectrum[id={}].deltaSigma", fat.id, peak.id), loc("Stress range", "Spannungsschwingbreite")),
                    stress_pa(peak.delta_sigma),
                    stress_pa(ds_req),
                    loc(
                        &format!("Reduce peak Δσ to at most {:.1} MPa.", ds_req / 1e6),
                        &format!("Spitzen-Δσ auf höchstens {:.1} MPa reduzieren.", ds_req / 1e6),
                    ),
                ));
            }
        }
        report.push(b.build());
    }

    // Fire 1-2
    for fire in &document.fire_exposures {
        let rating = part_1_2::parse_rating(&fire.rating);
        let theta_heating = part_1_2::steel_temperature_c(
            rating,
            fire.section_factor,
            fire.protection_thickness,
            fire.protection_conductivity,
            fire.protection_density,
            fire.protection_specific_heat,
        );
        let theta_a = fire.design_temperature.max(theta_heating);
        let theta_cr = part_1_2::critical_temperature_c(fire.mu0);
        let t_req = part_1_2::required_protection_thickness(
            rating,
            fire.section_factor,
            fire.mu0,
            fire.protection_conductivity,
            fire.protection_density,
            fire.protection_specific_heat,
        );
        let ky = part_1_2::k_y_theta(theta_a);
        let ke = part_1_2::k_e_theta(theta_a);
        let lambda_theta = if ke > 1e-9 { (ky / ke).sqrt() } else { 10.0 };
        // χ_fi uses member fy if available else S355
        let fy_fire = document
            .members
            .iter()
            .find(|m| m.id == fire.member_id)
            .and_then(|m| find_material(document, &m.material_id))
            .map(|m| m.fy)
            .unwrap_or(355.0e6);
        let chi_fi = part_1_2::chi_fi(lambda_theta, fy_fire);

        let mut b = CheckResult::assess(
            format!("en1993.1-2.prot.{}", fire.id),
            "DIN EN 1993-1-2",
            ClauseId::new("EN 1993-1-2", "4", "4.2.5"),
            SubjectRef::new(&fire.id, &format!("fireExposures[id={}].protectionThickness", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brandschutz {}", fire.id))),
            loc("Fire protection thickness", "Brandschutzdicke"),
        )
        .annex(annex)
        .minimum(length_m(fire.protection_thickness), length_m(t_req))
        .explanation(loc(
            &format!("t_p={:.0} mm, required={:.0} mm (§4.2.5 incremental, Am/V={:.0} m⁻¹, θ_a={:.0} °C).", fire.protection_thickness * 1000.0, t_req * 1000.0, fire.section_factor, theta_a),
            &format!("t_p={:.0} mm, erforderlich={:.0} mm (§4.2.5 inkrementell, Am/V={:.0} m⁻¹, θ_a={:.0} °C).", fire.protection_thickness * 1000.0, t_req * 1000.0, fire.section_factor, theta_a),
        ));
        if fire.protection_thickness + 1e-9 < t_req {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&fire.id, &format!("fireExposures[id={}].protectionThickness", fire.id), loc("Protection thickness", "Brandschutzdicke")),
                length_m(fire.protection_thickness),
                length_m(t_req),
                loc(
                    &format!("Increase protection from {:.0} mm to at least {:.0} mm.", fire.protection_thickness * 1000.0, t_req * 1000.0),
                    &format!("Brandschutz von {:.0} mm auf mindestens {:.0} mm erhöhen.", fire.protection_thickness * 1000.0, t_req * 1000.0),
                ),
            ));
        }
        report.push(b.build());

        let mut b = CheckResult::assess(
            format!("en1993.1-2.theta.{}", fire.id),
            "DIN EN 1993-1-2",
            ClauseId::new("EN 1993-1-2", "4", "4.2.4"),
            SubjectRef::new(&fire.id, &format!("fireExposures[id={}].mu0", fire.id), loc(&format!("Fire {}", fire.id), &format!("Brand {}", fire.id))),
            loc("Critical steel temperature", "Kritische Stahltemperatur"),
        )
        .annex(annex)
        .minimum(temperature_c(theta_cr), temperature_c(theta_a))
        .explanation(loc(
            &format!("θ_a,cr={:.0} °C ≥ θ_a={:.0} °C (μ₀={:.2}, k_y,θ={:.2}, k_E,θ={:.2}, χ_fi={:.2}).", theta_cr, theta_a, fire.mu0, ky, ke, chi_fi),
            &format!("Kritische Stahltemperatur θ_a,cr={:.0} °C ≥ berechnete θ_a={:.0} °C (Ausnutzung μ₀={:.2}, Abminderung k_y,θ={:.2}, k_E,θ={:.2}, χ_fi={:.2}).", theta_cr, theta_a, fire.mu0, ky, ke, chi_fi),
        ));
        if theta_cr + 1e-6 < theta_a {
            let mut mu = fire.mu0;
            for _ in 0..40 {
                if part_1_2::critical_temperature_c(mu) >= theta_a {
                    break;
                }
                mu *= 0.95;
            }
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&fire.id, &format!("fireExposures[id={}].mu0", fire.id), loc("Load level μ0", "Ausnutzung μ0")),
                dimensionless(fire.mu0),
                dimensionless(mu),
                loc(
                    &format!("Reduce cold utilization μ₀ from {:.2} to at most {:.2}.", fire.mu0, mu),
                    &format!("Kaltausnutzung μ₀ von {:.2} auf höchstens {:.2} senken.", fire.mu0, mu),
                ),
            ));
            if t_req > fire.protection_thickness {
                b = b.remedy(Remedy::at_least(
                    SubjectRef::new(&fire.id, &format!("fireExposures[id={}].protectionThickness", fire.id), loc("Protection thickness", "Brandschutzdicke")),
                    length_m(fire.protection_thickness),
                    length_m(t_req),
                    loc(
                        &format!("Or increase protection to at least {:.0} mm.", t_req * 1000.0),
                        &format!("Oder Brandschutz auf mindestens {:.0} mm erhöhen.", t_req * 1000.0),
                    ),
                ));
            }
        }
        report.push(b.build());
    }

    // Cold-formed 1-3
    for cf in &document.cold_formed_members {
        let forces: Vec<(String, f64)> = cf.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let lp = part_1_3::lambda_p(cf.b_bar, cf.thickness, cf.fy, cf.k_sigma);
        let rho = part_1_3::reduction_factor(lp, cf.psi);
        let n_eff = rho * cf.gross_resistance;
        let mut b = CheckResult::assess(
            format!("en1993.1-3.cf.{}", cf.id),
            "DIN EN 1993-1-3",
            ClauseId::new("EN 1993-1-3", "5", "5.5.2"),
            SubjectRef::new(&cf.id, &format!("coldFormedMembers[id={}].thickness", cf.id), loc(&format!("CF {}", cf.id), &format!("Kaltprofil {}", cf.id))),
            loc("Cold-formed effective section", "Kaltprofil wirksamer Querschnitt"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(n_eff))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), N_eff,Rd={:.1} kN (ρ={:.3}, λ_p={:.3}).", n_ed / 1000.0, gov, n_eff / 1000.0, rho, lp),
            &format!("Kaltprofil N_Ed={:.1} kN (maßgebend {}), N_eff,Rd={:.1} kN (ρ={:.3}, λ_p={:.3}).", n_ed / 1000.0, gov, n_eff / 1000.0, rho, lp),
        ));
        if n_ed > n_eff {
            let t_req = cf.thickness * (n_ed / n_eff.max(1e-9)).sqrt();
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&cf.id, &format!("coldFormedMembers[id={}].thickness", cf.id), loc("Thickness", "Dicke")),
                length_m(cf.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase thickness from {:.2} mm to at least {:.2} mm.", cf.thickness * 1000.0, t_req * 1000.0),
                    &format!("Dicke von {:.2} mm auf mindestens {:.2} mm erhöhen.", cf.thickness * 1000.0, t_req * 1000.0),
                ),
            ));
            let gr_req = n_ed / rho.max(1e-9);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&cf.id, &format!("coldFormedMembers[id={}].grossResistance", cf.id), loc("Gross resistance", "Bruttotragfähigkeit")),
                force_n(cf.gross_resistance),
                force_n(gr_req),
                loc(
                    &format!("Or raise gross resistance to ≥ {:.1} kN.", gr_req / 1000.0),
                    &format!("Oder Bruttotragfähigkeit auf ≥ {:.1} kN anheben.", gr_req / 1000.0),
                ),
            ));
        }
        report.push(b.build());
    }

    // Stainless materials checked via members with kind=stainless
    for material in document.materials.iter().filter(|m| m.kind == "stainless") {
        for member in document.members.iter().filter(|m| m.material_id == material.id) {
            let Some(section) = find_section(document, &member.section_id) else { continue };
            let m_rd = part_1_4::bending_resistance_nm(section.w_pl_y, material.fy);
            let combinations = combine_member_actions(document, &member.id);
            if let Some(gov) = governing_effects(&combinations, "uls") {
                let mut b = CheckResult::assess(
                    format!("en1993.1-4.ss.{}", member.id),
                    "DIN EN 1993-1-4",
                    ClauseId::new("EN 1993-1-4", "6", "6"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("Stainless bending", "Nichtrostender Stahl Biegung"),
                )
                .annex(annex)
                .utilization(moment_nm(gov.action.my.abs()), moment_nm(m_rd))
                .explanation(loc(
                    &format!("M_Ed={:.1} kNm (gov. {}), M_Rd={:.1} kNm (γ_M=1.1).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0),
                    &format!("Edelstahl Biegung M_Ed={:.1} kNm (maßgebend {}), M_Rd={:.1} kNm (γ_M=1.1).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0),
                ));
                if gov.action.my.abs() > m_rd {
                    let grades = ["S355", "S460"].iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
                    b = b.remedy(Remedy::one_of(
                        subject_member(member, &format!("materials[id={}].grade", material.id)),
                        grades.clone(),
                        loc(
                            &format!("Increase steel grade; options: {}.", grades.join(", ")),
                            &format!("Stahlgüte erhöhen; Optionen: {}.", grades.join(", ")),
                        ),
                    ));
                }
                report.push(b.build());
            }
        }
    }

    // Plated 1-5
    for panel in &document.plated_panels {
        let forces: Vec<(String, f64)> = panel.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let sigma_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let lp = part_1_5::lambda_p(panel.b, panel.thickness, panel.fy, panel.k_sigma);
        let rd = part_1_5::local_buckling_stress_rd(panel.fy, lp, params);
        let mut b = CheckResult::assess(
            format!("en1993.1-5.plate.{}", panel.id),
            "DIN EN 1993-1-5",
            ClauseId::new("EN 1993-1-5", "4", "4.1"),
            SubjectRef::new(&panel.id, &format!("platedPanels[id={}].thickness", panel.id), loc(&format!("Plate {}", panel.id), &format!("Beulblech {}", panel.id))),
            loc("Plate buckling", "Plattenbeulen"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma_ed), stress_pa(rd))
        .explanation(loc(
            &format!("σ_Ed={:.1} MPa (gov. {}), σ_c,Rd={:.1} MPa (λ_p={:.3}).", sigma_ed / 1e6, gov, rd / 1e6, lp),
            &format!("Plattenbeulen σ_Ed={:.1} MPa (maßgebend {}), σ_c,Rd={:.1} MPa (λ_p={:.3}).", sigma_ed / 1e6, gov, rd / 1e6, lp),
        ));
        if sigma_ed > rd {
            let t_req = panel.thickness * (sigma_ed / rd.max(1e-9)).sqrt();
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&panel.id, &format!("platedPanels[id={}].thickness", panel.id), loc("Thickness", "Dicke")),
                length_m(panel.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase plate thickness to at least {:.1} mm.", t_req * 1000.0),
                    &format!("Blechdicke auf mindestens {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&panel.id, &format!("platedPanels[id={}].b", panel.id), loc("Panel width", "Blechbreite")),
                length_m(panel.b),
                length_m(panel.b * (rd / sigma_ed.max(1e-9)).sqrt()),
                loc(
                    &format!("Or reduce panel width b."),
                    &format!("Oder Blechbreite b verringern."),
                ),
            ));
        }
        report.push(b.build());
    }

    // Shell / silo 1-6 + 4-1
    for silo in &document.silo_shells {
        let p_h = part_4::janssen_pressure(silo.k, silo.gamma, silo.depth);
        let sigma_ed = part_4::membrane_hoop_stress(p_h, silo.radius, silo.thickness);
        let sigma_rcr = part_1_6::sigma_x_rcr(silo.thickness, silo.radius, 210.0e9);
        let lam = part_1_6::lambda_bar(silo.fy, sigma_rcr);
        let alpha = part_1_6::alpha_imperfection(silo.radius, silo.thickness);
        let chi = part_1_6::chi(lam, alpha);
        let sigma_rd = part_1_6::design_resistance(silo.fy, chi, params);
        let mut b = CheckResult::assess(
            format!("en1993.1-6.shell.{}", silo.id),
            "DIN EN 1993-1-6",
            ClauseId::new("EN 1993-1-6", "8", "8.5.2"),
            SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc(&format!("Shell {}", silo.id), &format!("Schale {}", silo.id))),
            loc("Shell buckling", "Schalenbeulen"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma_ed), stress_pa(sigma_rd))
        .explanation(loc(
            &format!("σ_Ed={:.1} MPa (Janssen hoop), σ_x,Rd={:.1} MPa (χ={:.3}).", sigma_ed / 1e6, sigma_rd / 1e6, chi),
            &format!("Schalenbeulen σ_Ed={:.1} MPa (Janssen Ringspannung), σ_x,Rd={:.1} MPa (χ={:.3}).", sigma_ed / 1e6, sigma_rd / 1e6, chi),
        ));
        if sigma_ed > sigma_rd {
            let t_req = silo.thickness * (sigma_ed / sigma_rd);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc("Wall thickness", "Wanddicke")),
                length_m(silo.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase silo wall thickness to at least {:.1} mm.", t_req * 1000.0),
                    &format!("Silowanddicke auf mindestens {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&silo.id, &format!("siloShells[id={}].depth", silo.id), loc("Fill depth", "Füllhöhe")),
                length_m(silo.depth),
                length_m(silo.depth * sigma_rd / sigma_ed.max(1e-9)),
                loc(
                    &format!("Or reduce fill depth."),
                    &format!("Oder Füllhöhe verringern."),
                ),
            ));
        }
        report.push(b.build());

        let mut b = CheckResult::assess(
            format!("en1993.4-1.silo.{}", silo.id),
            "DIN EN 1993-4-1",
            ClauseId::new("EN 1993-4-1", "5", "5.3"),
            SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc(&format!("Silo {}", silo.id),
                    &format!("Nachweis: Silo {}", silo.id))),
            loc("Silo wall membrane stress", "Silowand Membranspannung"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma_ed), stress_pa(sigma_rd))
        .explanation(loc(
            &format!("Janssen hoop membrane σ_θ,Ed={:.1} MPa versus shell design resistance.", sigma_ed / 1e6),
            &format!("Janssen-Ringspannung σ_θ,Ed={:.1} MPa gegenüber Schalenbemessungswiderstand.", sigma_ed / 1e6),
        ));
        if sigma_ed > sigma_rd {
            let t_req = silo.thickness * (sigma_ed / sigma_rd);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&silo.id, &format!("siloShells[id={}].thickness", silo.id), loc("Wall thickness", "Wanddicke")),
                length_m(silo.thickness),
                length_m(t_req),
                loc(
                    &format!("Increase thickness to ≥ {:.1} mm.", t_req * 1000.0),
                    &format!("Dicke auf ≥ {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
        }
        report.push(b.build());
    }

    // 1-10 through-thickness
    for member in &document.members {
        let Some(section) = find_section(document, &member.section_id) else { continue };
        let Some(material) = find_material(document, &member.material_id) else { continue };
        let t = section.tf.max(section.tw);
        let t_max = part_1_10::max_permissible_thickness(&material.subgrade, 0.0);
        let mut b = CheckResult::assess(
            format!("en1993.1-10.t.{}", member.id),
            "DIN EN 1993-1-10",
            ClauseId::new("EN 1993-1-10", "2", "2.1"),
            subject_member(member, &format!("materials[id={}].subgrade", material.id)),
            loc("Through-thickness toughness", "Zähigkeitsanforderung"),
        )
        .annex(annex)
        .utilization(length_m(t), length_m(t_max))
        .explanation(loc(
            &format!("t={:.0} mm, t_max={:.0} mm for subgrade {}.", t * 1000.0, t_max * 1000.0, material.subgrade),
            &format!("t={:.0} mm, t_max={:.0} mm für Güte {}.", t * 1000.0, t_max * 1000.0, material.subgrade),
        ));
        if t > t_max {
            let options = ["J2", "K2", "M", "N"].iter().filter(|s| part_1_10::max_permissible_thickness(s, 0.0) >= t).map(|s| (*s).to_string()).collect::<Vec<_>>();
            b = b.remedy(Remedy::one_of(
                subject_member(member, &format!("materials[id={}].subgrade", material.id)),
                options.clone(),
                loc(
                    &format!("Upgrade steel subgrade to one of: {}.", options.join(", ")),
                    &format!("Stahlgütegruppe anheben auf: {}.", options.join(", ")),
                ),
            ));
        }
        report.push(b.build());
    }

    // 1-11 tension components
    for tc in &document.tension_components {
        let forces: Vec<(String, f64)> = tc.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let rd = part_1_11::tension_component_resistance_n(tc.f_uk, tc.f_k);
        let mut b = CheckResult::assess(
            format!("en1993.1-11.tc.{}", tc.id),
            "DIN EN 1993-1-11",
            ClauseId::new("EN 1993-1-11", "6", "6.2"),
            SubjectRef::new(&tc.id, &format!("tensionComponents[id={}].actions", tc.id), loc(&format!("Cable {}", tc.id), &format!("Zugglied {}", tc.id))),
            loc("Tension component", "Zugglied"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(rd))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), F_Rd={:.1} kN.", n_ed / 1000.0, gov, rd / 1000.0),
            &format!("Zugglied N_Ed={:.1} kN (maßgebend {}), F_Rd={:.1} kN.", n_ed / 1000.0, gov, rd / 1000.0),
        ));
        if n_ed > rd {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&tc.id, &format!("tensionComponents[id={}].fK", tc.id), loc("Characteristic resistance", "Charakteristische Tragfähigkeit")),
                force_n(tc.f_k),
                force_n(n_ed),
                loc(
                    &format!("Increase F_k to at least {:.1} kN.", n_ed / 1000.0),
                    &format!("F_k auf mindestens {:.1} kN erhöhen.", n_ed / 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&tc.id, &format!("tensionComponents[id={}].fUk", tc.id), loc("Ultimate strength", "Bruchfestigkeit")),
                force_n(tc.f_uk),
                force_n(n_ed * 1.5),
                loc(
                    &format!("Or raise F_uk to ≥ {:.1} kN.", n_ed * 1.5 / 1000.0),
                    &format!("Oder F_uk auf ≥ {:.1} kN anheben.", n_ed * 1.5 / 1000.0),
                ),
            ));
        }
        report.push(b.build());
    }

    // 1-12 HSS materials
    for material in document.materials.iter().filter(|m| m.kind == "hss" || m.fy >= 460.0e6) {
        for member in document.members.iter().filter(|m| m.material_id == material.id) {
            let Some(section) = find_section(document, &member.section_id) else { continue };
            let class = part_1_1::section_class_rolled_i(section, material.fy);
            let m_rd = part_1_12::elastic_bending_resistance_nm(section.w_el_y, material.fy, class, params);
            let combinations = combine_member_actions(document, &member.id);
            if let Some(gov) = governing_effects(&combinations, "uls") {
                let mut b = CheckResult::assess(
                    format!("en1993.1-12.hss.{}", member.id),
                    "DIN EN 1993-1-12",
                    ClauseId::new("EN 1993-1-12", "4", "4.1"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("HSS elastic bending", "HSS elastische Biegung"),
                )
                .annex(annex)
                .utilization(moment_nm(gov.action.my.abs()), moment_nm(m_rd.max(1.0)))
                .explanation(loc(
                    &format!("M_Ed={:.1} kNm (gov. {}), M_el,Rd={:.1} kNm (class {}).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0, class),
                    &format!("HSS Biegung M_Ed={:.1} kNm (maßgebend {}), M_el,Rd={:.1} kNm (Klasse {}).", gov.action.my.abs() / 1000.0, gov.combination_id, m_rd / 1000.0, class),
                ));
                if gov.action.my.abs() > m_rd {
                    let options = next_section_options(section, section.area, section.w_pl_y);
                    b = b.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Select stockier section: {}.", options.join(", ")),
                            &format!("Gedrungeneren Querschnitt wählen: {}.", options.join(", ")),
                        ),
                    ));
                }
                report.push(b.build());
            }
        }
    }

    // Bridge 2
    for bf in &document.bridge_fatigue {
        let method = part_1_9::parse_method(&bf.method);
        let gmf = part_1_9::gamma_mf(method);
        let delta_e2 = part_2::damage_equivalent_stress(bf.lambda, bf.phi2, bf.delta_sigma_p);
        let limit = part_1_9::detail_category_pa(bf.category) / gmf;
        let mut b = CheckResult::assess(
            format!("en1993.2.fat.{}", bf.id),
            "DIN EN 1993-2",
            ClauseId::new("EN 1993-2", "9", "9.3"),
            SubjectRef::new(&bf.id, &format!("bridgeFatigue[id={}].deltaSigmaP", bf.id), loc(&format!("Bridge {}", bf.id), &format!("Brücke {}", bf.id))),
            loc("Bridge damage-equivalent fatigue", "Schädigungsäquivalente Ermüdung Brücke"),
        )
        .annex(annex)
        .utilization(stress_pa(delta_e2), stress_pa(limit))
        .explanation(loc(
            &format!("Δσ_E2={:.1} MPa (λ={:.2}, φ2={:.2}), limit={:.1} MPa.", delta_e2 / 1e6, bf.lambda, bf.phi2, limit / 1e6),
            &format!("Schädigungsäquivalente Δσ_E2={:.1} MPa (λ={:.2}, φ2={:.2}), Grenzwert={:.1} MPa.", delta_e2 / 1e6, bf.lambda, bf.phi2, limit / 1e6),
        ));
        if delta_e2 > limit {
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&bf.id, &format!("bridgeFatigue[id={}].deltaSigmaP", bf.id), loc("Stress range", "Spannungsschwingbreite")),
                stress_pa(bf.delta_sigma_p),
                stress_pa(limit / (bf.lambda * bf.phi2).max(1e-9)),
                loc(
                    &format!("Reduce Δσ_p to at most {:.1} MPa.", limit / (bf.lambda * bf.phi2).max(1e-9) / 1e6),
                    &format!("Δσ_p auf höchstens {:.1} MPa reduzieren.", limit / (bf.lambda * bf.phi2).max(1e-9) / 1e6),
                ),
            ));
            b = b.remedy(Remedy::one_of(
                SubjectRef::new(&bf.id, &format!("bridgeFatigue[id={}].category", bf.id), loc("Detail category", "Kerbfall")),
                vec!["100".into(), "112".into(), "125".into()],
                loc(
                    &format!("Or improve bridge detail category."),
                    &format!("Oder Brücken-Kerbfall verbessern."),
                ),
            ));
        }
        report.push(b.build());

        if let Some(member) = document.members.iter().find(|m| m.id == bf.member_id) {
            if let (Some(section), Some(material)) = (find_section(document, &member.section_id), find_material(document, &member.material_id)) {
                let combinations = combine_member_actions(document, &member.id);
                if let Some(gov) = governing_effects(&combinations, "uls") {
                    let n_rd = part_1_1::axial_resistance_n(section.area, material.fy, params);
                    let m_rd = part_1_1::bending_resistance_nm(section.w_pl_y, material.fy, 2, params);
                    let eta = part_2::bridge_interaction_eta(gov.action.n, n_rd, gov.action.my, m_rd);
                    let mut b = CheckResult::assess(
                        format!("en1993.2.int.{}", bf.id),
                        "DIN EN 1993-2",
                        ClauseId::new("EN 1993-2", "6", "6"),
                        subject_member(member, &format!("members[id={}].id", member.id)),
                        loc("Bridge member interaction", "Brückenbauteil Interaktion"),
                    )
                    .annex(annex)
                    .utilization(dimensionless(eta), dimensionless(1.0))
                    .explanation(loc(
                        &format!("Bridge interaction η={:.3} (gov. {}).", eta, gov.combination_id),
                        &format!("Brücken-Interaktion η={:.3} (maßgebende Kombination {}).", eta, gov.combination_id),
                    ));
                    if eta > 1.0 {
                        let options = next_section_options(section, section.area * eta, section.w_pl_y * eta);
                        b = b.remedy(Remedy::one_of(
                            subject_member(member, &format!("members[id={}].sectionId", member.id)),
                            options.clone(),
                            loc(&format!("Upsize: {}.", options.join(", ")), &format!("Querschnitt vergrößern: {}.", options.join(", "))),
                        ));
                    }
                    report.push(b.build());
                }
            }
        }
    }

    // Tower 3-1
    for tower in &document.tower_legs {
        let Some(member) = document.members.iter().find(|m| m.id == tower.member_id) else { continue };
        let Some(section) = find_section(document, &member.section_id) else { continue };
        let Some(material) = find_material(document, &member.material_id) else { continue };
        let forces: Vec<(String, f64)> = tower.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let iy = part_1_1::radius_of_gyration(section.iy, section.area);
        let curve = part_1_1::buckling_curve_rolled_i(section, 'y', material.fy);
        let lam = part_1_1::lambda_bar(member.buckling_length_y, iy, material.e_modulus, material.fy);
        let chi = part_1_1::chi(lam, curve);
        let n_rd = part_3::tower_buckling_n(section.area, material.fy, chi, tower.force_coefficient, tower.dynamic_factor, params);
        let mut b = CheckResult::assess(
            format!("en1993.3-1.tower.{}", tower.id),
            "DIN EN 1993-3-1",
            ClauseId::new("EN 1993-3-1", "5", "5"),
            SubjectRef::new(&tower.id, &format!("towerLegs[id={}].actions", tower.id), loc(&format!("Tower {}", tower.id), &format!("Turm {}", tower.id))),
            loc("Tower leg buckling", "Turmstiel Knicken"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(n_rd))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), N_b,Rd={:.1} kN (χ={:.3}, c_f={:.2}, c_d={:.2} Annex B).", n_ed / 1000.0, gov, n_rd / 1000.0, chi, tower.force_coefficient, tower.dynamic_factor),
            &format!("Turmstiel N_Ed={:.1} kN (maßgebend {}), N_b,Rd={:.1} kN (χ={:.3}, c_f={:.2}, c_d={:.2} Anhang B).", n_ed / 1000.0, gov, n_rd / 1000.0, chi, tower.force_coefficient, tower.dynamic_factor),
        ));
        if n_ed > n_rd {
            let options = next_section_options(section, section.area * n_ed / n_rd.max(1e-9), section.w_pl_y);
            b = b.remedy(Remedy::one_of(
                subject_member(member, &format!("members[id={}].sectionId", member.id)),
                options.clone(),
                loc(&format!("Upsize tower section: {}.", options.join(", ")), &format!("Turmquerschnitt vergrößern: {}.", options.join(", "))),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&tower.id, &format!("towerLegs[id={}].forceCoefficient", tower.id), loc("Force coefficient", "Kraftbeiwert")),
                dimensionless(tower.force_coefficient),
                dimensionless((tower.force_coefficient * n_rd / n_ed.max(1e-9)).max(0.5)),
                loc(
                    &format!("Or reduce force coefficient c_f."),
                    &format!("Oder Kraftbeiwert c_f verringern."),
                ),
            ));
        }
        report.push(b.build());
    }

    // Piles 5
    for pile in &document.piles {
        let Some(section) = find_section(document, &pile.section_id) else { continue };
        let Some(material) = find_material(document, &pile.material_id) else { continue };
        let limit = part_5::pile_driving_stress_limit(material.fy);
        let mut b = CheckResult::assess(
            format!("en1993.5.drive.{}", pile.id),
            "DIN EN 1993-5",
            ClauseId::new("EN 1993-5", "12", "12.1"),
            SubjectRef::new(&pile.id, &format!("piles[id={}].drivingStress", pile.id), loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id))),
            loc("Pile driving stress", "Rammspannung"),
        )
        .annex(annex)
        .utilization(stress_pa(pile.driving_stress), stress_pa(limit))
        .explanation(loc(
            &format!("σ={:.0} MPa, limit 0.9·f_y={:.0} MPa.", pile.driving_stress / 1e6, limit / 1e6),
            &format!("Rammspannung σ={:.0} MPa, Grenze 0.9·f_y={:.0} MPa.", pile.driving_stress / 1e6, limit / 1e6),
        ));
        if pile.driving_stress > limit {
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&pile.id, &format!("piles[id={}].drivingStress", pile.id), loc("Driving stress", "Rammspannung")),
                stress_pa(pile.driving_stress),
                stress_pa(limit),
                loc(
                    &format!("Reduce driving stress to ≤ {:.0} MPa.", limit / 1e6),
                    &format!("Rammspannung auf ≤ {:.0} MPa reduzieren.", limit / 1e6),
                ),
            ));
        }
        report.push(b.build());

        let k_red = part_5::pile_soil_reduction(pile.driving_stress, material.fy, pile.embedded_length, pile.shaft_perimeter);
        let n_rd = part_5::pile_compression_n(section.area, material.fy, k_red, params);
        let forces: Vec<(String, f64)> = pile.actions.iter().map(|a| (a.load_case_id.clone(), a.force)).collect();
        let combos = combine_scalar_forces(document, &forces);
        let n_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let mut b = CheckResult::assess(
            format!("en1993.5.comp.{}", pile.id),
            "DIN EN 1993-5",
            ClauseId::new("EN 1993-5", "6", "6.1"),
            SubjectRef::new(&pile.id, &format!("piles[id={}].actions", pile.id), loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id))),
            loc("Pile compression", "Pfahl Druck"),
        )
        .annex(annex)
        .utilization(force_n(n_ed), force_n(n_rd))
        .explanation(loc(
            &format!("N_Ed={:.1} kN (gov. {}), N_c,Rd={:.1} kN (k_red={:.2} derived from driving/geometry).", n_ed / 1000.0, gov, n_rd / 1000.0, k_red),
            &format!("Pfahl Druck N_Ed={:.1} kN (maßgebend {}), N_c,Rd={:.1} kN (k_red={:.2} aus Rammspannung/Geometrie abgeleitet).", n_ed / 1000.0, gov, n_rd / 1000.0, k_red),
        ));
        if n_ed > n_rd {
            let options = next_section_options(section, section.area * n_ed / n_rd.max(1e-9), section.w_pl_y);
            b = b.remedy(Remedy::one_of(
                SubjectRef::new(&pile.id, &format!("piles[id={}].sectionId", pile.id), loc("Pile section", "Pfahlquerschnitt")),
                options.clone(),
                loc(&format!("Upsize pile section: {}.", options.join(", ")), &format!("Pfahlquerschnitt vergrößern: {}.", options.join(", "))),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&pile.id, &format!("piles[id={}].drivingStress", pile.id), loc("Driving stress", "Rammspannung")),
                stress_pa(pile.driving_stress),
                stress_pa(0.6 * material.fy),
                loc(
                    &format!("Or reduce driving stress to raise k_red."),
                    &format!("Oder Rammspannung senken, um k_red anzuheben."),
                ),
            ));
        }
        report.push(b.build());
    }

    // Crane 6
    for crane in &document.crane_runways {
        let forces: Vec<(String, f64)> = crane.actions.iter().map(|a| (a.load_case_id.clone(), a.force * crane.phi.max(1.0))).collect();
        let combos = combine_scalar_forces(document, &forces);
        let f_ed = governing_scalar(&combos, "uls").map(|c| c.value).unwrap_or(0.0);
        let gov = governing_scalar(&combos, "uls").map(|c| c.combination_id.as_str()).unwrap_or("—");
        let l_eff = part_6::effective_length(crane.wheel_contact_length, crane.dispersion);
        let sigma = part_6::wheel_load_web_stress(f_ed, l_eff, crane.web_thickness);
        let limit = crane.fy / params.gamma_m0;
        let mut b = CheckResult::assess(
            format!("en1993.6.crane.{}", crane.id),
            "DIN EN 1993-6",
            ClauseId::new("EN 1993-6", "5", "5.7.1"),
            SubjectRef::new(&crane.id, &format!("craneRunways[id={}].webThickness", crane.id), loc(&format!("Crane {}", crane.id), &format!("Kranbahn {}", crane.id))),
            loc("Crane runway web stress", "Kranbahn Stegspannung"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma), stress_pa(limit))
        .explanation(loc(
            &format!("σ_oz={:.1} MPa (gov. {}, φ={:.2}), f_y/γ_M0={:.1} MPa.", sigma / 1e6, gov, crane.phi, limit / 1e6),
            &format!("Kranbahn-Stegspannung σ_oz={:.1} MPa (maßgebend {}, φ={:.2}), f_y/γ_M0={:.1} MPa.", sigma / 1e6, gov, crane.phi, limit / 1e6),
        ));
        if sigma > limit {
            let t_req = f_ed / (l_eff * limit);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new(&crane.id, &format!("craneRunways[id={}].webThickness", crane.id), loc("Web thickness", "Stegdicke")),
                length_m(crane.web_thickness),
                length_m(t_req),
                loc(
                    &format!("Increase web thickness to at least {:.1} mm.", t_req * 1000.0),
                    &format!("Stegdicke auf mindestens {:.1} mm erhöhen.", t_req * 1000.0),
                ),
            ));
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&crane.id, &format!("craneRunways[id={}].phi", crane.id), loc("Dynamic factor", "Dynamikfaktor")),
                dimensionless(crane.phi),
                dimensionless((crane.phi * limit / sigma.max(1e-9)).max(1.0)),
                loc(
                    &format!("Or reduce dynamic factor φ."),
                    &format!("Oder Dynamikfaktor φ verringern."),
                ),
            ));
        }
        report.push(b.build());
    }


    report
}
//#endregion 🔖️ComplianceHelpers

#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
