//! 📐️ Stability checks of EN 1995-1-1 6.3 and 5.4.4 as catalogue entries T-20 … T-24: column buckling, lateral-torsional buckling, biaxial interaction and second-order bow imperfection. Units: N, mm, N/mm². See [`super`].

use super::*;
use std::f64::consts::PI;

/// 🧮 Relative slenderness λ_rel = λ/π·√(f_c,0,k/E_0,05) (EN 1995-1-1 6.21).
pub fn lambda_rel_c(lef: f64, i: f64, fc0k: f64, e005: f64) -> f64 {
    lef / i / PI * (fc0k / e005).sqrt()
}

/// 🧮 Column buckling factor k_c of EN 1995-1-1 6.25 – 6.28 with imperfection factor β_c.
pub fn kc_factor(lambda_rel: f64, beta_c: f64) -> f64 {
    if lambda_rel <= 0.3 {
        return 1.0;
    }
    let k = 0.5 * (1.0 + beta_c * (lambda_rel - 0.3) + lambda_rel * lambda_rel);
    1.0 / (k + (k * k - lambda_rel * lambda_rel).max(0.0).sqrt())
}

/// 🧮 Lateral buckling factor k_crit of EN 1995-1-1 6.34.
pub fn kcrit_factor(lambda_rel_m: f64) -> f64 {
    if lambda_rel_m <= 0.75 {
        1.0
    } else if lambda_rel_m <= 1.4 {
        1.56 - 0.75 * lambda_rel_m
    } else {
        1.0 / (lambda_rel_m * lambda_rel_m)
    }
}

fn beta_c(c: &Case, m: &Mat) -> f64 {
    c.opt("beta_c").unwrap_or(if m.row.product == TimberProduct::Solid { 0.2 } else { 0.1 })
}

struct Col {
    kcy: f64,
    kcz: f64,
    ly: f64,
    lz: f64,
    ry: f64,
    rz: f64,
}

fn column(c: &Case, m: &Mat, s: &Sec) -> Result<Col, String> {
    let (ly, lz) = (c.n("lef_y")? / (s.iy / s.a).sqrt(), c.n("lef_z")? / (s.iz / s.a).sqrt());
    let bc = beta_c(c, m);
    let ry = lambda_rel_c(c.n("lef_y")?, (s.iy / s.a).sqrt(), m.row.f_c_0_k, m.row.e_0_05);
    let rz = lambda_rel_c(c.n("lef_z")?, (s.iz / s.a).sqrt(), m.row.f_c_0_k, m.row.e_0_05);
    Ok(Col { kcy: kc_factor(ry, bc), kcz: kc_factor(rz, bc), ly, lz, ry, rz })
}

/// 🧭 One buckling axis of a column: the effective length in mm, the relative slenderness λ_rel (6.21) and the factor k_c (6.25 – 6.28).
#[derive(Clone, Copy, Debug)]
pub struct BucklingAxis {
    pub lef: f64,
    pub lambda_rel: f64,
    pub kc: f64,
}

/// 🧭 What the buckling diagram of T-20, T-21 and T-23 needs: the imperfection factor β_c and both axes.
#[derive(Clone, Copy, Debug)]
pub struct ColumnProbe {
    pub beta_c: f64,
    pub y: BucklingAxis,
    pub z: BucklingAxis,
}

/// 🧭 The [`ColumnProbe`] of a T-20 case.
pub fn column_probe(c: &Case) -> Result<ColumnProbe, String> {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let col = column(c, &m, &s)?;
    Ok(ColumnProbe { beta_c: beta_c(c, &m), y: BucklingAxis { lef: c.n("lef_y")?, lambda_rel: col.ry, kc: col.kcy }, z: BucklingAxis { lef: c.n("lef_z")?, lambda_rel: col.rz, kc: col.kcz } })
}

fn t20(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let col = column(c, &m, &s)?;
    let nc = c.n("n_c")?;
    let km = s.km(c);
    let (sy, sz) = (c.o("m_y", 0.0) / s.wy, c.o("m_z", 0.0) / s.wz);
    let fc = m.fd(m.row.f_c_0_k);
    let fmy = m.kh_bending(c, c.opt("kh_dim").unwrap_or(s.h)) * m.fd(m.row.f_m_k);
    let fmz = m.kh_bending(c, c.opt("kh_dim_z").unwrap_or(s.b)) * m.fd(m.row.f_m_k);
    let sc = nc / s.a;
    let (u1, u2, tag) = if col.ry <= 0.3 && col.rz <= 0.3 {
        ((sc / fc).powi(2) + sy / fmy + km * sz / fmz, (sc / fc).powi(2) + km * sy / fmy + sz / fmz, "6.19/6.20")
    } else {
        (sc / (col.kcy * fc) + sy / fmy + km * sz / fmz, sc / (col.kcz * fc) + km * sy / fmy + sz / fmz, "6.23/6.24")
    };
    let lmax = c.o("lambda_max", 150.0);
    Ok(vec![
        Outcome::ratio("buckling_y", "Column buckling about y", "Knicken um y", u1, 1.0, format!("[{tag}] λ_rel,y={} k_c,y={} → {}", f(col.ry), f(col.kcy), f(u1))),
        Outcome::ratio("buckling_z", "Column buckling about z", "Knicken um z", u2, 1.0, format!("[{tag}] λ_rel,z={} k_c,z={} → {}", f(col.rz), f(col.kcz), f(u2))),
        Outcome::ratio("slenderness", "Slenderness limit", "Schlankheitsgrenze", col.ly.max(col.lz), lmax, format!("λ_y={} λ_z={} ≤ {}", f(col.ly), f(col.lz), f(lmax))),
    ])
}

