//! 🧱 EN 1992 design of concrete structures.

use norm_core::{table_lookup_linear, AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, Quantity, TableEntry1D};
use serde::{Deserialize, Serialize};

// #region 🔖NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
    use super::AnnexChoice;

    /// 🇪🇺 Material factors that genuinely diverge between the EN-recommended values and DIN EN 1992-1-1/NA: α_cc, α_ct per §3.1.6(1)P/(2)P.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexParams {
        pub alpha_cc: f64,
        pub alpha_ct: f64,
        pub gamma_c: f64,
        pub gamma_s: f64,
    }

    impl AnnexParams {
        /// 🇪🇺 EN-recommended values (α_cc = α_ct = 1.0).
        pub fn en() -> Self {
            Self { alpha_cc: 1.0, alpha_ct: 1.0, gamma_c: 1.5, gamma_s: 1.15 }
        }

        /// 🇩🇪 DIN EN 1992-1-1/NA values (α_cc = α_ct = 0.85).
        pub fn de() -> Self {
            Self { alpha_cc: 0.85, alpha_ct: 0.85, gamma_c: 1.5, gamma_s: 1.15 }
        }

        pub fn for_choice(choice: AnnexChoice) -> Self {
            match choice {
                AnnexChoice::En => Self::en(),
                AnnexChoice::De => Self::de(),
            }
        }

        /// 📐 Design compressive strength f_cd = α_cc·f_ck/γ_C [MPa].
        pub fn f_cd_mpa(&self, f_ck_mpa: f64) -> f64 {
            self.alpha_cc * f_ck_mpa / self.gamma_c
        }

        /// 📐 Design tensile strength f_ctd = α_ct·f_ctk/γ_C [MPa].
        pub fn f_ctd_mpa(&self, f_ctk_mpa: f64) -> f64 {
            self.alpha_ct * f_ctk_mpa / self.gamma_c
        }
    }
}
// #endregion 🔖NaDe

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📐 Flexural resistance M_Rd [kNm] per EN 1992-1-1 §6.1.
    pub fn flexural_resistance_knm(f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, annex: AnnexChoice) -> f64 {
        let f_cd = na_de::AnnexParams::for_choice(annex).f_cd_mpa(f_ck) / 1000.0;
        let f_yd = f_yk / 1.15 / 1000.0;
        let x = a_s_mm2 * f_yd / (0.8 * b_mm * f_cd);
        let z = d_mm - 0.4 * x;
        a_s_mm2 * f_yd * z / 1_000_000.0
    }

    /// 📐 Shear resistance V_Rd,c [kN] per EN 1992-1-1 §6.2.2.
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

    /// 🔁 Torsional resistance T_Rd [kNm] per EN 1992-1-1 §6.3.2 (thin-walled hollow section).
    pub fn torsion_resistance_knm(f_ck: f64, a_k_mm2: f64, t_mm: f64, annex: AnnexChoice) -> f64 {
        let f_cd = na_de::AnnexParams::for_choice(annex).f_cd_mpa(f_ck) / 1000.0;
        let nu = 0.6 * (1.0 - f_ck / 250.0);
        let alpha_cw = 1.0;
        2.0 * nu * alpha_cw * f_cd * t_mm * a_k_mm2 / 1_000_000.0
    }

    /// 📏 Slenderness λ = l_0 / i.
    pub fn slenderness_lambda(l_0_mm: f64, i_mm: f64) -> f64 {
        l_0_mm / i_mm
    }

    /// 📏 Radius of gyration i [mm] from area and second moment.
    pub fn radius_of_gyration_mm(a_mm2: f64, i_mm4: f64) -> f64 {
        (i_mm4 / a_mm2).sqrt()
    }

    /// 🪟 Crack width w_k [mm] per EN 1992-1-1 Eq. 7.8.
    pub fn crack_width_wk_mm(eps_sm: f64, eps_cm: f64, s_r_max_mm: f64) -> f64 {
        (eps_sm - eps_cm).max(0.0) * s_r_max_mm
    }

    /// 🪟 Mean steel strain ε_sm per EN 1992-1-1 Eq. 7.9.
    pub fn steel_strain_eps_sm(sigma_s_mpa: f64, rho_p_eff: f64, f_ct_eff_mpa: f64, e_s_mpa: f64) -> f64 {
        let term = (f_ct_eff_mpa / rho_p_eff / e_s_mpa).max(0.6 * sigma_s_mpa / e_s_mpa);
        (sigma_s_mpa / e_s_mpa) * (1.0 - term).max(0.4)
    }

    /// 📉 Immediate deflection δ [mm] of simply supported beam under UDL.
    pub fn deflection_ss_udl_mm(w_kn_m: f64, span_m: f64, e_mpa: f64, i_mm4: f64) -> f64 {
        let w = w_kn_m;
        let l = span_m * 1000.0;
        5.0 * w * l.powi(4) / (384.0 * e_mpa * i_mm4)
    }

    pub fn check_flexure(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§6.1", "6.1"), Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0), Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0), "flexural ULS", annex)
    }

    pub fn check_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§6.2", "6.2"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "shear ULS", annex)
    }

    pub fn check_punching(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§6.4", "6.4"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "punching shear ULS", annex)
    }

    pub fn check_torsion(t_ed_knm: f64, t_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§6.3", "6.3"), Quantity::new(norm_core::QuantityKind::Moment, t_ed_knm * 1_000_000.0), Quantity::new(norm_core::QuantityKind::Moment, t_rd_knm * 1_000_000.0), "torsion ULS", annex)
    }

    pub fn check_crack_width(w_k_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§7.3", "7.3"), Quantity::length_m(w_k_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "crack width SLS", annex)
    }

    pub fn check_deflection(delta_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§7.4", "7.4"), Quantity::length_m(delta_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "deflection SLS", annex)
    }

    /// 🎯 Transfer stress σ_c = P / A_c [MPa] at prestressing.
    pub fn prestress_transfer_stress_mpa(p_kn: f64, a_c_mm2: f64) -> f64 {
        p_kn * 1000.0 / a_c_mm2
    }

    /// 🎯 Maximum transfer stress limit 0.6·f_ck per EN 1992-1-1 §5.10.9.
    pub fn prestress_transfer_limit_mpa(f_ck_mpa: f64) -> f64 {
        0.6 * f_ck_mpa
    }

    pub fn check_prestress_transfer(sigma_c_mpa: f64, f_ck_mpa: f64, annex: AnnexChoice) -> CheckResult {
        let limit = prestress_transfer_limit_mpa(f_ck_mpa);
        CheckResult::from_utilization(ClauseId::new("EN 1992-1-1", "§5.10", "5.10"), Quantity::stress_mpa(sigma_c_mpa), Quantity::stress_mpa(limit), "prestress transfer ULS", annex)
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    /// 🏗️ Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }

    /// 🏗️ Structural element type for fire cover lookup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ElementType {
        Slab,
        Beam,
        Column,
    }

    /// 🔥 Minimum axis distance a_min [mm] per EN 1992-1-2 Table 5.5 (simplified tabulated values).
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

    /// 🔥 Table 5.5 (b_min, a) [mm] combinations per EN 1992-1-2 §5.6.3 for simply-supported rectangular beams.
    fn table_5_5(rating: FireRating) -> &'static [TableEntry1D] {
        match rating {
            FireRating::R30 => &[TableEntry1D { x: 80.0, y: 25.0 }, TableEntry1D { x: 120.0, y: 15.0 }],
            FireRating::R60 => &[TableEntry1D { x: 120.0, y: 40.0 }, TableEntry1D { x: 160.0, y: 35.0 }, TableEntry1D { x: 200.0, y: 30.0 }, TableEntry1D { x: 300.0, y: 25.0 }],
            FireRating::R90 => &[TableEntry1D { x: 150.0, y: 55.0 }, TableEntry1D { x: 200.0, y: 45.0 }, TableEntry1D { x: 300.0, y: 40.0 }, TableEntry1D { x: 400.0, y: 35.0 }],
            FireRating::R120 => &[TableEntry1D { x: 200.0, y: 65.0 }, TableEntry1D { x: 240.0, y: 60.0 }, TableEntry1D { x: 300.0, y: 55.0 }, TableEntry1D { x: 500.0, y: 50.0 }],
        }
    }

    /// 🔥 Required axis distance a [mm] for a simply-supported rectangular beam of given width, interpolated from Table 5.5.
    pub fn required_axis_distance_beam_mm(width_mm: f64, rating: FireRating) -> f64 {
        table_lookup_linear(table_5_5(rating), width_mm)
    }

    /// 🔥 Simply-supported beam fire check: provided axis distance vs Table 5.5 requirement for the given width.
    pub fn check_fire_beam_axis_distance(width_mm: f64, provided_a_mm: f64, rating: FireRating) -> CheckResult {
        let required = required_axis_distance_beam_mm(width_mm, rating);
        CheckResult::from_minimum(ClauseId::new("EN 1992-1-2", "Table 5.5", "5.6.3"), Quantity::length_m(provided_a_mm / 1000.0), Quantity::length_m(required / 1000.0), "fire simply-supported beam axis distance", AnnexChoice::En)
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    /// 🌉 Load-cycle amplification factor γ_F,fat for the fatigue action per EN 1992-2 §6.8.
    pub const GAMMA_F_FAT: f64 = 1.0;

    /// 🌉 Partial factor γ_S,fat for reinforcement fatigue resistance per EN 1992-1-1 §2.4.2.4.
    pub const GAMMA_S_FAT: f64 = 1.15;

    /// 🔁 Reinforcement fatigue stress range Δσ_Rsk(N*) [MPa] at N* = 10⁶ cycles, straight bars, per EN 1992-1-1 Table 6.3N.
    pub const DELTA_SIGMA_RSK_MPA: f64 = 162.5;

    pub fn check_bridge_flexure(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::En)
    }

    /// 🌉 Concrete compressive stress limit 0.6·f_ck [MPa] under the frequent combination per EN 1992-2 §7.2.
    pub fn concrete_stress_limit_frequent_mpa(f_ck: f64) -> f64 {
        0.6 * f_ck
    }

    pub fn check_bridge_concrete_stress(sigma_c_mpa: f64, f_ck: f64) -> CheckResult {
        let limit = concrete_stress_limit_frequent_mpa(f_ck);
        CheckResult::from_utilization(ClauseId::new("EN 1992-2", "§7.2", "7.2"), Quantity::stress_mpa(sigma_c_mpa), Quantity::stress_mpa(limit), "bridge concrete compressive stress, frequent combination", AnnexChoice::En)
    }

    /// 🔁 Design fatigue resistance Δσ_Rsk(N*)/γ_S,fat [MPa] per EN 1992-1-1 §6.8.
    pub fn fatigue_resistance_design_mpa() -> f64 {
        DELTA_SIGMA_RSK_MPA / GAMMA_S_FAT
    }

    /// 🔁 Reinforcement fatigue verification γ_F,fat·Δσ_s ≤ Δσ_Rsk(N*)/γ_S,fat per EN 1992-2 §6.8.
    pub fn check_bridge_fatigue(delta_sigma_s_mpa: f64) -> CheckResult {
        let demand = GAMMA_F_FAT * delta_sigma_s_mpa;
        let resistance = fatigue_resistance_design_mpa();
        CheckResult::from_utilization(ClauseId::new("EN 1992-2", "§6.8", "6.8.4"), Quantity::stress_mpa(demand), Quantity::stress_mpa(resistance), "reinforcement fatigue", AnnexChoice::En)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    /// 💧 Exposure class steel stress limit σ_s,lim [MPa] per EN 1992-3 Table 7.1N.
    pub fn steel_stress_limit_mpa(exposure: &str) -> f64 {
        match exposure {
            "XC1" | "XC2" => 250.0,
            "XC3" | "XC4" => 200.0,
            "XD1" | "XD2" | "XD3" => 160.0,
            "XS1" | "XS2" | "XS3" => 160.0,
            _ => 200.0,
        }
    }

    /// 🪟 Liquid-retaining crack width w_k [mm] with steel stress limit per EN 1992-3 §7.
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

    /// 💧 Tightness class per EN 1992-3 Table 7.105: required degree of protection against leakage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum TightnessClass {
        Tc0,
        Tc1,
        Tc2,
    }

    /// 💧 Tightness-class crack-width limit w_k,lim [mm] per EN 1992-3 Table 7.1N; `None` means TC0 has no crack-width requirement. TC2 interpolates between w_k1 = 0.2mm (h_D/h = 5) and w_k1 = 0.05mm (h_D/h = 35).
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

    /// 🪟 Liquid-retaining crack width [mm], reusing the general EN 1992-1-1 §7.3 mechanics (Eq. 7.8/7.9).
    pub fn crack_width_tightness_mm(sigma_s_mpa: f64, rho_p_eff: f64, f_ct_eff_mpa: f64, e_s_mpa: f64, s_r_max_mm: f64) -> f64 {
        let eps_sm = part_1_1::steel_strain_eps_sm(sigma_s_mpa, rho_p_eff, f_ct_eff_mpa, e_s_mpa);
        part_1_1::crack_width_wk_mm(eps_sm, 0.0, s_r_max_mm)
    }

    pub fn check_tightness_crack_width(w_k_mm: f64, class: TightnessClass, hd_over_h: f64) -> CheckResult {
        let clause = ClauseId::new("EN 1992-3", "Table 7.1N", "7.3.2");
        match tightness_crack_width_limit_mm(class, hd_over_h) {
            Some(limit) => CheckResult::from_utilization(clause, Quantity::length_m(w_k_mm / 1000.0), Quantity::length_m(limit / 1000.0), "liquid retaining tightness-class crack width", AnnexChoice::En),
            None => CheckResult { clause, status: CheckStatus::NotApplicable, computed: Quantity::length_m(w_k_mm / 1000.0), limit: Quantity::length_m(0.0), utilization: 0.0, message: "TC0: no crack-width requirement".into(), annex: AnnexChoice::En },
        }
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
/// ⚓ EN 1992-4: design of fastenings (anchors) to concrete — steel, concrete cone and edge breakout failure modes.
pub mod part_4 {
    use super::*;

    /// ⚓ Partial factor for concrete failure modes γ_Mc per EN 1992-4 §4.4.
    pub const GAMMA_MC: f64 = 1.5;

    /// ⚓ Partial factor for steel failure γ_Ms = max(1.2·f_uk/f_yk, 1.4) per EN 1992-4 §4.4.2.
    pub fn gamma_ms(f_uk_mpa: f64, f_yk_mpa: f64) -> f64 {
        (1.2 * f_uk_mpa / f_yk_mpa).max(1.4)
    }

    /// ⚓ Steel failure characteristic resistance N_Rk,s = A_s·f_uk [N] per EN 1992-4 §7.2.1.4.
    pub fn steel_resistance_n_rk_s_n(a_s_mm2: f64, f_uk_mpa: f64) -> f64 {
        a_s_mm2 * f_uk_mpa
    }

    pub fn steel_resistance_design_n(a_s_mm2: f64, f_uk_mpa: f64, f_yk_mpa: f64) -> f64 {
        steel_resistance_n_rk_s_n(a_s_mm2, f_uk_mpa) / gamma_ms(f_uk_mpa, f_yk_mpa)
    }

    /// ⚓ Concrete cone factor k [N^0.5/mm^0.5] per EN 1992-4 §7.2.1.5: cracked vs uncracked concrete.
    pub fn concrete_cone_k(cracked: bool) -> f64 {
        if cracked {
            7.7
        } else {
            11.0
        }
    }

    /// ⚓ Basic concrete cone characteristic resistance N⁰_Rk,c = k·√f_ck·h_ef^1.5 [N, mm, MPa] per EN 1992-4 Eq. 7.2.
    pub fn concrete_cone_resistance_n0_rk_c_n(f_ck_mpa: f64, h_ef_mm: f64, cracked: bool) -> f64 {
        concrete_cone_k(cracked) * f_ck_mpa.sqrt() * h_ef_mm.powf(1.5)
    }

    pub fn concrete_cone_resistance_design_n(f_ck_mpa: f64, h_ef_mm: f64, cracked: bool) -> f64 {
        concrete_cone_resistance_n0_rk_c_n(f_ck_mpa, h_ef_mm, cracked) / GAMMA_MC
    }

    /// ⚓ Simplified single-anchor concrete edge breakout characteristic resistance V⁰_Rk,c = 1.6·√d·√h_ef·√f_ck·c₁^1.5 [N, mm, MPa], a simplified form of EN 1992-4 §7.2.2.5.
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
// #endregion 🔖Part4

/// 📋 RC beam ULS check end-to-end.
pub fn check_rc_beam(m_ed_knm: f64, v_ed_kn: f64, f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, rho_l: f64, n_ed_kn: f64, annex: AnnexChoice) -> CheckReport {
    let m_rd = part_1_1::flexural_resistance_knm(f_ck, b_mm, d_mm, a_s_mm2, f_yk, annex);
    let v_rd = part_1_1::shear_resistance_vrdc_kn(b_mm, d_mm, f_ck, rho_l, n_ed_kn);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_flexure(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_shear(v_ed_kn, v_rd, annex));
    report
}

/// 📋 Full EN 1992 RC beam check with optional prestress transfer.
pub fn check_full_rc_beam(m_ed_knm: f64, v_ed_kn: f64, f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, rho_l: f64, n_ed_kn: f64, p_kn: f64, a_c_mm2: f64, annex: AnnexChoice) -> CheckReport {
    let mut report = check_rc_beam(m_ed_knm, v_ed_kn, f_ck, b_mm, d_mm, a_s_mm2, f_yk, rho_l, n_ed_kn, annex);
    if p_kn > 0.0 {
        let sigma_c = part_1_1::prestress_transfer_stress_mpa(p_kn, a_c_mm2);
        report.push(part_1_1::check_prestress_transfer(sigma_c, f_ck, annex));
    }
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOperation};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "en1992", layout = "lines")]
pub struct Document {
    pub annex: AnnexChoice,
    pub m_ed_knm: f64,
    pub v_ed_kn: f64,
    pub f_ck: f64,
    pub b_mm: f64,
    pub d_mm: f64,
    pub a_s_mm2: f64,
    pub f_yk: f64,
    pub rho_l: f64,
    pub n_ed_kn: f64,
    pub p_kn: f64,
    pub a_c_mm2: f64,
    pub use_fem: bool,
    pub span_m: f64,
    pub udl_kn_m: f64,
    pub fire_rating: part_1_2::FireRating,
    pub provided_axis_distance_mm: f64,
    pub bridge_sigma_c_mpa: f64,
    pub bridge_delta_sigma_s_mpa: f64,
    pub tightness_class: part_3::TightnessClass,
    pub hd_over_h: f64,
    pub liquid_sigma_s_mpa: f64,
    pub liquid_rho_p_eff: f64,
    pub liquid_f_ct_eff_mpa: f64,
    pub liquid_e_s_mpa: f64,
    pub liquid_s_r_max_mm: f64,
    pub anchor_h_ef_mm: f64,
    pub anchor_cracked: bool,
    pub anchor_f_uk_mpa: f64,
    pub anchor_f_yk_mpa: f64,
    pub anchor_a_s_mm2: f64,
    pub anchor_d_mm: f64,
    pub anchor_c1_mm: f64,
    pub anchor_n_ed_kn: f64,
    pub anchor_v_ed_kn: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 120.0,
            v_ed_kn: 80.0,
            f_ck: 30.0,
            b_mm: 300.0,
            d_mm: 450.0,
            a_s_mm2: 1200.0,
            f_yk: 500.0,
            rho_l: 0.01,
            n_ed_kn: 0.0,
            p_kn: 0.0,
            a_c_mm2: 135_000.0,
            use_fem: false,
            span_m: 6.0,
            udl_kn_m: 20.0,
            fire_rating: part_1_2::FireRating::R60,
            provided_axis_distance_mm: 30.0,
            bridge_sigma_c_mpa: 12.0,
            bridge_delta_sigma_s_mpa: 100.0,
            tightness_class: part_3::TightnessClass::Tc1,
            hd_over_h: 10.0,
            liquid_sigma_s_mpa: 200.0,
            liquid_rho_p_eff: 0.01,
            liquid_f_ct_eff_mpa: 2.9,
            liquid_e_s_mpa: 200_000.0,
            liquid_s_r_max_mm: 250.0,
            anchor_h_ef_mm: 80.0,
            anchor_cracked: false,
            anchor_f_uk_mpa: 800.0,
            anchor_f_yk_mpa: 640.0,
            anchor_a_s_mm2: 84.3,
            anchor_d_mm: 12.0,
            anchor_c1_mm: 100.0,
            anchor_n_ed_kn: 10.0,
            anchor_v_ed_kn: 5.0,
        }
    }
}

