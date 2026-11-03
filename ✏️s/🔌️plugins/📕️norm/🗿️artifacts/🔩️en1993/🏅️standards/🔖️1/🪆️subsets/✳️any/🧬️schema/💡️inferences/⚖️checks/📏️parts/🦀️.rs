//! 📏️ EN 1993 clause parts: the clause tables and resistance formulas of EN 1993-1-1 to 1-12 and parts 2 to 6.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_clauses as clauses;
use semio_s_artifact_norm_en1993_model::{SteelMaterial, SteelSection};
use semio_s_plugin_norm_compliance::AnnexChoice;

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

impl AnnexParams {
    /// 🔖️ Registry ids (`ndp.en1993.*`) of the partial factors; a check cites exactly the ones its resistances use.
    pub const GAMMA_M0: &'static str = "ndp.en1993.gamma-m0";
    pub const GAMMA_M1: &'static str = "ndp.en1993.gamma-m1";
    pub const GAMMA_M2: &'static str = "ndp.en1993.gamma-m2";
    pub const GAMMA_M3: &'static str = "ndp.en1993.gamma-m3";
    pub const GAMMA_MF: &'static str = "ndp.en1993.gamma-mf";

    /// 🇪🇺 EN recommended values of the shared registry.
    pub fn en() -> Self {
        Self::for_choice(AnnexChoice::En)
    }

    /// 🇩🇪 DIN EN 1993-1-1/NA values of the shared registry (γ_M1 = 1.1).
    pub fn de() -> Self {
        Self::for_choice(AnnexChoice::De)
    }

    pub fn for_choice(choice: AnnexChoice) -> Self {
        let n = |id: &str| semio_s_plugin_norm_compliance::ndp_number(id, choice);
        semio_s_plugin_norm_compliance::record_ndp_reads(|| Self { choice, gamma_m0: n(Self::GAMMA_M0), gamma_m1: n(Self::GAMMA_M1), gamma_m2: n(Self::GAMMA_M2), gamma_m3: n(Self::GAMMA_M3), gamma_mf: n(Self::GAMMA_MF) }).0
    }
}


//#region 🔖️ComplianceHelpers
pub mod part_1_1 {
    use crate::*;
    use semio_s_artifact_norm_en1993_model::SectionKind as SecKind;

    /// 🧭 Whether the section kind is a rolled or welded I or H section: the kinds with a warping constant, the I formulas of Table 5.2 and Table 6.4 and the 6.2.9.1 I-section interaction; hollow sections and angles take their own formulas.
    pub fn is_i_section(kind: SecKind) -> bool {
        match kind {
            SecKind::RolledI | SecKind::WeldedI => true,
            SecKind::Hss | SecKind::Chs | SecKind::Angle => false,
        }
    }

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

    /// 🏷️ Governing section class by the typed kind (Table 5.2): I sections as the envelope of pure bending and pure compression, `Hss` as rectangular hollow section, `Chs` by d/t (50 ε², 70 ε², 90 ε², sheet 3), `Angle` by the sheet-3 compression rule (h/t ≤ 15 ε and (b+h)/2t ≤ 11.5 ε is class 3, otherwise 4; bending is classified like compression, which is conservative).
    pub fn section_class_rolled_i(section: &SteelSection, fy_pa: f64) -> u8 {
        let fy = fy_pa / 1.0e6;
        let t = section.tf.max(section.tw) * 1e3;
        match section.kind {
            SecKind::Hss => {
                let (h, b) = (section.h * 1e3, section.b * 1e3);
                crate::clauses::classification::classify_rhs(h, b, t, fy, -1.0, 0.5).max(crate::clauses::classification::classify_rhs(h, b, t, fy, 1.0, 1.0))
            }
            SecKind::Chs => crate::clauses::classification::chs_class(section.h * 1e3 / t, epsilon(fy_pa)),
            SecKind::Angle => crate::clauses::classification::angle_class(section.h.max(section.b) * 1e3, section.h.min(section.b) * 1e3, t, epsilon(fy_pa)),
            SecKind::RolledI | SecKind::WeldedI => {
                let g = crate::clauses::classification::Geometry { h: section.h * 1e3, b: section.b * 1e3, tw: section.tw * 1e3, tf: section.tf * 1e3, r: section.r * 1e3, weld_throat: 0.0 };
                let bending = crate::clauses::classification::classify_i_section(&g, fy, 0.0, -1.0, true).section;
                let compression = crate::clauses::classification::classify_i_section(&g, fy, 1.0e12, 1.0, true).section;
                bending.max(compression)
            }
        }
    }

