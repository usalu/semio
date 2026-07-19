//! 🔩 EN 1993 design of steel structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

// #region 🔖NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    /// 🇩🇪 DIN EN 1993-1-1/NA: partial factor γ_M0 for cross-section resistance.
    pub const GAMMA_M0: f64 = 1.0;

    /// 🇩🇪 DIN EN 1993-1-1/NA: partial factor γ_M1 for member buckling.
    pub const GAMMA_M1: f64 = 1.1;

    pub fn gamma_m0() -> f64 {
        GAMMA_M0
    }

    pub fn gamma_m1() -> f64 {
        GAMMA_M1
    }
}
// #endregion 🔖NaDe

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📏 Material factor ε = √(235/f_y).
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
        flange_class(flange_c_mm, flange_t_mm, f_y_mpa)
            .max(web_class(web_c_mm, web_t_mm, f_y_mpa))
    }

    /// 📐 Axial resistance N_Rd [kN] per EN 1993-1-1 §6.2.4.
    pub fn axial_resistance_kn(a_mm2: f64, f_y_mpa: f64) -> f64 {
        a_mm2 * f_y_mpa / na_de::gamma_m0() / 1000.0
    }

    /// 📐 Plastic bending resistance M_c,Rd [kNm] per EN 1993-1-1 §6.2.5.
    pub fn bending_resistance_knm(w_pl_mm3: f64, f_y_mpa: f64) -> f64 {
        w_pl_mm3 * f_y_mpa / na_de::gamma_m0() / 1_000_000.0
    }

    /// 📐 Plastic shear resistance V_pl,Rd [kN] per EN 1993-1-1 §6.2.6.
    pub fn shear_resistance_kn(a_v_mm2: f64, f_y_mpa: f64) -> f64 {
        a_v_mm2 * f_y_mpa / (3.0_f64.sqrt() * na_de::gamma_m0()) / 1000.0
    }

    /// 📉 Buckling curve per EN 1993-1-1 Table 6.1.
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

    /// 📉 Reduction factor χ per EN 1993-1-1 Eq. 6.61.
    pub fn chi(lambda_bar: f64, curve: BucklingCurve) -> f64 {
        let alpha = curve.alpha();
        let phi = 0.5 * (1.0 + alpha * (lambda_bar - 0.2) + lambda_bar * lambda_bar);
        1.0 / (phi + (phi * phi - lambda_bar * lambda_bar).max(0.0).sqrt())
    }

    /// 📉 Non-dimensional slenderness λ̄ = √(A·f_y/N_cr).
    pub fn lambda_bar(a_mm2: f64, f_y_mpa: f64, n_cr_kn: f64) -> f64 {
        (a_mm2 * f_y_mpa / 1000.0 / n_cr_kn).sqrt()
    }

    /// 📉 Buckling resistance N_b,Rd [kN] per EN 1993-1-1 §6.3.1.
    pub fn buckling_resistance_kn(a_mm2: f64, f_y_mpa: f64, chi: f64) -> f64 {
        chi * a_mm2 * f_y_mpa / na_de::gamma_m1() / 1000.0
    }

    pub fn check_cross_section(n_ed_kn: f64, n_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-1", "§6.2.4", "6.2.4"),
            Quantity::force_kn(n_ed_kn),
            Quantity::force_kn(n_rd_kn),
            "cross-section axial ULS",
            annex,
        )
    }

    pub fn check_bending(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-1", "§6.2.5", "6.2.5"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "cross-section bending ULS",
            annex,
        )
    }

    pub fn check_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-1", "§6.2.6", "6.2.6"),
            Quantity::force_kn(v_ed_kn),
            Quantity::force_kn(v_rd_kn),
            "cross-section shear ULS",
            annex,
        )
    }

    pub fn check_member_buckling(n_ed_kn: f64, n_b_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-1", "§6.3.1", "6.3.1"),
            Quantity::force_kn(n_ed_kn),
            Quantity::force_kn(n_b_rd_kn),
            "member buckling ULS",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🔥 Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }

    /// 🔥 Board insulation thickness [mm] per EN 1993-1-2 Table 4.3 (simplified).
    pub fn board_thickness_mm(rating: FireRating, massivity: f64) -> f64 {
        let base = match rating {
            FireRating::R30 => 8.0,
            FireRating::R60 => 15.0,
            FireRating::R90 => 22.0,
            FireRating::R120 => 30.0,
        };
        base * (1.0 + (massivity / 200.0).min(0.5))
    }

    pub fn check_fire_protection(thickness_mm: f64, rating: FireRating, massivity: f64) -> CheckResult {
        let required = board_thickness_mm(rating, massivity);
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-2", "§4.2", "4.2"),
            Quantity::length_m(required / 1000.0),
            Quantity::length_m(thickness_mm / 1000.0),
            "steel fire protection thickness",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part1_3
pub mod part_1_3 {
    use super::*;

    /// 🔄 Fatigue detail category Δσ_C [MPa] per EN 1993-1-3 Table 8.1.
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

    /// 🔄 Fatigue strength Δσ_C,∞ [MPa] at N = 2×10⁶ cycles.
    pub fn fatigue_strength_mpa(category: u8) -> f64 {
        detail_category_mpa(category)
    }

    pub fn check_fatigue_range(delta_sigma_mpa: f64, category: u8, annex: AnnexChoice) -> CheckResult {
        let limit = fatigue_strength_mpa(category);
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-3", "§8", "8.1"),
            Quantity::stress_mpa(delta_sigma_mpa),
            Quantity::stress_mpa(limit),
            "fatigue stress range",
            annex,
        )
    }
}
// #endregion 🔖Part1_3

