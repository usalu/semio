//! 🌲️ Material and annex factors of the EN 1995 timber checks: the strength class properties, the annex parameters and the modification, creep, size, buckling and shear factors. See [`super`].

use super::*;

/// 📋 Tabulated characteristic properties for EN 338 / EN 14080 / LVL / CLT classes.
pub fn properties_for_class(class: &str) -> Option<TimberProperties> {
    let key = class.trim().to_ascii_uppercase().replace(' ', "");
    let solid = |fm, ft0, ft90, fc0, fc90, fv, e0, e05, e90, g, rho| TimberProperties {
        f_m_k: fm * 1e6,
        f_t_0_k: ft0 * 1e6,
        f_t_90_k: ft90 * 1e6,
        f_c_0_k: fc0 * 1e6,
        f_c_90_k: fc90 * 1e6,
        f_v_k: fv * 1e6,
        e_0_mean: e0 * 1e9,
        e_0_05: e05 * 1e9,
        e_90_mean: e90 * 1e9,
        g_mean: g * 1e9,
        rho_k: rho,
        product: TimberProduct::Solid,
    };
    let glulam = |fm, ft0, ft90, fc0, fc90, fv, e0, e05, e90, g, rho| TimberProperties {
        f_m_k: fm * 1e6,
        f_t_0_k: ft0 * 1e6,
        f_t_90_k: ft90 * 1e6,
        f_c_0_k: fc0 * 1e6,
        f_c_90_k: fc90 * 1e6,
        f_v_k: fv * 1e6,
        e_0_mean: e0 * 1e9,
        e_0_05: e05 * 1e9,
        e_90_mean: e90 * 1e9,
        g_mean: g * 1e9,
        rho_k: rho,
        product: TimberProduct::Glulam,
    };
    match key.as_str() {
        "C14" => Some(solid(14.0, 8.0, 0.4, 16.0, 2.0, 3.0, 7.0, 4.7, 0.23, 0.44, 290.0)),
        "C16" => Some(solid(16.0, 10.0, 0.5, 17.0, 2.2, 3.2, 8.0, 5.4, 0.27, 0.50, 310.0)),
        "C18" => Some(solid(18.0, 11.0, 0.5, 18.0, 2.2, 3.4, 9.0, 6.0, 0.30, 0.56, 320.0)),
        "C20" => Some(solid(20.0, 12.0, 0.5, 19.0, 2.3, 3.6, 9.5, 6.4, 0.32, 0.59, 330.0)),
        "C22" => Some(solid(22.0, 13.0, 0.5, 20.0, 2.4, 3.8, 10.0, 6.7, 0.33, 0.63, 340.0)),
        "C24" => Some(solid(24.0, 14.0, 0.5, 21.0, 2.5, 4.0, 11.0, 7.4, 0.37, 0.69, 350.0)),
        "C27" => Some(solid(27.0, 16.0, 0.6, 22.0, 2.6, 4.0, 11.5, 7.7, 0.38, 0.72, 370.0)),
        "C30" => Some(solid(30.0, 18.0, 0.6, 23.0, 2.7, 4.0, 12.0, 8.0, 0.40, 0.75, 380.0)),
        "C35" => Some(solid(35.0, 21.0, 0.6, 25.0, 2.8, 3.8, 13.0, 8.7, 0.43, 0.81, 400.0)),
        "C40" => Some(solid(40.0, 24.0, 0.6, 26.0, 2.9, 3.8, 14.0, 9.4, 0.47, 0.88, 420.0)),
        "C45" => Some(solid(45.0, 27.0, 0.6, 27.0, 3.1, 3.8, 15.0, 10.0, 0.50, 0.94, 440.0)),
        "C50" => Some(solid(50.0, 30.0, 0.6, 29.0, 3.2, 3.8, 16.0, 10.7, 0.53, 1.00, 460.0)),
        "GL20H" => Some(glulam(20.0, 16.0, 0.5, 20.0, 2.5, 3.5, 8.4, 7.0, 0.30, 0.54, 340.0)),
        "GL22H" => Some(glulam(22.0, 17.6, 0.5, 22.0, 2.5, 3.5, 10.5, 8.8, 0.30, 0.65, 370.0)),
        "GL24H" => Some(glulam(24.0, 19.2, 0.5, 24.0, 2.5, 3.5, 11.5, 9.6, 0.30, 0.72, 385.0)),
        "GL24C" => Some(glulam(24.0, 17.0, 0.5, 21.5, 2.5, 3.5, 11.0, 9.1, 0.30, 0.69, 350.0)),
        "GL26H" => Some(glulam(26.0, 20.8, 0.5, 26.0, 2.5, 3.5, 12.0, 10.0, 0.30, 0.75, 405.0)),
        "GL26C" => Some(glulam(26.0, 19.0, 0.5, 23.0, 2.5, 3.5, 11.5, 9.6, 0.30, 0.72, 385.0)),
        "GL28H" => Some(glulam(28.0, 22.3, 0.5, 28.0, 2.5, 3.5, 12.6, 10.5, 0.30, 0.78, 425.0)),
        "GL28C" => Some(glulam(28.0, 19.5, 0.5, 24.0, 2.5, 3.5, 12.5, 10.4, 0.30, 0.78, 390.0)),
        "GL30H" => Some(glulam(30.0, 24.0, 0.5, 30.0, 2.5, 3.5, 13.6, 11.3, 0.30, 0.85, 430.0)),
        "GL30C" => Some(glulam(30.0, 19.5, 0.5, 24.5, 2.5, 3.5, 13.0, 10.8, 0.30, 0.81, 390.0)),
        "GL32H" => Some(glulam(32.0, 25.6, 0.5, 32.0, 2.5, 3.5, 14.2, 11.8, 0.30, 0.89, 440.0)),
        "GL32C" => Some(glulam(32.0, 19.5, 0.5, 24.5, 2.5, 3.5, 13.5, 11.2, 0.30, 0.84, 400.0)),
        "LVL32" | "LVL-32" => Some(TimberProperties {
            f_m_k: 36e6,
            f_t_0_k: 26e6,
            f_t_90_k: 0.9e6,
            f_c_0_k: 35e6,
            f_c_90_k: 6.0e6,
            f_v_k: 4.5e6,
            e_0_mean: 14e9,
            e_0_05: 11.6e9,
            e_90_mean: 0.5e9,
            g_mean: 0.6e9,
            rho_k: 510.0,
            product: TimberProduct::Lvl,
        }),
        "CLT100" | "CLT" => Some(TimberProperties {
            f_m_k: 24e6,
            f_t_0_k: 14e6,
            f_t_90_k: 0.5e6,
            f_c_0_k: 21e6,
            f_c_90_k: 2.5e6,
            f_v_k: 4.0e6,
            e_0_mean: 11e9,
            e_0_05: 7.4e9,
            e_90_mean: 0.37e9,
            g_mean: 0.69e9,
            rho_k: 400.0,
            product: TimberProduct::Clt,
        }),
        _ => None,
    }
}