fn t21(c: &Case) -> R {
    let mut out = t20(c)?;
    let (y, z) = (out[0].demand, out[1].demand);
    out.push(Outcome::info("governing", "Governing buckling axis (0 = y, 1 = z)", "Maßgebende Knickachse (0 = y, 1 = z)", QuantityKind::Dimensionless, if y >= z { 0.0 } else { 1.0 }, format!("η_y={} η_z={}", f(y), f(z))));
    Ok(out)
}

/// 📏 Effective length of Table 6.1 as a function of span, load case, position of the load and depth.
pub fn lef_ltb(l: f64, h: f64, load_case: &str, load_pos: &str) -> f64 {
    let ratio = match load_case {
        "udl" => 0.9,
        "point_mid" => 0.8,
        "cantilever_udl" => 0.5,
        "cantilever_point" => 0.8,
        _ => 1.0,
    };
    let shift = match load_pos {
        "compression" => -0.5 * h,
        "tension" => 2.0 * h,
        _ => 0.0,
    };
    (ratio * l + shift).max(1.0)
}

/// 🧮 Critical bending stress σ_m,crit: simplified rectangular softwood form 0.78·b²·E_0,05/(h·l_ef) (EN 1995-1-1 6.32).
pub fn sigma_m_crit_simplified(b: f64, h: f64, lef: f64, e005: f64) -> f64 {
    0.78 * b * b * e005 / (h * lef)
}

/// 🧮 Exact σ_m,crit = π·√(E_0,05·I_z·G_0,05·I_tor)/(l_ef·W_y) (EN 1995-1-1 6.31) with G_0,05 = E_0,05/16.
pub fn sigma_m_crit_exact(b: f64, h: f64, lef: f64, e005: f64) -> f64 {
    let iz = h * b.powi(3) / 12.0;
    let itor = h * b.powi(3) * (1.0 / 3.0 - 0.21 * b / h * (1.0 - b.powi(4) / (12.0 * h.powi(4))));
    PI * (e005 * iz * (e005 / 16.0) * itor).sqrt() / (lef * (b * h * h / 6.0))
}

/// 🧭 What the lateral buckling diagram of T-22 needs: the effective length in mm, whether the compression edge is restrained, the section and the strengths; [`LtbProbe::at`] gives λ_rel,m and k_crit at another effective length.
#[derive(Clone, Copy, Debug)]
pub struct LtbProbe {
    pub lef: f64,
    pub restrained: bool,
    pub b: f64,
    pub h: f64,
    pub e_0_05: f64,
    pub f_m_k: f64,
}

impl LtbProbe {
    /// 🧭 λ_rel,m of 6.30 with the simplified σ_m,crit and k_crit of 6.34 at the effective length `lef`.
    pub fn at(&self, lef: f64) -> (f64, f64) {
        let rel = (self.f_m_k / sigma_m_crit_simplified(self.b, self.h, lef, self.e_0_05)).sqrt();
        (rel, if self.restrained { 1.0 } else { kcrit_factor(rel) })
    }
}

/// 🧭 The [`LtbProbe`] of a T-22 case; the effective length is the input `lef` or Table 6.1 from the span `l`.
pub fn ltb_probe(c: &Case) -> Result<LtbProbe, String> {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let lef = c.opt("lef").unwrap_or_else(|| lef_ltb(c.n("l").unwrap_or(0.0), s.h, c.so("load_case", "udl"), c.so("load_pos", "centroid")));
    if lef <= 1.0 && c.opt("lef").is_none() {
        return Err("missing input l or lef".into());
    }
    Ok(LtbProbe { lef, restrained: c.o("restrained", 0.0) > 0.5, b: s.b, h: s.h, e_0_05: m.row.e_0_05, f_m_k: m.row.f_m_k })
}