    /// 🏷️ Section class under the actual N_Ed (compression positive, N) and M_y,Ed (N·m) of one action row; hollow sections and angles are classified by their dimensions alone (their Table 5.2 limits do not depend on the stress distribution).
    pub fn section_class_for_action(section: &SteelSection, fy_pa: f64, n_ed: f64, my_ed: f64) -> u8 {
        match section.kind {
            SecKind::Hss | SecKind::Chs | SecKind::Angle => section_class_rolled_i(section, fy_pa),
            SecKind::RolledI | SecKind::WeldedI => {
                let g = crate::clauses::classification::Geometry { h: section.h * 1e3, b: section.b * 1e3, tw: section.tw * 1e3, tf: section.tf * 1e3, r: section.r * 1e3, weld_throat: 0.0 };
                let c_w = g.h - 2.0 * g.tf - 2.0 * g.r;
                let sigma_n = n_ed / section.area;
                let sigma_m = my_ed.abs() / section.w_el_y.max(1e-12) * (c_w / g.h);
                let (s1, s2) = (sigma_n + sigma_m, sigma_n - sigma_m);
                let psi = if s1 <= 0.0 { 1.0 } else { (s2 / s1).clamp(-3.0, 1.0) };
                crate::clauses::classification::classify_i_section(&g, fy_pa / 1.0e6, n_ed, psi, true).section
            }
        }
    }

    /// 📐️ EN 1993-1-5 §4.4 plate buckling reduction ρ for an internal element including the ψ term.
    pub fn plate_rho(lambda_p: f64, psi: f64) -> f64 {
        crate::clauses::effective_width::rho_internal(lambda_p, psi).max(0.0)
    }

    /// 📐️ Effective area in uniform compression by the typed kind: I sections (outstand flanges, internal web, EN 1993-1-5 §4.4), `Hss` (four internal walls of c = width − 3t), `Angle` (the two leg outstands, k_σ = 0.43) and `Chs` (axial shell buckling reduction χ_x of EN 1993-1-6 on the gross area, the route Table 5.2 sheet 3 points to above d/t = 90 ε²).
    pub fn effective_area_class4(section: &SteelSection, fy_pa: f64) -> f64 {
        use crate::clauses::effective_width::*;
        let eps = epsilon(fy_pa);
        let t = section.tf.max(section.tw);
        match section.kind {
            SecKind::RolledI | SecKind::WeldedI => {
                let c_f = ((section.b - section.tw - 2.0 * section.r) / 2.0).max(1e-6);
                let c_w = (section.h - 2.0 * section.tf - 2.0 * section.r).max(1e-6);
                let rho_f = rho_outstand(lambda_p(c_f, section.tf, eps, 0.43));
                let rho_w = rho_internal(lambda_p(c_w, section.tw, eps, 4.0), 1.0).max(0.0);
                (section.area - 4.0 * c_f * section.tf * (1.0 - rho_f) - c_w * section.tw * (1.0 - rho_w)).max(1e-12)
            }
            SecKind::Hss => {
                let rho = |width: f64| rho_internal(lambda_p((width - 3.0 * t).max(1e-6), t, eps, 4.0), 1.0).clamp(0.0, 1.0);
                (section.area - 2.0 * (section.b - 3.0 * t).max(0.0) * t * (1.0 - rho(section.b)) - 2.0 * (section.h - 3.0 * t).max(0.0) * t * (1.0 - rho(section.h))).max(1e-12)
            }
            SecKind::Angle => {
                let (h, b) = (section.h.max(section.b), section.h.min(section.b));
                let rho = |leg: f64| rho_outstand(lambda_p((leg - t).max(1e-6), t, eps, 0.43)).clamp(0.0, 1.0);
                (section.area - (h - t).max(0.0) * t * (1.0 - rho(h)) - (b - t).max(0.0) * t * (1.0 - rho(b))).max(1e-12)
            }
            SecKind::Chs => {
                let r = ((section.h - t) / 2.0).max(1e-9);
                let lambda = super::part_1_6::lambda_bar(fy_pa, super::part_1_6::sigma_x_rcr(t, r, crate::clauses::stability::e_steel()));
                (section.area * super::part_1_6::chi(lambda, super::part_1_6::alpha_imperfection(r, t)).clamp(0.0, 1.0)).max(1e-12)
            }
        }
    }

