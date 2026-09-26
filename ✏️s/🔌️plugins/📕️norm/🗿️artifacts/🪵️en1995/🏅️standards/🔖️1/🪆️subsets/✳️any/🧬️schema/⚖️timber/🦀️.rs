//! 🪵 EN 1995-1-1 / DIN NA / 1-2 / 2 timber compliance formulas and check builders.

use crate::document::{
    AnnexChoice, CheckBuilder, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy,
    SubjectRef,
};
use crate::{
    CharacteristicAction, ConnectionAction, MemberRole, SupportType, TimberConnection, TimberMember,
    TimberProduct, TimberProperties,
};

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}
fn member_subject(m: &TimberMember, leaf: &str) -> SubjectRef {
    SubjectRef::new(&m.id, format!("members[id={}].{leaf}", m.id), loc(&m.label_en, &m.label_de))
}
fn conn_subject(c: &TimberConnection, leaf: &str) -> SubjectRef {
    SubjectRef::new(&c.id, format!("connections[id={}].{leaf}", c.id), loc(&c.label_en, &c.label_de))
}

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
    pub fn gamma_m(&self, product: TimberProduct) -> f64 {
        match product {
            TimberProduct::Glulam => 1.25,
            _ => 1.3,
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

pub const K0_7_MM: f64 = 0.007;
pub fn d_char_n(fire_duration_s: f64, beta_n: f64) -> f64 {
    beta_n * fire_duration_s
}
pub fn d_ef(fire_duration_s: f64, beta_n: f64) -> f64 {
    let d_char = d_char_n(fire_duration_s, beta_n);
    let k0 = if fire_duration_s >= 1200.0 {
        1.0
    } else {
        fire_duration_s / 1200.0
    };
    d_char + k0 * K0_7_MM
}
pub fn residual_bh(b_m: f64, h_m: f64, d_ef: f64) -> (f64, f64) {
    ((b_m - 2.0 * d_ef).max(0.0), (h_m - 2.0 * d_ef).max(0.0))
}

pub fn f_h_k(rho_k: f64, d_m: f64, fastener: &str) -> f64 {
    let d_mm = d_m * 1000.0;
    if fastener.to_ascii_lowercase().contains("nail") && d_mm < 8.0 {
        0.082 * rho_k * d_mm.powf(-0.3) * 1e6
    } else {
        0.082 * (1.0 - 0.01 * d_mm) * rho_k * 1e6
    }
}
/// 🔩 Yield moment M_y,Rk = 0.3·f_u,k·d^2.6 (eq. 8.14 / 8.30) — empirical in N·mm, MPa and mm, returned in N·m.
pub fn m_y_k(f_u_k: f64, d_m: f64) -> f64 {
    0.3 * (f_u_k / 1e6) * (d_m * 1000.0).powf(2.6) / 1000.0
}

/// 🔩 Johansen timber–timber single shear (§8.2.2).
pub fn johansen_single_shear_f_v_rk(
    t1: f64,
    t2: f64,
    d: f64,
    f_h1: f64,
    f_h2: f64,
    m_y: f64,
    f_ax_rk: f64,
) -> f64 {
    let beta = if f_h1 > 0.0 { f_h2 / f_h1 } else { 1.0 };
    let f_a = f_h1 * t1 * d;
    let f_b = f_h2 * t2 * d;
    let ratio = if t2 > 0.0 { t1 / t2 } else { 1.0 };
    let f_c = (f_h1 * t1 * d / (1.0 + beta))
        * ((beta + 2.0 * beta.powi(2) * (1.0 + ratio + ratio.powi(2)) + beta.powi(3) * ratio.powi(2))
            .sqrt()
            - beta * (1.0 + ratio));
    let f_d = 1.05
        * (f_h1 * t1 * d / (2.0 + beta))
        * ((2.0 * beta * (1.0 + beta)
            + (4.0 * beta * (2.0 + beta) * m_y) / (f_h1 * d * t1.powi(2)).max(1e-18))
            .sqrt()
            - beta);
    let f_e = 1.05
        * (f_h1 * t2 * d / (1.0 + 2.0 * beta))
        * ((2.0 * beta.powi(2) * (1.0 + beta)
            + (4.0 * beta * (1.0 + 2.0 * beta) * m_y) / (f_h1 * d * t2.powi(2)).max(1e-18))
            .sqrt()
            - beta);
    let f_f = 1.15 * ((2.0 * beta) / (1.0 + beta)).sqrt() * (2.0 * m_y * f_h1 * d).sqrt();
    let modes = [f_a, f_b, f_c.max(0.0), f_d.max(0.0), f_e.max(0.0), f_f.max(0.0)];
    let base = modes.into_iter().fold(f64::INFINITY, f64::min);
    let rope = (f_ax_rk / 4.0).min(base * 0.25);
    base + rope
}

/// 🔩 Johansen timber–timber double shear (§8.2.2).
pub fn johansen_double_shear_f_v_rk(
    t1: f64,
    t2: f64,
    d: f64,
    f_h1: f64,
    f_h2: f64,
    m_y: f64,
    f_ax_rk: f64,
) -> f64 {
    let beta = if f_h1 > 0.0 { f_h2 / f_h1 } else { 1.0 };
    let f_g = f_h1 * t1 * d;
    let f_h = 0.5 * f_h2 * t2 * d;
    let f_j = 1.05
        * (f_h1 * t1 * d / (2.0 + beta))
        * ((2.0 * beta * (1.0 + beta)
            + (4.0 * beta * (2.0 + beta) * m_y) / (f_h1 * d * t1.powi(2)).max(1e-18))
            .sqrt()
            - beta);
    let f_k = 1.15 * ((2.0 * beta) / (1.0 + beta)).sqrt() * (2.0 * m_y * f_h1 * d).sqrt();
    let modes = [f_g, f_h, f_j.max(0.0), f_k.max(0.0)];
    let base = modes.into_iter().fold(f64::INFINITY, f64::min);
    let rope = (f_ax_rk / 4.0).min(base * 0.25);
    base + rope
}

/// 🔩 Johansen steel–timber (§8.2.3) — thin plate when t ≤ 0.5d, else thick.
pub fn johansen_steel_timber_f_v_rk(
    t_timber: f64,
    t_steel: f64,
    d: f64,
    f_h: f64,
    m_y: f64,
    f_ax_rk: f64,
) -> f64 {
    let thin = t_steel <= 0.5 * d;
    let f_a = 0.4 * f_h * t_timber * d;
    let f_b = 1.15 * (2.0 * m_y * f_h * d).sqrt();
    let f_c = if thin {
        f_h * t_timber * d
            * ((2.0 + (4.0 * m_y) / (f_h * d * t_timber.powi(2)).max(1e-18)).sqrt() - 1.0)
    } else {
        2.3 * (m_y * f_h * d).sqrt()
    };
    let modes = [f_a.max(0.0), f_b.max(0.0), f_c.max(0.0)];
    let base = modes.into_iter().fold(f64::INFINITY, f64::min);
    let rope = (f_ax_rk / 4.0).min(base * 0.25);
    base + rope
}

pub fn n_ef(n: u32, a1_m: f64, d_m: f64) -> f64 {
    if n <= 1 {
        return 1.0;
    }
    let a1 = a1_m.max(1e-6);
    let d = d_m.max(1e-6);
    let cand = (n as f64).powf(0.9) * (a1 / (13.0 * d)).powf(0.25);
    (n as f64).min(cand)
}

/// 📏 Table 8.2 / 8.4 / 8.5 minima (α = 0 parallel-to-grain force direction).
pub fn spacing_minima(c: &TimberConnection) -> (f64, f64, f64) {
    let d = c.diameter_m.max(1e-6);
    let ft = c.fastener_type.to_ascii_lowercase();
    let (a1, a3t, a4t) = if ft.contains("nail") {
        (5.0 * d, 10.0 * d, 5.0 * d)
    } else if ft.contains("screw") {
        (5.0 * d, 10.0 * d, 5.0 * d)
    } else {
        (5.0 * d, 7.0 * d, 3.0 * d)
    };
    (a1, a3t, a4t)
}

/// ⚖️ Characteristic internals from loads or analysed forces.
#[derive(Clone, Copy, Debug, Default)]
pub struct ActionInternals {
    pub m_k_nm: f64,
    pub v_k_n: f64,
    pub n_k_n: f64,
    pub n_t_k_n: f64,
    pub f_c90_k_n: f64,
    pub q_line_n_per_m: f64,
    pub f_point_n: f64,
}

/// 🧮 Derive M/V/N from SupportType when loads are nonzero; else use analysed forces.
pub fn action_internals(m: &TimberMember, a: &CharacteristicAction) -> ActionInternals {
    let q = a.q_line_n_per_m;
    let f = a.f_point_n;
    let l = m.span_m.max(0.0);
    let loads = q.abs() > 0.0 || f.abs() > 0.0;
    if loads {
        let (mk, vk) = match m.support {
            SupportType::SimplySupported => {
                (q * l * l / 8.0 + f * l / 4.0, q * l / 2.0 + f / 2.0)
            }
            SupportType::Cantilever => (q * l * l / 2.0 + f * l, q * l + f),
            SupportType::ContinuousTwoSpan => {
                (q * l * l / 14.0 + f * l / 6.0, q * l * 0.6 + f * 0.6)
            }
        };
        let nk = if m.role == MemberRole::Column {
            q * l + f
        } else {
            0.0
        };
        let fc90 = vk.max(0.0);
        ActionInternals {
            m_k_nm: mk,
            v_k_n: vk,
            n_k_n: nk,
            n_t_k_n: 0.0,
            f_c90_k_n: fc90,
            q_line_n_per_m: q,
            f_point_n: f,
        }
    } else {
        ActionInternals {
            m_k_nm: a.m_k_nm,
            v_k_n: a.v_k_n,
            n_k_n: a.n_k_n,
            n_t_k_n: a.n_t_k_n,
            f_c90_k_n: a.f_c90_k_n,
            q_line_n_per_m: 0.0,
            f_point_n: 0.0,
        }
    }
}

fn action_kind_family(kind: &str) -> &'static str {
    match kind.trim().to_ascii_lowercase().as_str() {
        "permanent" | "g" | "dead" => "permanent",
        "accidental" | "a" => "accidental",
        "snow" => "snow",
        "wind" => "wind",
        _ => "imposed",
    }
}

/// 📦 Limit-state combination kind (EN 1990 + DE NA eq 6.10 family).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboKind {
    Uls,
    SlsCharacteristic,
    SlsQuasiPermanent,
    Accidental,
}