pub type Operation = SetDocumentOperation<Document>;
pub type Host = NormHost<En1992Family>;

pub fn evaluate(document: &Document) -> CheckReport {
    let mut report = if document.use_fem {
        check_rc_beam_from_fem(document.span_m, document.udl_kn_m, document.f_ck, document.b_mm, document.d_mm, document.a_s_mm2, document.f_yk, document.rho_l, document.annex).unwrap_or_else(|_| CheckReport::default())
    } else {
        check_full_rc_beam(
            document.m_ed_knm,
            document.v_ed_kn,
            document.f_ck,
            document.b_mm,
            document.d_mm,
            document.a_s_mm2,
            document.f_yk,
            document.rho_l,
            document.n_ed_kn,
            document.p_kn,
            document.a_c_mm2,
            document.annex,
        )
    };

    report.push(part_1_2::check_fire_beam_axis_distance(document.b_mm, document.provided_axis_distance_mm, document.fire_rating));

    report.push(part_2::check_bridge_concrete_stress(document.bridge_sigma_c_mpa, document.f_ck));
    report.push(part_2::check_bridge_fatigue(document.bridge_delta_sigma_s_mpa));

    let w_k_liquid = part_3::crack_width_tightness_mm(document.liquid_sigma_s_mpa, document.liquid_rho_p_eff, document.liquid_f_ct_eff_mpa, document.liquid_e_s_mpa, document.liquid_s_r_max_mm);
    report.push(part_3::check_tightness_crack_width(w_k_liquid, document.tightness_class, document.hd_over_h));

    let anchor_n_ed_n = document.anchor_n_ed_kn * 1000.0;
    report.push(part_4::check_anchor_steel(anchor_n_ed_n, document.anchor_a_s_mm2, document.anchor_f_uk_mpa, document.anchor_f_yk_mpa));
    report.push(part_4::check_anchor_concrete_cone(anchor_n_ed_n, document.f_ck, document.anchor_h_ef_mm, document.anchor_cracked));
    report.push(part_4::check_anchor_edge_shear(document.anchor_v_ed_kn * 1000.0, document.anchor_d_mm, document.anchor_h_ef_mm, document.f_ck, document.anchor_c1_mm));

    report
}

