//! ⚙️ EN 1993 design of steel structures — headless compute (constitutional: engine).

use crate::artifacts::en1993::En1993Snapshot;
use crate::artifacts::en1993::mutations::En1993Mutation;
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, NormFamily, NormFamilyId, NormHost, Quantity};

// #region 🔖️AnnexParams
/// 🇪️🇺️🇩️🇪️ Resolved partial safety factors for one national annex choice, threaded through every EN 1993 formula.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub choice: AnnexChoice,
    pub gamma_m0: f64,
    pub gamma_m1: f64,
    pub gamma_m2: f64,
    pub gamma_mf: f64,
}

impl AnnexParams {
    /// 🇪️🇺️ EN 1993-1-1 §6.1 base recommended values (no DIN EN NA increase on γ_M1).
    pub fn en() -> Self {
        Self { choice: AnnexChoice::En, gamma_m0: 1.0, gamma_m1: 1.0, gamma_m2: 1.25, gamma_mf: 1.15 }
    }

    /// 🇩️🇪️ DIN EN 1993-1-1/NA: the genuine EN-vs-DE divergence raises γ_M1 to 1.1 for member buckling.
    pub fn de() -> Self {
        Self { choice: AnnexChoice::De, gamma_m0: 1.0, gamma_m1: 1.1, gamma_m2: 1.25, gamma_mf: 1.15 }
    }

    pub fn for_choice(choice: AnnexChoice) -> Self {
        match choice {
            AnnexChoice::En => Self::en(),
            AnnexChoice::De => Self::de(),
        }
    }
}
// #endregion 🔖️AnnexParams

// #region 🔖️Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📏️ Material factor ε = √(235/f_y).
    pub fn epsilon(f_y_mpa: f64) -> f64 {
        (235.0 / f_y_mpa).sqrt()
    }

    /// 🏷️ Cross-section class 1–4 per EN 1993-1-1 Table 5.2 (flange outstand in compression).
    pub fn flange_class(c_mm: f64, t_mm: f64, f_y_mpa: f64) -> u8 {
        let eps = epsilon(f_y_mpa);
        let ratio = c_mm / t_mm;
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

    /// 🏷️ Web class 1–4 per EN 1993-1-1 Table 5.2 (web in bending).
    pub fn web_class(c_mm: f64, t_mm: f64, f_y_mpa: f64) -> u8 {
        let eps = epsilon(f_y_mpa);
        let ratio = c_mm / t_mm;
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

    /// 🏷️ Overall section class (governing).
    pub fn section_class(flange_c_mm: f64, flange_t_mm: f64, web_c_mm: f64, web_t_mm: f64, f_y_mpa: f64) -> u8 {
        flange_class(flange_c_mm, flange_t_mm, f_y_mpa).max(web_class(web_c_mm, web_t_mm, f_y_mpa))
    }

    /// ⭕️ CHS (circular hollow section) class 1–3 per EN 1993-1-1 Table 5.2 (d/t vs 50ε², 70ε², 90ε²).
    pub fn chs_class(d_mm: f64, t_mm: f64, f_y_mpa: f64) -> u8 {
        let eps = epsilon(f_y_mpa);
        let ratio = d_mm / t_mm;
        if ratio <= 50.0 * eps * eps {
            1
        } else if ratio <= 70.0 * eps * eps {
            2
        } else if ratio <= 90.0 * eps * eps {
            3
        } else {
            4
        }
    }

    /// 📐️ Axial resistance N_Rd [kN] per EN 1993-1-1 §6.2.4.
    pub fn axial_resistance_kn(a_mm2: f64, f_y_mpa: f64, params: AnnexParams) -> f64 {
        a_mm2 * f_y_mpa / params.gamma_m0 / 1000.0
    }

    /// 📐️ Plastic bending resistance M_c,Rd [kNm] per EN 1993-1-1 §6.2.5.
    pub fn bending_resistance_knm(w_pl_mm3: f64, f_y_mpa: f64, params: AnnexParams) -> f64 {
        w_pl_mm3 * f_y_mpa / params.gamma_m0 / 1_000_000.0
    }

    /// 📐️ Plastic shear resistance V_pl,Rd [kN] per EN 1993-1-1 §6.2.6.
    pub fn shear_resistance_kn(a_v_mm2: f64, f_y_mpa: f64, params: AnnexParams) -> f64 {
        a_v_mm2 * f_y_mpa / (3.0_f64.sqrt() * params.gamma_m0) / 1000.0
    }

    /// 🔗️ Net-section tension resistance N_t,Rd [kN] = min(gross yield, net rupture) per EN 1993-1-1 §6.2.3 — rupture MUST use γ_M2, not γ_M0.
    pub fn net_tension_resistance_kn(a_mm2: f64, a_net_mm2: f64, f_y_mpa: f64, f_u_mpa: f64, params: AnnexParams) -> f64 {
        let gross_yield_kn = a_mm2 * f_y_mpa / params.gamma_m0 / 1000.0;
        let net_rupture_kn = 0.9 * a_net_mm2 * f_u_mpa / params.gamma_m2 / 1000.0;
        gross_yield_kn.min(net_rupture_kn)
    }

    /// 📉️ Buckling curve per EN 1993-1-1 Table 6.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BucklingCurve {
        A0,
        A,
        B,
        C,
        D,
    }

    impl BucklingCurve {
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

    /// 📉️ Reduction factor χ per EN 1993-1-1 Eq. 6.61.
    pub fn chi(lambda_bar: f64, curve: BucklingCurve) -> f64 {
        let alpha = curve.alpha();
        let phi = 0.5 * (1.0 + alpha * (lambda_bar - 0.2) + lambda_bar * lambda_bar);
        1.0 / (phi + (phi * phi - lambda_bar * lambda_bar).max(0.0).sqrt())
    }

    /// 📉️ Non-dimensional slenderness λ̄ = √(A·f_y/N_cr).
    pub fn lambda_bar(a_mm2: f64, f_y_mpa: f64, n_cr_kn: f64) -> f64 {
        (a_mm2 * f_y_mpa / 1000.0 / n_cr_kn).sqrt()
    }

    /// 📉️ Buckling resistance N_b,Rd [kN] per EN 1993-1-1 §6.3.1.
    pub fn buckling_resistance_kn(a_mm2: f64, f_y_mpa: f64, chi: f64, params: AnnexParams) -> f64 {
        chi * a_mm2 * f_y_mpa / params.gamma_m1 / 1000.0
    }

    pub fn check_cross_section(n_ed_kn: f64, n_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-1", "§6.2.4", "6.2.4"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd_kn), "cross-section axial ULS", annex)
    }

    pub fn check_bending(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-1", "§6.2.5", "6.2.5"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "cross-section bending ULS",
            annex,
        )
    }

    pub fn check_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-1", "§6.2.6", "6.2.6"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "cross-section shear ULS", annex)
    }

    pub fn check_member_buckling(n_ed_kn: f64, n_b_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-1", "§6.3.1", "6.3.1"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_b_rd_kn), "member buckling ULS", annex)
    }

    pub fn check_net_tension(n_ed_kn: f64, n_t_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-1", "§6.2.3", "6.2.3"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_t_rd_kn), "net-section tension ULS", annex)
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

    /// 🔥️ Board insulation thickness [mm] per EN 1993-1-2 Table 4.3 (simplified).
    pub fn board_thickness_mm(rating: FireRating, massivity: f64) -> f64 {
        let base = match rating {
            FireRating::R30 => 8.0,
            FireRating::R60 => 15.0,
            FireRating::R90 => 22.0,
            FireRating::R120 => 30.0,
        };
        base * (1.0 + (massivity / 200.0).min(0.5))
    }

    pub fn check_fire_protection(thickness_mm: f64, rating: FireRating, massivity: f64, annex: AnnexChoice) -> CheckResult {
        let required = board_thickness_mm(rating, massivity);
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-2", "§4.2", "4.2"), Quantity::length_m(required / 1000.0), Quantity::length_m(thickness_mm / 1000.0), "steel fire protection thickness", annex)
    }

    /// 🔥️ Critical steel temperature θ_a,cr [°C] per EN 1993-1-2 Eq. 4.22, μ₀ = degree of utilization.
    pub fn critical_temperature_c(mu_0: f64) -> f64 {
        39.19 * (1.0 / (0.9674 * mu_0.powf(3.833)) - 1.0).ln() + 482.0
    }

    pub fn check_critical_temperature(mu_0: f64, design_temperature_c: f64, annex: AnnexChoice) -> CheckResult {
        let theta_a_cr = critical_temperature_c(mu_0);
        CheckResult::from_minimum(
            ClauseId::new("EN 1993-1-2", "§4.2.3", "4.22"),
            Quantity::new(crate::document::QuantityKind::Temperature, theta_a_cr),
            Quantity::new(crate::document::QuantityKind::Temperature, design_temperature_c),
            "critical steel temperature",
            annex,
        )
    }
}
// #endregion 🔖️Part1_2