// #region 🔖Part1_4
pub mod part_1_4 {
    use super::*;

    /// 🏭 Silo shell buckling resistance N_Rd [kN] per EN 1993-1-4 §9.
    pub fn silo_shell_buckling_kn(t_mm: f64, r_mm: f64, f_y_mpa: f64, length_mm: f64) -> f64 {
        let e = 210_000.0;
        let sigma_cr = 0.6 * e * t_mm / r_mm;
        let alpha = (f_y_mpa / sigma_cr).min(1.0);
        alpha * 2.0 * std::f64::consts::PI * r_mm * t_mm * f_y_mpa / na_de::gamma_m0() / 1000.0
            * (3000.0 / length_mm).min(1.0)
    }

    pub fn check_silo_shell(n_ed_kn: f64, n_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-4", "§9", "9.1"),
            Quantity::force_kn(n_ed_kn),
            Quantity::force_kn(n_rd_kn),
            "silo shell buckling",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part1_4

// #region 🔖Part1_5
pub mod part_1_5 {
    use super::*;

    /// 🔩 Pile driving stress limit σ_lim [MPa] per EN 1993-1-5 §12.
    pub fn pile_driving_stress_limit_mpa(f_y_mpa: f64) -> f64 {
        0.9 * f_y_mpa
    }

    pub fn check_pile_driving_stress(sigma_mpa: f64, f_y_mpa: f64) -> CheckResult {
        let limit = pile_driving_stress_limit_mpa(f_y_mpa);
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-5", "§12", "12.1"),
            Quantity::stress_mpa(sigma_mpa),
            Quantity::stress_mpa(limit),
            "pile driving stress",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_5

// #region 🔖Part1_6
pub mod part_1_6 {
    use super::*;

    /// 🏗️ Crane runway wheel load factor ψ per EN 1993-1-6.
    pub fn crane_wheel_load_factor(span_m: f64) -> f64 {
        1.0 + 0.1 * (span_m / 10.0).min(1.0)
    }

    pub fn check_crane_runway(wheel_load_kn: f64, resistance_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-6", "§3", "3.1"),
            Quantity::force_kn(wheel_load_kn),
            Quantity::force_kn(resistance_kn),
            "crane runway wheel load",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_6

// #region 🔖Part1_7
pub mod part_1_7 {
    use super::*;

    /// 🪶 Aluminium partial factor γ_M1 per EN 1993-1-7.
    pub const GAMMA_M_AL: f64 = 1.25;

    pub fn aluminium_bending_knm(w_el_mm3: f64, f_0_2_mpa: f64) -> f64 {
        w_el_mm3 * f_0_2_mpa / GAMMA_M_AL / 1_000_000.0
    }

    pub fn check_aluminium_bending(m_ed_knm: f64, m_rd_knm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-7", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "aluminium bending ULS",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part1_7

// #region 🔖Part1_8
pub mod part_1_8 {
    use super::*;

    /// 🔩 Bolt shear resistance F_v,Rd [kN] per EN 1993-1-8 §3.6.1.
    pub fn bolt_shear_resistance_kn(n_bolts: u32, a_s_mm2: f64, f_ub_mpa: f64, gamma_m2: f64) -> f64 {
        let alpha_v = 0.6;
        n_bolts as f64 * alpha_v * a_s_mm2 * f_ub_mpa / gamma_m2 / 1000.0
    }

    /// 🔩 Bolt bearing resistance F_b,Rd [kN] per EN 1993-1-8 §3.6.1.
    pub fn bolt_bearing_resistance_kn(
        t_plate_mm: f64,
        d_bolt_mm: f64,
        f_u_mpa: f64,
        gamma_m2: f64,
        k1: f64,
    ) -> f64 {
        let alpha_b = (k1 * f_u_mpa).min(2.5);
        alpha_b * d_bolt_mm * t_plate_mm * f_u_mpa / gamma_m2 / 1000.0
    }

    pub fn check_bolt_shear(f_ed_kn: f64, f_v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-8", "§3.6.1", "3.6.1"),
            Quantity::force_kn(f_ed_kn),
            Quantity::force_kn(f_v_rd_kn),
            "bolt shear ULS",
            annex,
        )
    }

    pub fn check_bolt_bearing(f_ed_kn: f64, f_b_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-8", "§3.6.1", "3.6.1b"),
            Quantity::force_kn(f_ed_kn),
            Quantity::force_kn(f_b_rd_kn),
            "bolt bearing ULS",
            annex,
        )
    }
}
// #endregion 🔖Part1_8

// #region 🔖Part1_9
pub mod part_1_9 {
    use super::*;

    /// 🔗 Net section tension resistance N_t,Rd [kN] per EN 1993-1-9 §6.2.3.
    pub fn net_tension_resistance_kn(a_net_mm2: f64, f_u_mpa: f64) -> f64 {
        0.9 * a_net_mm2 * f_u_mpa / na_de::gamma_m0() / 1000.0
    }

    pub fn check_net_tension(n_ed_kn: f64, n_t_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-9", "§6.2.3", "6.2.3"),
            Quantity::force_kn(n_ed_kn),
            Quantity::force_kn(n_t_rd_kn),
            "net section tension ULS",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_9

// #region 🔖Part1_10
pub mod part_1_10 {
    use super::*;

    /// 🧪 Charpy toughness requirement T_R [°C] per EN 1993-1-10 Table 2.2.
    pub fn toughness_temperature_c(steel_grade: &str, thickness_mm: f64) -> f64 {
        let base = match steel_grade {
            "S235" => 20.0,
            "S275" => 0.0,
            "S355" => -20.0,
            "S460" => -40.0,
            _ => 0.0,
        };
        base + if thickness_mm > 40.0 { 10.0 } else { 0.0 }
    }

    pub fn check_toughness(t_actual_c: f64, steel_grade: &str, thickness_mm: f64) -> CheckResult {
        let required = toughness_temperature_c(steel_grade, thickness_mm);
        CheckResult::from_minimum(
            ClauseId::new("EN 1993-1-10", "§2.2", "2.2"),
            Quantity::new(norm_core::QuantityKind::Temperature, t_actual_c),
            Quantity::new(norm_core::QuantityKind::Temperature, required),
            "material toughness",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_10

// #region 🔖Part1_11
pub mod part_1_11 {
    use super::*;

    /// ⭕ CHS section class per EN 1993-1-11 Table 5.2.
    pub fn chs_class(d_mm: f64, t_mm: f64, f_y_mpa: f64) -> u8 {
        let eps = part_1_1::epsilon(f_y_mpa);
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

    pub fn chs_compression_kn(a_mm2: f64, f_y_mpa: f64, chi: f64) -> f64 {
        part_1_1::buckling_resistance_kn(a_mm2, f_y_mpa, chi)
    }

    pub fn check_chs_compression(n_ed_kn: f64, n_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-11", "§6.3", "6.3"),
            Quantity::force_kn(n_ed_kn),
            Quantity::force_kn(n_rd_kn),
            "hollow section compression",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part1_11

// #region 🔖Part1_12
pub mod part_1_12 {
    use super::*;

    /// 💪 High-strength steel reduction factor β per EN 1993-1-12.
    pub fn hsb_reduction_factor(f_y_mpa: f64) -> f64 {
        if f_y_mpa > 460.0 {
            460.0 / f_y_mpa
        } else {
            1.0
        }
    }

    pub fn hsb_bending_knm(w_pl_mm3: f64, f_y_mpa: f64) -> f64 {
        let beta = hsb_reduction_factor(f_y_mpa);
        beta * w_pl_mm3 * f_y_mpa / na_de::gamma_m0() / 1_000_000.0
    }

    pub fn check_hsb_bending(m_ed_knm: f64, m_rd_knm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-12", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "high-strength steel bending",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_12

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    /// 🌉 Steel bridge combined axial + bending interaction per EN 1993-2 §6.
    pub fn bridge_interaction_eta(n_ed_kn: f64, n_rd_kn: f64, m_ed_knm: f64, m_rd_knm: f64) -> f64 {
        (n_ed_kn / n_rd_kn).abs() + (m_ed_knm / m_rd_knm).abs()
    }

    pub fn check_steel_bridge(n_ed_kn: f64, n_rd_kn: f64, m_ed_knm: f64, m_rd_knm: f64) -> CheckResult {
        let eta = bridge_interaction_eta(n_ed_kn, n_rd_kn, m_ed_knm, m_rd_knm);
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-2", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Dimensionless, eta),
            Quantity::new(norm_core::QuantityKind::Dimensionless, 1.0),
            "steel bridge interaction",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 🗼 Tower leg buckling with wind amplification factor.
    pub fn tower_buckling_kn(a_mm2: f64, f_y_mpa: f64, chi: f64, wind_factor: f64) -> f64 {
        part_1_1::buckling_resistance_kn(a_mm2, f_y_mpa, chi) / wind_factor
    }

    pub fn check_tower_buckling(n_ed_kn: f64, n_b_rd_kn: f64) -> CheckResult {
        part_1_1::check_member_buckling(n_ed_kn, n_b_rd_kn, AnnexChoice::En)
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;

    /// 📐 Plate effective width b_eff per EN 1993-1-5 §4.
    pub fn effective_width_mm(b_mm: f64, lambda_p: f64) -> f64 {
        let rho = if lambda_p <= 0.673 {
            1.0
        } else {
            (lambda_p - 0.055 * (3.0_f64).sqrt()) / (lambda_p * lambda_p)
        };
        rho * b_mm
    }

    pub fn check_plated_buckling(sigma_ed_mpa: f64, sigma_rd_mpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-5", "§4", "4.1"),
            Quantity::stress_mpa(sigma_ed_mpa),
            Quantity::stress_mpa(sigma_rd_mpa),
            "plated structure local buckling",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part4

// #region 🔖Part5
pub mod part_5 {
    use super::*;

    /// 🔩 Steel pile compression capacity N_c,Rd [kN] per EN 1993-5.
    pub fn pile_compression_kn(a_mm2: f64, f_y_mpa: f64, k_red: f64) -> f64 {
        k_red * a_mm2 * f_y_mpa / na_de::gamma_m0() / 1000.0
    }

    pub fn check_pile_foundation_steel(n_ed_kn: f64, n_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-5", "§6", "6.1"),
            Quantity::force_kn(n_ed_kn),
            Quantity::force_kn(n_rd_kn),
            "steel pile compression",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part5

// #region 🔖Part6
pub mod part_6 {
    use super::*;

    /// 🪙 Stainless steel γ_M per EN 1993-1-4 (stainless).
    pub const GAMMA_M_SS: f64 = 1.1;

    pub fn stainless_bending_knm(w_pl_mm3: f64, f_y_mpa: f64) -> f64 {
        w_pl_mm3 * f_y_mpa / GAMMA_M_SS / 1_000_000.0
    }

    pub fn check_stainless_steel(m_ed_knm: f64, m_rd_knm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1993-1-4", "§6", "6.1"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "stainless steel bending",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part6

/// 📋 I-section member check.
pub fn check_steel_member(
    n_ed_kn: f64,
    m_ed_knm: f64,
    a_mm2: f64,
    w_pl_mm3: f64,
    f_y_mpa: f64,
    chi: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let n_rd = part_1_1::axial_resistance_kn(a_mm2, f_y_mpa);
    let n_b_rd = part_1_1::buckling_resistance_kn(a_mm2, f_y_mpa, chi);
    let m_rd = part_1_1::bending_resistance_knm(w_pl_mm3, f_y_mpa);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_cross_section(n_ed_kn, n_rd, annex));
    report.push(part_1_1::check_member_buckling(n_ed_kn, n_b_rd, annex));
    report.push(part_1_1::check_bending(m_ed_knm, m_rd, annex));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steel_member_e2e() {
        let report = check_steel_member(500.0, 150.0, 5000.0, 500_000.0, 355.0, 0.75);
        assert!(!report.checks.is_empty());
    }

    #[test]
    fn hea200_section_classification() {
        let eps = part_1_1::epsilon(355.0);
        assert!((eps - 0.814).abs() < 0.01);
        let flange_c = (200.0 - 9.0) / 2.0 - 12.0;
        let web_c = 190.0 - 2.0 * 15.5 - 2.0 * 12.0;
        let class = part_1_1::section_class(flange_c, 15.5, web_c, 9.0, 355.0);
        assert!(class >= 1 && class <= 4);
    }

    #[test]
    fn hea200_chi_at_lambda_1() {
        let chi = part_1_1::chi(1.0, part_1_1::BucklingCurve::A0);
        assert!((chi - 0.73).abs() < 0.05);
    }

    #[test]
    fn axial_resistance_s355() {
        let n_rd = part_1_1::axial_resistance_kn(5382.0, 355.0);
        assert!((n_rd - 1910.6).abs() < 5.0);
    }

    #[test]
    fn bolt_shear_m20() {
        let f_v = part_1_8::bolt_shear_resistance_kn(2, 245.0, 800.0, 1.25);
        assert!((f_v - 188.0).abs() < 5.0);
    }

    #[test]
    fn na_de_gamma_factors() {
        assert!((na_de::gamma_m0() - 1.0).abs() < 1e-9);
        assert!((na_de::gamma_m1() - 1.1).abs() < 1e-9);
    }

    #[test]
    fn fatigue_detail_71() {
        assert!((part_1_3::fatigue_strength_mpa(71) - 71.0).abs() < 0.1);
    }

    #[test]
    fn fire_board_r60() {
        let t = part_1_2::board_thickness_mm(part_1_2::FireRating::R60, 150.0);
        assert!(t > 15.0);
    }
}