pub struct En1992Family;

impl NormFamily for En1992Family {
    type Document = Document;
    type Operation = Operation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1992
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion 🔖Session

// #region 🔖Fem
use fem_core::{BeamEb2, Dof, MemberUdl, Model, Node, Support};

fn max_beam_moment_knm(result: &fem_core::StaticResult, element_id: &str) -> f64 {
    let (_, fem_core::ElementResult::Beam { stations }) = result.elements.iter().find(|(id, _)| id == element_id).expect("beam element result") else {
        panic!("expected beam element result");
    };
    stations.iter().map(|s| s.m.abs()).fold(0.0_f64, f64::max) / 1000.0
}

fn max_beam_shear_kn(result: &fem_core::StaticResult, element_id: &str) -> f64 {
    let (_, fem_core::ElementResult::Beam { stations }) = result.elements.iter().find(|(id, _)| id == element_id).expect("beam element result") else {
        panic!("expected beam element result");
    };
    stations.iter().map(|s| s.v.abs()).fold(0.0_f64, f64::max) / 1000.0
}

/// 🏗️ Solve a simply supported RC beam with `fem_core` and run EN 1992 ULS checks.
pub fn check_rc_beam_from_fem(span_m: f64, udl_kn_m: f64, f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64, rho_l: f64, annex: AnnexChoice) -> Result<CheckReport, fem_core::FemError> {
    let mut model = Model::default();
    model.nodes.push(Node { id: "n0".into(), pos: [0.0, 0.0, 0.0] });
    model.nodes.push(Node { id: "n1".into(), pos: [span_m, 0.0, 0.0] });
    model.supports.push(Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty] });
    model.supports.push(Support { node_id: "n1".into(), fixed: vec![Dof::Ty] });
    model.elements.push(Box::new(BeamEb2 { id: "b1".into(), start: "n0".into(), end: "n1".into(), e: 30e9, area: b_mm * d_mm / 1e6, iy: b_mm * d_mm.powi(3) / 12e12, density: 2500.0 }));
    model.member_loads.push(("b1".into(), MemberUdl { wx: 0.0, wy: -udl_kn_m * 1000.0, wz: 0.0 }));

    let result = fem_core::solve_linear_static(&model)?;
    let m_ed_knm = max_beam_moment_knm(&result, "b1");
    let v_ed_kn = max_beam_shear_kn(&result, "b1");

    Ok(check_rc_beam(m_ed_knm, v_ed_kn, f_ck, b_mm, d_mm, a_s_mm2, f_yk, rho_l, 0.0, annex))
}
// #endregion 🔖Fem