/// 📜 Strength-class pick list for remedies.
pub fn strength_class_options() -> &'static [&'static str] {
    &[
        "C14", "C16", "C18", "C20", "C22", "C24", "C27", "C30", "C35", "C40", "C45", "C50", "GL20h",
        "GL22h", "GL24h", "GL24c", "GL26h", "GL26c", "GL28h", "GL28c", "GL30h", "GL30c", "GL32h",
        "GL32c", "LVL32", "CLT100",
    ]
}

/// 🌍️ National-annex parameters for EN 1995 checks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub annex: AnnexChoice,
}

impl AnnexParams {
    pub fn for_annex(annex: AnnexChoice) -> Self {
        Self { annex }
    }
    pub fn en() -> Self {
        Self::for_annex(AnnexChoice::En)
    }
    pub fn de() -> Self {
        Self::for_annex(AnnexChoice::De)
    }
    /// 🛡️ γ_M — EN 1995-1-1 Table 2.3 (glulam 1.25, LVL 1.2, else 1.3); DIN EN 1995-1-1/NA Table NA.2 sets 1.3 for
    /// every timber product.
    pub fn gamma_m(&self, product: TimberProduct) -> f64 {
        match (self.annex, product) {
            (AnnexChoice::De, _) => 1.3,
            (AnnexChoice::En, TimberProduct::Glulam) => 1.25,
            (AnnexChoice::En, TimberProduct::Lvl) => 1.2,
            (AnnexChoice::En, _) => 1.3,
        }
    }
    pub fn gamma_m_connection(&self) -> f64 {
        1.3
    }
    pub fn k_cr(&self, f_v_k_pa: f64) -> f64 {
        match self.annex {
            AnnexChoice::En => 0.67,
            AnnexChoice::De => {
                let f = f_v_k_pa / 1e6;
                if f <= 0.0 {
                    1.0
                } else {
                    (2.5 / f).min(1.0)
                }
            }
        }
    }
    pub fn w_inst_limit_divisor(&self) -> f64 {
        300.0
    }
    pub fn w_fin_limit_divisor(&self) -> f64 {
        match self.annex {
            AnnexChoice::De => 200.0,
            AnnexChoice::En => 250.0,
        }
    }
    pub fn floor_a_limit(&self) -> f64 {
        match self.annex {
            AnnexChoice::De => 0.05,
            AnnexChoice::En => 0.10,
        }
    }
    /// 🪵 EN 1995-1-1 §7.3.3(2) unit-load deflection limit a; DE NA residential 1.5 mm, EN recommended 1.7 mm.
    pub fn floor_w_1kn_limit_m(&self) -> f64 {
        match self.annex {
            AnnexChoice::De => 0.0015,
            AnnexChoice::En => 0.0017,
        }
    }
    /// 🪵 EN 1995-1-1 §7.3.3(2) velocity limit base b (residential).
    pub fn floor_velocity_b(&self) -> f64 {
        100.0
    }
    pub fn bridge_a_vert_limit(&self) -> f64 {
        0.7
    }
    pub fn bridge_a_hor_limit(&self) -> f64 {
        0.2
    }
    /// 🔥 EN 1995-1-2 §2.3 — k_fi for 20 % fractile conversion (Table 2.1).
    pub fn k_fi(&self, product: TimberProduct) -> f64 {
        match product {
            TimberProduct::Solid => 1.25,
            TimberProduct::Glulam => 1.15,
            TimberProduct::Lvl => 1.1,
            TimberProduct::Clt => 1.15,
        }
    }
    pub fn k_mod_fi(&self) -> f64 {
        1.0
    }
    pub fn gamma_m_fi(&self) -> f64 {
        1.0
    }
    /// 🔥 Design charring rate β_n (m/s) — Table 3.1 softwood / glulam / LVL.
    pub fn beta_n(&self, product: TimberProduct) -> f64 {
        let mm_per_min = match product {
            TimberProduct::Solid => 0.80,
            TimberProduct::Glulam => 0.70,
            TimberProduct::Lvl => 0.70,
            TimberProduct::Clt => 0.65,
        };
        mm_per_min * 1e-3 / 60.0
    }
    pub fn beta_0(&self, product: TimberProduct) -> f64 {
        let mm_per_min = match product {
            TimberProduct::Solid => 0.65,
            TimberProduct::Glulam => 0.65,
            TimberProduct::Lvl => 0.65,
            TimberProduct::Clt => 0.65,
        };
        mm_per_min * 1e-3 / 60.0
    }
}