    /// 📐️ Effective W_el,y in bending about y (u-u for an angle) by the typed kind: I sections with the effective flanges and web (ψ = −1); `Hss`, `Chs` and `Angle` take W_el,y reduced in proportion to their effective area, an approximation that is not in EN 1993-1-5.
    pub fn effective_w_el_y_class4(section: &SteelSection, fy_pa: f64) -> f64 {
        match section.kind {
            SecKind::RolledI | SecKind::WeldedI => {
                let e = crate::clauses::effective_width::i_section_bending(section.h * 1e3, section.b * 1e3, section.tw * 1e3, section.tf * 1e3, fy_pa / 1.0e6);
                (e.w_eff_y * 1e-9).min(section.w_el_y)
            }
            SecKind::Hss | SecKind::Chs | SecKind::Angle => section.w_el_y * (effective_area_class4(section, fy_pa) / section.area.max(1e-12)).min(1.0),
        }
    }

    /// 📐️ W of the bending resistance about the weak axis (z-z, v-v for an angle) by section class: W_pl,z (class 1, 2), W_el,z (class 3), W_el,z reduced in proportion to the effective area (class 4, approximation).
    pub fn bending_modulus_z(section: &SteelSection, class: u8, fy_pa: f64) -> f64 {
        match class {
            1 | 2 => section.w_pl_z,
            3 => section.w_el_z,
            _ => section.w_el_z * (effective_area_class4(section, fy_pa) / section.area.max(1e-12)).min(1.0),
        }
    }

    /// 📐️ W_y of the bending and LTB resistance by section class: W_pl,y (class 1, 2), W_el,y (class 3), W_eff,y (class 4).
    pub fn bending_modulus_y(section: &SteelSection, class: u8, fy_pa: f64) -> f64 {
        match class {
            1 | 2 => section.w_pl_y,
            3 => section.w_el_y,
            _ => effective_w_el_y_class4(section, fy_pa),
        }
    }

    /// 📐️ e_N,y of the effective section under uniform compression (m, towards the top flange positive; along v-v for an angle): zero for the doubly symmetric `Hss` and `Chs`.
    pub fn e_n_y_class4(section: &SteelSection, fy_pa: f64) -> f64 {
        match section.kind {
            SecKind::RolledI | SecKind::WeldedI => crate::clauses::effective_width::i_section_compression_shift(section.h * 1e3, section.b * 1e3, section.b * 1e3, section.tw * 1e3, section.tf * 1e3, section.r * 1e3, fy_pa / 1.0e6) * 1e-3,
            SecKind::Hss | SecKind::Chs => 0.0,
            SecKind::Angle => {
                use crate::clauses::effective_width::*;
                let (h, b, t, eps) = (section.h.max(section.b) * 1e3, section.h.min(section.b) * 1e3, section.tf.max(section.tw) * 1e3, epsilon(fy_pa));
                let rho = |leg: f64| rho_outstand(lambda_p((leg - t).max(1e-6), t, eps, 0.43)).clamp(0.0, 1.0);
                crate::clauses::parametric_sections::angle_tip_shift(h, b, t, rho(h), rho(b)) * 1e-3
            }
        }
    }

    /// 📐️ N_Rd = A·fy/γ_M0 [N].
    pub fn axial_resistance_n(area: f64, fy_pa: f64, params: AnnexParams) -> f64 {
        area * fy_pa / params.gamma_m0
    }