// #region 🔖Dsl
// `Document`'s `vcs::DocumentDsl` impl and `FireRating`/`TightnessClass`'s scalar-tag mappings are
// now generated by `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslScalar)]` on the type
// definitions themselves (see `part_1_2::FireRating`, `part_3::TightnessClass`, and `Document`
// above) — the engine's `dsl_schema` grammar replaces this crate's own hand-rolled kv printer.
// #endregion 🔖Dsl

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_beam_e2e() {
        let report = check_rc_beam(120.0, 80.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, 200.0, AnnexChoice::De);
        assert!(!report.checks.is_empty());
        assert!(report.checks[0].utilization > 0.0);
    }

    #[test]
    fn punching_v_rd_max_c30() {
        let v = part_1_1::punching_v_rd_max_mpa(30.0, AnnexChoice::De);
        assert!((v - 4.488).abs() < 0.1);
    }

    #[test]
    fn slenderness_column() {
        let i = part_1_1::radius_of_gyration_mm(300_000.0, 2.25e9);
        let lambda = part_1_1::slenderness_lambda(3000.0, i);
        assert!((lambda - 34.6).abs() < 1.0);
    }

    #[test]
    fn crack_width_wk() {
        let eps_sm = part_1_1::steel_strain_eps_sm(200.0, 0.01, 2.9, 200_000.0);
        let wk = part_1_1::crack_width_wk_mm(eps_sm, 0.0001, 300.0);
        assert!(wk > 0.0 && wk < 0.5);
    }

    #[test]
    fn deflection_ss_udl() {
        let delta = part_1_1::deflection_ss_udl_mm(20.0, 6.0, 30_000.0, 1.875e9);
        assert!((delta - 6.0).abs() < 0.5);
    }

    #[test]
    fn fire_cover_beam_r60() {
        let req = part_1_2::min_axis_distance_mm(part_1_2::ElementType::Beam, part_1_2::FireRating::R60);
        assert!((req - 35.0).abs() < 0.1);
    }

    #[test]
    fn liquid_retaining_stress_limit() {
        assert!((part_3::steel_stress_limit_mpa("XD1") - 160.0).abs() < 0.1);
        let wk = part_3::crack_width_liquid_mm(220.0, "XD1", 250.0, 200_000.0);
        assert!(wk < 0.25);
    }

    #[test]
    fn na_de_alpha_cc() {
        assert!((na_de::AnnexParams::de().alpha_cc - 0.85).abs() < 1e-9);
    }

    #[test]
    fn rc_beam_from_fem_e2e() {
        let report = check_rc_beam_from_fem(6.0, 20.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, AnnexChoice::De).expect("fem solve");
        assert!(!report.checks.is_empty());
        let m_ed = report.checks[0].computed.value / 1_000_000.0;
        assert!((m_ed - 90.0).abs() < 1.0);
    }

    #[test]
    fn prestress_transfer_c30() {
        let sigma = part_1_1::prestress_transfer_stress_mpa(800.0, 135_000.0);
        assert!((sigma - 5.93).abs() < 0.1);
        let limit = part_1_1::prestress_transfer_limit_mpa(30.0);
        assert!((limit - 18.0).abs() < 1e-9);
        let report = check_full_rc_beam(120.0, 80.0, 30.0, 300.0, 450.0, 1200.0, 500.0, 0.01, 0.0, 800.0, 135_000.0, AnnexChoice::De);
        assert_eq!(report.checks.len(), 3);
        assert!(report.checks[2].utilization < 1.0);
    }

    #[test]
    fn evaluate_fem_path() {
        let mut doc = Document::default();
        doc.use_fem = true;
        let report = evaluate(&doc);
        assert!(!report.checks.is_empty());
        let m_ed = report.checks[0].computed.value / 1_000_000.0;
        assert!((m_ed - 90.0).abs() < 1.0);
    }

    #[test]
    fn evaluate_analytical_with_prestress() {
        let mut doc = Document::default();
        doc.p_kn = 800.0;
        let report = evaluate(&doc);
        assert_eq!(report.checks.len(), 10);
        assert!(report.checks.iter().all(|c| c.status != CheckStatus::NotApplicable));
    }

    #[test]
    fn evaluate_covers_all_parts() {
        let report = evaluate(&Document::default());
        assert_eq!(report.checks.len(), 9);
        let families: Vec<&str> = report.checks.iter().map(|c| c.clause.family.as_str()).collect();
        assert!(families.contains(&"EN 1992-1-1"));
        assert!(families.contains(&"EN 1992-1-2"));
        assert!(families.contains(&"EN 1992-2"));
        assert!(families.contains(&"EN 1992-3"));
        assert!(families.contains(&"EN 1992-4"));
    }

    #[test]
    fn annex_params_alpha_cc_de_vs_en_divergence() {
        let f_ck = 30.0;
        let f_cd_de = na_de::AnnexParams::de().f_cd_mpa(f_ck);
        let f_cd_en = na_de::AnnexParams::en().f_cd_mpa(f_ck);
        assert!((f_cd_de - 17.0).abs() < 1e-9);
        assert!((f_cd_en - 20.0).abs() < 1e-9);
        assert!(f_cd_en > f_cd_de);
    }

    #[test]
    fn flexural_resistance_annex_divergence() {
        let m_rd_de = part_1_1::flexural_resistance_knm(30.0, 300.0, 450.0, 1200.0, 500.0, AnnexChoice::De);
        let m_rd_en = part_1_1::flexural_resistance_knm(30.0, 300.0, 450.0, 1200.0, 500.0, AnnexChoice::En);
        assert!(m_rd_en > m_rd_de);
    }

    #[test]
    fn fire_r60_required_axis_distance_at_160mm() {
        let a = part_1_2::required_axis_distance_beam_mm(160.0, part_1_2::FireRating::R60);
        assert!((a - 35.0).abs() < 1e-9);
        let pass = part_1_2::check_fire_beam_axis_distance(160.0, 35.0, part_1_2::FireRating::R60);
        assert!(pass.status != CheckStatus::Fail);
        let fail = part_1_2::check_fire_beam_axis_distance(160.0, 20.0, part_1_2::FireRating::R60);
        assert_eq!(fail.status, CheckStatus::Fail);
    }

    #[test]
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

    #[test]
    fn tightness_class_crack_width_limits() {
        assert!(part_3::tightness_crack_width_limit_mm(part_3::TightnessClass::Tc0, 10.0).is_none());
        assert!((part_3::tightness_crack_width_limit_mm(part_3::TightnessClass::Tc1, 10.0).unwrap() - 0.3).abs() < 1e-9);
        let tc2_mid = part_3::tightness_crack_width_limit_mm(part_3::TightnessClass::Tc2, 20.0).unwrap();
        assert!((tc2_mid - 0.125).abs() < 1e-9);
        let tc0_check = part_3::check_tightness_crack_width(0.2, part_3::TightnessClass::Tc0, 10.0);
        assert_eq!(tc0_check.status, CheckStatus::NotApplicable);
    }

    #[test]
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

    #[test]
    fn anchor_edge_shear_breakout() {
        let v_rk_c = part_4::concrete_edge_resistance_v0_rk_c_n(12.0, 80.0, 30.0, 100.0);
        assert!(v_rk_c > 0.0);
        let check = part_4::check_anchor_edge_shear(5_000.0, 12.0, 80.0, 30.0, 100.0);
        assert!(check.status != CheckStatus::Fail);
    }

    #[test]
    fn document_dsl_round_trips() {
        vcs::test_support::assert_dsl_round_trip(&Document::default());
    }

    #[test]
    fn document_dsl_parse_error_reports_the_real_line_of_the_bad_field() {
        // The engine's per-token spans are a concrete improvement over the old `dsl_kv` printer,
        // whose errors always reported `TextSpan::at(1, 1)` regardless of which line actually
        // failed. `fire-rating` (kebab-cased from `fire_rating`) is the 16th `key value` line in
        // `print_dsl`'s fixed field order.
        let printed = <Document as vcs::DocumentDsl>::print_dsl(&Document::default());
        let bad = printed.replacen("fire-rating=r60", "fire-rating=not-a-rating", 1);
        assert_ne!(bad, printed, "fire_rating's printed line must match the literal replaced above");
        let bad_line = bad.lines().position(|l| l.contains("not-a-rating")).expect("bad line present") as u32 + 1;
        let error = <Document as vcs::DocumentDsl>::parse_dsl(&bad).expect_err("an unknown fire_rating tag must fail to parse");
        assert_eq!(error.span.line, bad_line, "error span must point at the actual malformed line, not (1, 1)");
    }

    #[test]
    fn set_document_op_text_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let envelope = vcs::create_document_vcs_envelope("norm.en1992/v1", "en1992", Document::default(), None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        let mut next = Document::default();
        next.m_ed_knm = 200.0;
        store
            .dispatch(vcs::DocumentVcsCommand::Apply {
                operations: vec![Operation::SetDocument { document: next }],
                description: None,
            })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
}
