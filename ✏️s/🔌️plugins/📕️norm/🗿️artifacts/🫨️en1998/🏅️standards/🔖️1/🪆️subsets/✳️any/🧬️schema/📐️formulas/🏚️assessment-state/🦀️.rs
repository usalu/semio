//! 🏚️ EN 1998-3 assessment state: analysis method gating by knowledge level (AS-08), material strengths from tests (AS-03), cyclic shear capacity V_R (AS-06) and the verification rule of ductile and brittle mechanisms (AS-07). See [`super::part_3`] and the spec table 3.3 of `📐️specs/en1997-en1998-target.md`.

use super::part_3::{self, AnnexA, RcMember};
use semio_s_artifact_norm_en1998_model::{En1998Assessment, KnowledgeLevel};
use semio_s_plugin_norm_compliance::{ndp_number, AnnexChoice};

/// 🧭 Analysis method of an assessment (EN 1998-3 4.3): lateral force, modal response spectrum, nonlinear static (pushover) or nonlinear time history.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalysisMethod {
    LateralForce,
    ModalResponseSpectrum,
    NonlinearStatic,
    NonlinearTimeHistory,
}

impl AnalysisMethod {
    pub const ALL: [Self; 4] = [Self::LateralForce, Self::ModalResponseSpectrum, Self::NonlinearStatic, Self::NonlinearTimeHistory];

    pub const fn token(self) -> &'static str {
        match self {
            Self::LateralForce => "lf",
            Self::ModalResponseSpectrum => "mrs",
            Self::NonlinearStatic => "pushover",
            Self::NonlinearTimeHistory => "nlth",
        }
    }

    /// 🧭 Methods permitted by the knowledge level: KL1 lateral force or modal response spectrum, KL2 and KL3 all (EN 1998-3 Table 3.1).
    pub const fn permitted_at(self, kl: KnowledgeLevel) -> bool {
        matches!(self, Self::LateralForce | Self::ModalResponseSpectrum) || !matches!(kl, KnowledgeLevel::Kl1)
    }

    /// 🧭 The methods a knowledge level permits.
    pub fn permitted_methods(kl: KnowledgeLevel) -> Vec<Self> {
        Self::ALL.into_iter().filter(|m| m.permitted_at(kl)).collect()
    }
}

/// 🔍 Failure mechanism class of the verification rule (EN 1998-3 4.4): ductile mechanisms are verified by the demand over capacity ratio D/C ≤ 1 with the mean strengths over CF, brittle ones by E_d ≤ R_d with the mean strengths over CF and the partial factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mechanism {
    Ductile,
    Brittle,
}

impl Mechanism {
    /// 🔍 Strength of the mechanism from the mean value of tests: f_m/CF (ductile) or f_m/(CF γ_m) (brittle).
    pub fn design_strength(self, f_m: f64, cf: f64, gamma_m: f64) -> f64 {
        if cf <= 0.0 || gamma_m <= 0.0 {
            return 0.0;
        }
        match self {
            Self::Ductile => f_m / cf,
            Self::Brittle => f_m / (cf * gamma_m),
        }
    }
}

/// 🔤 Lower-case key `kl1`, `kl2` or `kl3` of the knowledge level.
pub const fn knowledge_level_token(kl: KnowledgeLevel) -> &'static str {
    match kl {
        KnowledgeLevel::Kl1 => "kl1",
        KnowledgeLevel::Kl2 => "kl2",
        KnowledgeLevel::Kl3 => "kl3",
    }
}

/// 🔍 Demand over capacity D/C; infinite for a demand without capacity, zero without demand.
pub fn demand_capacity_ratio(demand: f64, capacity: f64) -> f64 {
    if capacity > 0.0 { demand / capacity } else if demand > 0.0 { f64::INFINITY } else { 0.0 }
}

/// 🔍 Whether a mechanism is verified: D/C ≤ 1 for ductile and E_d ≤ R_d for brittle mechanisms are the same ratio rule on their own capacities.
pub fn verified(ratio: f64) -> bool {
    ratio <= 1.0
}