/// 📦 Factored design internals for one load combination.
#[derive(Clone, Debug)]
pub struct LoadCombo {
    pub id: String,
    pub kind: ComboKind,
    pub duration: LoadDuration,
    pub m_ed_nm: f64,
    pub v_ed_n: f64,
    pub n_ed_n: f64,
    pub n_t_ed_n: f64,
    pub f_c90_ed_n: f64,
    pub q_ed_line: f64,
    pub f_ed_point: f64,
    pub psi2: f64,
}

fn scale_internals(i: &ActionInternals, factor: f64) -> ActionInternals {
    ActionInternals {
        m_k_nm: i.m_k_nm * factor,
        v_k_n: i.v_k_n * factor,
        n_k_n: i.n_k_n * factor,
        n_t_k_n: i.n_t_k_n * factor,
        f_c90_k_n: i.f_c90_k_n * factor,
        q_line_n_per_m: i.q_line_n_per_m * factor,
        f_point_n: i.f_point_n * factor,
    }
}

fn add_internals(a: &mut ActionInternals, b: &ActionInternals) {
    a.m_k_nm += b.m_k_nm;
    a.v_k_n += b.v_k_n;
    a.n_k_n += b.n_k_n;
    a.n_t_k_n += b.n_t_k_n;
    a.f_c90_k_n += b.f_c90_k_n;
    a.q_line_n_per_m += b.q_line_n_per_m;
    a.f_point_n += b.f_point_n;
}

fn combo_from(id: String, kind: ComboKind, duration: LoadDuration, i: ActionInternals, psi2: f64) -> LoadCombo {
    LoadCombo {
        id,
        kind,
        duration,
        m_ed_nm: i.m_k_nm,
        v_ed_n: i.v_k_n,
        n_ed_n: i.n_k_n,
        n_t_ed_n: i.n_t_k_n,
        f_c90_ed_n: i.f_c90_k_n,
        q_ed_line: i.q_line_n_per_m,
        f_ed_point: i.f_point_n,
        psi2,
    }
}

fn shortest_duration<'a>(actions: impl IntoIterator<Item = &'a CharacteristicAction>) -> LoadDuration {
    actions
        .into_iter()
        .map(|a| parse_load_duration(&a.load_duration))
        .max()
        .unwrap_or(LoadDuration::Medium)
}