// #region 🔖️Part1_3
pub mod part_1_3 {
    use super::*;

    /// 📏️ Cold-formed plate slenderness λ_p = (b̄/t)/(28.4·ε·√k_σ) per EN 1993-1-3 §5.5.
    pub fn lambda_p(b_bar_mm: f64, t_mm: f64, f_y_mpa: f64, k_sigma: f64) -> f64 {
        (b_bar_mm / t_mm) / (28.4 * part_1_1::epsilon(f_y_mpa) * k_sigma.sqrt())
    }

    /// 📉️ Effective-width reduction factor ρ = (λ_p − 0.055·(3+ψ))/λ_p² (capped at 1.0) per EN 1993-1-3 §5.5.2.
    pub fn reduction_factor(lambda_p: f64, psi: f64) -> f64 {
        if lambda_p <= 0.673 {
            1.0
        } else {
            ((lambda_p - 0.055 * (3.0 + psi)) / (lambda_p * lambda_p)).min(1.0)
        }
    }

    /// 📐️ Effective cross-section resistance [kN] = ρ · gross resistance per EN 1993-1-3 §5.5.
    pub fn effective_resistance_kn(rho: f64, gross_resistance_kn: f64) -> f64 {
        rho * gross_resistance_kn
    }

    pub fn check_cold_formed_effective_section(n_ed_kn: f64, n_eff_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-3", "§5.5", "5.5.2"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_eff_rd_kn), "cold-formed effective section ULS", annex)
    }
}
// #endregion 🔖️Part1_3

// #region 🔖️Part1_4
pub mod part_1_4 {
    use super::*;

    /// 🪙️ Stainless steel γ_M0 = γ_M1 = 1.1 per EN 1993-1-4 §2.2 — a material safety factor fixed in the base EN document, not a DE/EN annex divergence.
    pub const GAMMA_M_STAINLESS: f64 = 1.1;

    /// 📏️ Stainless steel material factor ε = √(235/f_y · E/210000) per EN 1993-1-4 Table 2.2.
    pub fn epsilon(f_y_mpa: f64, e_mpa: f64) -> f64 {
        (235.0 / f_y_mpa * (e_mpa / 210_000.0)).sqrt()
    }

    pub fn bending_resistance_knm(w_pl_mm3: f64, f_y_mpa: f64) -> f64 {
        w_pl_mm3 * f_y_mpa / GAMMA_M_STAINLESS / 1_000_000.0
    }

    pub fn check_stainless_steel(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-4", "§6", "6.1"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "stainless steel bending",
            annex,
        )
    }
}
// #endregion 🔖️Part1_4

// #region 🔖️Part1_5
pub mod part_1_5 {
    use super::*;

    /// 📐️ Plate reduction factor ρ per EN 1993-1-5 §4.4.
    pub fn plate_reduction_factor(lambda_p: f64) -> f64 {
        if lambda_p <= 0.673 {
            1.0
        } else {
            (lambda_p - 0.055 * (3.0_f64).sqrt()) / (lambda_p * lambda_p)
        }
    }

    /// 📐️ Plate effective width b_eff per EN 1993-1-5 §4.
    pub fn effective_width_mm(b_mm: f64, lambda_p: f64) -> f64 {
        plate_reduction_factor(lambda_p) * b_mm
    }

    /// 📐️ Local buckling design stress σ_c,Rd [MPa] per EN 1993-1-5 §4.
    pub fn local_buckling_stress_rd_mpa(f_y_mpa: f64, lambda_p: f64, params: AnnexParams) -> f64 {
        plate_reduction_factor(lambda_p) * f_y_mpa / params.gamma_m0
    }

    pub fn check_plated_buckling(sigma_ed_mpa: f64, sigma_rd_mpa: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-5", "§4", "4.1"), Quantity::stress_mpa(sigma_ed_mpa), Quantity::stress_mpa(sigma_rd_mpa), "plated structure local buckling", annex)
    }
}
// #endregion 🔖️Part1_5

// #region 🔖️Part1_6
pub mod part_1_6 {
    use super::*;

    /// 🐚️ Fabrication quality parameter Q for tolerance quality class B per EN 1993-1-6 Table D.2.
    pub const FABRICATION_QUALITY_Q_CLASS_B: f64 = 25.0;

    /// 🐚️ Elastic critical meridional buckling stress σ_x,Rcr [MPa] per EN 1993-1-6 Annex D.1.2.1.
    pub fn sigma_x_rcr_mpa(t_mm: f64, r_mm: f64, e_mpa: f64) -> f64 {
        0.605 * e_mpa * t_mm / r_mm
    }

    /// 📉️ Shell meridional slenderness λ̄ = √(f_y/σ_x,Rcr) per EN 1993-1-6 §D.1.2.1.
    pub fn lambda_bar(f_y_mpa: f64, sigma_x_rcr_mpa: f64) -> f64 {
        (f_y_mpa / sigma_x_rcr_mpa).sqrt()
    }

    /// 🐚️ Elastic imperfection reduction factor α from fabrication quality class B per EN 1993-1-6 §D.1.2.2.
    pub fn alpha_imperfection(r_mm: f64, t_mm: f64) -> f64 {
        let delta_wk_over_t = (1.0 / FABRICATION_QUALITY_Q_CLASS_B) * (r_mm / t_mm).sqrt();
        0.62 / (1.0 + 1.91 * delta_wk_over_t.powf(1.44))
    }