/// 🔍 Knowledge level, confidence factor and return period of an assessment: CF of Table 3.1 as applied (1 for the limit state DL), the table CF, T_R of the limit state and the a_g factor (T_R/475)^(1/k).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Basis {
    pub knowledge_level: KnowledgeLevel,
    pub cf_table: f64,
    pub cf: f64,
    pub return_period_yr: f64,
    pub ag_factor: f64,
}

pub fn basis(a: &En1998Assessment, annex: AnnexChoice) -> Basis {
    let ls = a.limit_state.token();
    let cf_table = part_3::confidence_factor(a.knowledge_level);
    Basis { knowledge_level: a.knowledge_level, cf_table, cf: if part_3::limit_state_applies_cf(&ls) { cf_table } else { 1.0 }, return_period_yr: part_3::limit_state_return_period_yr(&ls, annex), ag_factor: part_3::limit_state_ag_factor(&ls, annex) }
}

/// 🔍 Unverified partial factors γ_c and γ_s of the brittle strengths (EN 1992-1-1 persistent situation, registry rows of EN 1992).
pub fn material_factors(annex: AnnexChoice) -> (f64, f64) {
    (ndp_number("ndp.en1992.m-01.gamma-c-persistent", annex), ndp_number("ndp.en1992.gamma-s", annex))
}

/// 🔧 Coefficients of the cyclic shear capacity of EN 1998-3 A.3.3 from the registry (unverified rows).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShearCoefficients {
    pub axial_limit: f64,
    pub ductility_cap: f64,
    pub ductility_slope: f64,
    pub base: f64,
    pub rho_min: f64,
    pub slenderness_slope: f64,
    pub slenderness_cap: f64,
}

impl ShearCoefficients {
    pub fn from_registry(annex: AnnexChoice) -> Self {
        let n = |name: &str| ndp_number(&format!("ndp.en1998.en3.shear.{name}"), annex);
        Self { axial_limit: n("axial-limit"), ductility_cap: n("ductility-cap"), ductility_slope: n("ductility-slope"), base: n("base"), rho_min: n("rho-min"), slenderness_slope: n("slenderness-slope"), slenderness_cap: n("slenderness-cap") }
    }
}

/// 🔍 Plastic part of the ductility demand μ_Δ^pl = max(θ_Ed/θ_y − 1, 0); the definition μ_Δ = θ_Ed/θ_y is the assumption of this kernel, not a registry value.
pub fn plastic_ductility(theta_ed: f64, theta_y: f64) -> f64 {
    if theta_y > 0.0 { (theta_ed / theta_y - 1.0).max(0.0) } else { 0.0 }
}

/// 🔍 Terms of the cyclic shear capacity V_R = (1/γ_el)[(h − x)/(2 L_V) min(N; 0.55 A_c f_c) + (1 − 0.05 min(5; μ_pl)) (0.16 max(0.5; 100 ρ_tot)(1 − 0.16 min(5; L_V/h)) √f_c A_c + V_w)] in newtons.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CyclicShear {
    pub axial_n: f64,
    pub concrete_n: f64,
    pub stirrup_n: f64,
    pub degradation: f64,
    pub neutral_axis_m: f64,
    pub plastic_ductility: f64,
    pub resistance_n: f64,
}