    /// 📐️ M_c,Rd = W_pl·fy/γ_M0 for class ≤ 2 [N·m].
    pub fn bending_resistance_nm(w_pl: f64, fy_pa: f64, _class: u8, params: AnnexParams) -> f64 {
        w_pl * fy_pa / params.gamma_m0
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

    /// 🏭 True for a hollow section (`Hss`, `Chs`) whose fabrication route is not chosen: Table 6.2 has no curve for it, so the member checks report the open choice instead of buckling rows ([`buckling_curve_rolled_i`] then only returns the cold-formed curve as a numeric placeholder for diagrams).
    pub fn hollow_route_open(section: &SteelSection) -> bool {
        match section.kind {
            SecKind::Hss | SecKind::Chs => section.fabrication == semio_s_artifact_norm_en1993_model::HollowFabrication::Unset,
            SecKind::RolledI | SecKind::WeldedI | SecKind::Angle => false,
        }
    }

    /// 🔄 True when EN 1993-1-1 6.3.2 gives no lateral-torsional buckling rule for the section kind: 6.3.2 and Table 6.4 are written for rolled and welded I or H sections, an angle has no such curve and `m_cr_rolled_nm` has no closed form for it.
    pub fn ltb_not_applicable(section: &SteelSection) -> bool {
        match section.kind {
            SecKind::Angle => true,
            SecKind::RolledI | SecKind::WeldedI | SecKind::Hss | SecKind::Chs => false,
        }
    }

    /// 📉 Buckling curve of Table 6.2 selected by the section kind and the typed fabrication route: rolled and welded I, `Hss` and `Chs` hot finished (curve a, a0 for S460) or cold formed (curve c), `Angle` curve b about any axis (the L row of Table 6.2; v-v is the z-z axis of the angle row).
    pub fn buckling_curve_rolled_i(section: &SteelSection, axis: char, fy_pa: f64) -> BucklingCurve {
        use crate::clauses::stability::{curve_table_6_2, Curve, BucklingSectionKind};
        let kind = match section.kind {
            semio_s_artifact_norm_en1993_model::SectionKind::WeldedI => BucklingSectionKind::WeldedI,
            semio_s_artifact_norm_en1993_model::SectionKind::Hss | semio_s_artifact_norm_en1993_model::SectionKind::Chs => match section.fabrication {
                semio_s_artifact_norm_en1993_model::HollowFabrication::HotFinished => BucklingSectionKind::HotHollow,
                semio_s_artifact_norm_en1993_model::HollowFabrication::ColdFormed | semio_s_artifact_norm_en1993_model::HollowFabrication::Unset => BucklingSectionKind::ColdHollow,
            },
            semio_s_artifact_norm_en1993_model::SectionKind::Angle => BucklingSectionKind::Angle,
            semio_s_artifact_norm_en1993_model::SectionKind::RolledI => BucklingSectionKind::RolledI,
        };
        match curve_table_6_2(kind, axis == 'y', section.h * 1e3, section.b * 1e3, section.tf * 1e3, fy_pa / 1.0e6) {
            Curve::A0 => BucklingCurve::A0,
            Curve::A => BucklingCurve::A,
            Curve::B => BucklingCurve::B,
            Curve::C => BucklingCurve::C,
            Curve::D => BucklingCurve::D,
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

    /// 🔄 M_cr of a doubly symmetric I: ECCS closed form with C1/C2 from the moment diagram and the load height z_g.
    pub fn m_cr_rolled_nm(section: &SteelSection, material: &SteelMaterial, l_lt: f64, psi: f64, load_app: &str, moment_diagram: &str) -> f64 {
        if section.kind == SecKind::Angle {
            return f64::INFINITY;
        }
        use crate::clauses::stability::{c1_end_moments, mcr_closed_form};
        let (c1, c2) = match moment_diagram {
            "uniform" => (1.0, 0.0),
            "parabolic" => (1.127, 0.454),
            _ => (c1_end_moments(psi), 0.0),
        };
        let zg = match load_app {
            "topFlange" => section.h / 2.0,
            "bottomFlange" => -section.h / 2.0,
            _ => 0.0,
        };
        mcr_closed_form(material.e_modulus, material.g_modulus, section.iz, section.it, section.iw, l_lt, 1.0, 1.0, c1, c2, zg)
    }

    /// 📉 LTB curve Table 6.4 (rolled/welded I) — DE-NA uses same selection; §6.3.2.3 special case optional.
    pub fn ltb_curve_table_6_4(section: &SteelSection) -> BucklingCurve {
        let hb = section.h / section.b.max(1e-9);
        let welded = match section.kind {
            SecKind::WeldedI => true,
            SecKind::RolledI | SecKind::Hss | SecKind::Chs | SecKind::Angle => false,
        };
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

    /// 📉 M_y,V,Rd of §6.2.8(3) for class 1–2 sections with the absolute shear |V_Ed|: I sections by Eq. 6.30, hollow sections and angles by the reduced yield strength (1 − ρ) f_y over the whole section (conservative); class 3–4 use W_el,y unreduced.
    pub fn shear_reduced_m_y_rd(section: &SteelSection, class: u8, fy_pa: f64, v_ed: f64, v_rd: f64, params: AnnexParams) -> f64 {
        let m_c = section.w_pl_y * fy_pa / params.gamma_m0;
        if v_rd <= 0.0 || class > 2 {
            return if class > 2 { section.w_el_y * fy_pa / params.gamma_m0 } else { m_c };
        }
        let rho = crate::clauses::resistance::shear_rho(v_ed.abs(), v_rd);
        match section.kind {
            SecKind::RolledI | SecKind::WeldedI => {
                let aw = (section.h - 2.0 * section.tf) * section.tw;
                crate::clauses::resistance::m_y_v_rd_i(section.w_pl_y, rho, aw, section.tw, fy_pa, params.gamma_m0).clamp(0.0, m_c)
            }
            SecKind::Hss | SecKind::Chs | SecKind::Angle => m_c * (1.0 - rho),
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

    /// 🔗 §6.2.9 of a section by its typed kind. I and H sections: [`mn_interaction_eta`]. `Hss`: 6.2.9.1(5) (Eq. 6.39, 6.40: M_N,Rd = M_pl,Rd (1 − n)/(1 − 0.5 a), a = a_w about y and a_f about z, at most M_pl,Rd) with the exponents α = β = 1.66/(1 − 1.13 n²) ≤ 6 of 6.2.9.1(6). `Chs`: M_N,Rd = M_pl,Rd (1 − n^1.7) (ECCS form of the repo clause table, unverified against the print) with α = β = 2 (6.2.9.1(6)). `Angle`: class 3 or 4 by Table 5.2 sheet 3, so the elastic criterion of 6.2.9.2 on the principal axes u-u (y row) and v-v (z row) applies as the linear sum n + M_u/M_u,Rd + M_v/M_v,Rd of the class resistances; EN 1993-3-1 Annex G and H (effective slenderness and eccentricity of angle legs) are not part of the repo spec and are not applied. Classes 3 and 4 of every kind add linearly (conservative form of 6.2.9.2). See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).
    pub fn section_mn_eta(section: &SteelSection, n_ed: f64, my_ed: f64, mz_ed: f64, n_pl_rd: f64, m_pl_y_rd: f64, m_pl_z_rd: f64, class: u8) -> f64 {
        match section.kind {
            SecKind::RolledI | SecKind::WeldedI => return mn_interaction_eta(n_ed, my_ed, mz_ed, n_pl_rd, m_pl_y_rd, m_pl_z_rd, section.area, section.b, section.tf, class),
            SecKind::Hss | SecKind::Chs | SecKind::Angle => {}
        }
        if n_pl_rd <= 0.0 || m_pl_y_rd <= 0.0 || m_pl_z_rd <= 0.0 {
            return 0.0;
        }
        let n = (n_ed / n_pl_rd).abs();
        if class >= 3 || section.kind == SecKind::Angle {
            return n + (my_ed / m_pl_y_rd).abs() + (mz_ed / m_pl_z_rd).abs();
        }
        let (m_n_y, m_n_z, alpha, beta) = if section.kind == SecKind::Chs {
            let (alpha, beta) = crate::clauses::resistance::biaxial_exponents(1, n);
            (crate::clauses::resistance::m_n_rd_chs(m_pl_y_rd, n).max(0.0), crate::clauses::resistance::m_n_rd_chs(m_pl_z_rd, n).max(0.0), alpha, beta)
        } else {
            let t = section.tf.max(section.tw);
            let a_w = crate::clauses::resistance::a_w_hollow(section.area, section.b, t).max(0.0);
            let a_f = crate::clauses::resistance::a_w_hollow(section.area, section.h, t).max(0.0);
            let (alpha, beta) = crate::clauses::resistance::biaxial_exponents(2, n);
            (crate::clauses::resistance::m_n_rd_rhs(m_pl_y_rd, n, a_w).max(0.0), crate::clauses::resistance::m_n_rd_rhs(m_pl_z_rd, n, a_f).max(0.0), alpha, beta)
        };
        if n >= 1.0 || m_n_y <= 1e-12 || m_n_z <= 1e-12 {
            return n + (my_ed / m_pl_y_rd).abs() + (mz_ed / m_pl_z_rd).abs();
        }
        crate::clauses::resistance::biaxial_utilisation(my_ed, mz_ed, m_n_y, m_n_z, alpha, beta)
    }

    /// 🏗️ Reduction factor χ of the flexural buckling of a tower leg by the typed kind: the y-y axis (length `l_y`) for I, hollow and circular sections as before, both principal axes (u-u with `l_y`, v-v with `l_z`) for an angle, the smaller χ governing (Table 6.2 curve b, Eq. 6.49).
    pub fn tower_flexural_chi(section: &SteelSection, l_y: f64, l_z: f64, e_pa: f64, fy_pa: f64) -> f64 {
        let about = |axis: char, l: f64, i_moment: f64| chi(lambda_bar(l, radius_of_gyration(i_moment, section.area), e_pa, fy_pa), buckling_curve_rolled_i(section, axis, fy_pa));
        match section.kind {
            SecKind::RolledI | SecKind::WeldedI | SecKind::Hss | SecKind::Chs => about('y', l_y, section.iy),
            SecKind::Angle => about('y', l_y, section.iy).min(about('z', l_z, section.iz)),
        }
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
        if mu0 >= 1.0 {
            return 20.0;
        }
        let mu = mu0.clamp(0.013, 1.0);
        39.19 * (1.0 / 0.9674 / mu.powf(3.833) - 1.0).ln() + 482.0
    }

    /// 🔥 k_y,θ Table 3.1 (carbon steel), linearly interpolated.
    pub fn k_y_theta(theta: f64) -> f64 {
        crate::clauses::fatigue_fire::k_y_theta(theta)
    }

    /// 🔥 k_E,θ Table 3.1, linearly interpolated.
    pub fn k_e_theta(theta: f64) -> f64 {
        crate::clauses::fatigue_fire::k_e_theta(theta)
    }

    /// 🔥 χ_fi §4.2.3.2 for the room-temperature slenderness λ̄ at steel temperature θ [°C].
    pub fn chi_fi(lambda_bar: f64, theta: f64, fy_pa: f64) -> f64 {
        crate::clauses::fatigue_fire::chi_fi(lambda_bar, theta, fy_pa / 1.0e6)
    }

    /// 🔥 §4.2.5 incremental steel temperature [°C] with ISO 834 and temperature-dependent c_a, unprotected (d_p≤0) or protected.
    pub fn steel_temperature_c(
        rating: FireRating,
        section_factor: f64,
        d_p: f64,
        lambda_p: f64,
        rho_p: f64,
        c_p: f64,
    ) -> f64 {
        use crate::clauses::fatigue_fire::*;
        let t_end = rating_minutes(rating) * 60.0;
        let iso = |t_min: f64| gas_iso834(t_min);
        let history = if d_p <= 1e-6 {
            unprotected_steel_temperature(&iso, 1.0, section_factor.max(1.0), t_end, 5.0, 0.7, 1.0)
        } else {
            protected_steel_temperature(&iso, section_factor.max(1.0), d_p, lambda_p, rho_p, c_p, t_end, 5.0)
        };
        history.last().map(|p| p.1).unwrap_or(20.0)
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
        crate::clauses::effective_width::rho_internal(lambda_p, psi).clamp(0.0, 1.0)
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
        crate::clauses::effective_width::rho_internal(lambda_p, 1.0).clamp(0.0, 1.0)
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
        crate::clauses::plates_shells::cylinder_alpha_x(r, t, 25.0)
    }

    pub fn chi(lambda_bar: f64, alpha: f64) -> f64 {
        crate::clauses::plates_shells::cylinder_chi_x(lambda_bar, alpha)
    }

    pub fn design_resistance(fy_pa: f64, chi: f64, params: AnnexParams) -> f64 {
        chi * fy_pa / params.gamma_m1
    }
}

pub mod part_1_8 {
    /// 🔩 Bolt tensile stress area from diameter [m] → m² (approx ISO coarse).
    pub fn bolt_as(d: f64) -> f64 {
        crate::clauses::joints::bolt_as(d * 1000.0) * 1e-6
    }

    pub fn bolt_fub(class: &str) -> f64 {
        crate::clauses::joints::bolt_fub(class) * 1e6
    }

    pub fn bolt_shear_resistance_n(n_bolts: u32, a_s: f64, f_ub: f64, n_planes: u32, alpha_v: f64, gamma_m2: f64) -> f64 {
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
        crate::clauses::joints::beta_w(grade)
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

    /// 🔧 Directional method (§4.5.3.2) for the weld force along the axis `f_parallel` and the force `f_transverse` perpendicular to it along a leg of the equal-leg fillet (throat-plane angle 45°); returns (design effect, resistance) as forces, the effect being the governing utilisation of both criteria times the resistance.
    pub fn fillet_weld_directional_n(a: f64, length: f64, f_u: f64, beta_w: f64, gamma_m2: f64, f_parallel: f64, f_transverse: f64) -> (f64, f64) {
        let area = a * length;
        if area <= 0.0 {
            return (f_parallel.hypot(f_transverse), 0.0);
        }
        let limit = f_u / (beta_w * gamma_m2) * area;
        let (sigma, tau_perp, tau_par) = crate::clauses::joints::weld_throat_stresses(f_parallel, f_transverse, std::f64::consts::FRAC_PI_4, a, length);
        (crate::clauses::joints::weld_directional(sigma, tau_perp, tau_par, f_u, beta_w, gamma_m2) * limit, limit)
    }
}

pub mod part_1_9 {
    pub fn detail_category_pa(category: u8) -> f64 {
        category as f64 * 1e6
    }

    pub enum AssessmentMethod {
        DamageTolerant,
        SafeLife,
        LowConsequence,
    }

    /// 🔄 Palmgren-Miner damage with tri-linear Δσ_C/Δσ_D/Δσ_L (m=3 then m=5).
    pub fn miner_damage(spectrum: &[(f64, f64)], delta_c: f64, gamma_mf: f64) -> f64 {
        let delta_c = delta_c / gamma_mf.max(1e-9);
        spectrum.iter().filter(|(_, n)| *n > 0.0).filter_map(|&(ds, n)| crate::clauses::fatigue_fire::endurance(ds, delta_c).map(|n_r| n / n_r.max(1e-12))).sum()
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

    /// 🪵 Driving stress limit as a fraction of f_y, read from the registry (`ndp.en1993.5.pile-driving-limit-ratio`).
    pub fn pile_driving_limit_ratio() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.5.pile-driving-limit-ratio")
    }

    /// 🪵 Driving stress ratio up to which the full pile capacity is kept, read from the registry (`ndp.en1993.5.pile-full-capacity-driving-ratio`).
    pub fn pile_full_capacity_driving_ratio() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.5.pile-full-capacity-driving-ratio")
    }

    pub fn pile_driving_stress_limit(fy_pa: f64) -> f64 {
        pile_driving_limit_ratio() * fy_pa
    }

    /// 🪵 EN 1993-5 §5.3 — driving-damage reduction from driving stress level; shaft geometry scales geotechnical factor.
    pub fn pile_soil_reduction(driving_stress: f64, fy_pa: f64, embedded_length: f64, shaft_perimeter: f64) -> f64 {
        let ratio = driving_stress / fy_pa.max(1.0);
        let (full, limit) = (pile_full_capacity_driving_ratio(), pile_driving_limit_ratio());
        let drive = if ratio <= full {
            1.0
        } else if ratio <= limit {
            1.0 - 0.25 * (ratio - full) / (limit - full)
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

#[cfg(test)]
mod kind_tests {
    use super::part_1_1::*;
    use semio_s_artifact_norm_en1993_model::{HollowFabrication, SectionKind, SteelSection};

    const ANGLE_L100X6_A_EFF: f64 = 0.000_928_587_971_890_112;

    fn section(kind: SectionKind, h: f64, b: f64, t: f64, area: f64, iy: f64, iz: f64) -> SteelSection {
        SteelSection { id: "s".into(), designation: "s".into(), kind, h, b, tw: t, tf: t, r: 0.0, area, shear_area_y: 0.0, shear_area_z: 0.0, iy, iz, it: 0.0, iw: 0.0, w_el_y: 0.0, w_el_z: 0.0, w_pl_y: 0.0, w_pl_z: 0.0, area_net: area, fabrication: HollowFabrication::Unset }
    }

    #[test]
    fn circular_hollow_sections_are_classified_by_d_over_t_with_the_sheet_3_limits() {
        for (t, class) in [(0.0063, 1), (0.0029, 3), (0.002, 4)] {
            assert_eq!(section_class_rolled_i(&section(SectionKind::Chs, 0.1683, 0.1683, t, 1.0e-3, 1.0e-6, 1.0e-6), 355.0e6), class, "t = {t}");
        }
    }

    #[test]
    fn angles_are_class_3_or_4_by_the_sheet_3_rule() {
        let l100x10 = section(SectionKind::Angle, 0.1, 0.1, 0.01, 0.0019, 2.865_833_333e-6, 7.342_543_86e-7);
        assert_eq!(section_class_rolled_i(&l100x10, 235.0e6), 3);
        assert_eq!(section_class_rolled_i(&l100x10, 355.0e6), 4);
        assert_eq!(
            section_class_rolled_i(&section(SectionKind::Angle, 0.065, 0.1, 0.008, 0.001256, 1.0e-6, 1.0e-7), 355.0e6),
            section_class_rolled_i(&section(SectionKind::Angle, 0.1, 0.065, 0.008, 0.001256, 1.0e-6, 1.0e-7), 355.0e6)
        );
    }

    #[test]
    fn the_slender_angle_loses_area_at_the_leg_outstands() {
        let l100x6 = section(SectionKind::Angle, 0.1, 0.1, 0.006, 0.001164, 1.0e-6, 3.0e-7);
        assert_eq!(section_class_rolled_i(&l100x6, 355.0e6), 4);
        let a_eff = effective_area_class4(&l100x6, 355.0e6);
        assert!((a_eff - ANGLE_L100X6_A_EFF).abs() / ANGLE_L100X6_A_EFF < 1e-6, "A_eff {a_eff}");
    }

    #[test]
    fn the_buckling_curve_is_c_for_cold_formed_circular_hollow_sections_and_b_for_angles() {
        let mut chs = section(SectionKind::Chs, 0.1683, 0.1683, 0.0063, 3.2e-3, 1.0e-5, 1.0e-5);
        chs.fabrication = HollowFabrication::ColdFormed;
        let angle = section(SectionKind::Angle, 0.1, 0.1, 0.01, 0.0019, 2.9e-6, 7.3e-7);
        for axis in ['y', 'z'] {
            assert_eq!(buckling_curve_rolled_i(&chs, axis, 355.0e6), BucklingCurve::C);
            assert_eq!(buckling_curve_rolled_i(&angle, axis, 235.0e6), BucklingCurve::B);
        }
    }

    #[test]
    fn the_hollow_fabrication_route_selects_the_table_6_2_curve() {
        for kind in [SectionKind::Hss, SectionKind::Chs] {
            let mut tube = section(kind, 0.1683, 0.1683, 0.0063, 3.2e-3, 1.0e-5, 1.0e-5);
            assert!(hollow_route_open(&tube));
            tube.fabrication = HollowFabrication::HotFinished;
            assert!(!hollow_route_open(&tube));
            assert_eq!(buckling_curve_rolled_i(&tube, 'y', 355.0e6), BucklingCurve::A);
            assert_eq!(buckling_curve_rolled_i(&tube, 'z', 460.0e6), BucklingCurve::A0);
            tube.fabrication = HollowFabrication::ColdFormed;
            assert_eq!(buckling_curve_rolled_i(&tube, 'y', 460.0e6), BucklingCurve::C);
        }
        assert!(!hollow_route_open(&section(SectionKind::Angle, 0.1, 0.1, 0.01, 0.0019, 2.9e-6, 7.3e-7)));
    }

    #[test]
    fn only_the_angle_has_no_lateral_torsional_rule() {
        assert!(ltb_not_applicable(&section(SectionKind::Angle, 0.1, 0.1, 0.01, 0.0019, 2.9e-6, 7.3e-7)));
        assert!(!ltb_not_applicable(&section(SectionKind::Chs, 0.1683, 0.1683, 0.0063, 3.2e-3, 1.0e-5, 1.0e-5)));
    }

    #[test]
    fn the_angle_tower_leg_buckles_about_the_minor_principal_axis() {
        let angle = section(SectionKind::Angle, 0.1, 0.1, 0.01, 0.0019, 2.865_833_333e-6, 7.342_543_86e-7);
        let chi_v = tower_flexural_chi(&angle, 3.0, 3.0, 210.0e9, 235.0e6);
        assert!((chi_v - 0.300_051_563_404_449_9).abs() < 1e-6, "chi {chi_v}");
        assert!((chi_v - tower_flexural_chi(&angle, 0.5, 3.0, 210.0e9, 235.0e6)).abs() < 1e-12);
    }

    #[test]
    fn the_section_interaction_follows_the_kind() {
        let chs = section(SectionKind::Chs, 0.1683, 0.1683, 0.0063, 3.2e-3, 1.0e-5, 1.0e-5);
        let eta = section_mn_eta(&chs, 30.0, 2.0, 1.0, 100.0, 10.0, 10.0, 1);
        assert!((eta - 0.065_930_560_675_786_77).abs() < 1e-9, "CHS eta {eta}");
        let angle = section(SectionKind::Angle, 0.1, 0.1, 0.01, 0.0019, 2.9e-6, 7.3e-7);
        assert!((section_mn_eta(&angle, 20.0, 1.0, 0.5, 100.0, 10.0, 10.0, 3) - 0.35).abs() < 1e-12);
        assert!((section_mn_eta(&angle, 20.0, 1.0, 0.5, 100.0, 10.0, 10.0, 4) - 0.35).abs() < 1e-12);
    }

    #[test]
    fn only_rolled_and_welded_i_sections_are_i_sections() {
        assert!(is_i_section(SectionKind::RolledI) && is_i_section(SectionKind::WeldedI));
        assert!(!is_i_section(SectionKind::Hss) && !is_i_section(SectionKind::Chs) && !is_i_section(SectionKind::Angle));
    }
}