    /// 📉️ Shell buckling reduction factor χ, λ̄₀=0.2, β=0.6, η=1.0 per EN 1993-1-6 §8.5.2.
    pub fn chi(lambda_bar: f64, alpha: f64) -> f64 {
        let lambda_0 = 0.2;
        let beta = 0.6;
        let eta = 1.0;
        let lambda_p = (alpha / (1.0 - beta)).sqrt();
        if lambda_bar <= lambda_0 {
            1.0
        } else if lambda_bar < lambda_p {
            1.0 - beta * ((lambda_bar - lambda_0) / (lambda_p - lambda_0)).powf(eta)
        } else {
            alpha / (lambda_bar * lambda_bar)
        }
    }

    /// 📐️ Shell design buckling resistance σ_x,Rd [MPa] = χ·f_y/γ_M1 per EN 1993-1-6 §8.5.2.
    pub fn design_resistance_mpa(f_y_mpa: f64, chi: f64, params: AnnexParams) -> f64 {
        chi * f_y_mpa / params.gamma_m1
    }

    pub fn check_shell_buckling(sigma_x_ed_mpa: f64, sigma_x_rd_mpa: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-6", "§8.5.2", "8.5.2"), Quantity::stress_mpa(sigma_x_ed_mpa), Quantity::stress_mpa(sigma_x_rd_mpa), "shell meridional buckling", annex)
    }
}
// #endregion 🔖️Part1_6

// #region 🔖️Part1_8
pub mod part_1_8 {
    use super::*;

    /// 🔩️ Bolt shear resistance F_v,Rd [kN] per EN 1993-1-8 §3.6.1.
    pub fn bolt_shear_resistance_kn(n_bolts: u32, a_s_mm2: f64, f_ub_mpa: f64, gamma_m2: f64) -> f64 {
        let alpha_v = 0.6;
        n_bolts as f64 * alpha_v * a_s_mm2 * f_ub_mpa / gamma_m2 / 1000.0
    }

    /// 📐️ Bearing coefficient α_b = min(e₁/(3·d₀), f_ub/f_u, 1.0) per EN 1993-1-8 Table 3.4.
    pub fn bearing_alpha_b(e1_mm: f64, d0_mm: f64, f_ub_mpa: f64, f_u_mpa: f64) -> f64 {
        (e1_mm / (3.0 * d0_mm)).min(f_ub_mpa / f_u_mpa).min(1.0)
    }

    /// 📐️ Bearing coefficient k₁ = min(2.8·e₂/d₀ − 1.7, 2.5) per EN 1993-1-8 Table 3.4.
    pub fn bearing_k1(e2_mm: f64, d0_mm: f64) -> f64 {
        (2.8 * e2_mm / d0_mm - 1.7).min(2.5)
    }

    /// 🔩️ Bolt bearing resistance F_b,Rd [kN] = k₁·α_b·f_u·d·t/γ_M2 per EN 1993-1-8 Table 3.4.
    pub fn bolt_bearing_resistance_kn(k1: f64, alpha_b: f64, f_u_mpa: f64, d_mm: f64, t_mm: f64, gamma_m2: f64) -> f64 {
        k1 * alpha_b * f_u_mpa * d_mm * t_mm / gamma_m2 / 1000.0
    }

    /// 🪛️ Correlation factor β_w by steel grade per EN 1993-1-8 Table 4.1.
    pub fn beta_w(steel_grade: &str) -> f64 {
        match steel_grade {
            "S235" => 0.8,
            "S275" => 0.85,
            "S355" => 0.9,
            "S460" => 1.0,
            _ => 0.9,
        }
    }

    /// 🪛️ Fillet weld resistance F_w,Rd [kN] = a·l·f_u/(√3·β_w·γ_M2) per EN 1993-1-8 §4.5.3.3 simplified method.
    pub fn fillet_weld_resistance_kn(a_mm: f64, l_mm: f64, f_u_mpa: f64, beta_w: f64, gamma_m2: f64) -> f64 {
        a_mm * l_mm * f_u_mpa / (3.0_f64.sqrt() * beta_w * gamma_m2) / 1000.0
    }

    pub fn check_bolt_shear(f_ed_kn: f64, f_v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-8", "§3.6.1", "3.6.1"), Quantity::force_kn(f_ed_kn), Quantity::force_kn(f_v_rd_kn), "bolt shear ULS", annex)
    }

    pub fn check_bolt_bearing(f_ed_kn: f64, f_b_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-8", "§3.6.1", "3.6.1b"), Quantity::force_kn(f_ed_kn), Quantity::force_kn(f_b_rd_kn), "bolt bearing ULS", annex)
    }

    pub fn check_fillet_weld(f_ed_kn: f64, f_w_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-8", "§4.5.3.3", "4.5.3.3"), Quantity::force_kn(f_ed_kn), Quantity::force_kn(f_w_rd_kn), "fillet weld ULS", annex)
    }
}
// #endregion 🔖️Part1_8

// #region 🔖️Part1_9
pub mod part_1_9 {
    use super::*;

    /// 🔄️ Fatigue detail category Δσ_C [MPa] per EN 1993-1-9 Table 8.1.
    pub fn detail_category_mpa(category: u8) -> f64 {
        match category {
            36 => 36.0,
            40 => 40.0,
            45 => 45.0,
            50 => 50.0,
            56 => 56.0,
            63 => 63.0,
            71 => 71.0,
            80 => 80.0,
            90 => 90.0,
            100 => 100.0,
            112 => 112.0,
            125 => 125.0,
            140 => 140.0,
            150 => 150.0,
            160 => 160.0,
            _ => 71.0,
        }
    }

    /// 🔄️ Fatigue strength Δσ_C,∞ [MPa] at N = 2×10⁶ cycles.
    pub fn fatigue_strength_mpa(category: u8) -> f64 {
        detail_category_mpa(category)
    }

    /// 🔄️ Fatigue assessment method selecting γ_Mf per EN 1993-1-9 Table 3.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AssessmentMethod {
        LowConsequence,
        DamageTolerant,
        SafeLife,
    }

    impl AssessmentMethod {
        pub fn gamma_mf(self) -> f64 {
            match self {
                Self::LowConsequence => 1.0,
                Self::DamageTolerant => 1.15,
                Self::SafeLife => 1.35,
            }
        }
    }

    /// 🔄️ Cycles to failure N per EN 1993-1-9 §7 constant-amplitude S-N curve, slope m=3.
    pub fn cycles_to_failure(delta_sigma_mpa: f64, category: u8) -> f64 {
        const M: f64 = 3.0;
        2.0e6 * (detail_category_mpa(category) / delta_sigma_mpa).powf(M)
    }

    /// 🔄️ Fatigue verification γ_Ff·Δσ_E2 ≤ Δσ_C/γ_Mf per EN 1993-1-9 §8.
    pub fn check_fatigue_range(gamma_ff_delta_sigma_e2_mpa: f64, category: u8, gamma_mf: f64, annex: AnnexChoice) -> CheckResult {
        let limit = fatigue_strength_mpa(category) / gamma_mf;
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-9", "§8", "8.1"), Quantity::stress_mpa(gamma_ff_delta_sigma_e2_mpa), Quantity::stress_mpa(limit), "fatigue stress range verification", annex)
    }
}
// #endregion 🔖️Part1_9

