//! 🌲️ Material and annex factors of the EN 1995 timber checks: the strength class properties, the annex parameters and the modification, creep, size, buckling and shear factors. See [`super`].

use super::*;

/// 📋 Tabulated characteristic properties for EN 338 / EN 14080 / LVL / CLT classes (see [`strength_row`]).
pub fn properties_for_class(class: &str) -> Option<TimberProperties> {
    strength_row(class).map(|r| row_properties(&r))
}

/// 📜 Strength-class pick list for remedies.
pub fn strength_class_options() -> Vec<&'static str> {
    strength_class_names()
}

/// 🌍️ National-annex parameters for EN 1995 checks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub annex: AnnexChoice,
    pub uls_ab: bool,
    pub factors: semio_s_plugin_norm_compliance::ActionFactors,
}

impl AnnexParams {
    /// 🛡️ The parameters of an annex and the reliability class of the document: K_FI of Table B3 scales γ_G,sup and γ_Q of every ULS combination.
    pub fn new(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Self {
        Self { annex, uls_ab: false, factors: semio_s_plugin_norm_compliance::ActionFactors::new(annex, reliability_class, semio_s_plugin_norm_compliance::DesignSituation::Persistent) }
    }
    /// 🔥 Design charring rate β_n (m/s) of an annex: independent of the reliability class.
    pub fn charring_rate(annex: AnnexChoice, product: TimberProduct) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1995.beta-n.{}", product_key(product)), annex) * 1e-3 / 60.0
    }
    /// 🧮 Selects the ULS equation of the document: `Eq610ab` enables 6.10a and 6.10b (ξ = 0.85).
    pub fn with_uls_equation(mut self, equation: semio_s_artifact_norm_en1995_model::UlsEquation) -> Self {
        self.uls_ab = equation == semio_s_artifact_norm_en1995_model::UlsEquation::Eq610ab;
        self
    }
    pub fn en() -> Self {
        Self::new(AnnexChoice::En, semio_s_plugin_norm_compliance::ReliabilityClass::RC2)
    }
    pub fn de() -> Self {
        Self::new(AnnexChoice::De, semio_s_plugin_norm_compliance::ReliabilityClass::RC2)
    }
    /// 🛡️ γ_M of EN 1995-1-1 Table 2.3 and DIN EN 1995-1-1/NA per product (see [`gamma_m_for`]).
    pub fn gamma_m(&self, product: TimberProduct) -> f64 {
        gamma_m_for(self.annex, product_key(product), false).unwrap_or(f64::NAN)
    }
    pub fn gamma_m_connection(&self) -> f64 {
        gamma_m_for(self.annex, "connection", false).unwrap_or(f64::NAN)
    }
    /// 🪵 k_cr of EN 1995-1-1 6.1.7 (2): 0.67 (EN) or 2.5/f_v,k (NA-DE) for solid timber and glulam, 1.0 for every other product.
    /// 🪵 k_cr of EN 1995-1-1 6.1.7 (2) (see [`k_cr_value`]).
    pub fn k_cr(&self, product: TimberProduct, f_v_k_pa: f64) -> f64 {
        k_cr_value(self.annex, product, f_v_k_pa)
    }
    pub fn w_inst_limit_divisor(&self) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number("ndp.en1995.w-inst-divisor", self.annex)
    }
    pub fn w_fin_limit_divisor(&self) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number("ndp.en1995.w-fin-divisor", self.annex)
    }
    /// 🔥 EN 1995-1-2 §2.3 — k_fi for 20 % fractile conversion (Table 2.1), read from the shared registry.
    pub fn k_fi(&self, product: TimberProduct) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1995.k-fi.{}", product_key(product)), self.annex)
    }
    pub fn k_mod_fi(&self) -> f64 {
        1.0
    }
    pub fn gamma_m_fi(&self) -> f64 {
        1.0
    }
    /// 🔥 Design charring rate β_n (m/s) — Table 3.1 softwood / glulam / LVL, read from the shared registry.
    pub fn beta_n(&self, product: TimberProduct) -> f64 {
        Self::charring_rate(self.annex, product)
    }
    pub fn beta_0(&self, _product: TimberProduct) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number("ndp.en1995.beta-0", self.annex) * 1e-3 / 60.0
    }
}

