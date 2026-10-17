//! 🧰️ EN 1992 quantity and path helpers, the national annex and the clause modules of parts 1-1, 1-2, 2, 3 and 4.

use super::*;

//#region 🔖️Helpers
pub(super) fn moment_nm(nm: f64) -> Quantity {
    Quantity::new(QuantityKind::Moment, nm)
}
pub(super) fn force_n(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}
pub(super) fn stress_pa(pa: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, pa)
}
pub(super) fn length_m(m: f64) -> Quantity {
    Quantity::length_m(m)
}
pub(super) fn area_m2(a: f64) -> Quantity {
    Quantity::area_m2(a)
}
pub(super) fn member_path(member_id: &str, field: &str) -> String {
    format!("members[id={member_id}].{field}")
}
pub(super) fn layer_path(member_id: &str, layer_id: &str, field: &str) -> String {
    format!("members[id={member_id}].longitudinal[id={layer_id}].{field}")
}
fn anchor_path(anchor_id: &str, field: &str) -> String {
    format!("anchors[id={anchor_id}].{field}")
}
pub(super) fn member_ref(doc: &En1992Snapshot, member: &RcMember, path: &str) -> SubjectRef {
    let base = member.subject_label();
    let label = l(
        &format!("{} — {}", doc.title, base.en),
        &format!("{} — {}", doc.title, base.de),
    );
    SubjectRef::new(member.id.clone(), path.to_string(), label)
}
pub(super) fn l(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}
pub(super) fn clause_11(section: &str) -> ClauseId {
    ClauseId::new("EN 1992-1-1", section, section.trim_start_matches('§'))
}
pub(super) fn part_11() -> &'static str {
    "DIN EN 1992-1-1"
}
pub(super) fn part_12() -> &'static str {
    "DIN EN 1992-1-2"
}
pub(super) fn part_2() -> &'static str {
    "DIN EN 1992-2"
}
pub(super) fn part_3() -> &'static str {
    "DIN EN 1992-3"
}
pub(super) fn part_4() -> &'static str {
    "DIN EN 1992-4"
}
//#endregion 🔖️Helpers

/// 🇩🇪 National annex parameters for EN 1992.
pub mod na_de {
    use semio_s_plugin_norm_compliance::AnnexChoice;

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
    use crate::*;

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
    pub fn structural_class_delta(design_life_years: f64, cement_type: &str, f_ck_pa: f64, member_kind: MemberKind) -> i32 {
        let mut sc = 4;
        if design_life_years >= 100.0 { sc += 2; }
        else if design_life_years <= 25.0 { sc -= 1; }
        let ct = cement_type.to_uppercase();
        if ct.contains('R') { sc -= 1; }
        if f_ck_pa / 1.0e6 >= 35.0 && matches!(member_kind, MemberKind::Slab | MemberKind::FlatSlab | MemberKind::Beam) {
            sc -= 1;
        }
        (sc - 4).clamp(-1, 2)
    }

    /// 📏 c_min,dur with structural-class shift (±5 mm per class).
    pub fn c_min_dur_adjusted_m(exposure: ExposureClass, design_life_years: f64, cement_type: &str, f_ck_pa: f64, member_kind: MemberKind) -> f64 {
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
    use crate::*;

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
    pub fn required_for(kind: MemberKind, support: SupportCondition, rating: FireRating, column_method: &str, slab_system: &str) -> (f64, f64) {
        match kind {
            MemberKind::Column => {
                if column_method.eq_ignore_ascii_case("B") { column_method_b(rating) } else { column_method_a(rating) }
            }
            MemberKind::Wall => wall(rating),
            MemberKind::TensionMember => tension_member(rating),
            MemberKind::FlatSlab => {
                if slab_system.eq_ignore_ascii_case("two-way") { slab_two_way(rating) }
                else if slab_system.eq_ignore_ascii_case("ribbed") { ribbed_slab(rating) }
                else if slab_system.eq_ignore_ascii_case("one-way") { slab_one_way(rating) }
                else { flat_slab(rating) }
            }
            MemberKind::RibbedSlab => ribbed_slab(rating),
            MemberKind::Slab => {
                if slab_system.eq_ignore_ascii_case("two-way") { slab_two_way(rating) }
                else if slab_system.eq_ignore_ascii_case("ribbed") { ribbed_slab(rating) }
                else if slab_system.eq_ignore_ascii_case("flat") { flat_slab(rating) }
                else { slab_one_way(rating) }
            }
            MemberKind::Beam => match support {
                SupportCondition::Continuous | SupportCondition::Fixed => beam_cont(rating),
                _ => beam_ss(rating),
            },
            MemberKind::Bridge => beam_ss(rating),
            MemberKind::LiquidRetaining => wall(rating),
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
    #[cfg(test)] use crate::*;

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
    use crate::*;

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
    #[cfg(test)] use crate::*;

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