// #region 🔖️Part1_10
pub mod part_1_10 {
    use super::*;

    /// 🌡️ Reference temperature T_Ed axis [°C] for EN 1993-1-10 Table 2.1.
    const T_ED_C: [f64; 5] = [-50.0, -30.0, -10.0, 0.0, 10.0];

    /// 🏷️ Steel subgrade quality index axis (1=JR, 2=J0, 3=J2, 4=K2/N, 5=M/ML) for EN 1993-1-10 Table 2.1.
    const SUBGRADE_INDEX: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    /// 📊️ Max permissible element thickness [mm] at σ_Ed=0.75·f_y(t), EN 1993-1-10 Table 2.1 grid (row-major by subgrade × T_Ed).
    const MAX_THICKNESS_MM: [f64; 25] = [
        10.0, 20.0, 30.0, 35.0, 40.0, //
        15.0, 30.0, 45.0, 50.0, 55.0, //
        20.0, 40.0, 60.0, 65.0, 70.0, //
        30.0, 55.0, 75.0, 80.0, 85.0, //
        40.0, 70.0, 90.0, 95.0, 100.0,
    ];

    fn subgrade_index(steel_subgrade: &str) -> f64 {
        match steel_subgrade.to_ascii_uppercase().as_str() {
            "JR" => 1.0,
            "J0" => 2.0,
            "J2" => 3.0,
            "K2" | "N" => 4.0,
            "M" | "ML" => 5.0,
            _ => 2.0,
        }
    }

    /// 📊️ Max permissible thickness [mm] per EN 1993-1-10 Table 2.1 (bilinear lookup, not an ad-hoc offset).
    pub fn max_permissible_thickness_mm(steel_subgrade: &str, t_ed_c: f64) -> f64 {
        crate::document::table_lookup_bilinear(t_ed_c, subgrade_index(steel_subgrade), &T_ED_C, &SUBGRADE_INDEX, &MAX_THICKNESS_MM)
    }

    pub fn check_through_thickness(actual_thickness_mm: f64, steel_subgrade: &str, t_ed_c: f64, annex: AnnexChoice) -> CheckResult {
        let max_thickness = max_permissible_thickness_mm(steel_subgrade, t_ed_c);
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-10", "§2", "2.1"), Quantity::length_m(actual_thickness_mm / 1000.0), Quantity::length_m(max_thickness / 1000.0), "brittle fracture max thickness", annex)
    }
}
// #endregion 🔖️Part1_10

// #region 🔖️Part1_11
pub mod part_1_11 {
    use super::*;

    /// 🎣️ Tension component (cable/rod) resistance F_Rd = min(F_uk/(1.5·γ_R), F_k/γ_R), γ_R = 1.0, per EN 1993-1-11 §6.2.
    pub fn tension_component_resistance_kn(f_uk_kn: f64, f_k_kn: f64) -> f64 {
        const GAMMA_R: f64 = 1.0;
        (f_uk_kn / (1.5 * GAMMA_R)).min(f_k_kn / GAMMA_R)
    }

    pub fn check_tension_component(n_ed_kn: f64, n_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-1-11", "§6.2", "6.2"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd_kn), "tension component (cable/rod) ULS", annex)
    }
}
// #endregion 🔖️Part1_11

// #region 🔖️Part1_12
pub mod part_1_12 {
    use super::*;

    /// 💪️ EN 1993-1-12 restricts S460–S700 to elastic verification only — no plastic moment redistribution.
    pub fn is_high_strength_restricted(f_y_mpa: f64) -> bool {
        (460.0..=700.0).contains(&f_y_mpa)
    }

    /// 📐️ Elastic bending resistance M_Rd = W_el·f_y/γ_M0 [kNm], 0.0 if section class 4 (outside the elastic-only scope) per EN 1993-1-12 §4.
    pub fn elastic_bending_resistance_knm(w_el_mm3: f64, f_y_mpa: f64, section_class: u8, params: AnnexParams) -> f64 {
        if section_class > 3 {
            return 0.0;
        }
        w_el_mm3 * f_y_mpa / params.gamma_m0 / 1_000_000.0
    }