fn t22(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let p = ltb_probe(c)?;
    let lef = p.lef;
    let sc = sigma_m_crit_simplified(s.b, s.h, lef, m.row.e_0_05);
    let (rel, kcrit) = p.at(lef);
    let sm = c.n("m_y")? / s.wy;
    let fmd = m.kh_bending(c, c.opt("kh_dim").unwrap_or(s.h)) * m.fd(m.row.f_m_k);
    let mut out = vec![
        Outcome::stress("ltb", "Lateral-torsional buckling (6.33)", "Kippen (6.33)", sm, kcrit * fmd, format!("l_ef={} σ_m,crit={} λ_rel,m={} k_crit={}", f(lef), f(sc), f(rel), f(kcrit))),
        Outcome::info("sigma_m_crit_exact", "σ_m,crit from 6.31", "σ_m,crit aus 6.31", QuantityKind::Stress, sigma_m_crit_exact(s.b, s.h, lef, m.row.e_0_05) * 1e6, "π√(E I_z G I_tor)/(l_ef W_y)".into()),
    ];
    if let Some(nc) = c.opt("n_c") {
        let col = column(c, &m, &s)?;
        let u = (sm / (kcrit * fmd)).powi(2) + nc / s.a / (col.kcz * m.fd(m.row.f_c_0_k));
        out.push(Outcome::ratio("ltb_compression", "Bending and compression (6.35)", "Biegung und Druck (6.35)", u, 1.0, format!("(σ_m/(k_crit f_m))² + σ_c/(k_c,z f_c) = {}", f(u))));
    }
    Ok(out)
}

fn t23(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let mut out = t20(c)?;
    let col = column(c, &m, &s)?;
    let lef = c.opt("lef").unwrap_or_else(|| lef_ltb(c.o("l", 0.0), s.h, c.so("load_case", "udl"), c.so("load_pos", "centroid")));
    let rel = (m.row.f_m_k / sigma_m_crit_simplified(s.b, s.h, lef, m.row.e_0_05)).sqrt();
    let kcrit = if c.o("restrained", 0.0) > 0.5 { 1.0 } else { kcrit_factor(rel) };
    let fmd = m.kh_bending(c, c.opt("kh_dim").unwrap_or(s.h)) * m.fd(m.row.f_m_k);
    let fmz = m.kh_bending(c, c.opt("kh_dim_z").unwrap_or(s.b)) * m.fd(m.row.f_m_k);
    let u = (c.o("m_y", 0.0) / s.wy / (kcrit * fmd)).powi(2) + c.n("n_c")? / s.a / (col.kcz * m.fd(m.row.f_c_0_k)) + s.km(c) * c.o("m_z", 0.0) / s.wz / fmz;
    out.push(Outcome::ratio("biaxial_ltb", "Biaxial bending with lateral buckling and compression", "Zweiachsige Biegung mit Kippen und Druck", u, 1.0, format!("k_crit={} → {}", f(kcrit), f(u))));
    Ok(out)
}

fn t24(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let lef = c.n("lef_y")?;
    let n = c.n("n_c")?;
    let bow = lef / if m.row.product == TimberProduct::Solid { 300.0 } else { 500.0 };
    let ncr = PI * PI * m.row.e_0_05 * s.iy / (lef * lef);
    let amp = 1.0 / (1.0 - n / ncr).max(1e-6);
    let m2 = (c.o("m_y", 0.0) + n * bow) * amp;
    let sc = n / s.a;
    let fc = m.fd(m.row.f_c_0_k);
    let fm = m.kh_bending(c, c.opt("kh_dim").unwrap_or(s.h)) * m.fd(m.row.f_m_k);
    let u = (sc / fc).powi(2) + m2 / s.wy / fm;
    Ok(vec![
        Outcome::info("m_ii", "Second-order moment", "Moment nach Theorie II. Ordnung", QuantityKind::Moment, m2.abs() / 1e3, format!("e0={} mm, N_cr={} N, amplification {}", f(bow), f(ncr), f(amp))),
        Outcome::ratio("second_order", "Second-order resistance (6.19 without k_c)", "Nachweis nach Theorie II. Ordnung (6.19 ohne k_c)", if n >= ncr { 9.99 } else { u }, 1.0, format!("(σ_c/f_c)² + σ_m,II/f_m = {}", f(u))),
    ])
}

/// 🗂 Catalogue rows of the stability group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-20", part: "EN 1995-1-1", clause: "6.3.2", en: "Column buckling", de: "Knicken von Stäben", inputs: &["class", "n_c", "m_y", "m_z", "b", "h", "lef_y", "lef_z"], eval: t20 },
    Entry { id: "T-21", part: "EN 1995-1-1", clause: "6.3.2", en: "Compression with bending, governing axis", de: "Druck mit Biegung, maßgebende Achse", inputs: &["class", "n_c", "m_y", "m_z", "b", "h", "lef_y", "lef_z"], eval: t21 },
    Entry { id: "T-22", part: "EN 1995-1-1", clause: "6.3.3", en: "Lateral-torsional buckling", de: "Biegedrillknicken", inputs: &["class", "m_y", "b", "h", "l", "load_case", "load_pos", "lef", "restrained", "n_c"], eval: t22 },
    Entry { id: "T-23", part: "EN 1995-1-1", clause: "6.3.3", en: "Biaxial stability interaction", de: "Zweiachsige Stabilitätsinteraktion", inputs: &["class", "n_c", "m_y", "m_z", "b", "h", "lef_y", "lef_z", "l"], eval: t23 },
    Entry { id: "T-24", part: "EN 1995-1-1", clause: "5.4.4", en: "Second-order analysis with bow imperfection", de: "Berechnung nach Theorie II. Ordnung mit Vorkrümmung", inputs: &["class", "n_c", "m_y", "b", "h", "lef_y"], eval: t24 },
];
