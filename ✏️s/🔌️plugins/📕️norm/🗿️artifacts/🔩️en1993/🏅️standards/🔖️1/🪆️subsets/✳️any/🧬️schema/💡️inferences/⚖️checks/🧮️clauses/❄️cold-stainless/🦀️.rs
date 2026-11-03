//! ❄️ EN 1993-1-3 cold-formed and EN 1993-1-4 stainless steel (mm, N, MPa). See [EN 1993-1-3](https://eurocodes.jrc.ec.europa.eu/) and [EN 1993-1-4](https://eurocodes.jrc.ec.europa.eu/).

/// ❄️ Average yield strength after cold work f_ya = f_yb + (f_u − f_yb) k n t²/A_g, capped at (f_u+f_yb)/2.
pub fn f_ya(fyb: f64, fu: f64, k: f64, n_bends: f64, t: f64, a_g: f64) -> f64 {
    (fyb + (fu - fyb) * k * n_bends * t * t / a_g).min(0.5 * (fu + fyb))
}

/// ❄️ Elastic critical stress of an edge stiffener on elastic spring K: σ_cr,s = 2 √(K E I_s)/A_s.
pub fn sigma_cr_stiffener(k_spring: f64, e: f64, i_s: f64, a_s: f64) -> f64 {
    2.0 * (k_spring * e * i_s).sqrt() / a_s
}

/// ❄️ Spring stiffness per unit length of a flange edge stiffener, K = E t³/(4(1−ν²)) · 1/(b1² h_w + b1³ + 0.5 b1 b2 h_w k_f).
pub fn stiffener_spring(e: f64, t: f64, b1: f64, b2: f64, hw: f64, kf: f64) -> f64 {
    e * t.powi(3) / (4.0 * (1.0 - semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.poisson").powi(2))) / (b1 * b1 * hw + b1.powi(3) + 0.5 * b1 * b2 * hw * kf)
}

/// ❄️ Distortional buckling reduction χ_d (§5.5.3.2(3)).
pub fn chi_distortional(lambda_d: f64) -> f64 {
    if lambda_d <= 0.65 { 1.0 } else if lambda_d < 1.38 { 1.47 - 0.723 * lambda_d } else { 0.66 / lambda_d }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StainlessGrade {
    pub name: &'static str,
    pub fy: f64,
    pub fu: f64,
    pub e: f64,
    pub n: f64,
}

/// ✨ Stainless grades of EN 1993-1-4 Table 2.1 as a typed enum; the properties are read from `ndp.en1993.1-4.grade.<name>.<fy|fu|e|n>` (unverified).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StainlessGradeId {
    G1_4301,
    G1_4401,
    G1_4404,
    G1_4571,
    G1_4462,
    G1_4003,
}

impl StainlessGradeId {
    pub const ALL: [Self; 6] = [Self::G1_4301, Self::G1_4401, Self::G1_4404, Self::G1_4571, Self::G1_4462, Self::G1_4003];

    /// ✨ The material number as written on the grade, e.g. `1.4301`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::G1_4301 => "1.4301",
            Self::G1_4401 => "1.4401",
            Self::G1_4404 => "1.4404",
            Self::G1_4571 => "1.4571",
            Self::G1_4462 => "1.4462",
            Self::G1_4003 => "1.4003",
        }
    }

    /// ✨ The grade of a material number; unknown numbers are no grade.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|g| g.name() == name)
    }
}

/// ✨ Grade by name with its properties read from the registry.
pub fn stainless_grade(name: &str) -> Option<StainlessGrade> {
    let id = StainlessGradeId::from_name(name)?;
    let name = id.name();
    let key = name.replace('.', "-");
    let read = |symbol: &str| semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-4.grade.{key}.{symbol}"));
    Some(StainlessGrade { name, fy: read("fy"), fu: read("fu"), e: read("e"), n: read("n") })
}

/// ✨ ε = [(235/fy)(E/210000)]^½.
pub fn stainless_eps(fy: f64, e: f64) -> f64 {
    ((235.0 / fy) * (e / crate::stability::e_steel())).sqrt()
}

/// ✨ Ramberg–Osgood strain ε(σ) = σ/E + 0.002 (σ/f_y)^n.
pub fn ramberg_osgood_strain(sigma: f64, e: f64, fy: f64, n: f64) -> f64 {
    sigma / e + 0.002 * (sigma / fy).powf(n)
}

/// ✨ Tangent modulus E_t = E/(1 + 0.002 n E/f_y (σ/f_y)^(n−1)).
pub fn ramberg_osgood_tangent(sigma: f64, e: f64, fy: f64, n: f64) -> f64 {
    e / (1.0 + 0.002 * n * e / fy * (sigma / fy).powf(n - 1.0))
}

/// ✨ Flexural buckling (α, λ0) of Table 5.2: cold-formed open and hollow α = 0.49, λ0 = 0.40; welded open α = 0.49, λ0 = 0.20.
pub fn stainless_buckling_params(welded_open: bool) -> (f64, f64) {
    let kind = if welded_open { "welded-open" } else { "other" };
    (semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-4.flexural.{kind}.alpha")), semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-4.flexural.{kind}.lambda0")))
}

pub fn stainless_chi(lambda: f64, welded_open: bool) -> f64 {
    let (a, l0) = stainless_buckling_params(welded_open);
    if lambda <= l0 { 1.0 } else { crate::stability::chi_general(lambda, a, l0, 1.0) }
}