    pub fn check_high_strength_bending(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-12", "§4", "4.1"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "high-strength steel elastic bending",
            annex,
        )
    }
}
// #endregion 🔖️Part1_12

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🌉️ Steel bridge combined axial + bending interaction per EN 1993-2 §6.
    pub fn bridge_interaction_eta(n_ed_kn: f64, n_rd_kn: f64, m_ed_knm: f64, m_rd_knm: f64) -> f64 {
        (n_ed_kn / n_rd_kn).abs() + (m_ed_knm / m_rd_knm).abs()
    }

    pub fn check_steel_bridge(n_ed_kn: f64, n_rd_kn: f64, m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        let eta = bridge_interaction_eta(n_ed_kn, n_rd_kn, m_ed_knm, m_rd_knm);
        CheckResult::from_utilization(ClauseId::new("EN 1993-2", "§6", "6.1"), Quantity::new(crate::document::QuantityKind::Dimensionless, eta), Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0), "steel bridge interaction", annex)
    }

    /// 🌉️ Damage-equivalent fatigue stress range Δσ_E2 = λ·Φ₂·Δσ_p per EN 1993-2 §9.3.
    pub fn damage_equivalent_stress_mpa(lambda: f64, phi_2: f64, delta_sigma_p_mpa: f64) -> f64 {
        lambda * phi_2 * delta_sigma_p_mpa
    }

    pub fn check_bridge_fatigue(lambda: f64, phi_2: f64, delta_sigma_p_mpa: f64, category: u8, gamma_mf: f64, annex: AnnexChoice) -> CheckResult {
        let delta_sigma_e2 = damage_equivalent_stress_mpa(lambda, phi_2, delta_sigma_p_mpa);
        let limit = part_1_9::fatigue_strength_mpa(category) / gamma_mf;
        CheckResult::from_utilization(ClauseId::new("EN 1993-2", "§9.3", "9.3"), Quantity::stress_mpa(delta_sigma_e2), Quantity::stress_mpa(limit), "bridge damage-equivalent fatigue", annex)
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    /// 🗼️ Tower leg buckling with wind gust amplification factor per EN 1993-3-1 §5.
    pub fn tower_buckling_kn(a_mm2: f64, f_y_mpa: f64, chi: f64, wind_factor: f64, params: AnnexParams) -> f64 {
        part_1_1::buckling_resistance_kn(a_mm2, f_y_mpa, chi, params) / wind_factor
    }

    pub fn check_tower_buckling(n_ed_kn: f64, n_b_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-3-1", "§5", "5.1"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_b_rd_kn), "tower leg buckling with gust amplification", annex)
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 🌾️ Janssen-type horizontal silo pressure p_h = k·γ·z [kPa] per EN 1993-4-1 Annex C basis.
    pub fn janssen_pressure_kpa(k: f64, gamma_kn_m3: f64, depth_m: f64) -> f64 {
        k * gamma_kn_m3 * depth_m
    }

    /// 🛢️ Membrane hoop stress σ_θ = p_h·r/t [MPa] per EN 1993-4-1 §5.3.
    pub fn membrane_hoop_stress_mpa(p_h_kpa: f64, r_mm: f64, t_mm: f64) -> f64 {
        p_h_kpa * r_mm / t_mm / 1000.0
    }

    pub fn check_silo_wall(sigma_ed_mpa: f64, sigma_rd_mpa: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-4-1", "§5.3", "5.3"), Quantity::stress_mpa(sigma_ed_mpa), Quantity::stress_mpa(sigma_rd_mpa), "silo/tank membrane wall stress", annex)
    }
}
// #endregion 🔖️Part4

// #region 🔖️Part5
pub mod part_5 {
    use super::*;

    /// 🔩️ Steel pile compression capacity N_c,Rd [kN] per EN 1993-5.
    pub fn pile_compression_kn(a_mm2: f64, f_y_mpa: f64, k_red: f64, params: AnnexParams) -> f64 {
        k_red * a_mm2 * f_y_mpa / params.gamma_m0 / 1000.0
    }

    /// 🔨️ Pile driving stress limit σ_lim = 0.9·f_y per EN 1993-5 §12 (driving stresses).
    pub fn pile_driving_stress_limit_mpa(f_y_mpa: f64) -> f64 {
        0.9 * f_y_mpa
    }

    pub fn check_pile_driving_stress(sigma_mpa: f64, f_y_mpa: f64, annex: AnnexChoice) -> CheckResult {
        let limit = pile_driving_stress_limit_mpa(f_y_mpa);
        CheckResult::from_utilization(ClauseId::new("EN 1993-5", "§12", "12.1"), Quantity::stress_mpa(sigma_mpa), Quantity::stress_mpa(limit), "pile driving stress", annex)
    }

    pub fn check_pile_foundation_steel(n_ed_kn: f64, n_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1993-5", "§6", "6.1"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd_kn), "steel pile compression", annex)
    }
}
// #endregion 🔖️Part5

// #region 🔖️Part6
pub mod part_6 {
    use super::*;

    /// 🏗️ Effective load length l_eff at the crane rail per EN 1993-6 Table 5.1 (wheel contact length + 45° dispersion into the web).
    pub fn effective_length_mm(wheel_contact_length_mm: f64, dispersion_mm: f64) -> f64 {
        wheel_contact_length_mm + 2.0 * dispersion_mm
    }

    /// 🏗️ Local wheel-load web stress σ_oz = F_z,Ed/(l_eff·t_w) per EN 1993-6 §5.7.1.
    pub fn wheel_load_web_stress_mpa(f_z_ed_kn: f64, l_eff_mm: f64, t_w_mm: f64) -> f64 {
        f_z_ed_kn * 1000.0 / (l_eff_mm * t_w_mm)
    }

    pub fn check_crane_runway_web(sigma_oz_mpa: f64, f_y_mpa: f64, params: AnnexParams, annex: AnnexChoice) -> CheckResult {
        let limit = f_y_mpa / params.gamma_m0;
        CheckResult::from_utilization(ClauseId::new("EN 1993-6", "§5.7.1", "5.7.1"), Quantity::stress_mpa(sigma_oz_mpa), Quantity::stress_mpa(limit), "crane runway local wheel-load web stress", annex)
    }
}
// #endregion 🔖️Part6

/// 📋️ I-section member check.
pub fn check_steel_member(n_ed_kn: f64, m_ed_knm: f64, a_mm2: f64, w_pl_mm3: f64, f_y_mpa: f64, chi: f64) -> CheckReport {
    let annex = AnnexChoice::De;
    let params = AnnexParams::de();
    let n_rd = part_1_1::axial_resistance_kn(a_mm2, f_y_mpa, params);
    let n_b_rd = part_1_1::buckling_resistance_kn(a_mm2, f_y_mpa, chi, params);
    let m_rd = part_1_1::bending_resistance_knm(w_pl_mm3, f_y_mpa, params);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_cross_section(n_ed_kn, n_rd, annex));
    report.push(part_1_1::check_member_buckling(n_ed_kn, n_b_rd, annex));
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report
}

fn parse_fire_rating(value: &str) -> part_1_2::FireRating {
    match value.to_ascii_lowercase().as_str() {
        "r30" => part_1_2::FireRating::R30,
        "r90" => part_1_2::FireRating::R90,
        "r120" => part_1_2::FireRating::R120,
        _ => part_1_2::FireRating::R60,
    }
}

fn parse_fatigue_method(value: &str) -> part_1_9::AssessmentMethod {
    match value.to_ascii_lowercase().as_str() {
        "low_consequence" => part_1_9::AssessmentMethod::LowConsequence,
        "safe_life" => part_1_9::AssessmentMethod::SafeLife,
        _ => part_1_9::AssessmentMethod::DamageTolerant,
    }
}