/// 🔀 EN 1990 + DE NA eq 6.10 ULS (γG=1.35, γQ=1.5, ψ₀ accompanying), SLS char/QP, accidental.
pub fn enumerate_combos(m: &TimberMember) -> Vec<LoadCombo> {
    let mut out = Vec::new();
    if m.actions.is_empty() {
        return out;
    }
    let internals: Vec<ActionInternals> = m.actions.iter().map(|a| action_internals(m, a)).collect();
    let mut perm = Vec::new();
    let mut var = Vec::new();
    let mut acc = Vec::new();
    for (idx, a) in m.actions.iter().enumerate() {
        match action_kind_family(&a.kind) {
            "permanent" => perm.push(idx),
            "accidental" => acc.push(idx),
            _ => var.push(idx),
        }
    }
    let gamma_g = 1.35;
    let gamma_q = 1.5;

    let sum_perm = |factor: f64| {
        let mut s = ActionInternals::default();
        for &i in &perm {
            add_internals(&mut s, &scale_internals(&internals[i], factor));
        }
        s
    };

    if var.is_empty() {
        let dur = shortest_duration(perm.iter().map(|&i| &m.actions[i]));
        out.push(combo_from(
            "uls.g".into(),
            ComboKind::Uls,
            dur,
            sum_perm(gamma_g),
            1.0,
        ));
    } else {
        for (li, &lead) in var.iter().enumerate() {
            let mut s = sum_perm(gamma_g);
            add_internals(&mut s, &scale_internals(&internals[lead], gamma_q));
            let mut psi2_lead = 0.3;
            for (vi, &v) in var.iter().enumerate() {
                let (psi0, _, psi2) = psi_factors(&m.actions[v].kind, &m.actions[v].category);
                if vi == li {
                    psi2_lead = psi2;
                    continue;
                }
                add_internals(&mut s, &scale_internals(&internals[v], gamma_q * psi0));
            }
            let mut acts: Vec<&CharacteristicAction> = perm.iter().map(|&i| &m.actions[i]).collect();
            acts.extend(var.iter().map(|&i| &m.actions[i]));
            let dur = shortest_duration(acts);
            out.push(combo_from(
                format!("uls.6.10.lead.{}", m.actions[lead].id),
                ComboKind::Uls,
                dur,
                s,
                psi2_lead,
            ));
        }
    }

    if !var.is_empty() {
        for (li, &lead) in var.iter().enumerate() {
            let mut s = sum_perm(1.0);
            add_internals(&mut s, &scale_internals(&internals[lead], 1.0));
            let mut psi2_lead = 0.3;
            for (vi, &v) in var.iter().enumerate() {
                let (psi0, _, psi2) = psi_factors(&m.actions[v].kind, &m.actions[v].category);
                if vi == li {
                    psi2_lead = psi2;
                    continue;
                }
                add_internals(&mut s, &scale_internals(&internals[v], psi0));
            }
            let dur = shortest_duration(var.iter().chain(perm.iter()).map(|&i| &m.actions[i]));
            out.push(combo_from(
                format!("sls.char.lead.{}", m.actions[lead].id),
                ComboKind::SlsCharacteristic,
                dur,
                s,
                psi2_lead,
            ));
        }
        let mut s = sum_perm(1.0);
        let mut psi2_max: f64 = 0.0;
        for &v in &var {
            let (_, _, psi2) = psi_factors(&m.actions[v].kind, &m.actions[v].category);
            psi2_max = psi2_max.max(psi2);
            add_internals(&mut s, &scale_internals(&internals[v], psi2));
        }
        let dur = shortest_duration(var.iter().chain(perm.iter()).map(|&i| &m.actions[i]));
        out.push(combo_from(
            "sls.qp".into(),
            ComboKind::SlsQuasiPermanent,
            dur,
            s,
            psi2_max,
        ));
    } else {
        let dur = shortest_duration(perm.iter().map(|&i| &m.actions[i]));
        out.push(combo_from(
            "sls.char.g".into(),
            ComboKind::SlsCharacteristic,
            dur,
            sum_perm(1.0),
            1.0,
        ));
        out.push(combo_from(
            "sls.qp.g".into(),
            ComboKind::SlsQuasiPermanent,
            dur,
            sum_perm(1.0),
            1.0,
        ));
    }

    if !acc.is_empty() {
        let mut s = sum_perm(1.0);
        for &i in &acc {
            add_internals(&mut s, &internals[i]);
        }
        for &v in &var {
            let (_, _, psi2) = psi_factors(&m.actions[v].kind, &m.actions[v].category);
            add_internals(&mut s, &scale_internals(&internals[v], psi2));
        }
        let dur = LoadDuration::Instantaneous;
        out.push(combo_from("accidental".into(), ComboKind::Accidental, dur, s, 0.0));
    }
    out
}

fn connection_action_internals(a: &ConnectionAction) -> f64 {
    a.f_k_n
}

/// 🔀 Connection force combinations mirroring member EN 1990 eq 6.10.
pub fn enumerate_connection_combos(c: &TimberConnection) -> Vec<(String, ComboKind, LoadDuration, f64)> {
    let mut out = Vec::new();
    if c.actions.is_empty() {
        return out;
    }
    let mut perm = Vec::new();
    let mut var = Vec::new();
    let mut acc = Vec::new();
    for (idx, a) in c.actions.iter().enumerate() {
        match action_kind_family(&a.kind) {
            "permanent" => perm.push(idx),
            "accidental" => acc.push(idx),
            _ => var.push(idx),
        }
    }
    let f = |i: usize| connection_action_internals(&c.actions[i]);
    let sum_perm = |factor: f64| perm.iter().map(|&i| f(i) * factor).sum::<f64>();
    let gamma_g = 1.35;
    let gamma_q = 1.5;
    let dur_of = |idxs: &[usize]| {
        idxs.iter()
            .map(|&i| parse_load_duration(&c.actions[i].load_duration))
            .max()
            .unwrap_or(LoadDuration::Medium)
    };
    if var.is_empty() {
        out.push(("uls.g".into(), ComboKind::Uls, dur_of(&perm), sum_perm(gamma_g)));
    } else {
        for &lead in &var {
            let mut fed = sum_perm(gamma_g) + gamma_q * f(lead);
            for &v in &var {
                if v == lead {
                    continue;
                }
                let (psi0, _, _) = psi_factors(&c.actions[v].kind, "");
                fed += gamma_q * psi0 * f(v);
            }
            let mut idxs = perm.clone();
            idxs.extend_from_slice(&var);
            out.push((
                format!("uls.lead.{}", c.actions[lead].id),
                ComboKind::Uls,
                dur_of(&idxs),
                fed,
            ));
        }
    }
    if !acc.is_empty() {
        let mut fed = sum_perm(1.0);
        for &i in &acc {
            fed += f(i);
        }
        out.push((
            "accidental".into(),
            ComboKind::Accidental,
            LoadDuration::Instantaneous,
            fed,
        ));
    }
    out
}

fn deflection_m(m: &TimberMember, props: &TimberProperties, q: f64, f: f64) -> f64 {
    let ei = props.e_0_mean * i_m4(m);
    if ei <= 0.0 || m.span_m <= 0.0 {
        return 0.0;
    }
    let l = m.span_m;
    match m.support {
        SupportType::SimplySupported => 5.0 * q * l.powi(4) / (384.0 * ei) + f * l.powi(3) / (48.0 * ei),
        SupportType::Cantilever => q * l.powi(4) / (8.0 * ei) + f * l.powi(3) / (3.0 * ei),
        SupportType::ContinuousTwoSpan => {
            q * l.powi(4) / (185.0 * ei) + f * l.powi(3) / (100.0 * ei)
        }
    }
}