/// 🔍 Cyclic shear capacity of a member with the (brittle) design strengths in `m`, the neutral axis depth x, the plastic ductility demand and γ_el; N is taken compressive positive and tension gives no axial term, ρ_tot = (A_s1 + A_s2 + A_s,int)/(b h) counts all longitudinal reinforcement, the end layers and the intermediate bars, V_w = ρ_w b z f_yw with z = 2d − h.
pub fn cyclic_shear_capacity(m: &RcMember, x_m: f64, mu_pl: f64, gamma_el: f64, k: &ShearCoefficients) -> CyclicShear {
    let a_c = m.b * m.h;
    let axial_n = if m.l_v > 0.0 { (m.h - x_m).max(0.0) / (2.0 * m.l_v) * m.n.max(0.0).min(k.axial_limit * a_c * m.f_c * 1.0e6) } else { 0.0 };
    let rho_tot = (m.a_s1 + m.a_s2 + m.a_si) / a_c;
    let slenderness = if m.h > 0.0 { m.l_v / m.h } else { 0.0 };
    let concrete_n = k.base * k.rho_min.max(100.0 * rho_tot) * (1.0 - k.slenderness_slope * slenderness.min(k.slenderness_cap)) * m.f_c.sqrt() * a_c * 1.0e6;
    let stirrup_n = part_3::transverse_ratio(m) * m.b * (2.0 * m.d - m.h).max(0.0) * m.f_yw * 1.0e6;
    let degradation = 1.0 - k.ductility_slope * mu_pl.min(k.ductility_cap);
    let resistance_n = if gamma_el > 0.0 { (axial_n + degradation * (concrete_n + stirrup_n)) / gamma_el } else { 0.0 };
    CyclicShear { axial_n, concrete_n, stirrup_n, degradation, neutral_axis_m: x_m, plastic_ductility: mu_pl, resistance_n }
}

/// 🔍 Cyclic shear capacity V_R of an assessment with the brittle strengths f_m/(CF γ_m) and the plastic ductility from the demand and the design yield rotation; `None` while the section is not given.
pub fn assessment_shear_capacity(a: &En1998Assessment, annex: AnnexChoice) -> Option<CyclicShear> {
    let member = RcMember::of(a)?;
    let b = basis(a, annex);
    let k = AnnexA::from_registry(annex);
    let (gamma_c, gamma_s) = material_factors(annex);
    let caps = part_3::design_capacities(a, &k, b.cf);
    let strengths = caps.member.unwrap_or(member.over_cf(b.cf));
    let x_m = strengths.yield_neutral_axis_depth_m(&k).unwrap_or_else(|| part_3::yield_neutral_axis_ratio(&strengths, k.steel_modulus_mpa).max(0.0) * member.d);
    let mu_pl = plastic_ductility(a.chord_rotation_demand_rad, caps.yield_rad);
    let brittle = RcMember { f_c: Mechanism::Brittle.design_strength(member.f_c, b.cf, gamma_c), f_yw: Mechanism::Brittle.design_strength(member.f_yw, b.cf, gamma_s), ..member };
    Some(cyclic_shear_capacity(&brittle, x_m, mu_pl, a.gamma_el, &ShearCoefficients::from_registry(annex)))
}

/// 🔍 Demand over capacity of the two mechanisms of an assessed member and the capacities they rest on: ductile D/C = θ_Ed/θ_Rd of the limit state, brittle E_d/R_d = V_Ed/V_R.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MemberRatios {
    pub chord: Option<f64>,
    pub chord_capacity_rad: f64,
    pub shear: Option<f64>,
    pub shear_capacity_n: f64,
}

impl MemberRatios {
    pub fn governing(&self) -> Option<f64> {
        [self.chord, self.shear].into_iter().flatten().fold(None, |worst: Option<f64>, next| Some(worst.map_or(next, |w| w.max(next))))
    }
}

pub fn member_ratios(a: &En1998Assessment, annex: AnnexChoice) -> MemberRatios {
    let b = basis(a, annex);
    let k = AnnexA::from_registry(annex);
    let caps = part_3::design_capacities(a, &k, b.cf);
    let sd_factor = ndp_number("ndp.en1998.en3.chord-rotation.sd-factor", annex);
    let chord_capacity_rad = part_3::chord_rotation_capacity_rad(&a.limit_state.token(), caps.yield_rad, caps.ultimate_rad, sd_factor).unwrap_or(0.0);
    let chord = (a.chord_rotation_demand_rad > 0.0).then(|| demand_capacity_ratio(a.chord_rotation_demand_rad, chord_capacity_rad));
    let shear_capacity_n = assessment_shear_capacity(a, annex).map_or(0.0, |s| s.resistance_n);
    let shear = (a.shear_force_demand_n > 0.0).then(|| demand_capacity_ratio(a.shear_force_demand_n, shear_capacity_n));
    MemberRatios { chord, chord_capacity_rad, shear, shear_capacity_n }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