/// 📋️ Full steel member check across all sixteen EN 1993 parts (1-1 through 6), each reached from `evaluate`.
pub fn check_full_steel_member(document: &En1993Snapshot) -> CheckReport {
    let annex = document.annex;
    let params = AnnexParams { gamma_mf: parse_fatigue_method(&document.fatigue_method).gamma_mf(), ..AnnexParams::for_choice(annex) };

    let mut report = check_steel_member(document.n_ed_kn, document.m_ed_knm, document.a_mm2, document.w_pl_mm3, document.f_y_mpa, document.chi);

    // #region 🔖️Part1_1
    let v_rd = part_1_1::shear_resistance_kn(document.a_v_mm2, document.f_y_mpa, params);
    report.push(part_1_1::check_shear(document.v_ed_kn, v_rd, annex));
    let n_t_rd = part_1_1::net_tension_resistance_kn(document.a_mm2, document.a_net_mm2, document.f_y_mpa, document.f_u_mpa, params);
    report.push(part_1_1::check_net_tension(document.tension_n_ed_kn, n_t_rd, annex));
    // #endregion 🔖️Part1_1

    // #region 🔖️Part1_2
    report.push(part_1_2::check_fire_protection(document.fire_thickness_mm, parse_fire_rating(&document.fire_rating), document.fire_massivity, annex));
    report.push(part_1_2::check_critical_temperature(document.fire_mu_0, document.fire_design_temperature_c, annex));
    // #endregion 🔖️Part1_2

    // #region 🔖️Part1_3
    let cf_lambda_p = part_1_3::lambda_p(document.cf_b_bar_mm, document.cf_t_mm, document.f_y_mpa, document.cf_k_sigma);
    let cf_rho = part_1_3::reduction_factor(cf_lambda_p, document.cf_psi);
    let cf_n_eff_rd = part_1_3::effective_resistance_kn(cf_rho, document.cf_gross_resistance_kn);
    report.push(part_1_3::check_cold_formed_effective_section(document.cf_n_ed_kn, cf_n_eff_rd, annex));
    // #endregion 🔖️Part1_3

    // #region 🔖️Part1_4
    let stainless_m_rd = part_1_4::bending_resistance_knm(document.stainless_w_pl_mm3, document.stainless_f_y_mpa);
    report.push(part_1_4::check_stainless_steel(document.stainless_m_ed_knm, stainless_m_rd, annex));
    // #endregion 🔖️Part1_4

    // #region 🔖️Part1_5
    let plated_sigma_rd = part_1_5::local_buckling_stress_rd_mpa(document.f_y_mpa, document.plated_lambda_p, params);
    report.push(part_1_5::check_plated_buckling(document.plated_sigma_ed_mpa, plated_sigma_rd, annex));
    // #endregion 🔖️Part1_5

    // #region 🔖️Part1_6
    let shell_sigma_x_rcr = part_1_6::sigma_x_rcr_mpa(document.silo_t_mm, document.silo_r_mm, 210_000.0);
    let shell_lambda_bar = part_1_6::lambda_bar(document.f_y_mpa, shell_sigma_x_rcr);
    let shell_alpha = part_1_6::alpha_imperfection(document.silo_r_mm, document.silo_t_mm);
    let shell_chi = part_1_6::chi(shell_lambda_bar, shell_alpha);
    let shell_sigma_x_rd = part_1_6::design_resistance_mpa(document.f_y_mpa, shell_chi, params);
    report.push(part_1_6::check_shell_buckling(document.shell_sigma_x_ed_mpa, shell_sigma_x_rd, annex));
    // #endregion 🔖️Part1_6

    // #region 🔖️Part1_8
    let bearing_alpha_b = part_1_8::bearing_alpha_b(document.bolt_e1_mm, document.bolt_d0_mm, document.bolt_f_ub_mpa, document.bolt_f_u_mpa);
    let bearing_k1 = part_1_8::bearing_k1(document.bolt_e2_mm, document.bolt_d0_mm);
    let bolt_v_rd = part_1_8::bolt_shear_resistance_kn(document.bolt_n_bolts, document.bolt_a_s_mm2, document.bolt_f_ub_mpa, params.gamma_m2);
    let bolt_b_rd = part_1_8::bolt_bearing_resistance_kn(bearing_k1, bearing_alpha_b, document.bolt_f_u_mpa, document.bolt_d_mm, document.bolt_t_mm, params.gamma_m2);
    report.push(part_1_8::check_bolt_shear(document.bolt_f_ed_kn, bolt_v_rd, annex));
    report.push(part_1_8::check_bolt_bearing(document.bolt_f_ed_kn, bolt_b_rd, annex));
    let weld_beta_w = part_1_8::beta_w(&document.weld_steel_grade);
    let weld_w_rd = part_1_8::fillet_weld_resistance_kn(document.weld_a_mm, document.weld_l_mm, document.weld_f_u_mpa, weld_beta_w, params.gamma_m2);
    report.push(part_1_8::check_fillet_weld(document.weld_f_ed_kn, weld_w_rd, annex));
    // #endregion 🔖️Part1_8

    // #region 🔖️Part1_9
    report.push(part_1_9::check_fatigue_range(document.delta_sigma_mpa, document.fatigue_category, params.gamma_mf, annex));
    // #endregion 🔖️Part1_9

    // #region 🔖️Part1_10
    report.push(part_1_10::check_through_thickness(document.t10_actual_thickness_mm, &document.t10_steel_subgrade, document.t10_t_ed_c, annex));
    // #endregion 🔖️Part1_10

    // #region 🔖️Part1_11
    let tension_component_rd = part_1_11::tension_component_resistance_kn(document.tension_component_f_uk_kn, document.tension_component_f_k_kn);
    report.push(part_1_11::check_tension_component(document.tension_component_n_ed_kn, tension_component_rd, annex));
    // #endregion 🔖️Part1_11

    // #region 🔖️Part1_12
    let hss_m_rd = part_1_12::elastic_bending_resistance_knm(document.hss_w_el_mm3, document.hss_f_y_mpa, document.hss_section_class, params);
    report.push(part_1_12::check_high_strength_bending(document.hss_m_ed_knm, hss_m_rd, annex));
    // #endregion 🔖️Part1_12

    // #region 🔖️Part2
    let n_rd = part_1_1::axial_resistance_kn(document.a_mm2, document.f_y_mpa, params);
    let m_rd = part_1_1::bending_resistance_knm(document.w_pl_mm3, document.f_y_mpa, params);
    report.push(part_2::check_steel_bridge(document.n_ed_kn, n_rd, document.m_ed_knm, m_rd, annex));
    report.push(part_2::check_bridge_fatigue(document.bridge_lambda, document.bridge_phi_2, document.bridge_delta_sigma_p_mpa, document.fatigue_category, params.gamma_mf, annex));
    // #endregion 🔖️Part2

    // #region 🔖️Part3
    let tower_n_b_rd = part_3::tower_buckling_kn(document.a_mm2, document.f_y_mpa, document.chi, document.tower_wind_factor, params);
    report.push(part_3::check_tower_buckling(document.tower_n_ed_kn, tower_n_b_rd, annex));
    // #endregion 🔖️Part3

    // #region 🔖️Part4
    let silo_p_h = part_4::janssen_pressure_kpa(document.silo_k, document.silo_gamma_kn_m3, document.silo_depth_m);
    let silo_sigma_ed = part_4::membrane_hoop_stress_mpa(silo_p_h, document.silo_r_mm, document.silo_t_mm);
    report.push(part_4::check_silo_wall(silo_sigma_ed, shell_sigma_x_rd, annex));
    // #endregion 🔖️Part4

    // #region 🔖️Part5
    report.push(part_5::check_pile_driving_stress(document.pile_sigma_mpa, document.f_y_mpa, annex));
    let pile_n_rd = part_5::pile_compression_kn(document.a_mm2, document.f_y_mpa, document.pile_k_red, params);
    report.push(part_5::check_pile_foundation_steel(document.pile_n_ed_kn, pile_n_rd, annex));
    // #endregion 🔖️Part5

    // #region 🔖️Part6
    let crane_l_eff = part_6::effective_length_mm(document.crane_wheel_contact_length_mm, document.crane_dispersion_mm);
    let crane_sigma_oz = part_6::wheel_load_web_stress_mpa(document.crane_f_z_ed_kn, crane_l_eff, document.crane_t_w_mm);
    report.push(part_6::check_crane_runway_web(crane_sigma_oz, document.f_y_mpa, params, annex));
    // #endregion 🔖️Part6

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

/// 🏗️ Solve a simply supported steel beam with `fem_core` and run EN 1993 ULS checks.
#[cfg(feature = "cross-fem")]
pub fn check_steel_member_from_fem(span_m: f64, udl_kn_m: f64, a_mm2: f64, w_pl_mm3: f64, a_v_mm2: f64, f_y_mpa: f64, chi: f64) -> Result<CheckReport, fem::core::FemError> {
    let mut model = Model::default();
    model.nodes.push(Node { id: "n0".into(), pos: [0.0, 0.0, 0.0] });
    model.nodes.push(Node { id: "n1".into(), pos: [span_m, 0.0, 0.0] });
    model.supports.push(Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty] });
    model.supports.push(Support { node_id: "n1".into(), fixed: vec![Dof::Ty] });
    model.elements.push(Box::new(BeamEb2 { id: "b1".into(), start: "n0".into(), end: "n1".into(), e: 210e9, area: a_mm2 / 1e6, iy: a_mm2 * a_mm2 / 12e12, density: 7850.0 }));
    model.member_loads.push(("b1".into(), MemberUdl { wx: 0.0, wy: -udl_kn_m * 1000.0, wz: 0.0 }));

    let result = fem::core::solve_linear_static(&model)?;
    let m_ed_knm = max_beam_moment_knm(&result, "b1");
    let v_ed_kn = max_beam_shear_kn(&result, "b1");

    let annex = AnnexChoice::De;
    let params = AnnexParams::de();
    let n_rd = part_1_1::axial_resistance_kn(a_mm2, f_y_mpa, params);
    let n_b_rd = part_1_1::buckling_resistance_kn(a_mm2, f_y_mpa, chi, params);
    let m_rd = part_1_1::bending_resistance_knm(w_pl_mm3, f_y_mpa, params);
    let v_rd = part_1_1::shear_resistance_kn(a_v_mm2, f_y_mpa, params);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_cross_section(0.0, n_rd, annex));
    report.push(part_1_1::check_member_buckling(0.0, n_b_rd, annex));
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_shear(v_ed_kn, v_rd, annex));
    Ok(report)
}
// #endregion 🔖️Fem