/// 🌧 Service class SC1–SC3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceClass {
    Sc1,
    Sc2,
    Sc3,
}

/// ⏱ Load-duration class — Instantaneous is shortest (§3.1.3(2) enum order).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadDuration {
    Permanent,
    Long,
    Medium,
    Short,
    Instantaneous,
}

pub fn parse_service_class(v: u8) -> ServiceClass {
    match v {
        2 => ServiceClass::Sc2,
        3 => ServiceClass::Sc3,
        _ => ServiceClass::Sc1,
    }
}

pub fn parse_load_duration(s: &str) -> LoadDuration {
    match s.trim().to_ascii_lowercase().as_str() {
        "permanent" | "perm" => LoadDuration::Permanent,
        "long" => LoadDuration::Long,
        "short" => LoadDuration::Short,
        "instantaneous" | "inst" => LoadDuration::Instantaneous,
        _ => LoadDuration::Medium,
    }
}

/// 𝜓 EN 1990 Table A1.1 ψ₀/ψ₁/ψ₂ for categories A–E/H and snow/wind.
pub fn psi_factors(kind: &str, category: &str) -> (f64, f64, f64) {
    let k = kind.trim().to_ascii_lowercase();
    let c = category.trim().to_ascii_uppercase();
    match k.as_str() {
        "permanent" => (1.0, 1.0, 1.0),
        "accidental" => (0.0, 0.0, 0.0),
        "snow" => (0.5, 0.2, 0.0),
        "snow_high" => (0.7, 0.5, 0.2),
        "wind" => (0.6, 0.2, 0.0),
        "imposed" => match c.as_str() {
            "A" | "B" => (0.7, 0.5, 0.3),
            "C" | "D" => (0.7, 0.7, 0.6),
            "E" => (1.0, 0.9, 0.8),
            "H" => (0.0, 0.0, 0.0),
            _ => (0.7, 0.5, 0.3),
        },
        _ => (0.7, 0.5, 0.3),
    }
}

pub fn k_mod(service: ServiceClass, duration: LoadDuration) -> f64 {
    match (service, duration) {
        (ServiceClass::Sc1 | ServiceClass::Sc2, LoadDuration::Permanent) => 0.60,
        (ServiceClass::Sc1 | ServiceClass::Sc2, LoadDuration::Long) => 0.70,
        (ServiceClass::Sc1 | ServiceClass::Sc2, LoadDuration::Medium) => 0.80,
        (ServiceClass::Sc1 | ServiceClass::Sc2, LoadDuration::Short) => 0.90,
        (ServiceClass::Sc1 | ServiceClass::Sc2, LoadDuration::Instantaneous) => 1.10,
        (ServiceClass::Sc3, LoadDuration::Permanent) => 0.50,
        (ServiceClass::Sc3, LoadDuration::Long) => 0.55,
        (ServiceClass::Sc3, LoadDuration::Medium) => 0.65,
        (ServiceClass::Sc3, LoadDuration::Short) => 0.70,
        (ServiceClass::Sc3, LoadDuration::Instantaneous) => 0.90,
    }
}

