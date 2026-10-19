//! 📏️ EN 1993 clause parts: the clause tables and resistance formulas of EN 1993-1-1 to 1-12 and parts 2 to 6.


//#region 🔖️ComplianceHelpers
pub mod part_1_1 {
    use crate::*;

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
        let _a_web = section.h.max(section.tw) * section.tw;
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
        let w = if class <= 2 { w_pl } else { w_pl };
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
        let eta61 = (n_ed / n_b_rd_y.max(1e-9)).abs() + kyy * (my_ed / m_rd_y.max(1e-9)).abs() + kyz * (mz_ed / m_rd_z.max(1e-9)).abs();
        let eta62 = (n_ed / n_b_rd_z.max(1e-9)).abs() + kzy * (my_ed / m_rd_y.max(1e-9)).abs() + kzz * (mz_ed / m_rd_z.max(1e-9)).abs();
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
    #[cfg(test)] use crate::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    use crate::AnnexParams;
    /// ✨ Stainless γ_M0 = 1.1 (EN 1993-1-4).
    pub fn bending_resistance_nm(w_pl: f64, fy_pa: f64) -> f64 {
        let params = AnnexParams { choice: semio_s_plugin_norm_compliance::AnnexChoice::En, gamma_m0: 1.1, gamma_m1: 1.1, gamma_m2: 1.25, gamma_m3: 1.25, gamma_mf: 1.15 };
        w_pl * fy_pa / params.gamma_m0
    }
}

pub mod part_1_5 {
    use crate::AnnexParams;
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
    use crate::AnnexParams;
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
    use crate::AnnexParams;
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
    use crate::AnnexParams;
    /// 🗼 EN 1993-3-1 Annex B — design buckling resistance reduced by force coefficient c_f and dynamic factor c_d.
    pub fn tower_buckling_n(area: f64, fy_pa: f64, chi: f64, force_coefficient: f64, dynamic_factor: f64, params: AnnexParams) -> f64 {
        let wind_amp = (force_coefficient.max(0.1) * dynamic_factor.max(0.1)).max(1.0);
        chi * area * fy_pa / params.gamma_m1 / wind_amp
    }
}

pub mod part_4 {
    /// 🛢️ Janssen horizontal pressure [Pa].
    pub fn janssen_pressure(k: f64, gamma: f64, depth: f64) -> f64 {
        k * gamma * depth
    }

    pub fn membrane_hoop_stress(p_h: f64, r: f64, t: f64) -> f64 {
        p_h * r / t
    }
}

pub mod part_5 {
    use crate::AnnexParams;
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
//#endregion 🔖️ComplianceHelpers