// #region 🔖️Session

//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent En1993 artifact engine — owns the artifact; every transition is a mutation.
pub struct En1993Engine {
    artifact: crate::artifacts::en1993::schema::En1993Artifact,
    snapshot: En1993Snapshot,
}

impl En1993Engine {
    pub fn new(snapshot: En1993Snapshot) -> Self {
        let artifact = crate::artifacts::en1993::schema::En1993Artifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> En1993Snapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

pub type Host = NormHost<En1993Family>;

pub fn evaluate(document: &En1993Snapshot) -> CheckReport {
    check_full_steel_member(document)
}

pub struct En1993Family;

impl NormFamily for En1993Family {
    type Document = En1993Snapshot;
    type Mutation = En1993Mutation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1993
    }

    fn evaluate(document: &En1993Snapshot) -> CheckReport {
        evaluate(document)
    }
}
// #endregion 🔖️Session

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steel_member_e2e() {
        let report = check_steel_member(500.0, 150.0, 5000.0, 500_000.0, 355.0, 0.75);
        assert_eq!(report.checks.len(), 3);
        let params = AnnexParams::de();
        let n_rd = part_1_1::axial_resistance_kn(5000.0, 355.0, params);
        let n_b_rd = part_1_1::buckling_resistance_kn(5000.0, 355.0, 0.75, params);
        let m_rd = part_1_1::bending_resistance_knm(500_000.0, 355.0, params);
        assert!((report.checks[0].utilization - 500.0 / n_rd).abs() < 1e-6);
        assert!((report.checks[1].utilization - 500.0 / n_b_rd).abs() < 1e-6);
        assert!((report.checks[2].utilization - 150.0 / m_rd).abs() < 1e-6);
    }

    #[test]
    fn full_steel_member_e2e() {
        let report = check_full_steel_member(&En1993Snapshot::default());
        assert_eq!(report.checks.len(), 25);
    }

    #[test]
    fn every_part_reaches_evaluate() {
        let report = check_full_steel_member(&En1993Snapshot::default());
        let families: std::collections::BTreeSet<&str> = report.checks.iter().map(|c| c.clause.family.as_str()).collect();
        for expected in
            ["EN 1993-1-1", "EN 1993-1-2", "EN 1993-1-3", "EN 1993-1-4", "EN 1993-1-5", "EN 1993-1-6", "EN 1993-1-8", "EN 1993-1-9", "EN 1993-1-10", "EN 1993-1-11", "EN 1993-1-12", "EN 1993-2", "EN 1993-3-1", "EN 1993-4-1", "EN 1993-5", "EN 1993-6"]
        {
            assert!(families.contains(expected), "missing checks sourced from {expected}");
        }
    }

    #[test]
    #[cfg(feature = "cross-fem")]
    fn steel_member_from_fem_e2e() {
        let report = check_steel_member_from_fem(6.0, 20.0, 5000.0, 500_000.0, 2500.0, 355.0, 0.75).expect("fem solve");
        assert_eq!(report.checks.len(), 4);
        let m_ed = report.checks[2].computed.value / 1_000_000.0;
        assert!((m_ed - 90.0).abs() < 1.0);
    }

    #[test]
    fn plated_buckling_reduction() {
        let rho = part_1_5::plate_reduction_factor(0.5);
        assert!((rho - 1.0).abs() < 1e-9);
        let b_eff = part_1_5::effective_width_mm(200.0, 0.5);
        assert!((b_eff - 200.0).abs() < 1e-9);
    }

    #[test]
    fn hea200_section_classification() {
        let eps = part_1_1::epsilon(355.0);
        assert!((eps - 0.814).abs() < 0.01);
        let flange_c = (200.0 - 9.0) / 2.0 - 12.0;
        let web_c = 190.0 - 2.0 * 15.5 - 2.0 * 12.0;
        let class = part_1_1::section_class(flange_c, 15.5, web_c, 9.0, 355.0);
        assert!((1..=4).contains(&class));
    }

    #[test]
    fn hea200_chi_at_lambda_1() {
        let chi = part_1_1::chi(1.0, part_1_1::BucklingCurve::A0);
        assert!((chi - 0.73).abs() < 0.05);
    }

    #[test]
    fn axial_resistance_s355() {
        let n_rd = part_1_1::axial_resistance_kn(5382.0, 355.0, AnnexParams::de());
        assert!((n_rd - 1910.6).abs() < 5.0);
    }

    #[test]
    fn bolt_shear_m20() {
        let f_v = part_1_8::bolt_shear_resistance_kn(2, 245.0, 800.0, 1.25);
        assert!((f_v - 188.0).abs() < 5.0);
    }

    /// 🔩️ M20 bolt bearing worked example: e₁=e₂=40mm, d₀=22mm, t=10mm, f_u=510MPa, f_ub=800MPa → F_b,Rd ≈ 123.6 kN.
    #[test]
    fn bolt_bearing_m20_worked_example() {
        let alpha_b = part_1_8::bearing_alpha_b(40.0, 22.0, 800.0, 510.0);
        let k1 = part_1_8::bearing_k1(40.0, 22.0);
        assert!((alpha_b - (40.0_f64 / 66.0)).abs() < 1e-9);
        assert!((k1 - 2.5).abs() < 1e-9);
        let f_b_rd = part_1_8::bolt_bearing_resistance_kn(k1, alpha_b, 510.0, 20.0, 10.0, 1.25);
        assert!((f_b_rd - 123.636_363_636).abs() < 1e-3);
    }

    /// 🪛️ Fillet weld resistance sanity check for an S355 weld, β_w=0.9.
    #[test]
    fn fillet_weld_s355() {
        let beta_w = part_1_8::beta_w("S355");
        assert!((beta_w - 0.9).abs() < 1e-9);
        let f_w_rd = part_1_8::fillet_weld_resistance_kn(5.0, 100.0, 510.0, beta_w, 1.25);
        assert!(f_w_rd > 0.0);
        assert!((f_w_rd - (5.0 * 100.0 * 510.0 / (3.0_f64.sqrt() * 0.9 * 1.25) / 1000.0)).abs() < 1e-6);
    }