pub fn k_def(service: ServiceClass) -> f64 {
    match service {
        ServiceClass::Sc1 => 0.60,
        ServiceClass::Sc2 => 0.80,
        ServiceClass::Sc3 => 2.00,
    }
}

pub fn k_h(h_m: f64, product: TimberProduct) -> f64 {
    let h_mm = h_m * 1000.0;
    match product {
        TimberProduct::Solid => {
            if h_mm >= 150.0 {
                1.0
            } else {
                (150.0 / h_mm.max(1.0)).powf(0.2).min(1.3)
            }
        }
        _ => {
            if h_mm >= 600.0 {
                1.0
            } else {
                (600.0 / h_mm.max(1.0)).powf(0.1).min(1.1)
            }
        }
    }
}

pub fn k_crit(lambda_rel_m: f64) -> f64 {
    if lambda_rel_m <= 0.75 {
        1.0
    } else if lambda_rel_m <= 1.4 {
        1.56 - 0.75 * lambda_rel_m
    } else {
        1.0 / (lambda_rel_m * lambda_rel_m)
    }
}

pub fn lambda_rel_m(w_m3: f64, f_m_k: f64, m_crit_nm: f64) -> f64 {
    if m_crit_nm <= 0.0 || w_m3 <= 0.0 || f_m_k <= 0.0 {
        0.0
    } else {
        (w_m3 * f_m_k / m_crit_nm).sqrt()
    }
}

/// 🪢 Critical moment scaled by lateral-restraint spacing (§6.3.3): M_crit ∝ 1/ℓ_ef² with ℓ_ef = min(spacing, span).
pub fn effective_m_crit_nm(m: &TimberMember) -> f64 {
    let span = m.span_m.max(1e-6);
    let base = m.m_crit_nm.max(1.0);
    if m.lateral_restraint_spacing_m <= 0.0 {
        return base;
    }
    let l_ef = m.lateral_restraint_spacing_m.min(span).max(1e-6);
    base * (span / l_ef).powi(2)
}

pub fn k_c_buckling(lambda_rel: f64) -> f64 {
    let beta_c = 0.2;
    if lambda_rel <= 0.3 {
        return 1.0;
    }
    let k = 0.5 * (1.0 + beta_c * (lambda_rel - 0.3) + lambda_rel * lambda_rel);
    1.0 / (k + (k * k - lambda_rel * lambda_rel).max(0.0).sqrt())
}

pub fn lambda_rel_compression(lambda: f64, f_c_0_k: f64, e_0_05: f64) -> f64 {
    if e_0_05 <= 0.0 {
        0.0
    } else {
        (lambda / std::f64::consts::PI) * (f_c_0_k / e_0_05).sqrt()
    }
}

/// 📐 Compression-perpendicular factor k_c,90 from bearing and support lengths.
pub fn k_c_90(bearing_length_m: f64, support_length_m: f64) -> f64 {
    let l = bearing_length_m.max(1e-6);
    let a1 = 0.03_f64.min(l).min(support_length_m.max(0.0));
    let a2 = 0.03_f64.min(l);
    let l_ef = l + a1 + a2;
    (l_ef / l).min(1.5).max(1.0)
}

pub fn k_v(h_m: f64, notch_depth_m: f64, notch_distance_m: f64) -> f64 {
    if notch_depth_m <= 0.0 || h_m <= 0.0 {
        return 1.0;
    }
    let alpha = ((h_m - notch_depth_m) / h_m).clamp(0.05, 1.0);
    if alpha >= 0.999 {
        return 1.0;
    }
    let h_mm = h_m * 1000.0;
    let x = notch_distance_m.max(0.0);
    let kn = 5.0;
    let term = kn * (1.0 + 1.1 / h_mm.sqrt());
    let denom = h_mm.sqrt() * ((1.0 - alpha) - (x / h_m).powi(2)).max(1e-6)
        + term * (alpha * (1.0 - alpha)).sqrt().max(1e-6);
    (term / denom).min(1.0).max(0.05)
}

pub fn area_m2(m: &TimberMember) -> f64 {
    m.b_m * m.h_m
}

pub fn w_m3(m: &TimberMember) -> f64 {
    m.b_m * m.h_m.powi(2) / 6.0
}

pub fn i_m4(m: &TimberMember) -> f64 {
    m.b_m * m.h_m.powi(3) / 12.0
}