fn governing_uls(combos: &[LoadCombo]) -> Option<&LoadCombo> {
    combos
        .iter()
        .filter(|c| c.kind == ComboKind::Uls)
        .max_by(|a, b| {
            a.m_ed_nm
                .abs()
                .partial_cmp(&b.m_ed_nm.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

//#region 🔖️Assessment
/// 🧾 One assessed check before its remedies are attached — carries the utilization the builder keeps private.
struct Assessed<T: 'static> {
    id: String,
    utilization: f64,
    builder: CheckBuilder,
    levers: &'static [Lever<T>],
}

/// 📏 Resolution a lever's required value is rounded up to.
#[derive(Clone, Copy)]
enum Grain {
    Millimetre,
    Unit,
    Count,
}
impl Grain {
    fn resolution(self) -> f64 {
        match self {
            Self::Millimetre => 1e-3,
            Self::Unit | Self::Count => 1.0,
        }
    }
    fn round_up(self, x: f64) -> f64 {
        let r = self.resolution();
        (x / r - 1e-9).ceil() * r
    }
}

/// 🎚 One input field a remedy may raise, with its localized label for remedy copy.
struct Lever<T> {
    leaf: &'static str,
    label_en: &'static str,
    label_de: &'static str,
    unit: &'static str,
    grain: Grain,
    kind: QuantityKind,
    get: fn(&T) -> f64,
    set: fn(&mut T, f64),
}
impl<T> Lever<T> {
    fn display(&self, value: f64) -> String {
        match self.grain {
            Grain::Millimetre => format!("{:.0} mm", value * 1000.0),
            Grain::Unit => format!("{value:.0} {}", self.unit),
            Grain::Count => format!("{value:.0}"),
        }
    }
    fn quantity(&self, value: f64) -> Quantity {
        Quantity::new(self.kind, value)
    }
}

const H: Lever<TimberMember> = Lever { leaf: "hM", label_en: "section depth h", label_de: "Querschnittshöhe h", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |m| m.h_m, set: |m, v| m.h_m = v };
const B: Lever<TimberMember> = Lever { leaf: "bM", label_en: "section width b", label_de: "Querschnittsbreite b", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |m| m.b_m, set: |m, v| m.b_m = v };
const BEARING: Lever<TimberMember> = Lever { leaf: "bearingLengthM", label_en: "bearing length l", label_de: "Aufstandslänge l", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |m| m.bearing_length_m, set: |m, v| m.bearing_length_m = v };
const MCRIT: Lever<TimberMember> = Lever { leaf: "mCritNm", label_en: "critical moment M_crit (lateral restraint)", label_de: "Kippmoment M_crit (seitliche Halterung)", unit: "N·m", grain: Grain::Unit, kind: QuantityKind::Moment, get: |m| m.m_crit_nm, set: |m, v| m.m_crit_nm = v };
const MASS: Lever<TimberMember> = Lever { leaf: "massKgPerM", label_en: "linear mass", label_de: "längenbezogene Masse", unit: "kg/m", grain: Grain::Unit, kind: QuantityKind::Mass, get: |m| m.mass_kg_per_m, set: |m, v| m.mass_kg_per_m = v };
const NUMBER: Lever<TimberConnection> = Lever { leaf: "number", label_en: "number of fasteners", label_de: "Anzahl der Verbindungsmittel", unit: "", grain: Grain::Count, kind: QuantityKind::Dimensionless, get: |c| c.number as f64, set: |c, v| c.number = v.round().max(0.0) as u32 };
const DIAMETER: Lever<TimberConnection> = Lever { leaf: "diameterM", label_en: "fastener diameter d", label_de: "Durchmesser d", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.diameter_m, set: |c, v| c.diameter_m = v };
const SPACING: Lever<TimberConnection> = Lever { leaf: "spacingM", label_en: "spacing a₁", label_de: "Abstand a₁", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.spacing_m, set: |c, v| c.spacing_m = v };
const END: Lever<TimberConnection> = Lever { leaf: "endDistanceM", label_en: "end distance a₃,t", label_de: "Hirnholzabstand a₃,t", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.end_distance_m, set: |c, v| c.end_distance_m = v };
const EDGE: Lever<TimberConnection> = Lever { leaf: "edgeDistanceM", label_en: "edge distance a₄,t", label_de: "Randabstand a₄,t", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.edge_distance_m, set: |c, v| c.edge_distance_m = v };

const BENDING_LEVERS: &[Lever<TimberMember>] = &[H, MCRIT];
const DEPTH_LEVERS: &[Lever<TimberMember>] = &[H];
const SECTION_LEVERS: &[Lever<TimberMember>] = &[H, B];
const WIDTH_LEVERS: &[Lever<TimberMember>] = &[B];
const BEARING_LEVERS: &[Lever<TimberMember>] = &[BEARING];
const MASS_LEVERS: &[Lever<TimberMember>] = &[MASS];
const JOHANSEN_LEVERS: &[Lever<TimberConnection>] = &[NUMBER, DIAMETER];
const SPACING_LEVERS: &[Lever<TimberConnection>] = &[SPACING];
const END_LEVERS: &[Lever<TimberConnection>] = &[END];
const EDGE_LEVERS: &[Lever<TimberConnection>] = &[EDGE];

/// 🧾 Header shared by every assessed check (identity, clause, subject and annex).
struct Head {
    id: String,
    part: &'static str,
    clause: ClauseId,
    subject: SubjectRef,
    title: LocalizedCopy,
    annex: AnnexChoice,
}

fn assessed<T>(head: Head, computed: Quantity, limit: Quantity, explanation: LocalizedCopy, levers: &'static [Lever<T>]) -> Assessed<T> {
    let utilization = if limit.value.abs() < f64::EPSILON { 0.0 } else { computed.value / limit.value };
    let builder = CheckResult::assess(head.id.clone(), head.part, head.clause, head.subject, head.title)
        .utilization(computed, limit)
        .annex(head.annex)
        .explanation(explanation);
    Assessed { id: head.id, utilization, builder, levers }
}

/// 🔎 Smallest value of `lever` (rounded up to its grain) at which check `id` passes, re-evaluating through `probe`.
fn solve<T: Clone>(item: &T, id: &str, lever: &Lever<T>, probe: &dyn Fn(&T) -> Vec<(String, f64)>) -> Option<f64> {
    let passes = |x: f64| {
        let mut candidate = item.clone();
        (lever.set)(&mut candidate, x);
        probe(&candidate).into_iter().any(|(cid, u)| cid == id && u <= 1.0)
    };
    let current = (lever.get)(item).max(0.0);
    let mut low = current;
    let mut high = if current > 0.0 { current } else { lever.grain.resolution() };
    let mut feasible = false;
    for _ in 0..64 {
        high *= 1.25;
        if passes(high) {
            feasible = true;
            break;
        }
        low = high;
    }
    if !feasible {
        return None;
    }
    for _ in 0..48 {
        if high - low <= lever.grain.resolution() * 0.25 {
            break;
        }
        let mid = 0.5 * (low + high);
        if passes(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    let mut value = lever.grain.round_up(high);
    for _ in 0..64 {
        if value > current && passes(value) {
            return Some(value);
        }
        value = lever.grain.round_up(value + lever.grain.resolution());
    }
    None
}

/// 🩹 Builds every assessed check, attaching one verified remedy per lever that alone flips a failing check.
fn finish<T: Clone>(item: &T, checks: Vec<Assessed<T>>, subject: fn(&T, &str) -> SubjectRef, probe: &dyn Fn(&T) -> Vec<(String, f64)>) -> Vec<CheckResult> {
    checks
        .into_iter()
        .map(|check| {
            let mut builder = check.builder;
            if check.utilization > 1.0 {
                let mut solved = 0usize;
                for lever in check.levers {
                    let current = (lever.get)(item);
                    if let Some(required) = solve(item, &check.id, lever, probe) {
                        solved += 1;
                        builder = builder.remedy(Remedy::at_least(
                            subject(item, lever.leaf),
                            lever.quantity(current),
                            lever.quantity(required),
                            loc(
                                &format!("Increase {} from {} to at least {}.", lever.label_en, lever.display(current), lever.display(required)),
                                &format!("{} von {} auf mindestens {} erhöhen.", lever.label_de, lever.display(current), lever.display(required)),
                            ),
                        ));
                    }
                }
                if solved == 0 {
                    if let Some(lever) = check.levers.first() {
                        let current = (lever.get)(item);
                        let mut remedy = Remedy::at_least(
                            subject(item, lever.leaf),
                            lever.quantity(current),
                            lever.quantity(current),
                            loc(
                                &format!("Raising the {} alone cannot satisfy this check — combine it with other measures.", lever.label_en),
                                &format!("{} allein zu erhöhen genügt nicht — mit weiteren Maßnahmen kombinieren.", lever.label_de),
                            ),
                        );
                        remedy.applicable = false;
                        builder = builder.remedy(remedy);
                    }
                }
            }
            builder.build()
        })
        .collect()
}
//#endregion 🔖️Assessment

/// 🏗 Evaluate all members and connections for the chosen annex.
pub fn evaluate_structure(annex: AnnexChoice, members: &[TimberMember], connections: &[TimberConnection]) -> CheckReport {
    let mut report = CheckReport::default();
    let params = AnnexParams::for_annex(annex);
    if members.is_empty() {
        report.push(
            CheckResult::assess(
                "en1995.subject.empty",
                "EN 1995-1-1",
                ClauseId::new("EN 1995-1-1", "§1", "1"),
                SubjectRef::whole(loc("Timber structure", "Holztragwerk")),
                loc("Subject completeness", "Vollständigkeit des Gegenstands"),
            )
            .not_applicable(loc("No timber members defined.", "Keine Holzbauteile definiert."))
            .build(),
        );
    }
    for m in members {
        report.extend(evaluate_member(annex, &params, m));
    }
    for c in connections {
        report.extend(evaluate_connection(annex, &params, c));
    }
    report
}

fn evaluate_member(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember) -> Vec<CheckResult> {
    let Some(props) = properties_for_class(&m.strength_class) else {
        return vec![CheckResult::assess(
            format!("en1995.material.unknown.{}", m.id),
            "EN 1995-1-1",
            ClauseId::new("EN 1995-1-1", "§3", "3.2"),
            member_subject(m, "strengthClass"),
            loc("Strength class", "Festigkeitsklasse"),
        )
        .status(crate::document::CheckStatus::Fail)
        .explanation(loc(&format!("Unknown strength class '{}'.", m.strength_class), &format!("Unbekannte Festigkeitsklasse '{}'.", m.strength_class)))
        .remedy(Remedy::one_of(
            member_subject(m, "strengthClass"),
            strength_class_options().iter().map(|s| s.to_string()).collect(),
            loc("Select a tabulated strength class (EN 338 / EN 14080).", "Tabellierte Festigkeitsklasse wählen (EN 338 / EN 14080)."),
        ))
        .annex(annex)
        .build()];
    };
    let probe = |candidate: &TimberMember| assess_member(annex, params, candidate, &props).into_iter().map(|a| (a.id, a.utilization)).collect();
    finish(m, assess_member(annex, params, m, &props), member_subject, &probe)
}

fn assess_member(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties) -> Vec<Assessed<TimberMember>> {
    if m.role == MemberRole::Bridge {
        let mut out = assess_bridge_member(annex, params, m, props);
        if m.fire_duration_s > 0.0 {
            out.push(assess_fire(annex, params, m, props, None));
        }
        return out;
    }
    let mut out = Vec::new();
    let combos = enumerate_combos(m);
    let Some(uls) = governing_uls(&combos) else {
        return out;
    };
    let head = |kind: &str, clause: &str, leaf: &str, en: &str, de: &str| Head {
        id: format!("en1995.{clause}.{kind}.{}", m.id),
        part: "EN 1995-1-1",
        clause: ClauseId::new("EN 1995-1-1", &format!("§{clause}"), clause),
        subject: member_subject(m, leaf),
        title: loc(en, de),
        annex,
    };
    let sc = parse_service_class(m.service_class);
    let km = k_mod(sc, uls.duration);
    let kh = k_h(m.h_m, props.product);
    let gamma = params.gamma_m(props.product);
    let a = area_m2(m);
    let wsec = w_m3(m);
    let m_ed = uls.m_ed_nm;
    let v_ed = uls.v_ed_n;
    let n_ed = uls.n_ed_n;
    let n_t_ed = uls.n_t_ed_n;
    let f_c90_ed = uls.f_c90_ed_n;

    let kcrit = k_crit(lambda_rel_m(wsec, props.f_m_k, m.m_crit_nm.max(1.0)));
    let f_m_d = km * kh * kcrit * props.f_m_k / gamma;
    let sigma_m = if wsec > 0.0 { m_ed / wsec } else { 0.0 };
    out.push(assessed(
        head("bending", "6.1.6", "actions", "Bending stress", "Biegespannung"),
        Quantity::new(QuantityKind::Stress, sigma_m),
        Quantity::new(QuantityKind::Stress, f_m_d.max(1e-9)),
        loc(
            &format!("σ_m,d={:.3} MPa vs f_m,d={:.3} MPa (k_crit={:.3}, combo {}).", sigma_m / 1e6, f_m_d / 1e6, kcrit, uls.id),
            &format!("σ_m,d={:.3} MPa gegen f_m,d={:.3} MPa (k_crit={:.3}, Kombi {}).", sigma_m / 1e6, f_m_d / 1e6, kcrit, uls.id),
        ),
        BENDING_LEVERS,
    ));

    let kcr = params.k_cr(props.f_v_k);
    let kv = k_v(m.h_m, m.notch_depth_m, m.notch_distance_m);
    let b_ef = kcr * m.b_m;
    let f_v_d = km * props.f_v_k / gamma;
    let tau = if b_ef * m.h_m > 0.0 { 1.5 * v_ed / (b_ef * m.h_m) / kv } else { 0.0 };
    out.push(assessed(
        head("shear", "6.1.7", "actions", "Shear stress", "Schubspannung"),
        Quantity::new(QuantityKind::Stress, tau),
        Quantity::new(QuantityKind::Stress, f_v_d.max(1e-9)),
        loc(
            &format!("τ_d={:.3} MPa vs f_v,d={:.3} MPa (k_cr={:.3}, k_v={:.3}).", tau / 1e6, f_v_d / 1e6, kcr, kv),
            &format!("τ_d={:.3} MPa gegen f_v,d={:.3} MPa (k_cr={:.3}, k_v={:.3}).", tau / 1e6, f_v_d / 1e6, kcr, kv),
        ),
        SECTION_LEVERS,
    ));

    if n_t_ed > 0.0 {
        let f_t_d = km * props.f_t_0_k / gamma;
        let sigma_t = if a > 0.0 { n_t_ed / a } else { 0.0 };
        out.push(assessed(
            head("tension", "6.1.2", "actions", "Tension parallel", "Zug parallel"),
            Quantity::new(QuantityKind::Stress, sigma_t),
            Quantity::new(QuantityKind::Stress, f_t_d.max(1e-9)),
            loc(
                &format!("σ_t,0,d={:.3} MPa vs f_t,0,d={:.3} MPa.", sigma_t / 1e6, f_t_d / 1e6),
                &format!("σ_t,0,d={:.3} MPa gegen f_t,0,d={:.3} MPa.", sigma_t / 1e6, f_t_d / 1e6),
            ),
            WIDTH_LEVERS,
        ));
    }

    if n_ed > 0.0 {
        let iy = (i_m4(m) / a.max(1e-12)).sqrt();
        let iz = (m.h_m * m.b_m.powi(3) / 12.0 / a.max(1e-12)).sqrt();
        let lam_y = if iy > 0.0 { m.buckling_length_y_m / iy } else { 0.0 };
        let lam_z = if iz > 0.0 { m.buckling_length_z_m / iz } else { 0.0 };
        let kc_y = k_c_buckling(lambda_rel_compression(lam_y, props.f_c_0_k, props.e_0_05));
        let kc_z = k_c_buckling(lambda_rel_compression(lam_z, props.f_c_0_k, props.e_0_05));
        let kc = kc_y.min(kc_z);
        let f_c_d = km * props.f_c_0_k / gamma;
        let sigma_c = if a > 0.0 { n_ed / a } else { 0.0 };
        out.push(assessed(
            head("compression", "6.3.2", "actions", "Compression with buckling", "Druck mit Knicken"),
            Quantity::new(QuantityKind::Stress, sigma_c),
            Quantity::new(QuantityKind::Stress, (kc * f_c_d).max(1e-9)),
            loc(
                &format!("σ_c,0,d={:.3} MPa vs min(k_c,y,k_c,z)·f_c,0,d={:.3} MPa (k_c,y={:.3}, k_c,z={:.3}).", sigma_c / 1e6, kc * f_c_d / 1e6, kc_y, kc_z),
                &format!("σ_c,0,d={:.3} MPa gegen min(k_c,y,k_c,z)·f_c,0,d={:.3} MPa (k_c,y={:.3}, k_c,z={:.3}).", sigma_c / 1e6, kc * f_c_d / 1e6, kc_y, kc_z),
            ),
            SECTION_LEVERS,
        ));
        let util_c = if kc * f_c_d > 0.0 { sigma_c / (kc * f_c_d) } else { 0.0 };
        let util_m = if f_m_d > 0.0 { sigma_m / f_m_d } else { 0.0 };
        let combined = util_c.powi(2) + util_m;
        out.push(assessed(
            head("combined", "6.2.4", "actions", "Combined compression and bending", "Kombinierter Druck und Biegung"),
            Quantity::new(QuantityKind::Dimensionless, combined),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            loc(&format!("(σ_c/(k_c·f_c,d))² + σ_m/f_m,d = {combined:.3}."), &format!("(σ_c/(k_c·f_c,d))² + σ_m/f_m,d = {combined:.3}.")),
            SECTION_LEVERS,
        ));
    }

    if f_c90_ed > 0.0 && m.bearing_length_m > 0.0 {
        let kc90 = k_c_90(m.bearing_length_m, m.support_length_m);
        let f_c90_d = km * kc90 * props.f_c_90_k / gamma;
        let sigma90 = f_c90_ed / (m.b_m * m.bearing_length_m).max(1e-12);
        out.push(assessed(
            head("c90", "6.1.5", "bearingLengthM", "Compression perpendicular", "Druck quer zur Faser"),
            Quantity::new(QuantityKind::Stress, sigma90),
            Quantity::new(QuantityKind::Stress, f_c90_d.max(1e-9)),
            loc(
                &format!("σ_c,90,d={:.3} MPa vs k_c,90·f_c,90,d={:.3} MPa.", sigma90 / 1e6, f_c90_d / 1e6),
                &format!("σ_c,90,d={:.3} MPa gegen k_c,90·f_c,90,d={:.3} MPa.", sigma90 / 1e6, f_c90_d / 1e6),
            ),
            BEARING_LEVERS,
        ));
    }

    let sls_char = combos
        .iter()
        .filter(|c| c.kind == ComboKind::SlsCharacteristic)
        .max_by(|a, b| a.q_ed_line.abs().partial_cmp(&b.q_ed_line.abs()).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(sls) = sls_char {
        if m.span_m > 0.0 && (sls.q_ed_line.abs() > 0.0 || sls.f_ed_point.abs() > 0.0) {
            let w_inst = deflection_m(m, props, sls.q_ed_line, sls.f_ed_point);
            let w_fin = w_inst * (1.0 + k_def(sc) * sls.psi2);
            let lim_inst = m.span_m / params.w_inst_limit_divisor();
            let lim_fin = m.span_m / params.w_fin_limit_divisor();
            out.push(assessed(
                head("winst", "7.2", "actions", "Instantaneous deflection", "Sofortverformung"),
                Quantity::length_m(w_inst),
                Quantity::length_m(lim_inst),
                loc(
                    &format!("w_inst={:.1} mm vs L/{:.0}={:.1} mm.", w_inst * 1000.0, params.w_inst_limit_divisor(), lim_inst * 1000.0),
                    &format!("w_inst={:.1} mm gegen L/{:.0}={:.1} mm.", w_inst * 1000.0, params.w_inst_limit_divisor(), lim_inst * 1000.0),
                ),
                DEPTH_LEVERS,
            ));
            out.push(assessed(
                head("wfin", "7.2", "actions", "Final deflection", "Endverformung"),
                Quantity::length_m(w_fin),
                Quantity::length_m(lim_fin),
                loc(
                    &format!("w_fin=w_inst·(1+k_def·ψ₂)={:.1} mm vs L/{:.0}={:.1} mm.", w_fin * 1000.0, params.w_fin_limit_divisor(), lim_fin * 1000.0),
                    &format!("w_fin=w_inst·(1+k_def·ψ₂)={:.1} mm gegen L/{:.0}={:.1} mm.", w_fin * 1000.0, params.w_fin_limit_divisor(), lim_fin * 1000.0),
                ),
                DEPTH_LEVERS,
            ));
        }
    }

    if m.role == MemberRole::Floor {
        out.push(assess_floor(annex, params, m, props));
    }
    if m.fire_duration_s > 0.0 {
        out.push(assess_fire(annex, params, m, props, Some(uls)));
    }
    out
}

fn assess_floor(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties) -> Assessed<TimberMember> {
    let l = m.span_m.max(1e-6);
    let ei = props.e_0_mean * i_m4(m);
    let mu = if m.mass_kg_per_m > 0.0 {
        m.mass_kg_per_m
    } else if m.mass_kg_per_m2 > 0.0 {
        m.mass_kg_per_m2 * m.b_m.max(0.05)
    } else {
        props.rho_k * area_m2(m)
    };
    let support_factor = match m.support {
        SupportType::SimplySupported => 1.0,
        SupportType::Cantilever => 0.56,
        SupportType::ContinuousTwoSpan => 1.5,
    };
    let f1 = if mu > 0.0 && ei > 0.0 { support_factor * (std::f64::consts::PI / (2.0 * l * l)) * (ei / mu).sqrt() } else { 0.0 };
    let w_1kn = deflection_m(m, props, 0.0, 1000.0);
    let xi = if m.damping_xi > 0.0 { m.damping_xi } else { 0.01 };
    let m_star = mu * l / 2.0;
    let a_vert = if w_1kn > 0.0 && m_star > 0.0 && f1 > 0.0 { (1.0 / (m_star * xi.sqrt()).max(1e-9)) * (f1 / 8.0).max(0.1) * 0.25 } else { 0.0 };
    let lim = params.floor_a_limit();
    assessed(
        Head {
            id: format!("en1995.7.3.vibration.{}", m.id),
            part: "EN 1995-1-1",
            clause: ClauseId::new("EN 1995-1-1", "§7.3", "7.3"),
            subject: member_subject(m, "massKgPerM"),
            title: loc("Floor vibration", "Deckenschwingungen"),
            annex,
        },
        Quantity::acceleration_m_s2(a_vert),
        Quantity::acceleration_m_s2(lim),
        loc(
            &format!("f1={:.2} Hz, w(1kN)={:.2} mm, a_vert={:.3} m/s² vs limit {:.3} m/s².", f1, w_1kn * 1000.0, a_vert, lim),
            &format!("f1={:.2} Hz, w(1kN)={:.2} mm, a_vert={:.3} m/s² gegen Grenzwert {:.3} m/s².", f1, w_1kn * 1000.0, a_vert, lim),
        ),
        MASS_LEVERS,
    )
}

fn assess_fire(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties, uls: Option<&LoadCombo>) -> Assessed<TimberMember> {
    let m_ed = uls.map(|u| u.m_ed_nm).unwrap_or_else(|| enumerate_combos(m).into_iter().filter(|c| c.kind == ComboKind::Uls).map(|c| c.m_ed_nm.abs()).fold(0.0_f64, f64::max));
    let deff = d_ef(m.fire_duration_s, params.beta_n(props.product));
    let (b_fi, h_fi) = residual_bh(m.b_m, m.h_m, deff);
    let w_fi = b_fi * h_fi.powi(2) / 6.0;
    let k_fi = params.k_fi(props.product);
    let k_mod_fi = params.k_mod_fi();
    let gamma_m_fi = params.gamma_m_fi();
    let m_rd_fi = k_mod_fi * k_fi * props.f_m_k / gamma_m_fi * w_fi;
    assessed(
        Head {
            id: format!("en1995.1-2.4.fire.{}", m.id),
            part: "EN 1995-1-2",
            clause: ClauseId::new("EN 1995-1-2", "§4.2", "4.2"),
            subject: member_subject(m, "hM"),
            title: loc("Fire residual bending", "Brand Restbiegetragfähigkeit"),
            annex,
        },
        Quantity::new(QuantityKind::Moment, m_ed),
        Quantity::new(QuantityKind::Moment, m_rd_fi.max(1e-9)),
        loc(
            &format!("d_ef={:.1} mm (β_n), k_fi={:.2}, k_mod,fi={:.2}, γ_M,fi={:.2}; M_Ed={:.1} kNm vs M_Rd,fi={:.1} kNm.", deff * 1000.0, k_fi, k_mod_fi, gamma_m_fi, m_ed / 1000.0, m_rd_fi / 1000.0),
            &format!("d_ef={:.1} mm (β_n), k_fi={:.2}, k_mod,fi={:.2}, γ_M,fi={:.2}; M_Ed={:.1} kNm gegen M_Rd,fi={:.1} kNm.", deff * 1000.0, k_fi, k_mod_fi, gamma_m_fi, m_ed / 1000.0, m_rd_fi / 1000.0),
        ),
        SECTION_LEVERS,
    )
}

fn crowd_line_n_per_m(m: &TimberMember) -> f64 {
    m.bridge_crowd_per_m2.max(0.0) * 800.0 * m.b_m.max(0.05)
}

fn assess_bridge_member(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties) -> Vec<Assessed<TimberMember>> {
    let head = |id: String, clause: ClauseId, leaf: &str, en: &str, de: &str| Head { id, part: "EN 1995-2", clause, subject: member_subject(m, leaf), title: loc(en, de), annex };
    let km = k_mod(parse_service_class(m.service_class), LoadDuration::Medium);
    let gamma = params.gamma_m(props.product);
    let kh = k_h(m.h_m, props.product);
    let wsec = w_m3(m);
    let q_crowd = crowd_line_n_per_m(m);
    let moment_coefficient = match m.support {
        SupportType::SimplySupported => 1.0 / 8.0,
        SupportType::Cantilever => 1.0 / 2.0,
        SupportType::ContinuousTwoSpan => 1.0 / 14.0,
    };
    let m_crowd = q_crowd * m.span_m.powi(2) * moment_coefficient;
    let m_ed = 1.5 * m_crowd;
    let f_m_d = km * kh * k_crit(lambda_rel_m(wsec, props.f_m_k, m.m_crit_nm.max(1.0))) * props.f_m_k / gamma;
    let sigma_m = if wsec > 0.0 { m_ed / wsec } else { 0.0 };

    let m_fat = 0.4 * m_crowd;
    let delta_sigma = if wsec > 0.0 { m_fat / wsec } else { 0.0 };
    let delta_mpa = (delta_sigma / 1e6).max(1e-6);
    let n = (m.bridge_n_obs * m.bridge_t_l_years).max(1.0);
    let a_w = if m.bridge_a > 0.0 { m.bridge_a } else { 15.0 };
    let b_w = if m.bridge_b > 0.0 { m.bridge_b } else { 4.0 };
    let beta = if m.bridge_beta > 0.0 { m.bridge_beta } else { 5.0 };
    let n_r = 10.0_f64.powf(a_w - b_w * delta_mpa.log10()).max(1.0);
    let k_fat = (n_r / n).powf(1.0 / beta).min(1.0);
    let f_fat_d = km * k_fat * props.f_m_k / gamma;
    let mut out = vec![assessed(
        head(format!("en1995.2.a.fatigue.{}", m.id), ClauseId::new("EN 1995-2", "Annex A", "A"), "bridgeNObs", "Bridge fatigue (Annex A)", "Brückenermüdung (Anhang A)"),
        Quantity::new(QuantityKind::Stress, delta_sigma),
        Quantity::new(QuantityKind::Stress, f_fat_d.max(1e-9)),
        loc(
            &format!("Δσ={:.3} MPa, N={:.0}, N_R={:.0}, k_fat={:.3}; σ vs f_fat,d={:.3} MPa.", delta_mpa, n, n_r, k_fat, f_fat_d / 1e6),
            &format!("Δσ={:.3} MPa, N={:.0}, N_R={:.0}, k_fat={:.3}; σ gegen f_fat,d={:.3} MPa.", delta_mpa, n, n_r, k_fat, f_fat_d / 1e6),
        ),
        DEPTH_LEVERS,
    )];

    let ei = props.e_0_mean * i_m4(m);
    let mu = if m.mass_kg_per_m > 0.0 { m.mass_kg_per_m } else { props.rho_k * area_m2(m) };
    let f1 = if mu > 0.0 && ei > 0.0 && m.span_m > 0.0 { (std::f64::consts::PI / (2.0 * m.span_m.powi(2))) * (ei / mu).sqrt() } else { 1.0 };
    let xi = if m.damping_xi > 0.0 { m.damping_xi } else { 0.01 };
    let a_vert = (m.bridge_crowd_per_m2.max(0.0) / 1.5) * (5.0 / f1.max(0.5)) * (0.01 / xi).sqrt() * 0.35;
    let a_hor = 0.3 * a_vert;
    let (lim_v, lim_h) = (params.bridge_a_vert_limit(), params.bridge_a_hor_limit());
    out.push(assessed(
        head(format!("en1995.2.b.avert.{}", m.id), ClauseId::new("EN 1995-2", "Annex B", "B"), "bridgeCrowdPerM2", "Bridge pedestrian a_vert", "Brücke Fußgänger a_vert"),
        Quantity::acceleration_m_s2(a_vert),
        Quantity::acceleration_m_s2(lim_v),
        loc(&format!("a_vert={a_vert:.3} m/s² vs ≤ {lim_v:.3} m/s² (f1={f1:.2} Hz)."), &format!("a_vert={a_vert:.3} m/s² gegen ≤ {lim_v:.3} m/s² (f1={f1:.2} Hz).")),
        DEPTH_LEVERS,
    ));
    out.push(assessed(
        head(format!("en1995.2.b.ahor.{}", m.id), ClauseId::new("EN 1995-2", "Annex B", "B"), "bridgeCrowdPerM2", "Bridge pedestrian a_hor", "Brücke Fußgänger a_hor"),
        Quantity::acceleration_m_s2(a_hor),
        Quantity::acceleration_m_s2(lim_h),
        loc(&format!("a_hor={a_hor:.3} m/s² vs ≤ {lim_h:.3} m/s²."), &format!("a_hor={a_hor:.3} m/s² gegen ≤ {lim_h:.3} m/s².")),
        DEPTH_LEVERS,
    ));
    out.push(assessed(
        head(format!("en1995.2.uls.bending.{}", m.id), ClauseId::new("EN 1995-2", "§5", "5"), "bridgeCrowdPerM2", "Bridge ULS bending", "Brücke GZT Biegung"),
        Quantity::new(QuantityKind::Stress, sigma_m),
        Quantity::new(QuantityKind::Stress, f_m_d.max(1e-9)),
        loc(
            &format!("Crowd LM ULS σ_m={:.3} MPa vs f_m,d={:.3} MPa.", sigma_m / 1e6, f_m_d / 1e6),
            &format!("Personen-LM GZT σ_m={:.3} MPa gegen f_m,d={:.3} MPa.", sigma_m / 1e6, f_m_d / 1e6),
        ),
        DEPTH_LEVERS,
    ));
    if m.span_m > 0.0 {
        let w = deflection_m(m, props, q_crowd, 0.0);
        let lim = m.span_m / 400.0;
        out.push(assessed(
            head(format!("en1995.2.sls.deflection.{}", m.id), ClauseId::new("EN 1995-2", "§7", "7"), "bridgeCrowdPerM2", "Bridge SLS deflection", "Brücke GZG Durchbiegung"),
            Quantity::length_m(w),
            Quantity::length_m(lim),
            loc(&format!("w={:.1} mm vs L/400={:.1} mm.", w * 1000.0, lim * 1000.0), &format!("w={:.1} mm gegen L/400={:.1} mm.", w * 1000.0, lim * 1000.0)),
            DEPTH_LEVERS,
        ));
    }
    out
}

fn evaluate_connection(annex: AnnexChoice, params: &AnnexParams, c: &TimberConnection) -> Vec<CheckResult> {
    let Some(props) = properties_for_class(&c.strength_class) else {
        return vec![CheckResult::assess(
            format!("en1995.conn.material.{}", c.id),
            "EN 1995-1-1",
            ClauseId::new("EN 1995-1-1", "§8", "8"),
            conn_subject(c, "strengthClass"),
            loc("Connection timber class", "Holzklasse der Verbindung"),
        )
        .not_applicable(loc("Unknown timber class for connection.", "Unbekannte Holzklasse für Verbindung."))
        .annex(annex)
        .build()];
    };
    let probe = |candidate: &TimberConnection| assess_connection(annex, params, candidate, &props).into_iter().map(|a| (a.id, a.utilization)).collect();
    finish(c, assess_connection(annex, params, c, &props), conn_subject, &probe)
}

fn assess_connection(annex: AnnexChoice, params: &AnnexParams, c: &TimberConnection, props: &TimberProperties) -> Vec<Assessed<TimberConnection>> {
    let combos = enumerate_connection_combos(c);
    let Some((_, _, duration, f_ed)) = combos
        .iter()
        .filter(|(_, k, _, _)| *k == ComboKind::Uls)
        .max_by(|a, b| a.3.abs().partial_cmp(&b.3.abs()).unwrap_or(std::cmp::Ordering::Equal))
        .cloned()
    else {
        return Vec::new();
    };
    let head = |id: String, clause: &str, leaf: &str, en: &str, de: &str| Head {
        id,
        part: "EN 1995-1-1",
        clause: ClauseId::new("EN 1995-1-1", &format!("§{clause}"), clause),
        subject: conn_subject(c, leaf),
        title: loc(en, de),
        annex,
    };
    let km = k_mod(parse_service_class(c.service_class), duration);
    let gamma = params.gamma_m_connection();
    let fh = f_h_k(props.rho_k, c.diameter_m, &c.fastener_type);
    let my = m_y_k(c.f_u_k, c.diameter_m);
    let fax = if c.fastener_type.to_ascii_lowercase().contains("screw") { 0.2 * fh * c.diameter_m * c.t1_m } else { 0.0 };
    let f_v_rk_1 = if c.steel_plate {
        johansen_steel_timber_f_v_rk(c.t1_m, c.steel_plate_thickness_m, c.diameter_m, fh, my, fax)
    } else if c.shear_planes >= 2 {
        johansen_double_shear_f_v_rk(c.t1_m, c.t2_m, c.diameter_m, fh, fh, my, fax)
    } else {
        johansen_single_shear_f_v_rk(c.t1_m, c.t2_m, c.diameter_m, fh, fh, my, fax)
    };
    let nef = n_ef(c.number, c.spacing_m, c.diameter_m);
    let planes = if c.steel_plate {
        1.0
    } else if c.shear_planes >= 2 {
        (c.shear_planes as f64) / 2.0
    } else {
        c.shear_planes.max(1) as f64
    };
    let f_v_rd = km * nef * planes * f_v_rk_1 / gamma;
    let clause = if c.steel_plate { "8.2.3" } else { "8.2.2" };
    let mut out = vec![assessed(
        head(format!("en1995.{clause}.johansen.{}", c.id), clause, "actions", "Dowel-type connection (Johansen EYM)", "Stiftförmige Verbindung (Johansen)"),
        Quantity::force_kn(f_ed / 1000.0),
        Quantity::force_kn(f_v_rd / 1000.0),
        loc(
            &format!("F_Ed={:.2} kN vs F_v,Rd={:.2} kN (F_v,Rk={:.2} kN, n_ef={:.2}, planes={:.1}).", f_ed / 1000.0, f_v_rd / 1000.0, f_v_rk_1 / 1000.0, nef, planes),
            &format!("F_Ed={:.2} kN gegen F_v,Rd={:.2} kN (F_v,Rk={:.2} kN, n_ef={:.2}, Ebenen={:.1}).", f_ed / 1000.0, f_v_rd / 1000.0, f_v_rk_1 / 1000.0, nef, planes),
        ),
        JOHANSEN_LEVERS,
    )];
    let (a1_min, a3_min, a4_min) = spacing_minima(c);
    for (kind, leaf, minimum, provided, en, de, levers) in [
        ("a1", "spacingM", a1_min, c.spacing_m, "Fastener spacing a₁", "Verbindungsmittelabstand a₁", SPACING_LEVERS),
        ("a3t", "endDistanceM", a3_min, c.end_distance_m, "End distance a₃,t", "Hirnholzabstand a₃,t", END_LEVERS),
        ("a4t", "edgeDistanceM", a4_min, c.edge_distance_m, "Edge distance a₄,t", "Randabstand a₄,t", EDGE_LEVERS),
    ] {
        out.push(assessed(
            head(format!("en1995.8.spacing.{kind}.{}", c.id), "8.3", leaf, en, de),
            Quantity::length_m(minimum),
            Quantity::length_m(provided.max(1e-12)),
            loc(
                &format!("Required ≥ {:.1} mm, provided {:.1} mm (Tables 8.2–8.5, α = 0).", minimum * 1000.0, provided * 1000.0),
                &format!("Erforderlich ≥ {:.1} mm, vorhanden {:.1} mm (Tabellen 8.2–8.5, α = 0).", minimum * 1000.0, provided * 1000.0),
            ),
            levers,
        ));
    }
    out
}