    #[test]
    fn annex_params_gamma_m1_divergence() {
        let en = AnnexParams::en();
        let de = AnnexParams::de();
        assert!((en.gamma_m0 - de.gamma_m0).abs() < 1e-9);
        assert!((en.gamma_m2 - de.gamma_m2).abs() < 1e-9);
        assert!((de.gamma_m1 / en.gamma_m1 - 1.1).abs() < 1e-9);
        let n_rd_en = part_1_1::buckling_resistance_kn(5000.0, 355.0, 0.75, en);
        let n_rd_de = part_1_1::buckling_resistance_kn(5000.0, 355.0, 0.75, de);
        assert!((n_rd_en / n_rd_de - 1.1).abs() < 1e-9);
    }

    /// 🔥️ Critical steel temperature worked example, EN 1993-1-2 Eq. 4.22 at μ₀=0.5.
    #[test]
    fn critical_steel_temperature_mu_0_5() {
        let mu_0: f64 = 0.5;
        let expected = 39.19 * (1.0 / (0.9674 * mu_0.powf(3.833)) - 1.0).ln() + 482.0;
        let theta = part_1_2::critical_temperature_c(mu_0);
        assert!((theta - expected).abs() < 1e-9);
        assert!((theta - 584.665).abs() < 1e-2);
    }

    #[test]
    fn net_tension_uses_gamma_m2_for_rupture() {
        let params = AnnexParams::en();
        let n_t_rd = part_1_1::net_tension_resistance_kn(5000.0, 4250.0, 355.0, 510.0, params);
        let net_rupture_kn = 0.9 * 4250.0 * 510.0 / params.gamma_m2 / 1000.0;
        let gross_yield_kn = 5000.0 * 355.0 / params.gamma_m0 / 1000.0;
        assert!((n_t_rd - net_rupture_kn.min(gross_yield_kn)).abs() < 1e-6);
        assert!(net_rupture_kn < gross_yield_kn, "worked example should be governed by net rupture");
    }

    #[test]
    fn chs_classification_table_5_2() {
        let class = part_1_1::chs_class(200.0, 8.0, 355.0);
        assert!((1..=4).contains(&class));
        assert_eq!(part_1_1::chs_class(10.0, 8.0, 355.0), 1);
    }

    /// 🐚️ Shell meridional critical buckling stress worked example: t=8mm, r=3000mm, E=210000MPa.
    #[test]
    fn shell_sigma_x_rcr_worked_example() {
        let sigma = part_1_6::sigma_x_rcr_mpa(8.0, 3000.0, 210_000.0);
        assert!((sigma - 338.8).abs() < 1e-9);
    }

    /// 📐️ Cold-formed reduction factor worked example: λ_p=1.0, ψ=1 → ρ=(1−0.055·4)/1=0.78.
    #[test]
    fn cold_formed_reduction_factor_worked_example() {
        let rho = part_1_3::reduction_factor(1.0, 1.0);
        assert!((rho - 0.78).abs() < 1e-9);
    }

    #[test]
    fn fatigue_detail_71() {
        assert!((part_1_9::fatigue_strength_mpa(71) - 71.0).abs() < 0.1);
    }

    #[test]
    fn fatigue_cycles_to_failure_slope_m3() {
        let n = part_1_9::cycles_to_failure(71.0, 71);
        assert!((n - 2.0e6).abs() < 1.0);
        let n_half_stress = part_1_9::cycles_to_failure(35.5, 71);
        assert!((n_half_stress / n - 8.0).abs() < 1e-6);
    }

    #[test]
    fn fatigue_assessment_methods() {
        assert!((part_1_9::AssessmentMethod::LowConsequence.gamma_mf() - 1.0).abs() < 1e-9);
        assert!((part_1_9::AssessmentMethod::DamageTolerant.gamma_mf() - 1.15).abs() < 1e-9);
        assert!((part_1_9::AssessmentMethod::SafeLife.gamma_mf() - 1.35).abs() < 1e-9);
    }

    #[test]
    fn tension_component_resistance() {
        let f_rd = part_1_11::tension_component_resistance_kn(500.0, 350.0);
        assert!((f_rd - (500.0_f64 / 1.5).min(350.0)).abs() < 1e-9);
    }

    #[test]
    fn high_strength_steel_class_4_guard() {
        assert!(part_1_12::is_high_strength_restricted(460.0));
        assert!(!part_1_12::is_high_strength_restricted(355.0));
        let m_rd_class4 = part_1_12::elastic_bending_resistance_knm(400_000.0, 460.0, 4, AnnexParams::de());
        assert_eq!(m_rd_class4, 0.0);
        let m_rd_class2 = part_1_12::elastic_bending_resistance_knm(400_000.0, 460.0, 2, AnnexParams::de());
        assert!(m_rd_class2 > 0.0);
    }

    #[test]
    fn stainless_steel_gamma_is_1_1_regardless_of_annex() {
        assert!((part_1_4::GAMMA_M_STAINLESS - 1.1).abs() < 1e-9);
        let m_rd = part_1_4::bending_resistance_knm(300_000.0, 220.0);
        assert!(m_rd > 0.0);
    }

    #[test]
    fn fire_board_r60() {
        let t = part_1_2::board_thickness_mm(part_1_2::FireRating::R60, 150.0);
        assert!(t > 15.0);
    }

    #[test]
    fn through_thickness_table_lookup() {
        let t_max = part_1_10::max_permissible_thickness_mm("J2", 0.0);
        assert!((t_max - 65.0).abs() < 1e-9);
    }

    #[test]
    fn silo_membrane_hoop_stress() {
        let p_h = part_4::janssen_pressure_kpa(0.4, 18.0, 5.0);
        assert!((p_h - 36.0).abs() < 1e-9);
        let sigma = part_4::membrane_hoop_stress_mpa(p_h, 3000.0, 8.0);
        assert!((sigma - 13.5).abs() < 1e-9);
    }

    #[test]
    fn crane_runway_web_stress() {
        let l_eff = part_6::effective_length_mm(100.0, 50.0);
        assert!((l_eff - 200.0).abs() < 1e-9);
        let sigma = part_6::wheel_load_web_stress_mpa(50.0, l_eff, 10.0);
        assert!((sigma - 25.0).abs() < 1e-9);
    }
}


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    register_artifact_schema();
    dsl::register_language(dsl::LanguageSpec {
        id: "en1993.document",
        extension: Some("en1993"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::en1992::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1992::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1993.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1993.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::en1992::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1992::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1993.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1993.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::en1992::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::en1992::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("en1993.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1993.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1993.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "en1993.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("en1993.spr"),
    });
}


//#region 🔖️SchemaRegistry
use std::sync::{Mutex, OnceLock};

/// 📌️ Registers the twenty handcrafted schema leaves for `s.norm.en1993`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::en1993::schema::en1993_artifact_schema_descriptor());
}

/// 💡️ Registers `s.norm.en1993.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::en1993::standards::v1::subsets::any::schema::inferences::en1993_artifact_inference_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🔖️IoFacet
pub fn register_io() {
    crate::artifacts::en1993::io_registry::register();
}
//#endregion 🔖️IoFacet
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::en1993::standards::v1::subsets::any::schema::En1993Composer as En1993AnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<En1993AnyComposer>(),
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