/// 🌧 Service class SC1–SC3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceClass {
    Sc1,
    Sc2,
    Sc3,
}

pub fn parse_service_class(v: u8) -> ServiceClass {
    match v {
        2 => ServiceClass::Sc2,
        3 => ServiceClass::Sc3,
        _ => ServiceClass::Sc1,
    }
}

/// 📉️ ψ₀/ψ₁/ψ₂ of an action from the registry rows `ndp.en1990.psi.*` of the annex of `factors`: snow by kind and category (above 1000 m a.s.l. the row `snow_high`), wind by kind, imposed actions by category letter A–H, any other kind or an unknown category follows offices; permanent and accidental actions carry no ψ (1, 1, 1).
pub fn psi_factors(factors: &semio_s_plugin_norm_compliance::ActionFactors, kind: semio_s_artifact_norm_en1995_model::ActionKind, category: semio_s_artifact_norm_en1995_model::ImposedCategory) -> (f64, f64, f64) {
    use semio_s_artifact_norm_en1995_model::{ActionKind as K, ImposedCategory as C};
    let key = match kind {
        K::Permanent | K::Accidental => return semio_s_plugin_norm_compliance::action_factors::PsiRow::UNITY.as_tuple(),
        K::Snow => match category {
            C::SnowHigh => "snow_high",
            _ => "snow",
        },
        K::Wind => "wind",
        K::Imposed => match category {
            C::NotImposed | C::SnowHigh => "office",
            C::A => "A",
            C::B => "B",
            C::C => "C",
            C::D => "D",
            C::E => "E",
            C::F => "F",
            C::G => "G",
            C::H => "H",
        },
    };
    factors.psi_known(key).or_else(|| factors.psi_known("office")).map_or((1.0, 1.0, 1.0), semio_s_plugin_norm_compliance::action_factors::PsiRow::as_tuple)
}

pub fn k_mod(service: ServiceClass, duration: LoadDuration) -> f64 {
    k_mod_for("solid", match service { ServiceClass::Sc1 => 1, ServiceClass::Sc2 => 2, ServiceClass::Sc3 => 3 }, duration).unwrap_or(0.5)
}

pub fn k_def(service: ServiceClass, product: TimberProduct) -> f64 {
    let sc = match service {
        ServiceClass::Sc1 => 1,
        ServiceClass::Sc2 => 2,
        ServiceClass::Sc3 => 3,
    };
    k_def_for(product_key(product), sc).unwrap_or(2.0)
}

pub fn k_h(h_m: f64, product: TimberProduct, rho_k: f64) -> f64 {
    size_factor_kh(product, rho_k, h_m * 1000.0, 0.12)
}

pub fn k_crit(lambda_rel_m: f64) -> f64 {
    kcrit_factor(lambda_rel_m)
}

pub fn lambda_rel_m(w_m3: f64, f_m_k: f64, m_crit_nm: f64) -> f64 {
    if m_crit_nm <= 0.0 || w_m3 <= 0.0 || f_m_k <= 0.0 {
        0.0
    } else {
        (w_m3 * f_m_k / m_crit_nm).sqrt()
    }
}

/// 🪢 Critical moment M_crit in N·m computed as σ_m,crit·W_y from the torsional-flexural eigenvalue of EN 1995-1-1 6.31 with l_ef = restraint spacing or the span.
pub fn effective_m_crit_nm(m: &TimberMember, e_0_05: f64) -> f64 {
    let span = m.span_m.max(1e-6);
    let lef = if m.lateral_restraint_spacing_m > 0.0 { m.lateral_restraint_spacing_m.min(span) } else { span };
    sigma_m_crit_exact(m.b_m * 1e3, m.h_m * 1e3, lef * 1e3, e_0_05 / 1e6) * 1e6 * w_m3(m)
}

pub fn lambda_rel_compression(lambda: f64, f_c_0_k: f64, e_0_05: f64) -> f64 {
    if e_0_05 <= 0.0 {
        0.0
    } else {
        lambda / std::f64::consts::PI * (f_c_0_k / e_0_05).sqrt()
    }
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
